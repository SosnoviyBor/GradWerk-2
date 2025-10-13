use std::cell::Cell;

use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NextElementType {
    Balanced,
    RoundRobin(Cell<usize>),
    Random,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
