mod modeler{
    pub mod components{
        pub mod create;
        pub mod element;
        pub mod process;
        pub mod dispose;
    }
    pub mod utils {
        pub mod consts;
        pub mod shortcuts;
        pub mod random;
    }
    pub mod model;
}

fn main() {
    modeler::components::element::Element::new(1,1.0, modeler::utils::consts::ElementType::Create);
    modeler::components::element::Element::new(1,1.0, modeler::utils::consts::ElementType::Process);
    modeler::components::element::Element::new(1,1.0, modeler::utils::consts::ElementType::Dispose);
}
