use std::sync::mpsc::{Receiver, Sender};

use crate::model::{command::Command, intent::Intent};

use super::classifier::ClassifierOutput;

pub fn main(classifier_rx: Receiver<ClassifierOutput>, executor_tx: Sender<ClassifierOutput>) {
    while let Ok(result) = classifier_rx.recv() {

        if let Ok(ref intent) = result {    
            match intent {
                Intent::Command(command) => execute_command(command),
                Intent::Question(_) => {}
            };
        }

        if executor_tx.send(result).is_err() {
            break;
        }
    }
}

fn execute_command(_command: &Command)  {
    // println!("Executing {:?}", command);
}