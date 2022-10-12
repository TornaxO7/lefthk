use lefthk_core::config::{command::{CommandConverter, Command as CoreCommand, self as core_command}, keybind::KeybindConverter};

use serde::{Deserialize, Serialize};

use crate::errors::LeftError;

use super::keybind::Keybind;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Command {
    Chord(Vec<Keybind>),
    Execute(String),
    Executes(Vec<String>),
    ExitChord,
    Reload,
    Kill,
}

impl CommandConverter for Command {
    type Error = LeftError;

    fn to_lefthk_core_command(&self) -> Result<Vec<Box<dyn CoreCommand>>, Self::Error> {
        match self {
            Command::Chord(children) if !children.is_empty() => self.convert_chord(children),
            Command::Execute(value) if !value.is_empty() => self.convert_execute(value),
            Command::Executes(values) if !values.is_empty() => self.convert_executes(values),
            Command::ExitChord => self.convert_exit_chord(),
            Command::Reload => self.convert_reload(),
            Command::Kill => self.convert_kill(),

            Command::Chord(_) => Err(LeftError::ChildrenNotFound),
            Command::Execute(_) => Err(LeftError::ValueNotFound),
            Command::Executes(_) => Err(LeftError::ValueNotFound),
        }
    }
}

impl Command {
    fn convert_chord(&self, children: &Vec<Keybind>) -> Result<Vec<Box<dyn CoreCommand>>, LeftError> {
        let children = children.iter()
            .filter_map(|keybind| match keybind.to_lefthk_core_keybind() {
                Ok(core_keybinds) => Some(core_keybinds),
                Err(err) => {
                    tracing::error!("Invalid key binding: {}\n{:?}", err, self);
                    None
                },
            })
            .flatten()
            .collect();

        Ok(vec![Box::new(core_command::Chord::new(children))])

    }

    fn convert_execute(&self, value: &str) -> Result<Vec<Box<dyn CoreCommand>>, LeftError> {
        Ok(vec![Box::new(core_command::Execute::new(value))])
    }

    fn convert_executes(&self, _values: &Vec<String>) -> Result<Vec<Box<dyn CoreCommand>>, LeftError> {
        todo!()
    }

    fn convert_exit_chord(&self) -> Result<Vec<Box<dyn CoreCommand>>, LeftError> {
        Ok(vec![Box::new(core_command::ExitChord::new())])
    }

    fn convert_reload(&self) -> Result<Vec<Box<dyn CoreCommand>>, LeftError> {
        Ok(vec![Box::new(core_command::Reload::new())])
    }

    fn convert_kill(&self) -> Result<Vec<Box<dyn CoreCommand>>, LeftError> {
        Ok(vec![Box::new(core_command::Kill::new())])
    }
}
