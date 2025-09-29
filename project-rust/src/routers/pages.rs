use rocket::form::Form;
use rocket_dyn_templates::{Template, context};

use crate::modeler::model::Results;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[derive(FromForm)]
pub struct InputData {
    result: String,
}

#[post("/results", data = "<form_data>")]
pub fn results(form_data: Form<InputData>) -> Template {
    let data: Results = serde_json::from_str(&form_data.result).unwrap();
    Template::render("results", data)
}
