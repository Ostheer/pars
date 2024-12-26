pub mod common;
pub mod opdefs;
pub mod macros;

use core::panic;
use std::io;
use std::env;
use std::iter::zip;

use crate::common::{Operation, CreateOperation};
use crate::opdefs::{Upper, Lower, Split, Replace, Replacen};

declare_operations_enum!(Operations, Upper, Lower, Split, Replace, Replacen);
impl_operation_for_enum!(Operations, Upper, Lower, Split, Replace, Replacen);

fn _create_op(op_spec: &str, dummy: bool, args: &Vec<&String>) -> impl Operation {
    match op_spec {
        "upper" => create_op_matchcase!(Upper, dummy, args),
        "lower" => create_op_matchcase!(Lower, dummy, args),
        "split" => create_op_matchcase!(Split, dummy, args),
        "replace" => create_op_matchcase!(Replace, dummy, args),
        "replacen" => create_op_matchcase!(Replacen, dummy, args),
        _ => panic!("Unknown operation: {}", op_spec)
    }
}
fn get_num_args(op_spec: &str) -> usize {
    _create_op(op_spec, true,&Vec::new()).num_args()
}
fn get_operation(op_spec: &str, args: &Vec<&String>) -> impl Operation {
    _create_op(op_spec, false, args)
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut i: usize = 1;
    let mut op_args: Vec<Vec<&String>> = Vec::new();
    let mut op_specs: Vec<&str> = Vec::new();
    loop {
        let os = &**args.get(i).expect("Expected an operation type here.");
        let num_args = get_num_args(os);
        let mut ta:  Vec<&String> = Vec::new();
        for i_arg in i+1..i+1+num_args {
            let arg = args.get(i_arg).expect(&format!("Operation {os} requires {num_args} arguments"));
            ta.push(arg);
        }
        i = i + 1 + num_args;
        op_args.push(ta);
        op_specs.push(os);
        if i == args.len() {
            break
        }
    }
    println!("{op_args:?}");

    let stdin = io::stdin();
    for line in stdin.lines(){
        match line {
            Ok(text) => process_line(text, &op_specs, &op_args),
            Err(_e) => return
        }
    }
}


fn process_line(mut line: String, op_specs: &Vec<&str>, op_args: &Vec<Vec<&String>>) {
    for (os, oa) in zip(op_specs, op_args) {
        let op = get_operation(os, &oa);
        line = op.process(&line);
        
    }
    println!("{line}");
}
