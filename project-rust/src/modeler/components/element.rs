use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::modeler::components::create;
use crate::modeler::components::dispose;
use crate::modeler::components::process;
use crate::modeler::utils::consts::{DistributionType, ElementType, NextElementType};
use crate::modeler::utils::random;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

fn get_next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

pub struct Element {
    pub id: usize,
    pub name: String,
    pub worker_count: u32,
    pub elem_type: ElementType,

    pub tnext: BinaryHeap<OrderedFloat<f32>>,
    pub tcurr: f32,

    pub distribution: DistributionType,
    pub delay_mean: f32,
    pub delay_dev: f32,
    pub k: u32,

    pub next_element_type: NextElementType,
    pub next_elements: Vec<Element>,
    pub round_robin_idx: usize,

    pub state: u32,
    pub queue: u32,
    pub quantity: u32,
    pub average_load: f32,

    // process-specific fields
    pub max_queue: u32,
    pub mean_queue: f32,
    pub wait_start: f32,
    pub wait_time: f32,
    pub failure: u32,
    pub state_sum: u32,
}

impl Element {
    pub fn new(worker_count: u32, delay: f32, elem_type: ElementType) -> Self {
        // prepare element data
        let id = get_next_id();
        let name;
        match elem_type {
            ElementType::Create => name = format!("Create{}", id),
            ElementType::Process => name = format!("Process{}", id),
            ElementType::Dispose => name = format!("Dispose{}", id),
        }

        let mut element = Element {
            id,
            name,
            worker_count,
            elem_type,
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
            state: 0,
            max_queue: 0,
            mean_queue: 0.0,
            wait_start: 0.0,
            wait_time: 0.0,
            failure: 0,
            state_sum: 0,
        };

        // initialize tnext based on element type
        match element.elem_type {
            ElementType::Create => element.put_tnext(0.00001),
            ElementType::Process => element.put_tnext(f32::INFINITY),
            ElementType::Dispose => element.put_tnext(f32::INFINITY),
        }

        element
    }

    pub fn get_delay(&self) -> f32 {
        match self.distribution {
            DistributionType::Exponential => random::exponential(self.delay_mean as f64) as f32,
            DistributionType::Normal => {
                random::normal(self.delay_mean as f64, self.delay_dev as f64) as f32
            }
            DistributionType::Uniform => random::uniform(
                self.delay_mean as f64 - self.delay_dev as f64,
                self.delay_mean as f64 + self.delay_dev as f64,
            ) as f32,
            DistributionType::Erlang => random::erlang(self.delay_mean as f64, self.k) as f32,
            DistributionType::Constant => self.delay_mean,
        }
    }

    pub fn get_tnext(&mut self) -> f32 {
        self.tnext.peek().map(|v| (*v).into()).unwrap_or(f32::INFINITY)
    }

    fn set_next_element_type(&mut self, next_type: NextElementType, next_elements: Vec<Element>) {
        self.next_element_type = next_type;
        self.next_elements = next_elements;
    }

    pub fn put_tnext(&mut self, t: f32) {
        self.tnext.push(OrderedFloat(t));
    }

    pub fn pop_tnext(&mut self) {
        self.tnext.pop();
    }

    pub fn in_act(&mut self) {
        match self.elem_type {
            ElementType::Create => create::in_act(),
            ElementType::Process => process::in_act(self),
            ElementType::Dispose => dispose::in_act(self),
        }
    }

    pub fn out_act(&mut self) {
        match self.elem_type {
            ElementType::Create => create::out_act(self),
            ElementType::Process => process::out_act(self),
            ElementType::Dispose => dispose::out_act(self),
        }
    }

    fn get_nearest_tnext(&mut self) -> String {
        let nearest_tnext = self.get_tnext();
        if nearest_tnext != f32::INFINITY {
            self.average_load = self.quantity as f32 / nearest_tnext;
            format!("{:.4}", nearest_tnext)
        } else {
            String::from("maxval")
        }
    }

    pub fn get_summary(&mut self) -> String {
        let nearest_tnext = self.get_nearest_tnext();
        match self.elem_type {
            ElementType::Create => create::get_summary(self, nearest_tnext),
            ElementType::Process => process::get_summary(self, nearest_tnext),
            ElementType::Dispose => dispose::get_summary(self),
        }
    }

    pub fn do_statistics(&mut self, delta: f32) {
        match self.elem_type {
            ElementType::Create => {},
            ElementType::Process => process::do_statistics(self, delta),
            ElementType::Dispose => {},
        }
    }
}
