use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use scrypto::types::*;

use crate::abi::*;
use crate::ledger::*;
use crate::rev2::*;

const ARG_TRACE: &str = "TRACE";
const ARG_PACKAGE: &str = "PACKAGE";
const ARG_BLUEPRINT: &str = "BLUEPRINT";

/// Constructs a `export-abi` subcommand.
pub fn make_export_abi_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_EXPORT_ABI)
        .about("Exports the ABI of a blueprint")
        .version(crate_version!())
        .arg(
            Arg::with_name(ARG_TRACE)
                .short("t")
                .long("trace")
                .help("Turns on tracing."),
        )
        .arg(
            Arg::with_name(ARG_PACKAGE)
                .help("Specify the package address.")
                .required(true),
        )
        .arg(
            Arg::with_name(ARG_BLUEPRINT)
                .help("Specify the blueprint name.")
                .required(true),
        )
}

/// Handles a `export-abi` request.
pub fn handle_export_abi(matches: &ArgMatches) -> Result<(), Error> {
    let trace = matches.is_present(ARG_TRACE);
    let package: Address = matches
        .value_of(ARG_PACKAGE)
        .ok_or_else(|| Error::MissingArgument(ARG_PACKAGE.to_owned()))?
        .parse()
        .map_err(Error::InvalidAddress)?;
    let blueprint = matches
        .value_of(ARG_BLUEPRINT)
        .ok_or_else(|| Error::MissingArgument(ARG_BLUEPRINT.to_owned()))?;

    let mut ledger = FileBasedLedger::new(get_data_dir()?);
    let result = export_abi(&mut ledger, package, blueprint, trace);

    match result {
        Err(e) => Err(Error::TxnExecutionError(e)),
        Ok(abi) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&abi).map_err(Error::JSONError)?
            );
            Ok(())
        }
    }
}
