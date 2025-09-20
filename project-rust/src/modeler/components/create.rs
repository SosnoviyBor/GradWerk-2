use crate::modeler::components::element::Element;
use crate::modeler::utils::shortcuts;

pub fn in_act() {
    // nothing
}

pub fn out_act(e: &mut Element) {
    e.quantity += 1; // super().out_act() equivalent
    e.put_tnext(e.tcurr + e.get_delay());
    shortcuts::out_act(e);
    e.pop_tnext();
}

pub fn get_summary(e: &Element, nearest_tnext: String) -> String {
    format!(
        "\n
        ##### {} #####\n
        quantity = {} | queue = {} | tnext = {} | average_load = {:.4}\n
        failure = {}",
        e.name, e.quantity, e.queue, nearest_tnext, e.average_load, e.failure
    )
}
