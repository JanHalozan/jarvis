use std::{path::PathBuf, sync::mpsc::{Receiver, Sender}};

use anyhow::Result;
// use hound::{WavReader, WavSpec, WavWriter};

// use crate::core::constants::AUDIO_SAMPLE_RATE;

pub fn main(chunker_rx: Receiver<Vec<f32>>, detector_tx: Sender<Vec<f32>>) -> Result<()> {
    // let wakeword_audio = load_wake_word()?;

    while let Ok(audio) = chunker_rx.recv() {

        // let audio_slice = &audio[0..=wakeword_audio.len()];
        // println!("Slejs {} je {}", wakeword_audio.len(), audio_slice.len());

        if detector_tx.send(audio).is_err() {
            break;
        }
    }

    Ok(())
}

// fn calculate_mfcc(audio: &[f32], sample_rate: usize) {
//     // let mut state = Transform::new(AUDIO_SAMPLE_RATE, buffer_size)
// }

// fn load_wake_word() -> Result<Vec<f32>> {
//     let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .join("config")
//         .join("wake_word.wav")
//         .to_str()
//         .expect("Could not construct the wake_word.wav path")
//         .to_owned();

//     let samples: Vec<f32> = WavReader::open(file)?
//         .samples::<f32>()
//         .map(|sample| sample.unwrap())
//         .collect();

//     Ok(samples)
// }

// fn write_audio_file(filename: &str, data: &Vec<f32>) {
//     let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(filename);
//     let spec = WavSpec {
//         channels: 1,
//         sample_rate: 16000,
//         bits_per_sample: 32,
//         sample_format: hound::SampleFormat::Float
//     };

//     let mut writer = WavWriter::create(path, spec).unwrap();
//     for &sample in data {
//         writer.write_sample(sample).unwrap();
//     }

//     writer.finalize().unwrap();
// }