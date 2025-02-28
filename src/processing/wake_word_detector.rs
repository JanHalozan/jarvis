// use std::{path::PathBuf, sync::mpsc::{Receiver, Sender}};
use std::sync::mpsc::{Receiver, Sender};
use porcupine::{Porcupine, BuiltinKeywords, PorcupineBuilder};

use anyhow::Result;

pub fn main(chunker_rx: Receiver<Vec<f32>>, detector_tx: Sender<Vec<f32>>) -> Result<()> {

    let porcupine: Porcupine = PorcupineBuilder::new_with_keyword_paths(
        "${ACCESS_KEY}",
        &["${KEYWORD_FILE_PATH}"],
    )
    .init()
    .expect("Unable to create Porcupine");

    while let Ok(audio) = chunker_rx.recv() {

        // let audio_slice = &audio[0..=wakeword_audio.len()];
        // println!("Slejs {} je {}", wakeword_audio.len(), audio_slice.len());

        if let Ok(keyword_index) = porcupine.process(&audio) {
            if keyword_index == 0 {
                if detector_tx.send(audio).is_err() {
                    break;
                }
            } else {
                // Keyword not detected
            }
        }
    }

    Ok(())
}