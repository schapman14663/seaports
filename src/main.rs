//use std::path::PathBuf;
use clap::{arg, Command};

fn main(){
    let _m = Command::new("seaports")
        .version("0.0.1")
        .about("to be done")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
            .about("Create New Project")
            .arg(arg!(<LANGUAGE> "The main language to use in the new project"))
            .arg_required_else_help(true),
            );

    println!("Hello World, seaports command is functional");
}
