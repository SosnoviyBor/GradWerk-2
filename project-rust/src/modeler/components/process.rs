use crate::modeler::components::element::Element;
use crate::modeler::utils::shortcuts;

pub fn in_act(e: &mut Element) {
    if e.state < e.worker_count {
        e.state += 1;
        e.put_tnext(e.tcurr + e.get_delay());
    } else if e.queue > 0 {
        e.queue += 1;
    } else {
        e.failure += 1;
    }
}

pub fn out_act(e: &mut Element) {
    e.quantity += 1;
    e.state -= 1;
    if e.queue > 0 {
        e.queue -= 1;
        e.state += 1;
        e.put_tnext(e.tcurr + e.get_delay());
    } else {
        e.wait_start = e.tcurr;
    }
    shortcuts::out_act(e);
    e.pop_tnext();
}

pub fn get_summary(e: &Element, nearest_tnext: String) -> String {
    format!(
        "\n
        ##### {} #####\n
        state = {} | quantity = {} | queue = {} | tnext = {} | average_load = {:.4}\n
        failure = {}",
        e.name, e.state, e.quantity, e.queue, nearest_tnext, e.average_load, e.failure
    )
}

pub fn do_statistics(e: &mut Element, delta: f64) {
    e.state_sum += e.state as u32 * delta as u32;
    e.mean_queue += e.queue as f64 * delta;
}