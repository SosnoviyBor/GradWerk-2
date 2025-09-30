use crate::modeler::components::element::Element;

pub fn in_act() {
    // nothing
}

pub fn out_act(e: &mut Element) {
    e.quantity += 1;
    e.put_tnext(e.tcurr + e.get_delay());
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
