use std::collections::HashMap;

use cpu_time::ProcessTime;
use rocket::serde::json::{Json, from_str};
use rocket::serde::Deserialize;

use super::utils::element_parser::{create_elements, parse_distribution};
use super::utils::capacity_calculator::calculate_capacity;
use crate::modeler::model::{Model, Results};
use crate::modeler::utils::round::round;

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
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
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

#[post("/simulate", format = "application/json", data = "<request>")]
pub fn simulate(request: Json<SimRequest>) -> Json<Results> {
    let time_start = ProcessTime::now();
    let data = request.into_inner();
    assert!(data.simtime > 0.0 && data.log_max_size > 0);

    let elements = create_elements(data.model);
    // print!("Modeling started!");
    let mut simdata = Model::new(elements, data.log_max_size as usize).simulate(data.simtime);
    // print!("Modeling finished!");
    simdata.total_time = round(time_start.elapsed().as_secs_f64(), 4).max(0.0001);

    // print!("{:#?}", simdata);
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

#[post("/capacity", data = "<request>")]
pub fn capacity(request: String) -> String {
    let data: LoadRequest = from_str(&request).unwrap();

    let capacity = calculate_capacity(
        data.deviation,
        parse_distribution(&data.dist).unwrap(),
        data.mean,
        data.replica,
    );

    capacity.to_string()
}
