use std::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use rand::Rng;
use rust_bert::{gpt2::GPT2Generator, pipelines::generation_utils::{GenerateConfig, LanguageGenerator}};

use crate::model::{command::Command, intent::Intent};

use super::classifier::ClassificationFailureReason;

pub fn main(intent_rx: Receiver<Result<Intent, ClassificationFailureReason>>, feedback_tx: Sender<String>) -> Result<()> {    
    let config = GenerateConfig {
        model_type: rust_bert::pipelines::common::ModelType::GPT2,
        max_length: Some(30),
        min_length: 5,
        length_penalty: 2.0,
        early_stopping: true,
        do_sample: false,
        num_beams: 5,
        temperature: 0.05,
        ..Default::default()
    };
    let model = GPT2Generator::new(config)?;

    while let Ok(result) = intent_rx.recv() { 
        let message = match result {
            Ok(intent) => feedback_for_intent(intent, &model),
            Err(error) => feedback_for_error(error)
        };
        println!("Feedback message: '{}'\n", message);
        if feedback_tx.send(message).is_err() {
            break;
        }
    }
    
    Ok(())
}

fn feedback_for_error(reason: ClassificationFailureReason) -> String {
    let str = match reason {
        ClassificationFailureReason::UnsupportedInstruction => "I don't know how to do this yet.",
        ClassificationFailureReason::UnrecognizedInstruction => "I'm not sure I recognize your instruction",
        ClassificationFailureReason::Unknown => "Sorry, something went wrong. Could you repeat that?"
    };

    str.to_string()
}

fn feedback_for_intent(intent: Intent, model: &GPT2Generator) -> String {
    match intent {
        Intent::Command(ref command) => feedback_for_command(command),
        Intent::Question(question) => answer_for_question(question, model)
    }
}

fn feedback_for_command(command: &Command) -> String {
    let action = command.action.to_string();
    let subject_description = format!("the {}", command.subject.to_string());
    let location_description = format!("in the {}", command.location);

    match rand::thread_rng().gen_range(0..=4) {
        0 => format!("I've {} {} {}", action, subject_description, location_description),
        1 => format!("{} {} has been {}", subject_description, location_description, action),
        2 => format!("{} {} is now {}", subject_description, location_description, action),
        3 => format!("I've successfully {} {} {}", action, subject_description, location_description),
        4 => format!("Done! {} {} is now {}", subject_description, location_description, action),
        _ => unreachable!(),
    }
}

fn answer_for_question(question: String, model: &GPT2Generator) -> String {
    let output = model.generate(Some(&[&question]), None);

    println!("Question {:?}", output);
    let empty_answer = "I don't know".to_string();
    match output {
        Ok(answer) => answer
            .first()
            .map(|answer| {
                answer.text
                .trim_start_matches(&question)
                .split('.')
                .next()
                .unwrap_or(&empty_answer)
                .to_string()
            }).unwrap_or_else(|| empty_answer),
        Err(_) => empty_answer
    }
}