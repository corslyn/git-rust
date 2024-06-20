use clap::{arg, error, Command};
use flate2;
use std::{
    fs, io,
    path::{self, Path, PathBuf},
};

fn cli() -> Command {
    Command::new("git-rust")
        .about("git remade in Rust")
        .author("corslyn")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("init a repository")
                .arg(arg!(<DIRECTORY> "The directory in which to initialize the repo")),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            create_repo(sub_matches.get_one::<String>("DIRECTORY").expect("."));
        }
        _ => unreachable!(),
    }
}

fn create_repo(directory: String) {
    todo!("oui");
}
