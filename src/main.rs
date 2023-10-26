/*
SQL Logic goal
- take a row
- find the next entry that matches the same student id
- if action of found entry is i (in) stop there
- if action is o (out) record the duration and write to another table
*/
use std::error::Error;
use cliclack::*;
mod program;

#[derive(Default, Clone, Eq, PartialEq)]
enum _State {
    #[default]
    Register,
    Log,
    Analyze,
}

fn main() -> Result<(), Box<dyn Error>> {
    intro("Robotics Logging System")?;

    let mut menu = select("")
        .item(_State::Log, "Log in/out system", "")
        .item(_State::Register, "User registration", "")
        .item(_State::Analyze, "Analyzing", "");
    let s = menu.interact()?;
    match s {
        _State::Register => program::_register()?,
        _State::Log => program::_register()?,
        _State::Analyze => program::log()?,
    }
    

    Ok(())
}
