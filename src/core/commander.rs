
use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};

use anyhow::{Ok, Result};

use crate::model::{command::Command, command_action::CommandAction, command_map::CommandMap, command_subject::CommandSubject};

pub struct Commander {
    pub commands: Vec<Command>,
    pub locations: Vec<String>
}

impl Commander {
    pub fn new() -> Self {
        let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("config")
            .join("command_map.yaml")
            .to_str()
            .expect("Could not construct the command_map.yaml path")
            .to_owned();

        let (map, locations) = parse_command_map(file)
            .expect("Invalid command_map.yaml structure");

        Commander {
            commands: map.commands,
            locations
        }
    }

    pub fn supports_command(&self, command: &Command) -> bool {
        for supported_command in &self.commands {
            if 
                supported_command.location == command.location &&
                supported_command.subject == command.subject &&
                supported_command.action.is_same_action(&command.action)
            {
                return true;
            }
        }
        
        false
    }
}

// A very crude parser for the command_map.yaml
// I tried using serde_yaml but it was more pain than
// benefit. This isn't great but gets the job done and 
// is not the point of this project
fn parse_command_map(file_path: String) -> Result<(CommandMap, Vec<String>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut locations = Vec::new();
    let mut commands = Vec::new();

    let mut is_parsing_commands = false;
    let mut current_location: Option<String> = None;
    let mut current_action: Option<CommandAction> = None;

    for line in reader.lines().flatten() {
        let (indentation, line) = cleaned_line_with_indentation(&line);

        if !is_parsing_commands {
            if line == "commands" {
                is_parsing_commands = true;
            }

            continue;
        }

        match indentation {
            1 => { // one level in are locations
                let location = line.to_string();
                locations.push(location.clone());
                current_location = Some(location);
            }
            2 => {
                if let Some(action) = CommandAction::from_command_parser(line) {
                    current_action = Some(action);
                } else {
                    current_action = None;
                    continue;
                }
            }
            3 => {
                let subject = CommandSubject::from_command_parser(line);
                let tuple = (current_location.clone(), current_action, subject);

                if let (Some(location), Some(action), Some(subject)) = tuple {
                    commands.push(Command { location, action, subject });
                } else {
                    continue;
                }
            }
            _ => {
                is_parsing_commands = false;
                continue;
            }
        };
    }

    Ok((CommandMap { commands }, locations))
}

fn cleaned_line_with_indentation(line: &str) -> (usize, &str) {
    const START_WHITESPACES: &[char; 4] = &[' ', '\t', '-', '"'];
    const END_WHITESPACES: &[char; 2] = &['"', ':'];

    let mut spaces = 0;
    let mut tabs = 0;

    for c in line.chars() {
        match c {
            ' ' => spaces += 1,
            '\t' => tabs += 1,
            _ => break
        };
    }

    let space_indent = spaces / 2;
    let tab_indent = tabs;

    let cleaned = line
        .trim_start_matches(START_WHITESPACES)
        .trim_end_matches(END_WHITESPACES);

    (space_indent + tab_indent, cleaned)
}