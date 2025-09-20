use crate::modeler::components::element::Element;

pub fn in_act(e: &mut Element) {
    e.out_act();
}

pub fn out_act(e: &mut Element) {
    e.quantity += 1;
}

pub fn get_summary(e: &Element) -> String {
    format!(
        "\n
        ##### {} #####\n
        quantity = {}",
        e.name,
        e.quantity
    )
}
