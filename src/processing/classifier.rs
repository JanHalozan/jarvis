use std::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use rust_bert::{pipelines::{sequence_classification::Label, zero_shot_classification::{ZeroShotClassificationConfig, ZeroShotClassificationModel}}, RustBertError};

use crate::{core::commander::Commander, model::{command::Command, command_action::CommandAction, command_subject::CommandSubject, intent::Intent}, traits::labelable::Labelable};

pub type ClassifierOutput = Result<Intent, ClassificationFailureReason>;

const SCORE_THRESHOLD: f64 = 0.85;

struct ClassificationLabels {
    intents: Vec<String>,
    locations: Vec<String>,
    actions: Vec<String>,
    subjects: Vec<String>
}

pub enum ClassificationFailureReason {
    Unknown, UnsupportedInstruction, UnrecognizedInstruction
}

pub fn main(command_rx: Receiver<String>, intent_tx: Sender<ClassifierOutput>) -> Result<()> {
    let model = load_model()?;
    let commander = Commander::new();
    let labels = build_labels(&commander);
    let model_labels: Vec<&str> = labels.locations
        .iter()
        .chain(labels.actions.iter())
        .chain(labels.subjects.iter())
        .chain(labels.intents.iter())
        .map(|s| s.as_str())
        .collect();

    while let Ok(instruction) = command_rx.recv() {
        let inputs = vec![instruction.as_str()];
        let output = match model.predict_multilabel(inputs, &model_labels, None, 128) {
            Ok(result) => result,
            Err(_) => {
                if intent_tx.send(Err(ClassificationFailureReason::Unknown)).is_err() {
                    break;
                }
                continue;
            }
        };
        
        let (intent, score) = intent_from_classification(&instruction, &output[0], &labels);
        let result: Result<Intent, ClassificationFailureReason>;

        if score < SCORE_THRESHOLD {
            println!("Instruction '{}'\nScore {} with output: {:?}\n", instruction, score, output[0]);
            result = Err(ClassificationFailureReason::UnrecognizedInstruction);
        } else if let Intent::Command(ref command) = intent {
            if commander.supports_command(command) {
                println!("Instruction '{}'\nExecuting {:?}\n", instruction, intent);
                result = Ok(intent);
            } else {
                result = Err(ClassificationFailureReason::UnsupportedInstruction);
            }
        } else if let Intent::Question(_) = intent {
            result = Ok(intent);
        } else { // Shouldn't really happen
            println!("No suitable command for '{}'\n", instruction);
            result = Err(ClassificationFailureReason::UnsupportedInstruction);
        }

        if intent_tx.send(result).is_err() {
            break;
        }
    }

    Ok(())
}

fn load_model() -> Result<ZeroShotClassificationModel, RustBertError> {
    let config = ZeroShotClassificationConfig {
        model_type: rust_bert::pipelines::common::ModelType::Bart,
        ..Default::default()
    };
    let model = ZeroShotClassificationModel::new(config)?;

    Ok(model)
}

fn build_labels(commander: &Commander) -> ClassificationLabels {
    ClassificationLabels {
        intents: Intent::labels(),
        locations: commander.locations.clone(),
        actions: CommandAction::labels(),
        subjects: CommandSubject::labels()
    }
}

fn intent_from_classification(instruction: &str, model_output: &Vec<Label>, data: &ClassificationLabels) -> (Intent, f64) {
    let mut action: (f64, usize) = (0.0, 0);
    let mut location: (f64, usize) = (0.0, 0);
    let mut subject: (f64, usize) = (0.0, 0);

    for (i, label) in model_output.iter().enumerate() {
        let score = label.score;

        // Figuring out if it's a question is a bit special
        if data.intents.contains(&label.text) && score > SCORE_THRESHOLD {
            // Only in the case of a recognized question we return early
            if Intent::is_label_question(&label.text) {
                return (Intent::Question(instruction.to_string()), score);
            }
        } else if data.actions.contains(&label.text) && score > action.0 {
            action = (score, i);
        } else if data.subjects.contains(&label.text) && score > subject.0 {
            subject = (score, i);
        } else if score > location.0 { // it's a location
            location = (score, i);
        }
    }

    let score = action.0.min(location.0).min(subject.0);
    let command = Command {
        location: model_output[location.1].text.clone(),
        action: model_output[action.1].text.parse::<CommandAction>().unwrap(),
        subject: model_output[subject.1].text.parse::<CommandSubject>().unwrap()
    };
    let intent = Intent::Command(command);

    (intent, score)
}