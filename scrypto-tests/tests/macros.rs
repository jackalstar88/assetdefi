#![cfg_attr(not(feature = "std"), no_std)]

use scrypto::buffer::{scrypto_decode, scrypto_encode, scrypto_wrap};
use scrypto::constructs::{Blueprint, Component};
use scrypto::kernel::*;
use scrypto::rust::string::ToString;
use scrypto::rust::vec::Vec;
use scrypto::types::*;
use scrypto::*;

const LOG_MESSAGE: &'static str = "Hello, Radix!";
const PACKAGE_ADDRESS: &'static str = "050377bac8066e51cd0d6b320c338d5abbcdbcca25572b6b3eee94";
const COMPONENT_ADDRESS: &'static str = "06c46576324df8c76f6d83611974e8d26a12fe648280c19974c979";
const BLUEPRINT_NAME: &'static str = "BlueprintName";
const FUNCTION_NAME: &'static str = "function";
const METHOD_NAME: &'static str = "method";
const RETURN: i32 = 5;

#[no_mangle]
pub extern "C" fn kernel(op: u32, input_ptr: *const u8, input_len: usize) -> *mut u8 {
    let mut input_bytes = Vec::<u8>::with_capacity(input_len);
    unsafe {
        core::ptr::copy(input_ptr, input_bytes.as_mut_ptr(), input_len);
        input_bytes.set_len(input_len);
    }
    let output_bytes;

    match op {
        EMIT_LOG => {
            let input: EmitLogInput = scrypto_decode(&input_bytes).unwrap();
            assert_eq!(input.message, LOG_MESSAGE);

            let output = EmitLogOutput {};
            output_bytes = scrypto_encode(&output);
        }
        CALL_BLUEPRINT => {
            let input: CallBlueprintInput = scrypto_decode(&input_bytes).unwrap();
            assert_eq!(input.package, Address::from(PACKAGE_ADDRESS));
            assert_eq!(input.blueprint, BLUEPRINT_NAME);
            assert_eq!(input.function, FUNCTION_NAME);

            let output = CallBlueprintOutput {
                rtn: scrypto_encode(&RETURN),
            };
            output_bytes = scrypto_encode(&output);
        }
        CALL_COMPONENT => {
            let input: CallComponentInput = scrypto_decode(&input_bytes).unwrap();
            assert_eq!(input.component, Address::from(COMPONENT_ADDRESS));
            assert_eq!(input.method, METHOD_NAME);

            let output = CallComponentOutput {
                rtn: scrypto_encode(&RETURN),
            };
            output_bytes = scrypto_encode(&output);
        }
        GET_COMPONENT_INFO => {
            let input: GetComponentInfoInput = scrypto_decode(&input_bytes).unwrap();
            assert_eq!(input.component, Address::from(COMPONENT_ADDRESS));

            let output = GetComponentInfoOutput {
                package: Address::from(PACKAGE_ADDRESS),
                blueprint: BLUEPRINT_NAME.to_string(),
            };
            output_bytes = scrypto_encode(&output);
        }
        _ => panic!("Unexpected operation: {}", op),
    }

    scrypto_wrap(&output_bytes)
}

#[test]
fn test_logging() {
    error!("Hello, {}!", "Radix");
    warn!("Hello, {}!", "Radix");
    info!("Hello, {}!", "Radix");
    debug!("Hello, {}!", "Radix");
    trace!("Hello, {}!", "Radix");
}

#[test]
fn test_call_blueprint() {
    let blueprint = Blueprint::from(Address::from(PACKAGE_ADDRESS), BLUEPRINT_NAME);
    let rtn: i32 = blueprint.call(FUNCTION_NAME, args!(123));
    assert_eq!(rtn, RETURN);
}

#[test]
fn test_call_component() {
    let component = Component::from(Address::from(COMPONENT_ADDRESS));
    component.call::<i32>(METHOD_NAME, args!(456));
}
