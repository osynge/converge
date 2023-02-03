use clap::{crate_name, crate_version};
use clap::{Arg, ArgAction, Command};

use std::convert::TryFrom;

fn log_setup(verbose_occurrences: u8, quiet_occurrences: u8) -> Option<i8> {
    if (0, 0) == (verbose_occurrences, quiet_occurrences) {
        return None;
    };
    let verbose = match i8::try_from(verbose_occurrences) {
        Ok(p) => p,
        Err(_) => i8::MAX,
    };
    let quiet = match i8::try_from(quiet_occurrences) {
        Ok(p) => p,
        Err(_) => i8::MAX,
    };
    Some(verbose.saturating_sub(quiet))
}

pub fn cli_clap() -> crate::Config {
    let application = Command::new(crate_name!())
        .about("Parses an input file to do awesome things")
        .version(crate_version!())
        .author("Owen Synge <osynge@googlemail.com>")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("Increase log output"),
        )
        .arg(
            Arg::new("quiet")
                .short('v')
                .long("quiet")
                .action(ArgAction::Count)
                .help("Decrease log output"),
        )
        .arg(
            Arg::new("environment_variable")
                .short('e')
                .long("environment_variable")
                .value_name("ENVIROMENT_VARIABLE")
                .action(ArgAction::Append)
                .help("Which environment variables to process"),
        )
        .arg(
            Arg::new("xunit")
                .long("xunit")
                .value_name("XUNIT_FILES")
                .action(ArgAction::Append)
                .help("glob of xunit files"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .action(ArgAction::Set)
                .help("Sets a custom config file"),
        )
        .arg(
            Arg::new("environment-key")
                .long("environment-id")
                .value_name("ENV_ID")
                .action(ArgAction::Append)
                .help("directory storing jobs shell scripts."),
        )
        .arg(
            Arg::new("project-identifier")
                .long("project-identifier")
                .value_name("PROJECT_IDENTIFIER")
                .help("An identifier for the project such as VCS checkout url")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("project-name")
                .long("project-name")
                .value_name("PROJECT_NAME")
                .help("A name for the project.")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("run-identifier")
                .long("run-identifier")
                .value_name("RUN_IDENTIFIER")
                .help("An identifier for the run such as jenkins build number")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("run-key")
                .long("run-key")
                .value_name("RUN_KEY")
                .help("The key for the run")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("service-url")
                .long("url")
                .value_name("SERVICE_URL")
                .help("Sets the service url to upload the xunit to.")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("globs")
                .help("Sets the input files to use, wild cards are allowed.")
                .value_name("GLOB")
                .action(ArgAction::Append),
        );

    let matches = application.get_matches();
    let loglevel = log_setup(matches.get_count("verbose"), matches.get_count("quiet"));
    let xunit_local_globs = matches
        .get_many::<String>("xunit")
        .map(|itr| itr.into_iter().map(String::from).collect());
    let environment_keys = matches
        .get_many::<String>("environment_variable")
        .map(|itr| itr.into_iter().map(String::from).collect());
    let config_file = matches.get_one::<String>("config").cloned();
    let environment_sk = matches.get_one::<String>("environment-key").cloned();
    let project_identifier = matches.get_one::<String>("project-identifier").cloned();
    let project_human_name = matches.get_one::<String>("project-name").cloned();
    let run_identifier = matches.get_one::<String>("run-identifier").cloned();
    let service_url = matches.get_one::<String>("service-url").cloned();
    crate::Config {
        config_file,
        loglevel,
        xunit_local_globs,
        environment_sk,
        environment_keys,
        project_sk: None,
        project_identifier,
        project_human_name,
        run_identifier,
        run_sk: None,
        service_url,
    }
}
