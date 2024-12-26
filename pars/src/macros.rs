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
            fn process(&self, input: &str) -> String {
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
            fn from_args(_args: &[&String]) -> Self {
                Self
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
        if $dummy {Operations::$enum_option($enum_option::from_nothing())} else {Operations::$enum_option($enum_option::from_args($args))}
    };
}
