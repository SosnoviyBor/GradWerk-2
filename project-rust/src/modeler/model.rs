use std::collections::HashMap;

use crate::modeler::components::element::Element;
use crate::modeler::utils::consts::ElementType;

pub struct Model {
    pub elements: Vec<Element>,
    pub iteration: i32,
    pub tnext: f64,
    pub tcurr: f64,
    pub log: HashMap<String, Vec<String>>,
    pub log_max_size: usize,
}

impl Model {
    pub fn new(elements: Vec<Element>, log_max_size: usize) -> Self {
        Model {
            elements,
            iteration: -1,
            tnext: 0.0,
            tcurr: 0.0,
            log: HashMap::new(),
            log_max_size,
        }
    }

    pub fn simulate(&mut self, time: f64) -> ((), HashMap<String, Vec<String>>, i32) {
        self.iteration = 0;
        self.log.insert(
            String::from("first"),
            vec![format!(
                "There are {} elements in the simulation",
                self.elements.len()
            )],
        );
        self.log.insert(String::from("last"), vec![]);

        // init measurements
        // ram
        // time

        self.mainloop(time);

        // finalize measurments

        self.log_sim_results();
        // trim trailing newline
        // TODO might be broken?
        if let Some(first) = self.log.get_mut("last").and_then(|v| v.get_mut(0)) {
            first.pop();
        }
        return (
            self.collect_sim_summary(),
            self.log.clone(),
            // time
            // ram
            self.iteration,
        );
    }

    fn mainloop(&mut self, time: f64) {
        // thats it
        // thats the whole algorithm for ya
        while self.tcurr < time {
            // searching for the nearest event
            self.tnext = f64::INFINITY;
            let mut event_id = 0;
            for element in &mut self.elements {
                if element.get_tnext() < self.tnext as f32 {
                    self.tnext = element.get_tnext() as f64;
                    event_id = element.id;
                }
            }
            // update current time of each element + calculate some stats
            let tcurr_old = self.tcurr;
            self.tcurr = self.tnext;
            for element in &mut self.elements {
                element.do_statistics((self.tcurr - tcurr_old) as f32);
                element.tcurr = self.tcurr as f32;
            }
            // move things between relevant elements queues
            self.elements[event_id].out_act();
            for element in &mut self.elements {
                if element.get_tnext() == self.tcurr as f32 {
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
        if self.log["first"].len() <= self.log_max_size {
            self.log.get_mut("first").unwrap().push(msg);
        } else {
            self.log.get_mut("last").unwrap().push(msg);
            if self.log["last"].len() > self.log_max_size {
                self.log.get_mut("last").unwrap().remove(0);
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
                    element.failure as f32 / (element.failure + element.quantity) as f32
                } else {
                    0.0
                };
                msg.push_str(&format!(
                    "Mean length of queue = {:.4}\n
                    Failure probability = {:.4}\n",
                    element.mean_queue / self.tcurr as f32,
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
        self.log.get_mut("last").unwrap().push(msg);
    }

    // TODO when i figure out the return format
    fn collect_sim_summary(&self) {}
}
