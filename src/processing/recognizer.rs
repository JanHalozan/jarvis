use std::sync::mpsc::{Receiver, Sender};

use crate::core::speech_recognizer::SpeechRecognizer;

pub fn main(chunker_rx: Receiver<Vec<f32>>, recognizer_tx: Sender<String>) {

    let recognizer = SpeechRecognizer::new(&SpeechRecognizer::default_model_path());

    while let Ok(audio) = chunker_rx.recv() {
        let text = match recognizer.recognize(&audio) {
            Ok(text) => text,
            Err(_) => continue
        };

        // Filter out a few well known phrases
        if is_noise(&text) {
            continue;
        }

        println!("Recognized speech: '{}'", text);
        if recognizer_tx.send(text).is_err() {
            break;
        }
    }
}

fn is_noise(text: &str) -> bool {
    // All of the noise and non speech has the format [SOMETHING] or
    // (SOMETHING) so it's easier to filter out by checking for
    // the [] or () symbols.

    let square_bracket = text.starts_with('[') && text.ends_with(']');
    let round_bracket = text.starts_with('(') && text.ends_with(')');

    square_bracket || round_bracket
}