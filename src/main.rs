mod config;
mod dump;
mod utils;
mod database;
mod compare;
mod init;

use std::ffi::OsString;
use clap::{arg, Command};
use crate::config::Config;
use crate::dump::dump;

const VERSION: &str = "v0.1.0-alpha";

const DUMP: &str = "dump";
const COMPARE: &str = "compare";
const RUN: &str = "run";
const INIT: &str = "init";


fn cli() -> Command {
    Command::new("storm")
        .version(VERSION)
        .about("Database migration tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new(DUMP)
                .about("Dump the database to a file")
                .arg(config_arg())
                .arg(env_arg())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(COMPARE)
                .about("Compare the database file")
                .arg(config_arg())
                .arg(src_arg())
                .arg(dst_arg())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(RUN)
                .about("Run sql commands from sql file")
                .arg(config_arg())
                .arg(url_arg())
                .arg(file_arg())
                .arg(env_arg())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(INIT)
                .about("Initialize a migrate project")
                .arg_required_else_help(true),
        )
}

fn config_arg() -> clap::Arg {
    arg!(-c --config <CONFIG>)
        .default_missing_value("~/.stormbreaker/storm.yaml")
        .help("The path to the configuration file. If not provided, the default path is ~/.stormbreaker/storm.yaml")
}

fn url_arg() -> clap::Arg {
    arg!(-u --url <URL>)
        .default_missing_value("mysql://localhost:3306")
        .help("The url to the database. If not provided, the default url is mysql://localhost:3306")
}

fn file_arg() -> clap::Arg {
    arg!(-f --file <FILE>)
        .default_missing_value("dump.sql")
        .help("The path to the sql file. If not provided, the default path is latest version sql file.")
}

fn env_arg() -> clap::Arg {
    arg!(-e --env <ENV>)
        .default_missing_value("dev")
        .help("The environment to use. If not provided, the default environment is dev.")
}

fn src_arg() -> clap::Arg {
    arg!(-s --src <SRC>)
        .default_missing_value("dev")
        .help("The source environment to use. If not provided, the default environment is dev.")
}

fn dst_arg() -> clap::Arg {
    arg!(-d --dst <DST>)
        .default_missing_value("dev")
        .help("The destination environment to use. If not provided, the default environment is dev.")
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some((DUMP, sub_matches)) => {
            let path = sub_matches.get_one::<String>("config").unwrap();
            println!("Dumping database with {path:?}");
            let config = Config::from_file(&path).unwrap();

            let env = sub_matches.get_one::<String>("env").unwrap();

            dump(&config, env).unwrap();
        }
        Some((COMPARE, sub_matches)) => {
            let path = sub_matches.get_one::<String>("config").unwrap();
            println!("Comparing database with {path:?}");
            let config = Config::from_file(&path).unwrap();

            let src = sub_matches.get_one::<String>("src").unwrap();
            let dst = sub_matches.get_one::<String>("dst").unwrap();

            compare::compare(&config, src, dst).unwrap();
        }
        Some((RUN, sub_matches)) => {
            let path = sub_matches.get_one::<String>("config").unwrap();
            println!("Running sql commands with {path:?}")
        }
        Some((INIT, _)) => {
            println!("Initializing a migrate project")
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}