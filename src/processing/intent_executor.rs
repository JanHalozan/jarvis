use std::sync::mpsc::{Receiver, Sender};

use anyhow::Result;

use crate::model::{command::Command, command_action::{CommandAction, CommandSwitchValue}, command_subject::CommandSubject, intent::Intent};

use super::classifier::ClassifierOutput;

pub fn main(classifier_rx: Receiver<ClassifierOutput>, executor_tx: Sender<ClassifierOutput>) {
    while let Ok(result) = classifier_rx.recv() {

        if let Ok(ref intent) = result {    
            match intent {
                Intent::Command(command) => execute_command(command).unwrap(),
                Intent::Question(_) => {}
            };
        }

        if executor_tx.send(result).is_err() {
            break;
        }
    }
}

// const GPIO_LIVING_ROOM: u8 = 19;
// const GPIO_HALLWAY: u8 = 26;

fn execute_command(command: &Command) -> Result<()> {
    //let gpio = Gpio::new()?;
    // pinctrl set 26 op pn dl

    if command.location == "living room" && command.subject == CommandSubject::Light {
        if let CommandAction::Switch(value) = command.action {
  	    
	    let switch_value: &str;
	    if value == CommandSwitchValue::On {
		switch_value = "dh";
	    } else {
		switch_value = "dl";
 	    }
	    let mut child = std::process::Command::new("pinctrl")
            .args(&["set", "26", "op", "pn", switch_value])
            .spawn()?;

            child.wait()?;
        }
    }

    if command.location == "hallway" && command.subject == CommandSubject::Light {
        if let CommandAction::Switch(value) = command.action {

	    let switch_value: &str;
	    if value == CommandSwitchValue::On {
		switch_value = "dh";
	    } else {
		switch_value = "dl";
 	    }
	    let mut child = std::process::Command::new("pinctrl")
            .args(&["set", "19", "op", "pn", switch_value])
            .spawn()?;

            child.wait()?;
        }
    }

    println!("Executing {:?}", command);

    Ok(())
}