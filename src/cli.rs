mod command;
mod commands;

pub use self::command::Command;

use clap::{App, Arg, ArgMatches};

use self::commands::{EndCommand, MigrateCommand, ProjectCommand, StartCommand};
use crate::config::Config;

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true),
        )
        .subcommand(ProjectCommand::cli())
        .subcommand(StartCommand::cli())
        .subcommand(EndCommand::cli())
        .subcommand(MigrateCommand::cli())
}

pub fn exec(config: &mut Config, args: &ArgMatches<'_>) -> Result<(), &'static str> {
    match args.subcommand() {
        ("project", Some(args)) => ProjectCommand::exec(config, args),
        ("start", Some(args)) => StartCommand::exec(config, args),
        ("end", Some(args)) => EndCommand::exec(config, args),
        ("migrate", Some(args)) => MigrateCommand::exec(config, args),
        ("", None) => Ok(()),
        _ => unreachable!(),
    }
}
