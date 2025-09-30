use std::collections::HashMap;

use crate::modeler::components::element::Element;
use crate::modeler::utils::consts::DistributionType;
use crate::modeler::utils::consts::ElementType;
use crate::modeler::utils::consts::NextElementType;
use crate::routers::simulator::ElementInfo;

pub fn create_elements(model: HashMap<String, ElementInfo>) -> Vec<Element> {
    let mut elements_by_id = HashMap::new();
    // initialize elements
    for (id, element_info) in &model {
        if !element_is_valid(&element_info) {
            continue;
        }
        let data = &element_info.data;
        let mut element = Element::new(
            data.replica,
            data.mean,
            data.deviation,
            parse_element_type(&element_info.class).unwrap(),
            parse_distribution(&data.dist).unwrap(),
            parse_next_element_type(&data.order).unwrap(),
            data.queuesize,
        );
        element.name = data.name.clone();
        elements_by_id.insert(id, element);
    }
    // chain elements together
    chain_elements(&model, &mut elements_by_id);
    let mut elements: Vec<Element> = elements_by_id.values().cloned().collect();
    // they need to be in order (ascending)
    elements.sort_by(|a, b| a.id.cmp(&b.id));
    elements
}

fn element_is_valid(element: &ElementInfo) -> bool {
    if element.inputs.is_empty() || element.outputs.is_empty() {
        true
    } else {
        element.inputs.len() > 0 && element.outputs.len() > 0
    }
}

pub fn parse_distribution(dist_name: &str) -> Option<DistributionType> {
    match dist_name.to_lowercase().as_str() {
        "exponential" => Some(DistributionType::Exponential),
        "normal" => Some(DistributionType::Normal),
        "uniform" => Some(DistributionType::Uniform),
        "erlang" => Some(DistributionType::Erlang),
        "constant" => Some(DistributionType::Constant),
        _ => None,
    }
}

pub fn parse_element_type(et_name: &str) -> Option<ElementType> {
    match et_name.to_lowercase().as_str() {
        "create" => Some(ElementType::Create),
        "process" => Some(ElementType::Process),
        "dispose" => Some(ElementType::Dispose),
        _ => None,
    }
}

pub fn parse_next_element_type(net_name: &str) -> Option<NextElementType> {
    match net_name.to_lowercase().as_str() {
        "balanced" => Some(NextElementType::Balanced),
        "round robin" => Some(NextElementType::RoundRobin),
        "random" => Some(NextElementType::Random),
        _ => None,
    }
}

fn chain_elements(
    model: &HashMap<String, ElementInfo>,
    elements_by_id: &mut HashMap<&String, Element>,
) {
    // required to not crossrefrence the og elements_by_id
    let readonly_elements_by_id = elements_by_id.clone();
    // from output to input
    for (id, element_info) in model {
        if !element_is_valid(element_info) {
            continue;
        }

        let element = match elements_by_id.get_mut(id) {
            Some(el) => el,
            None => continue,
        };

        // dispose doesn't have an output
        if element.elem_type == ElementType::Dispose {
            continue;
        }

        element.next_elements = element_info
            .outputs
            .iter()
            .map(|out_id| readonly_elements_by_id.get(&out_id.to_string()).unwrap().id)
            .collect();
    }
}
