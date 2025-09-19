use crate::modeler::components::element::Element;
use crate::modeler::utils::consts::NextElementType;

pub fn out_act(e: &mut impl Element) {
    match e.next_element_type {
        NextElementType::Random => {
            if !e.next_elements.is_empty() {
                let rand_index = rand::random::<usize>() % e.next_elements.len();
                let next_id = e.next_elements[rand_index];
                e.in_act();
            }
        }
        NextElementType::RoundRobin => {
            if !e.next_elements.is_empty() {
                let next_id = e.next_elements[e.round_robin_idx];
                e.round_robin_idx =
                    (e.round_robin_idx + 1) % e.next_elements.len();
                e.in_act();
            }
        }
        NextElementType::Balanced => {
            if !e.next_elements.is_empty() {
                let next_id = e.next_elements[0];
                e.next_elements.rotate_left(1);
                e.in_act();
            }
        }
    }
}