#![no_std]
#![no_main]
extern crate alloc;
pub mod types;
pub mod reader;
pub mod printer;
pub mod symbols;
pub mod eval;
pub mod env;
pub mod nekocore;
use alloc::vec::Vec;
use alloc::string::String;
use crate::env::Env;

pub fn rep_str(s:&str,env:Env) -> Vec<String> {
    let strs = reader::pre_read_str(&s,env.clone());
    let mut results:Vec<String> = Vec::new();
    //println!("{:?}",&strs);
    for st in strs {
        let s = rep(st.as_str(),env.clone());
        results.push(s);
    }
    return results;
}

pub fn rep(s:&str,env:Env) -> String {
    let mut n = reader::read_str(s,env.clone());
    n = eval::eval(n,env.clone());
    return printer::pr_str(n);
}
