use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

use crate::rev2::*;

const ARG_NAME: &str = "NAME";
const ARG_VALUE: &str = "VALUE";

/// Constructs a `config` subcommand.
pub fn make_config_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_CONFIG)
        .about("Config this simulator")
        .version(crate_version!())
        .arg(
            Arg::with_name(ARG_NAME)
                .help("Specify the name, e.g. `default.account`")
                .required(true),
        )
        .arg(
            Arg::with_name(ARG_VALUE)
                .help("Specify the value.")
                .required(true),
        )
}

/// Handles a `config` request.
pub fn handle_config(matches: &ArgMatches) -> Result<(), Error> {
    let name = matches
        .value_of(ARG_NAME)
        .ok_or_else(|| Error::MissingArgument(ARG_NAME.to_owned()))?;
    let value = matches
        .value_of(ARG_VALUE)
        .ok_or_else(|| Error::MissingArgument(ARG_VALUE.to_owned()))?;

    set_config(name, value)?;

    println!(
        "{}",
        serde_json::to_string_pretty(&get_configs()?).map_err(Error::JSONError)?
    );
    Ok(())
}
