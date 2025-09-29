mod modeler {
    pub mod components {
        pub mod create;
        pub mod dispose;
        pub mod element;
        pub mod process;
    }
    pub mod utils {
        pub mod consts;
        pub mod random;
        pub mod shortcuts;
    }
    pub mod model;
}
mod routers {
    pub mod utils {
        pub mod element_parser;
        pub mod load_calculator;
    }
    pub mod pages;
    pub mod simulator;
}

#[macro_use]
extern crate rocket;

use crate::routers::pages::{index, results};
use crate::routers::simulator::{load, simulate};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        // pages
        .mount("/", routes![index])
        .mount("/", routes![results])
        .mount("/", routes![simulate])
        .mount("/", routes![load])
        // everything else
        .mount("/static", FileServer::from(relative!("src/static")))
        .attach(Template::fairing())
}
