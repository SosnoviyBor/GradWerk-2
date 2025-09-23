use crate::modeler::components::element::Element;

pub fn create_elements(data: &str) -> Vec<Element> {
    let mut elements: Vec<Element> = vec![];
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }
        match Element::from_str(line) {
            Ok(element) => elements.push(element),
            Err(e) => eprintln!("Error parsing line '{}': {}", line, e),
        }
    }
    elements
}

pub fn parse_distribution(dist_name: &str) -> Option<crate::modeler::utils::consts::DistributionType> {
    match dist_name.to_lowercase().as_str() {
        "exponential" => Some(crate::modeler::utils::consts::DistributionType::Exponential),
        "normal" => Some(crate::modeler::utils::consts::DistributionType::Normal),
        "uniform" => Some(crate::modeler::utils::consts::DistributionType::Uniform),
        "erlang" => Some(crate::modeler::utils::consts::DistributionType::Erlang),
        "constant" => Some(crate::modeler::utils::consts::DistributionType::Constant),
        _ => None,
    }
}