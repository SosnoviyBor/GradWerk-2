mod modeler{
    pub mod components{
        pub mod create;
        pub mod element;
        pub mod process;
        // pub mod dispose;
    }
    pub mod utils {
        pub mod consts;
    }
    pub mod model;
}

fn main() {
    modeler::components::create::Create::new(1,1.0);
}
