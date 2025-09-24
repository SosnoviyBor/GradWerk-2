use std::collections::HashMap;

use crate::modeler::components::element::Element;
use crate::modeler::utils::consts::DistributionType;
use crate::modeler::utils::consts::ElementType;
use crate::modeler::utils::consts::NextElementType;
use crate::routers::simulator::{ElementInfo, IO};

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
            parse_next_element_type(&data.order),
            data.queuesize,
        );
        element.name = data.name.clone();
        elements_by_id.insert(id, element);
    }
    // chain elements together
    chain_elements(&model, &mut elements_by_id);
    elements_by_id.values().cloned().collect()
}

fn element_is_valid(element: &ElementInfo) -> bool {
    if element.inputs.is_empty() || element.outputs.is_empty() {
        true
    } else {
        has_connections(&element.inputs) && has_connections(&element.outputs)
    }
}

fn has_connections(io: &[IO]) -> bool {
    io.iter().any(|conn| !conn.connections.is_empty())
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

        let mut next_elements: Vec<Element> = vec![];
        let outputs = &element_info.outputs;

        for out_id in 1..outputs.len() + 1 {
            if let Some(out) = outputs.get(out_id) {
                for (_, connection) in &out.connections {
                    let element_id = format!("output_{}", connection.node);

                    if let Some(next_el) = readonly_elements_by_id.get(&element_id) {
                        next_elements.push(next_el.clone());
                    }
                }
            }
        }
        element.next_elements = next_elements;
    }
}
