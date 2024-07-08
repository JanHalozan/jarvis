use std::{mem::take, sync::mpsc::{Receiver, Sender}};

use crate::core::constants::MIC_SAMPLE_RATE;

// Processing 20ms of audio at a time
const WINDOW_SIZE: f32 = 20f32 / 1000f32; 

// Number of samples per window
const FRAME_SIZE: usize = (MIC_SAMPLE_RATE as f32 * WINDOW_SIZE) as usize;

// How loud it should be before we start paying attention
const SPEECH_ENERGY_THRESHOLD: f32 = 0.01;

// How much blank time we allow in between speech pauses
const EMPTY_FRAMES_PROCESS_THRESHOLD: i32 = 50;

// Max number of windows before we flush the buffer to avoid 
// memory allocations. At 20ms WINDOW_SIZE this is 20s of audio activity
const MAX_BUFFER_SIZE: usize = FRAME_SIZE * 1000;

pub fn main(mic_rx: Receiver<Vec<f32>>, chunker_tx: Sender<Vec<f32>>) {
    let mut data = Vec::<f32>::new();
    let mut speech_data = Vec::<f32>::new();
    let mut empty_frames = 0;

    while let Ok(mut partial) = mic_rx.recv() {
        data.append(&mut partial);

        while data.len() > FRAME_SIZE {
            let mut frame: Vec<f32> = data.drain(0..FRAME_SIZE).collect();
            let frame_energy = frame_energy(&frame);
            // println!("Frame energy {}", frame_energy);

            if frame_energy > SPEECH_ENERGY_THRESHOLD {
                speech_data.append(&mut frame);

                if speech_data.len() >= MAX_BUFFER_SIZE {
                    if chunker_tx.send(take(&mut speech_data)).is_err() {
                        break
                    }    
                }
            } else if !speech_data.is_empty() {
                // If it's an empty frame and we have some speech in the buffer
                empty_frames += 1;

                // We wait until there's enough blank space before processing
                // This avoids pauses in the speech to be treated as two separate blocks
                if empty_frames < EMPTY_FRAMES_PROCESS_THRESHOLD {
                    continue
                }

                if chunker_tx.send(take(&mut speech_data)).is_err() {
                    break
                }
            }
        }
    }
}

fn frame_energy(frame: &Vec<f32>) -> f32 {
    frame.iter().map(|val| val * val).sum::<f32>()
}