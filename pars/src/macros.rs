#[macro_export]
macro_rules! declare_operations_enum {
    ($enum_name:ident, $($variant:ident),*) => {
        enum $enum_name {
            $(
                $variant($variant),
            )*
        }
    };
}

#[macro_export]
macro_rules! impl_operation_for_enum {
    ($enum_name:ident, $($variant:ident),*) => {
        impl Operation for $enum_name {
            fn process(&self, input: &str) -> Result<String, String> {
                match self {
                    $(Self::$variant(op) => op.process(input),)*
                }
            }

            fn num_args(&self) -> usize {
                match self {
                    $(Self::$variant(op) => op.num_args(),)*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_createoperation_default {
    ($struct_name:ident) => {
        impl CreateOperation for $struct_name {
            fn from_args(_args: &[&String]) -> Result<Self, String> {
                Ok(Self)
            }
        
            fn from_nothing() -> Self {
                Self
            }
        }
    };
}

#[macro_export]
macro_rules! create_op_matchcase {
    ($enum_option:ident, $dummy:ident, $args:ident) => {
        if $dummy {
            Ok(Operations::$enum_option($enum_option::from_nothing()))
        }
        else {
            let e_o = $enum_option::from_args($args);
            if e_o.is_err() {
                return Err(e_o.err().unwrap())
            }
            Ok(Operations::$enum_option(e_o.unwrap()))
        }
    };
}
