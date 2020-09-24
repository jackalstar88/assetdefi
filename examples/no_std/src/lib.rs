// Disable linking to std.
#![cfg_attr(not(test), no_std)]
// Use default alloc error handler, i.e. to panic, and enable core intrinsics.
#![cfg_attr(not(test), feature(default_alloc_error_handler, core_intrinsics))]

// Abort when panicking.
#[cfg(not(test))]
#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

// Use WeeAlloc as our global heap allocator.
#[cfg(not(test))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//============================
// Scrypto code starts here //
//============================

use scrypto::prelude::*;

// A blueprint defines the structure and common behaviour of all its instances, called components.
// In this example, we're creating a `Hello` blueprint.  All components instantiated
// from this blueprint will airdrop 1 `HT` token to its caller.

blueprint! {
    /// Every `Hello` component will have a vault, used for storing the initial `HELLO` tokens.
    struct Hello {
        vault: Vault
    }

    impl Hello {
        /// This function creates 1000 `HT` tokens and a `Hello` component.
        pub fn new() -> Address {
            let bucket: Bucket = ResourceBuilder::new()
                .metadata("name", "HelloToken")
                .metadata("symbol", "HT")
                .create_fixed(1000);

            Self {
                vault: Vault::wrap(bucket)
            }
            .instantiate()
        }

        /// This method takes 1 `HT` token from its vault and returns it to the caller.
        pub fn airdrop(&mut self) -> Bucket {
            let bucket: Bucket = self.vault.take(1);

            info!("Balance: {} HT", self.vault.amount());

            bucket
        }
    }
}
