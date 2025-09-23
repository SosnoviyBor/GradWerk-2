#[derive(Clone)]
pub enum NextElementType {
    Balanced,
    RoundRobin,
    Random,
}

#[derive(Clone)]
pub enum DistributionType {
    Exponential,
    Normal,
    Uniform,
    Erlang,
    Constant,
}

#[derive(PartialEq, Eq, Clone)]
pub enum ElementType {
    Create,
    Process,
    Dispose,
}