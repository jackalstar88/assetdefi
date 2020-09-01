use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use scrypto::types::*;

use crate::cli::*;
use crate::ledger::*;
use crate::txn::*;
use crate::utils::*;

const ARG_COMPONENT: &'static str = "COMPONENT";
const ARG_METHOD: &'static str = "METHOD";
const ARG_ARGS: &'static str = "ARGS";

/// Constructs a `call-method` subcommand.
pub fn make_call_method_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_CALL_METHOD)
        .about("Calls a component method")
        .version(crate_version!())
        .arg(
            Arg::with_name(ARG_COMPONENT)
                .help("Specify the component address.")
                .required(true),
        )
        .arg(
            Arg::with_name(ARG_METHOD)
                .help("Specify the method name.")
                .required(true),
        )
        .arg(
            Arg::with_name(ARG_ARGS)
                .help("Specify the arguments, e.g. `123`, `hello` or `1000:01`.")
                .multiple(true),
        )
}

/// Handles a `call-method` request.
pub fn handle_call_method<'a>(matches: &ArgMatches<'a>) -> Result<(), Error> {
    let component: Address = matches
        .value_of(ARG_COMPONENT)
        .ok_or(Error::MissingArgument(ARG_COMPONENT.to_owned()))?
        .into();
    let method = matches
        .value_of(ARG_METHOD)
        .ok_or(Error::MissingArgument(ARG_METHOD.to_owned()))?;
    let mut args = Vec::new();
    if let Some(x) = matches.values_of(ARG_ARGS) {
        x.for_each(|a| args.push(a));
    }

    match get_config(CONF_DEFAULT_ACCOUNT)? {
        Some(a) => {
            let account: Address = a.as_str().into();
            let mut ledger = FileBasedLedger::new(get_data_dir()?);
            match build_call_method(&mut ledger, account, component, method, &args, false) {
                Ok(txn) => {
                    let receipt = execute(&mut ledger, txn, false);
                    dump_receipt(receipt);
                    Ok(())
                }
                Err(e) => Err(Error::ConstructionErr(e)),
            }
        }
        None => Err(Error::NoDefaultAccount),
    }
}
