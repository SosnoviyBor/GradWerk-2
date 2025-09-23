use rocket::serde::json::{Json, Value};
use rocket_dyn_templates::{context, Template};

// use super::utils::result_decoder::decode;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[post("/results", data = "<body>")]
pub fn results(body: Json<Value>) -> Template {
    // let data = decode(body);
    print!("Received body: {:?}", body);

    Template::render("results", &*body)
}