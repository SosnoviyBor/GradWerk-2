use peak_alloc::PeakAlloc;
use rocket::serde::Serialize;
use std::time::Instant;

use crate::modeler::components::element::Element;
use crate::modeler::utils::consts::ElementType;

static PEAK_ALLOC: PeakAlloc = PeakAlloc;

pub struct Model {
    pub elements: Vec<Element>,
    pub iteration: i32,
    pub tnext: f64,
    pub tcurr: f64,
    pub log_first: Vec<String>,
    pub log_last: Vec<String>,
    pub log_max_size: usize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Results {
    results: Vec<SimSummary>,
    log_first: Vec<String>,
    log_last: Vec<String>,
    time: u64,
    peak_mem: f32,
    interations: i32,
}

#[derive(Serialize)]
pub struct SimSummary {
    element: Element,
    quantity: u32,
    failures: i32,
    mean_queue_len: f64,
    fail_prob: f64,
}

impl Model {
    pub fn new(elements: Vec<Element>, log_max_size: usize) -> Self {
        Model {
            elements,
            iteration: -1,
            tnext: 0.0,
            tcurr: 0.0,
            log_first: vec![],
            log_last: vec![],
            log_max_size,
        }
    }

    pub fn simulate(&mut self, time: f64) -> Results {
        self.iteration = 0;
        self.log_first.push(format!(
            "There are {} elements in the simulation",
            self.elements.len()
        ));

        // init measurements
        let time_start = Instant::now();

        self.mainloop(time);

        // finalize measurments
        let time_elapsed = time_start.elapsed().as_secs();
        let peak_mem = PEAK_ALLOC.peak_usage_as_mb();

        self.log_sim_results();
        // trim trailing newline
        // TODO might be broken?
        // if let Some(first) = self.log.get_mut("last").and_then(|v| v.get_mut(0)) {
        //     first.pop();
        // }
        return Results {
            results: self.collect_sim_summary(),
            log_first: self.log_first.clone(),
            log_last: self.log_last.clone(),
            time: time_elapsed,
            peak_mem,
            interations: self.iteration,
        };
    }

    fn mainloop(&mut self, time: f64) {
        // thats it
        // thats the whole algorithm for ya
        while self.tcurr < time {
            // searching for the nearest event
            self.tnext = f64::INFINITY;
            let mut event_id = 0;
            for element in &mut self.elements {
                if element.get_tnext() < self.tnext as f64 {
                    self.tnext = element.get_tnext() as f64;
                    event_id = element.id;
                }
            }
            // update current time of each element + calculate some stats
            let tcurr_old = self.tcurr;
            self.tcurr = self.tnext;
            for element in &mut self.elements {
                element.do_statistics((self.tcurr - tcurr_old) as f64);
                element.tcurr = self.tcurr;
            }
            // move things between relevant elements queues
            self.elements[event_id].out_act();
            for element in &mut self.elements {
                if element.get_tnext() == self.tcurr as f64 {
                    element.out_act();
                }
            }
            // logging
            self.iteration += 1;
            self.log_event(event_id);
        }
    }

    fn log_event(&mut self, event_id: usize) {
        // generate message
        let mut msg = format!(
            "\n
            >>>     Event #{} in {}     <<<\n
            >>>     time: {:.4}     <<<",
            self.iteration, self.elements[event_id].name, self.tnext
        );
        for element in &mut self.elements {
            msg.push_str(&element.get_summary());
        }
        // update log
        if self.log_first.len() <= self.log_max_size {
            self.log_first.push(msg);
        } else {
            self.log_last.push(msg);
            if self.log_last.len() > self.log_max_size {
                self.log_last.remove(0);
            }
        }
    }

    fn log_sim_results(&mut self) {
        let mut msg = String::from("\n-------------RESULTS-------------\n");

        for element in &mut self.elements {
            msg.push_str(&format!(
                "##### {} #####\n
                quantity = {}\n",
                element.name, element.quantity
            ));

            if element.elem_type == ElementType::Process {
                let failure_prob = if element.quantity + element.failure != 0 {
                    element.failure as f64 / (element.failure + element.quantity) as f64
                } else {
                    0.0
                };
                msg.push_str(&format!(
                    "Mean length of queue = {:.4}\n
                    Failure probability = {:.4}\n",
                    element.mean_queue / self.tcurr,
                    failure_prob
                ));
            }
            msg.push_str("\n");
        }

        msg.pop();
        msg.push_str(
            "---------------------------------\n
            Simulation is done successfully!",
        );
        self.log_last.push(msg);
    }

    fn collect_sim_summary(&self) -> Vec<SimSummary> {
        let mut summary: Vec<SimSummary> = vec![];
        for e in &self.elements {
            summary.push(match e.elem_type {
                ElementType::Create | ElementType::Dispose => SimSummary {
                    element: e.clone(),
                    quantity: e.quantity,
                    failures: -1,
                    mean_queue_len: -1.0,
                    fail_prob: -1.0,
                },
                ElementType::Process => SimSummary {
                    element: e.clone(),
                    quantity: e.quantity,
                    failures: e.failure as i32,
                    mean_queue_len: e.mean_queue / self.tcurr,
                    fail_prob: if e.failure + e.quantity != 0 {
                        (e.failure / (e.failure + e.quantity)) as f64
                    } else {
                        0.0
                    },
                },
            });
        }
        summary
    }
}
