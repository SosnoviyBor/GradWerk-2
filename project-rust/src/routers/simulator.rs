use std::collections::HashMap;

use rocket::serde::json::{Json, from_str};
use rocket::serde::Deserialize;

use super::utils::element_parser::{create_elements, parse_distribution};
use super::utils::load_calculator::calculate_load;
use crate::modeler::model::{Model, Results};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SimRequest {
    model: HashMap<String, ElementInfo>,
    simtime: f64,
    log_max_size: u64,
}

#[derive(Deserialize)]
pub struct ElementInfo {
    pub class: String,
    pub data: ElementData,
    html: String,
    id: usize,
    pub inputs: Vec<IO>,
    name: String,
    pub outputs: Vec<IO>,
    pos_x: f64,
    pos_y: f64,
    typenode: bool,
}

#[derive(Deserialize)]
pub struct ElementData {
    pub deviation: f64,
    pub dist: String,
    pub mean: f64,
    pub name: String,
    pub order: String,
    pub queuesize: u32,
    pub replica: u32,
}

#[derive(Deserialize)]
pub struct IO {
    pub connections: HashMap<String, Connection>,
}

#[derive(Deserialize)]
pub struct Connection {
    pub node: usize,
    input: String,
}

#[post("/simulate", format = "application/json", data = "<request>")]
pub fn simulate(request: Json<SimRequest>) -> Json<Results> {
    let data = request.into_inner();
    assert!(data.simtime > 0.0 && data.log_max_size > 0);

    let elements = create_elements(data.model);
    print!("Modeling started!");
    let simdata = Model::new(elements, data.log_max_size as usize).simulate(data.simtime);
    print!("Modeling finished!");

    Json(simdata)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoadRequest {
    deviation: f64,
    dist: String,
    mean: f64,
    replica: u32,
}

#[post("/load", data = "<request>")]
pub fn load(request: String) -> String {
    let data: LoadRequest = from_str(&request).unwrap();

    let load = calculate_load(
        data.deviation,
        parse_distribution(&data.dist).unwrap(),
        data.mean,
        data.replica,
    );

    load.to_string()
}
