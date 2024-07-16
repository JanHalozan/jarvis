use std::path::PathBuf;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperError};

use super::constants::AUDIO_SAMPLE_RATE;

pub struct SpeechRecognizer {
    context: WhisperContext
}

// I don't like whisper's logging because I don't need it.
extern "C" fn silent_log_callback(
    _level: u32,
    _message: *const i8,
    _user_data: *mut std::ffi::c_void
) {
    //noop
}

impl SpeechRecognizer {
    pub fn new(model_path: &str) -> Self {
        unsafe {
            whisper_rs::set_log_callback(Some(silent_log_callback), std::ptr::null_mut());
        }

        let params = WhisperContextParameters::default();
        let context = WhisperContext::new_with_params(model_path, params)
            .expect("Unable to create SpeechRecognizer WhisperContext. Did you specify the correct path?");

        SpeechRecognizer {
            context
        }
    }
 
    pub fn default_model_path() -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("models")
            .join("ggml-model-whisper-tiny.en.bin")
            .to_str()
            .expect("No speech recognizer model found at the default path.")
            .to_owned()
    }

    /// Make sure audio is in 1 channel 16k sampling
    pub fn recognize(&self, audio: &[f32]) -> Result<String, WhisperError> {
        let mut state = self.context
            .create_state()?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_progress(false);
        params.set_single_segment(true);

        // If less than 1 second we need to pad
        let dif = (AUDIO_SAMPLE_RATE as i64) - (audio.len() as i64);
        if dif > 0 {
            let mut audio = audio.to_vec();
            audio.extend_from_slice(&vec![0.0; 16000]);
            state.full(params, &audio)?;
        } else {
            state.full(params, audio)?;
        }

        let num_segments = state.full_n_segments()?;
        let mut text = String::new();
        for i in 0..num_segments {
            match state.full_get_segment_text(i) {
                Ok(str) => text.push_str(&str),
                Err(_) => continue
            }
        }

        Ok(text.trim().to_string())
    }
}