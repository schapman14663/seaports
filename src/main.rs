//use std::path::PathBuf;
use clap::{arg, Arg, ArgAction, Command};

fn main(){
    let m = Command::new("seaports")
        .version("0.0.1")
        .about("to be done")
        .arg(
            Arg::new("new")
            .short('n')
            .long("new")
            )
        .arg(
            Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue),
            )
        .get_matches();

    println!("verbose: {:?}", m.get_flag("verbose"));

    println!("Hello World, seaports command is functional");
}
