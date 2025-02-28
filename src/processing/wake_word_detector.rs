// use std::{path::PathBuf, sync::mpsc::{Receiver, Sender}};
use std::sync::mpsc::{Receiver, Sender};
use porcupine::{Porcupine, PorcupineBuilder};

use anyhow::Result;

use crate::core::constants::PORCUPINE_ACCESS_KEY;

pub fn main(chunker_rx: Receiver<Vec<f32>>, detector_tx: Sender<Vec<f32>>) -> Result<()> {
    let porcupine: Porcupine = PorcupineBuilder::new_with_keyword_paths(
        PORCUPINE_ACCESS_KEY,
        &["/Users/janhalozan/Work/Rust/jarvis/config/wake_word.ppn"],
    )
    .init()
    .expect("Unable to create Porcupine");

    while let Ok(audio) = chunker_rx.recv() {

        // let audio_slice = &audio[0..=wakeword_audio.len()];
        // println!("Slejs {} je {}", wakeword_audio.len(), audio_slice.len());

        let audio_i16: Vec<i16> = audio.iter().map(|&sample| (sample * 32768.0) as i16).collect();
        if let Ok(keyword_index) = porcupine.process(&audio_i16) {
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