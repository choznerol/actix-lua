use std::fs::File;
use std::io::prelude::*;

use actor::LuaActor;
use rlua::{Error as LuaError, Lua};

pub type InitializeVM = Fn(&Lua) -> Result<(), LuaError>;

/// `LuaActorBuilder` creates a new `LuaActor` with given Lua script.
pub struct LuaActorBuilder {
    started: Option<String>,
    handle: Option<String>,
    stopped: Option<String>,
    initialize_vm: Option<Box<InitializeVM>>,
}

impl Default for LuaActorBuilder {
    fn default() -> LuaActorBuilder {
        let noop = Some("return".to_string());
        LuaActorBuilder {
            started: noop.clone(),
            handle: noop.clone(),
            stopped: noop.clone(),
            initialize_vm: None,
        }
    }
}

impl LuaActorBuilder {
    /// Initialize a new `LuaActorBuilder`
    pub fn new() -> Self {
        LuaActorBuilder::default()
    }

    /// create a `started` hook with given lua file
    pub fn on_started(mut self, filename: &str) -> Self {
        self.started = Some(read_to_string(filename));
        self
    }

    /// create a `started` hook with given lua script
    pub fn on_started_with_lua(mut self, script: &str) -> Self {
        self.started = Some(script.to_string());
        self
    }

    /// handle message with given lua file
    pub fn on_handle(mut self, filename: &str) -> Self {
        self.handle = Some(read_to_string(filename));
        self
    }

    /// handle message with given lua script
    pub fn on_handle_with_lua(mut self, script: &str) -> Self {
        self.handle = Some(script.to_string());
        self
    }

    /// create a `stopped` hook with given lua file.
    pub fn on_stopped(mut self, filename: &str) -> Self {
        self.stopped = Some(read_to_string(filename));
        self
    }

    /// create a `stopped` hook with given lua script
    pub fn on_stopped_with_lua(mut self, script: &str) -> Self {
        self.stopped = Some(script.to_string());
        self
    }

    /// config the actor's lua VM
    pub fn with_vm<F: Fn(&Lua) -> Result<(), LuaError> + 'static>(mut self, callback: F) -> Self {
        self.initialize_vm = Some(Box::new(callback));
        self
    }

    /// build the actor
    pub fn build(self) -> Result<LuaActor, LuaError> {
        LuaActor::new(
            self.started.clone(),
            self.handle.clone(),
            self.stopped.clone(),
            self.initialize_vm,
        )
    }
}

fn read_to_string(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut body = String::new();
    f.read_to_string(&mut body).expect("Failed to read file");

    body
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::discriminant;

    #[test]
    fn build_script_error() {
        let res = LuaActorBuilder::new()
            .on_handle_with_lua(r"return 1 +")
            .build();

        if let Err(e) = res {
            assert_eq!(
                discriminant(&LuaError::RuntimeError("unexpected symbol".to_string())),
                discriminant(&e)
            );
        // ok
        } else {
            panic!("should return error");
        }
    }

}
