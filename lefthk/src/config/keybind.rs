use crate::errors::LeftError;
use lefthk_core::config::command as command_mod;
use lefthk_core::config::command::Command as core_command;
use lefthk_core::config::keybind::Keybind as core_keybind;
use lefthk_core::config::keybind::KeybindConverter;
use serde::Deserialize;
use serde::Serialize;

use super::{command::Command, key::Key};

macro_rules! get_key {
    ($expr:expr $(,)?) => {
        match $expr {
            Key::Key(key) => key,
            Key::Keys(_) => return Err(LeftError::SingleKeyNeeded),
        }
    };
}

macro_rules! get_keys {
    ($expr:expr $(,)?) => {
        match $expr {
            Key::Key(_) => return Err(LeftError::MultipleKeysNeeded),
            Key::Keys(keys) => keys,
        }
    };
}

pub type Keybinds = Vec<Keybind>;

#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct Keybind {
    pub command: Command,
    pub modifier: Vec<String>,
    pub key: Key,
}

impl KeybindConverter for Keybind {
    type Error = LeftError;

    fn to_lefthk_core_keybind(&self) -> Result<Vec<core_keybind>, Self::Error> {
        let command_key_pairs: Vec<(Box<dyn core_command>, String)> = match &self.command {
            Command::Chord(children) if !children.is_empty() => {
                let key = get_key!(&self.key);
                let children = children
                    .iter()
                    .filter_map(|kb| {
                        kb.to_lefthk_core_keybind().map_or_else(
                            |err| {
                                tracing::error!("Invalid key binding: {}\n{:?}", err, self);
                                None
                            },
                            |converted_kb| Some(converted_kb),
                        )
                    })
                    .flatten()
                    .collect();

                vec![(Box::new(command_mod::Chord::new(children)), key.to_string())]
            }
            Command::Chord(_) => return Err(LeftError::ChildrenNotFound),
            Command::Execute(value) if !value.is_empty() => {
                let keys = get_key!(&self.key);
                vec![(Box::new(command_mod::Execute::new(value)), keys.to_string())]
            }
            Command::Execute(_) => return Err(LeftError::ValueNotFound),
            Command::Executes(values) if !values.is_empty() => {
                let keys = get_keys!(&self.key);
                if keys.len() != values.len() {
                    return Err(LeftError::NumberOfKeysDiffersFromValues);
                }
                values
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        (
                            Box::new(command_mod::Execute::new(v.to_owned()))
                                as Box<dyn core_command>,
                            keys[i].clone(),
                        )
                    })
                    .collect()
            }
            Command::Executes(_) => return Err(LeftError::ValuesNotFound),
            Command::ExitChord => {
                let keys = get_key!(&self.key);
                vec![(Box::new(command_mod::ExitChord::new()), keys.to_string())]
            }
            Command::Reload => {
                let keys = get_key!(&self.key);
                vec![(Box::new(command_mod::Reload::new()), keys.to_string())]
            }
            Command::Kill => {
                let keys = get_key!(&self.key);
                vec![(Box::new(command_mod::Kill::new()), keys.to_string())]
            }
        };

        let keybinds = command_key_pairs
            .iter()
            .map(|(c, k)| core_keybind {
                command: c.normalize(),
                modifier: self.modifier.clone(),
                key: k.to_owned(),
            })
            .collect();
        Ok(keybinds)
    }
}
