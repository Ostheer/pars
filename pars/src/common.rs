pub trait Operation {
    fn process(&self, input: &str) -> String;
    fn num_args(&self) -> usize {
        0
    }}

pub trait CreateOperation {
    fn from_args(args: &[&String]) -> Self;
    fn from_nothing() -> Self;
}
