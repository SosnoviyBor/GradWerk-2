use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NextElementType {
    Balanced,
    RoundRobin,
    Random,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DistributionType {
    Exponential,
    Normal,
    Uniform,
    Erlang,
    Constant,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum ElementType {
    Create,
    Process,
    Dispose,
}
