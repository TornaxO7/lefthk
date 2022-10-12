use self::{keybind::Keybind, command::Command};

pub mod command;
pub mod keybind;

pub trait Config {
    fn mapped_bindings(&self) -> Vec<Keybind>;
}

pub trait CommandAdapter {
    fn convert(&self) -> Vec<Box<dyn Command>>;
}
