// sub-modules

mod did;
mod hydra;
mod json;
mod morpheus;
mod sign;
mod vault;

// exports

pub use did::*;
pub use hydra::*;
pub use json::*;
pub use morpheus::*;
pub use sign::*;
pub use vault::*;

// imports from standard library

// imports from 3rd party crates

use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

// imports from own crates

use iop_keyvault::{multicipher::*, Networks, PrivateKey as _, PublicKey as _};
use iop_keyvault_wasm::*;
use iop_morpheus_core::{
    crypto::{
        hd::{hydra as hd_hydra, morpheus as hd_morpheus, BoundPlugin, Vault, VaultPlugin},
        json_digest::mask_json_value,
        sign::{Signable, Signed},
    },
    data::{
        did::Did,
        diddoc::BlockHeight,
        validation::{ValidationIssue, ValidationResult},
    },
};
