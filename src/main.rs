/// @file src/main.rs
mod act;
mod git;
mod paths;
use std::{env, string::String};

use crate::act::act_init;
use crate::git::get_git_repository;

fn help() -> String {
    return "
            usage: act [--flags]
            \t\t git-repo\n
            \t\t init
          "
    .to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 1 {
        let farg = &args[2];
        print!("{}", farg);
        match farg.as_str() {
            "git-repo" => {
                println!("{}", get_git_repository())
            }
            "init" => act_init(),
            &_ => todo!(),
        }
    }
    panic!("{}", help());
}
