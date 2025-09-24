use crate::modeler::components::element::Element;
use crate::modeler::utils::consts::NextElementType;

pub fn out_act(e: &mut Element) {
    match e.next_element_type.as_ref().unwrap() {
        NextElementType::Random => {
            if !e.next_elements.is_empty() {
                let rand_index = rand::random::<u32>() % e.next_elements.len() as u32;
                e.next_elements[rand_index as usize].in_act();
            }
        }
        NextElementType::RoundRobin => {
            if !e.next_elements.is_empty() {
                if e.round_robin_idx == e.next_elements.len() {
                    e.round_robin_idx = 0;
                }
                e.next_elements[e.round_robin_idx].in_act();
                e.round_robin_idx += 1;
            }
        }
        NextElementType::Balanced => {
            if !e.next_elements.is_empty() {
                let mut min_queue_idx = 0;
                let mut min_queue = i32::MAX;
                for (i, next_elem) in e.next_elements.iter().enumerate() {
                    let free_queue = next_elem.queue as i32 - next_elem.state as i32;
                    if free_queue < min_queue {
                        min_queue = free_queue;
                        min_queue_idx = i;
                    }
                }
                e.next_elements[min_queue_idx].in_act();
            }
        }
    }
}