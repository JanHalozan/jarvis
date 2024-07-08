use std::{fmt::Display, str::FromStr};

use crate::traits::labelable::Labelable;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandSwitchValue {
    On, Off
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandGradientValue {
    Min, Max, Less, More
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandAction {
    Switch(CommandSwitchValue),
    Gradient(CommandGradientValue)
}

impl CommandAction {
    pub fn from_command_parser(str: &str) -> Option<Self> {
        match str {
            "switch" => Some(Self::Switch(CommandSwitchValue::Off)),
            "gradient" => Some(Self::Gradient(CommandGradientValue::Min)),
            _ => None
        }
    }

    pub fn is_same_action(&self, other: &CommandAction) -> bool {
        matches!((self, other), 
            (CommandAction::Switch(_), CommandAction::Switch(_)) |
            (CommandAction::Gradient(_), CommandAction::Gradient(_)))
    }
}

impl Display for CommandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Switch(CommandSwitchValue::On) => "turned on",
            Self::Switch(CommandSwitchValue::Off) => "turned off",
            Self::Gradient(CommandGradientValue::Min) => "closed",
            Self::Gradient(CommandGradientValue::Max) => "opened",
            Self::Gradient(CommandGradientValue::More) => "raised",
            Self::Gradient(CommandGradientValue::Less) => "lowered"
        };

        write!(f, "{}", str)
    }
}

impl FromStr for CommandAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_label(s))
    }
}

impl Labelable for CommandAction {
    fn from_label(label: &str) -> Self {
        match label {
            "switch" | "turn off" => Self::Switch(CommandSwitchValue::Off),
            "turn on" => Self::Switch(CommandSwitchValue::On),
            "increase" => Self::Gradient(CommandGradientValue::More),
            "decrease" => Self::Gradient(CommandGradientValue::Less),
            "close" => Self::Gradient(CommandGradientValue::Min),
            "open" => Self::Gradient(CommandGradientValue::Max),
            _ => Self::Switch(CommandSwitchValue::Off)
        }
    }

    fn labels() -> Vec<String> {
        vec![
            "turn on".to_string(),
            "turn off".to_string(),
            "increase".to_string(),
            "decrease".to_string(),
            "close".to_string(),
            "open".to_string()
        ]
    }
}