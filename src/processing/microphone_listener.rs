use std::{sync::{mpsc::Sender, Arc}, time::Duration};

use anyhow::{Ok, Result};
use cpal::{traits::{HostTrait, StreamTrait}, InputCallbackInfo, SampleRate, StreamConfig};
use rodio::DeviceTrait;

use crate::{core::{constants::{AUDIO_SAMPLE_RATE, MIC_SAMPLE_RATE}, jarvis_signals::JarvisSignals}, errors::jarvis_error::JarvisError};

const INPUT_STREAM_CONFIG: StreamConfig = StreamConfig {
    channels: 1,
    sample_rate: SampleRate(MIC_SAMPLE_RATE),
    buffer_size: cpal::BufferSize::Default
};

pub fn main(signals: Arc<JarvisSignals>, microphone_tx: Sender<Vec<f32>>) -> Result<()> {

    let data_signals = signals.clone();
    let data_callback = move |data: &[f32], _: &InputCallbackInfo| {
        if data_signals.is_speaker_active() || data_signals.is_shutdown() {
            return;
        }

        let resampled = resample_audio(
            data,
            MIC_SAMPLE_RATE as usize,
            AUDIO_SAMPLE_RATE
        );
        
        if let Err(e) = microphone_tx.send(resampled) {
            // If we can't propagate mic anymore it doesn't make sense to stay alive
            data_signals.set_shutdown(Some(e.into())); 
        }
    };

    let error_signals = signals.clone();
    let error_callback = move |error: cpal::StreamError| {
        match error {
            cpal::StreamError::DeviceNotAvailable => error_signals.set_shutdown(Some(error.into())),
            _ => eprintln!("Capture stream error {:?}", error)
        };
    };

    let stream = cpal::default_host()
        .default_input_device()
        .ok_or(JarvisError::no_mic())?
        .build_input_stream(
            &INPUT_STREAM_CONFIG,
            data_callback,
            error_callback,
            None)?;

    stream.play()?;

    while !signals.is_shutdown() {
        std::thread::sleep(Duration::from_millis(100));
    }

    stream.pause()?;

    Ok(())
}

fn resample_audio(input: &[f32], input_rate: usize, output_rate: usize) -> Vec<f32> {
    let factor = input_rate / output_rate;
    let cutoff = output_rate as f32 / 2.0;

    let rc = 1.0 / (cutoff * 2.0 * std::f32::consts::PI);
    let dt = 1.0 / (input_rate as f32);
    let alpha = dt / (rc + dt);

    let mut output = Vec::with_capacity(input.len() / factor);
    let mut previous = input[0];
    let mut index = 0;

    while index < input.len() {
        let filtered_sample = previous + alpha * (input[index] - previous);
        if index % factor == 0 {
            output.push(filtered_sample);
        }
        previous = filtered_sample;
        index += 1;
    }

    output
}