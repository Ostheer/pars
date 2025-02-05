use crate::common::{Operation, CreateOperation};
use crate::impl_createoperation_default;

// Upper
pub struct Upper;

impl_createoperation_default!(Upper);

impl Operation for Upper {
    fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

// Lower
pub struct Lower;

impl_createoperation_default!(Lower);

impl Operation for Lower {
    fn process(&self, input: &str) -> String {
        input.to_lowercase()
    }
}

// Split
pub struct Split {
    sep: String,
    index: usize,
}

impl CreateOperation for Split {
    fn from_args(args: &[&String]) -> Result<Self, String> {
        let sep = args[0];
        let index = args[1].parse::<usize>();
        if index.is_err() {
            return Err("Second argument to Split must be an integer.".to_string())
        }
        
        Ok(
            Self {
                sep: sep.to_string(),
                index: index.unwrap(),
            }
        )
    }

    fn from_nothing() -> Self {
        Self {
            sep: "".to_string(),
            index: 0,
        }
    }
}

impl Operation for Split {
    fn process(&self, input: &str) -> String {
        input
            .split(&self.sep)
            .nth(self.index)
            .expect("Out of bounds")
            .to_string()
    }

    fn num_args(&self) -> usize {
        2
    }
}

// Replace
pub struct Replace {
    old: String,
    new: String,
}

impl CreateOperation for Replace {
    fn from_args(args: &[&String]) -> Result<Self, String> {
        Ok(
            Self {
                old: args[0].to_string(),
                new: args[1].to_string()
            }
        )
    }

    fn from_nothing() -> Self {
        Self {
            old: "".to_string(),
            new: "".to_string()
        }
    }
}

impl Operation for Replace {
    fn process(&self, input: &str) -> String {
        input.replace(&self.old, &self.new)
    }

    fn num_args(&self) -> usize {
        2
    }
}

// Replacen
pub struct Replacen {
    replace: Replace,
    n: usize
}

impl CreateOperation for Replacen {
    fn from_args(args: &[&String]) -> Result<Self, String> {
        let n = args[2].parse::<usize>();
        if n.is_err() {
            return Err("Argument must be integer".to_string())
        }

        let replace = Replace::from_args(&args[0..2]);
        if replace.is_err() {
            return Err(replace.err().unwrap())
        }

        Ok(
            Self {
                replace: replace.unwrap(),
                n: n.unwrap()
            }
        )
    }

    fn from_nothing() -> Self {
        Self {
            replace: Replace::from_nothing(),
            n: 0
        }
    }
}

impl Operation for Replacen {
    fn process(&self, input: &str) -> String {
        input.replacen(&self.replace.old, &self.replace.new, self.n)
    }

    fn num_args(&self) -> usize {
        3  // 2 + 1
    }
}
