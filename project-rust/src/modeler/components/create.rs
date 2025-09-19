use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;

use crate::modeler::components::element::get_next_id;
use crate::modeler::utils::consts::DistributionType;
use crate::modeler::utils::consts::NextElementType;

pub struct Create {
    id: usize,
    name: String,
    worker_count: u32,

    tnext: BinaryHeap<OrderedFloat<f32>>,
    tcurr: f32,

    distribution: DistributionType,
    delay_mean: f32,
    delay_dev: f32,
    k: u32,

    next_element_type: NextElementType,
    next_elements: Vec<Create>,
    round_robin_idx: usize,

    state: u8,
    queue: u32,
    quantity: u32,
    average_load: f32,
    failure: u32,
}



impl Create {
    pub fn new(worker_count: u32, delay: f32) -> Self {
        let id = get_next_id();
        let mut element = Create {
            id,
            name: format!("Create{}", id),
            worker_count,
            tnext: BinaryHeap::new(),
            tcurr: 0.0,
            distribution: DistributionType::Exponential,
            delay_mean: delay,
            delay_dev: 0.0,
            k: 0,
            next_element_type: NextElementType::Random,
            next_elements: Vec::new(),
            round_robin_idx: 0,
            queue: 0,
            quantity: 0,
            average_load: 0.0,
            failure: 0,
            state: 0,
        };
        element.put_tnext(0.00001);
        element
    }

    fn get_delay(&self) -> f32 {
        match self.distribution {
            DistributionType::Exponential => {
                let u: f32 = rand::random();
                -self.delay_mean * u.ln()
            }
            DistributionType::Normal => {
                let u1: f32 = rand::random();
                let u2: f32 = rand::random();
                let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
                self.delay_mean + self.delay_dev * z0
            }
            DistributionType::Uniform => {
                let u: f32 = rand::random();
                self.delay_mean * (1.0 + self.delay_dev * (u - 0.5))
            }
            DistributionType::Erlang => {
                let mut product = 1.0;
                for _ in 0..self.k {
                    let u: f32 = rand::random();
                    product *= u;
                }
                -self.delay_mean / self.k as f32 * product.ln()
            }
            DistributionType::Constant => self.delay_mean,
        }
    }

    fn get_tnext(&mut self) -> f32 {
        self.tnext.pop().map(Into::into).unwrap_or(f32::INFINITY)
    }

    fn set_next_element_type(&mut self, next_type: NextElementType, next_elements: Vec<Create>) {
        self.next_element_type = next_type;
        self.next_elements = next_elements;
    }

    fn put_tnext(&mut self, t: f32) {
        self.tnext.push(OrderedFloat(t));
    }

    fn pop_tnext(&mut self) {
        self.tnext.pop();
    }

    fn in_act(&mut self) {
        // Create does not have in_act behavior
    }

    fn out_act(&mut self) {
        self.quantity += 1;
        self.put_tnext(self.tcurr + self.get_delay());
        match self.next_element_type {
            NextElementType::Random => {
                if !self.next_elements.is_empty() {
                    let rand_index = rand::random_range(0..self.next_elements.len());
                    self.next_elements[rand_index].in_act();
                }
            }
            NextElementType::RoundRobin => {
                if !self.next_elements.is_empty() {
                    if self.round_robin_idx == self.next_elements.len() {
                        self.round_robin_idx = 0;
                    }
                    let next_element = &mut self.next_elements[self.round_robin_idx];
                    next_element.in_act();
                    self.round_robin_idx += 1;
                }
            }
            NextElementType::Balanced => {
                if !self.next_elements.is_empty() {
                    let mut shortest_queue_id = 0;
                    let mut shortest_queue = u32::MAX;
                    for i in 0..self.next_elements.len() {
                        let next_element = &mut self.next_elements[i];
                        let free_queue = next_element.queue - next_element.state as u32;
                        if free_queue < shortest_queue {
                            shortest_queue = free_queue;
                            shortest_queue_id = i;
                        }
                    }
                    let next_element = &mut self.next_elements[shortest_queue_id];
                    next_element.in_act();
                }
            }
        }
        self.pop_tnext();
    }

    fn get_summary(&mut self) -> String {
        let nearest_tnext = self.get_tnext();
        let nearest_tnext_str;
        if nearest_tnext != f32::INFINITY {
            self.average_load = self.quantity as f32 / nearest_tnext;
            nearest_tnext_str = format!("{:.4}", nearest_tnext);
        } else {
            nearest_tnext_str = String::from("maxval")
        };
        String::from(format!(
            "\n##### {} #####\n
            quantity = {} | queue = {} | tnext = {} | average_load = {:.4}\n
            failure = {}",
            self.name,
            self.quantity,
            self.queue,
            nearest_tnext_str,
            self.average_load,
            self.failure
        ))
    }
}
