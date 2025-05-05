#![cfg(feature = "build-web")]

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CargoMetadata {
    pub(crate) target_directory: Box<str>,
}
