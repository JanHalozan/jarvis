use super::{command_action::CommandAction, command_subject::CommandSubject};

#[derive(Debug, PartialEq, Eq)]
pub struct Command { 
    pub location: String,
    pub action: CommandAction,
    pub subject: CommandSubject
}