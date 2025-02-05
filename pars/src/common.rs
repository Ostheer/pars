pub trait Operation {
    fn process(&self, input: &str) -> Result<String, String>;
    fn num_args(&self) -> usize {
        0
    }
}

pub trait CreateOperation {
    fn from_args(args: &[&String]) -> Result<Self, String>
    where
        Self: Sized; // stating this constraint here makes it explicit that the from_args method cannot be used on the trait object iself (only on concrete types implementing it). This is required because Result requires its two variants to be sized (i.e. implement the Sized trait).
    fn from_nothing() -> Self;
}
