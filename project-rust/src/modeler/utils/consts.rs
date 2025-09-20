pub enum NextElementType {
    Balanced,
    RoundRobin,
    Random,
}

pub enum DistributionType {
    Exponential,
    Normal,
    Uniform,
    Erlang,
    Constant,
}

#[derive(PartialEq, Eq)]
pub enum ElementType {
    Create,
    Process,
    Dispose,
}