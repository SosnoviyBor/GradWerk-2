use cpu_time::ProcessTime;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rocket::serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::Write;
use std::time::{Duration, Instant};
use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::modeler::components::element::Element;
use crate::modeler::utils::consts::{ElementType, NextElementType};
use crate::modeler::utils::round::round;

#[derive(Debug)]
pub struct Model {
    elements: Vec<Element>,
    rng: SmallRng,
    iters: u32,
    tnext: f64,
    tcurr: f64,
    log_first: Vec<String>,
    log_last: VecDeque<String>,
    log_max_size: usize,

    mem_peak: f64,  // in KB
    mem_total: f64, // in KB
    mem_samples: u32,
    sample_interval: Duration,
    last_sample: Instant,
    sys: System,
    pid: Pid,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Results {
    results: Vec<SimSummary>,
    log_first: Vec<String>,
    log_last: VecDeque<String>,
    iters: u32,
    sim_time: f64,       // in seconds
    pub total_time: f64, // in seconds
    iter_per_sec: f64,
    mem_peak: f64, // in MB
    mem_mean: f64, // in MB
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimSummary {
    element: Element,
    quantity: u32,
    failures: u32,
    mean_queue_len: f64,
    fail_prob: f64,
    wait_time: f64,
}

impl Model {
    pub fn new(elements: Vec<Element>, log_max_size: usize) -> Self {
        Model {
            elements,
            rng: SmallRng::seed_from_u64(0),
            iters: 0,
            tnext: 0.0,
            tcurr: 0.0,
            log_first: Vec::with_capacity(log_max_size),
            log_last: VecDeque::with_capacity(log_max_size + 1), // +1 to account for total sim results
            log_max_size,
            mem_peak: 0.0,
            mem_total: 0.0,
            mem_samples: 0,
            sample_interval: Duration::from_millis(500), // 0.5 seconds
            last_sample: Instant::now(),
            sys: System::new_all(),
            pid: Pid::from_u32(std::process::id()),
        }
    }

    pub fn simulate(&mut self, time: f64) -> Results {
        self.log_first.push(format!(
            "There are {} elements in the simulation",
            self.elements.len()
        ));

        let time_start = ProcessTime::now();
        self.mainloop(time);
        let time_elapsed = round(time_start.elapsed().as_secs_f64(), 4);

        self.log_sim_results();
        // trim extra newline
        self.log_last.get_mut(0).unwrap().remove(0);

        // print!("{:#?}", self.collect_sim_summary());
        Results {
            results: self.collect_sim_summary(),
            log_first: self.log_first.clone(),
            log_last: self.log_last.clone(),
            iters: self.iters,
            sim_time: time_elapsed,
            total_time: 0.0,
            iter_per_sec: round(self.iters as f64 / time_elapsed, 4),
            // in MB
            mem_peak: round(self.mem_peak / 1024.0, 4),
            mem_mean: round(self.mem_total / self.mem_samples as f64 / 1024.0, 4),
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
            for (i, e) in self.elements.iter().enumerate() {
                if e.get_tnext() != self.tcurr {
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
                self.elements[i].out_act(&mut self.rng);
            }
            for &i in &to_in_act {
                self.elements[i].in_act(&mut self.rng);
            }
            to_out_act.clear();
            to_in_act.clear();

            // logging
            self.iters += 1;
            self.log_event(event_id);
            self.update_mem_stats();
        }
    }

    fn pick_in_act_element(&self, out_e_id: &usize) -> Option<usize> {
        let e = &self.elements[*out_e_id];

        if e.next_elements.len() == 1 {
            return Some(e.next_elements[0]);
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
            self.iters, e.name, self.tnext
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

    fn update_mem_stats(&mut self) {
        if self.last_sample.elapsed() >= self.sample_interval {
            self.sys.refresh_processes(ProcessesToUpdate::All, true);
            if let Some(proc) = self.sys.process(self.pid) {
                let mem_curr = proc.memory() as f64 / 1024.0;
                self.mem_peak = self.mem_peak.max(mem_curr);
                self.mem_total += mem_curr;
                self.mem_samples += 1;
            }
            self.last_sample = Instant::now();
        }
    }
}
