use std::{io::{Cursor, Read}, process::Stdio, sync::{mpsc::Receiver, Arc}};

use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, Sink};

use crate::core::jarvis_signals::JarvisSignals;

pub fn main(signals: Arc<JarvisSignals>, feedback_rx: Receiver<String>) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    while !signals.is_shutdown() {
        let text = match feedback_rx.recv() {
            std::result::Result::Ok(str) => str,
            Err(_) => break
        };

        let audio_data = match get_audio_data(text) {
            Ok(data) => data,
            Err(_) => read_fallback_feedback()
        };
        let source = Decoder::new_wav(
            Cursor::new(audio_data)
        )?;

        signals.set_speaker_active(true);
        sink.append(source);
        sink.sleep_until_end();
        signals.set_speaker_active(false);
    }

    Ok(())
}

fn get_audio_data(text: String) -> Result<Vec<u8>> {
    let mut child = std::process::Command::new("tts")
        .args(&["--text", text.trim(), "--pipe_out"])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut audio_data = Vec::new();
    child
        .stdout
        .as_mut()
        .context("Could not get TTS stdout.")?
        .read_to_end(&mut audio_data)?;

    child.wait()?;

    Ok(audio_data)
}

fn read_fallback_feedback() -> Vec<u8> {
    Vec::new()
}