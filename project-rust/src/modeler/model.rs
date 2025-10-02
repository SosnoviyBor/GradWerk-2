use peak_alloc::PeakAlloc;
use rocket::serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::Write;
use std::time::Instant;

use crate::modeler::components::element::{Element, reset_next_id};
use crate::modeler::utils::consts::{ElementType, NextElementType};

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

#[derive(Debug)]
pub struct Model {
    pub elements: Vec<Element>,
    pub iteration: i32,
    pub tnext: f64,
    pub tcurr: f64,
    pub log_first: Vec<String>,
    pub log_last: VecDeque<String>,
    pub log_max_size: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Results {
    pub results: Vec<SimSummary>,
    pub log_first: Vec<String>,
    pub log_last: VecDeque<String>,
    pub time: f64,
    pub peak_mem: f64,
    pub iterations: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimSummary {
    pub element: Element,
    pub quantity: u32,
    pub failures: u32,
    pub mean_queue_len: f64,
    pub fail_prob: f64,
    pub wait_time: f64,
}

impl Model {
    pub fn new(elements: Vec<Element>, log_max_size: usize) -> Self {
        Model {
            elements,
            iteration: -1,
            tnext: 0.0,
            tcurr: 0.0,
            log_first: Vec::with_capacity(log_max_size),
            log_last: VecDeque::with_capacity(log_max_size + 1), // +1 to account for total sim results
            log_max_size,
        }
    }

    pub fn simulate(&mut self, time: f64) -> Results {
        reset_next_id();

        self.iteration = 0;
        self.log_first.push(format!(
            "There are {} elements in the simulation",
            self.elements.len()
        ));

        // init measurements
        PEAK_ALLOC.reset_peak_usage();
        let time_start = Instant::now();

        self.mainloop(time);

        // finalize measurments
        let time_elapsed = time_start.elapsed().as_secs_f64();
        let peak_mem = PEAK_ALLOC.peak_usage_as_mb() as f64;

        self.log_sim_results();
        // trim extra newline
        self.log_last.get_mut(0).unwrap().remove(0);
        
        // print!("{:#?}", self.collect_sim_summary());
        Results {
            results: self.collect_sim_summary(),
            log_first: self.log_first.clone(),
            log_last: self.log_last.clone(),
            time: time_elapsed,
            peak_mem,
            iterations: self.iteration,
        }
    }

    fn mainloop(&mut self, time: f64) {
        // print!("{:#?}", self.elements);
        // preallocate memory for the vectors
        let mut to_out_act = Vec::with_capacity(self.elements.len());
        let mut to_in_act = Vec::with_capacity(self.elements.len());
        // thats it
        // thats the whole algorithm for ya
        while self.tcurr < time {
            // find next event
            self.tnext = f64::INFINITY;
            let mut event_id = 0;
            for e in &self.elements {
                let tnext_elem = e.get_tnext();
                if tnext_elem < self.tnext {
                    self.tnext = tnext_elem;
                    event_id = e.id;
                }
            }

            let tcurr_old = self.tcurr;
            self.tcurr = self.tnext;

            // update statistics
            for e in &mut self.elements {
                e.do_statistics(self.tcurr - tcurr_old);
                e.tcurr = self.tcurr;
            }

            // move things between relevant elements queues
            to_out_act.clear();
            to_in_act.clear();
            for (i, e) in self.elements.iter().enumerate() {
                if e.get_tnext() == self.tcurr {
                    continue;
                }
                // collect out_act elements
                to_out_act.push(i);
                // collect in_act elements
                if let Some(in_id) = self.pick_in_act_element(&i) {
                    to_in_act.push(in_id);
                }
            }
            // run actions
            for &i in &to_out_act {
                self.elements[i].out_act();
            }
            for &i in &to_in_act {
                self.elements[i].in_act();
            }

            // logging
            self.iteration += 1;
            self.log_event(event_id);
        }
    }

    fn pick_in_act_element(&self, out_e_id: &usize) -> Option<usize> {
        let e = &self.elements[*out_e_id];

        if e.next_elements.len() == 1 {
            return Some(e.next_elements[0])
        }

        match &e.next_element_type {
            NextElementType::Random => {
                if !e.next_elements.is_empty() {
                    let rand_index = rand::random::<u32>() % e.next_elements.len() as u32;
                    Some(rand_index as usize)
                } else {
                    None
                }
            }
            NextElementType::RoundRobin(idx) => {
                if !e.next_elements.is_empty() {
                    if idx.get() == e.next_elements.len() {
                        idx.set(0);
                    }
                    idx.set(idx.get() + 1);
                    Some(idx.get())
                } else {
                    None
                }
            }
            NextElementType::Balanced => {
                if !e.next_elements.is_empty() {
                    let next_elements_copy = e.next_elements.clone();
                    let mut min_queue_idx = 0;
                    let mut min_queue = u32::MAX;
                    for i in next_elements_copy.iter() {
                        let next_elem = self.elements.iter().find(|e| e.id == *i).unwrap();
                        let free_queue = next_elem.queue - next_elem.state;
                        if free_queue < min_queue {
                            min_queue = free_queue;
                            min_queue_idx = *i;
                        }
                    }
                    Some(min_queue_idx)
                } else {
                    None
                }
            }
        }
    }

    fn log_event(&mut self, event_id: usize) {
        let e = &self.elements[event_id];
        let mut msg = String::with_capacity(512 + self.elements.len() * 128);

        // header
        write!(
            &mut msg,
            "\n
            >>>     Event #{} in {}     <<<\n
            >>>     time: {:.4}     <<<\n\n",
            self.iteration, e.name, self.tnext
        )
        .unwrap();

        // element summaries
        for e in &mut self.elements {
            msg.push_str(&e.get_summary());
        }

        if self.log_first.len() <= self.log_max_size {
            self.log_first.push(msg);
        } else {
            if self.log_last.len() == self.log_max_size {
                self.log_last.pop_front();
            }
            self.log_last.push_back(msg);
        }
    }

    fn log_sim_results(&mut self) {
        // estimate capacity: small over-allocation to avoid reallocations
        let mut msg = String::with_capacity(1024 + self.elements.len() * 128);

        msg.push_str("\n\n-------------RESULTS-------------\n\n");

        for e in &self.elements {
            msg.push_str("##### ");
            msg.push_str(&e.name);
            msg.push_str(
                " #####\n
                quantity = ",
            );
            msg.push_str(&e.quantity.to_string());
            msg.push('\n');

            if e.elem_type == ElementType::Process {
                let failure_prob = if e.quantity + e.failure != 0 {
                    e.failure as f64 / (e.failure + e.quantity) as f64
                } else {
                    0.0
                };

                msg.push_str(&format!(
                    "\n
                    Mean length of queue = {:.4}\n
                    Failure probability = {:.4}\n
                    Total time stalling = {:.4}\n",
                    e.mean_queue / self.tcurr,
                    failure_prob,
                    e.wait_time
                ));
            }

            msg.push('\n'); // separate elements
        }

        msg.push_str(
            "---------------------------------\n
            Simulation is done successfully!",
        );

        self.log_last.push_back(msg);
    }

    fn collect_sim_summary(&self) -> Vec<SimSummary> {
        self.elements
            .iter()
            .map(|e| match e.elem_type {
                ElementType::Create | ElementType::Dispose => SimSummary {
                    element: e.clone(),
                    quantity: e.quantity,
                    failures: 0,
                    mean_queue_len: 0.0,
                    fail_prob: 0.0,
                    wait_time: 0.0,
                },
                ElementType::Process => SimSummary {
                    element: e.clone(),
                    quantity: e.quantity,
                    failures: e.failure,
                    mean_queue_len: e.mean_queue / self.tcurr,
                    fail_prob: if e.failure + e.quantity != 0 {
                        (e.failure as f64) / ((e.failure + e.quantity) as f64)
                    } else {
                        0.0
                    },
                    wait_time: e.wait_time,
                },
            })
            .collect()
    }
}
