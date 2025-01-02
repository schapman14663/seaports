use std::path::PathBuf;
use clap::{arg, Arg, ArgAction, Command};

fn main(){
    let m = Command::new("seaports")
        .version("0.0.1")
        .about("to be done")
        .subcommand(
            Command::new("new")
            .about("Create New Project")
            .arg_required_else_help(true)
            //argument that indicates the user wants to immediately change into the indicated
            //directory of the new project
            .arg(
                Arg::new("chase")
                .long("chase")
                .aliases(["cd", "zox"])
                .help("change into the project diretory when done")
                .action(ArgAction::SetTrue)
                )
            //argument that indicates the user wants to immediately open the project in their text
            //editor [USE ENVIRONMENT VARIABLE FOR EDITOR] (chases into the directory by default)
            .arg(
                Arg::new("edit")
                .long("edit")
                .aliases(["nano", "vim", "nvim", "code"])
                .help("open text editor for project when done")
                .action(ArgAction::SetTrue)
                )
            //argument that indicates the user wants the terminal to print the creation of each
            //file for the project

            .arg(
                Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("print each step of the project folder creation process")
            )
        )
        .get_matches();

    println!("Hello World, seaports command is functional");
    
}
