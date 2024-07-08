use std::{fmt::Display, str::FromStr};

use crate::traits::labelable::Labelable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommandSubject {
    Light, Teapot, WindowBlinds, Temperature, Ventilator
}

impl CommandSubject {
    pub fn from_command_parser(str: &str) -> Option<Self> {
        Self::internal_from_str(str)
    }

    // Unified for several methods here
    fn internal_from_str(str: &str) -> Option<Self> {
        match str {
            "light" => Some(Self::Light),
            "teapot" => Some(Self::Teapot),
            "windowblinds" => Some(Self::WindowBlinds),
            "temperature" => Some(Self::Temperature),
            "ventilator" => Some(Self::Ventilator),
            _ => None
        }
    }
}

impl FromStr for CommandSubject {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_label(s))
    }
}

impl Display for CommandSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "light"),
            Self::Teapot => write!(f, "teapot"),
            Self::WindowBlinds => write!(f, "window blinds"),
            Self::Temperature => write!(f, "temperature"),
            Self::Ventilator => write!(f, "ventilator")
        }
    }
}

impl Labelable for CommandSubject {
    fn from_label(label: &str) -> Self {
        match Self::internal_from_str(label) {
            Some(subject) => subject,
            None => Self::Light
        }
    }

    fn labels() -> Vec<String> {
        vec![
            "light".to_string(),
            "teapot".to_string(),
            "windowblinds".to_string(),
            "temperature".to_string(),
            "ventilator".to_string()
        ]
    }
}