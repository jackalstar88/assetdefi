use sbor::model::*;
use sbor::{Decode, Describe, Encode};

use crate::buffer::*;
use crate::kernel::*;
use crate::types::rust::borrow::ToOwned;
use crate::types::rust::string::String;
use crate::types::rust::string::ToString;
use crate::types::rust::vec::Vec;
use crate::types::*;

/// A piece of code that defines the structure and methods of components.
#[derive(Debug, Encode, Decode)]
pub struct Blueprint {
    package: Address,
    name: String,
}

impl Describe for Blueprint {
    fn describe() -> Type {
        Type::SystemType {
            name: "::scrypto::constructs::Blueprint".to_owned(),
        }
    }
}

impl Blueprint {
    pub fn from(package: Address, name: &str) -> Self {
        Self {
            package,
            name: name.to_owned(),
        }
    }

    pub fn call<T: Decode>(&self, function: &str, args: Vec<Vec<u8>>) -> T {
        let input = CallBlueprintInput {
            package: self.package,
            blueprint: self.name.clone(),
            function: function.to_string(),
            args,
        };
        let output: CallBlueprintOutput = call_kernel(CALL_BLUEPRINT, input);

        scrypto_decode(&output.rtn).unwrap()
    }

    pub fn package(&self) -> Address {
        self.package
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
