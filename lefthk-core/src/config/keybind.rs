use serde::{Deserialize, Serialize};

use super::command::utils::normalized_command::NormalizedCommand;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Keybind {
    pub command: NormalizedCommand,
    pub modifier: Vec<String>,
    pub key: String,
}

/// A trait for structs which should be able to convert themself into the keybind
/// of lefthk-core.
pub trait KeybindConverter {
    type Error;

    /// Returns a vector of lefthk-core-keybindings which can be used to
    /// immitate the keybind/struct which implements this.
    fn to_lefthk_core_keybind(&self) -> Result<Vec<Keybind>, Self::Error>;
}
