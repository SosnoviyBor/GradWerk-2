use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum NextElementType {
    Balanced,
    RoundRobin,
    Random,
}

#[derive(Clone, Serialize)]
pub enum DistributionType {
    Exponential,
    Normal,
    Uniform,
    Erlang,
    Constant,
}

#[derive(PartialEq, Eq, Clone, Serialize)]
pub enum ElementType {
    Create,
    Process,
    Dispose,
}