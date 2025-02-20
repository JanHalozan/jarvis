use std::sync::mpsc::{Receiver, Sender};

use anyhow::{Ok, Result};
use rppal::gpio::Gpio;

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

const GPIO_LIVING_ROOM: u8 = 23;
const GPIO_HALLWAY: u8 = 24;

fn execute_command(command: &Command) -> Result<()> {
    let gpio = Gpio::new()?;

    if command.location == "living room" && command.subject == CommandSubject::Light {
        if let CommandAction::Switch(value) = command.action {
            let mut pin = gpio.get(GPIO_LIVING_ROOM)?.into_output();
            if value == CommandSwitchValue::On {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    }

    if command.location == "hallway" && command.subject == CommandSubject::Light {
        if let CommandAction::Switch(value) = command.action {
            let mut pin = gpio.get(GPIO_HALLWAY)?.into_output();
            if value == CommandSwitchValue::On {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    }

    println!("Executing {:?}", command);

    Ok(())
}