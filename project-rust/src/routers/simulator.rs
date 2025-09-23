use rocket::serde::{Deserialize, json::Json};
use rocket_dyn_templates::{context, Template};

use super::utils::element_parser::{create_elements, parse_distribution};
use super::utils::load_calculator::calculate_load;
use crate::modeler::model::Model;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SimRequest {
    data: SimRequestBody,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SimRequestBody {
    model: String,
    simtime: f64,
    log_max_size: u64,
}

#[post("/simulate", data = "<request>")]
pub fn simulate(request: Json<SimRequest>) -> Template {
    let body = request.into_inner();
    let model = body.data.model;
    let simtime = body.data.simtime;
    let log_max_size = body.data.log_max_size;
    assert!(simtime > 0.0 && log_max_size > 0);

    let elements = create_elements(&model);
    print!("Modeling started!");
    let simdata = Model::new(elements, log_max_size as usize).simulate(simtime);
    print!("Modeling finished!");

    Template::render("index", context! {
        results: simdata.0,
        log: simdata.1,
        time: simdata.2,
        // memory: simdata.3,
        // iterations: simdata.4,
    })
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")] // important: tells serde to use Rocket's re-export
struct LoadRequest {
    data: LoadRequestBody,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoadRequestBody {
    deviation: f64,
    distribution: String,
    mean: f64,
    replica: u32,
}

#[post("/load", data = "<request>")]
pub fn load(request: Json<LoadRequest>) -> String {
    let body: LoadRequest = request.into_inner(); // take ownership of parsed struct

    let load = calculate_load(
        body.data.deviation,
        parse_distribution(&body.data.distribution).unwrap(),
        body.data.mean,
        body.data.replica,
    );

    format!("{}", load)
}
