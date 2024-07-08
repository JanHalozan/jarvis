mod core;
mod processing;
mod model;
mod traits;
mod errors;

use std::{sync::{mpsc::channel, Arc}, time::Duration};
use processing::classifier::ClassifierOutput;
use core::jarvis_signals::JarvisSignals;
use tokio::{signal, task::JoinSet};

#[tokio::main]
async fn main() {
    let signals = Arc::new(JarvisSignals::new());
    let mut thread_pool = JoinSet::new();

    let (mic_tx, mic_rx) = channel::<Vec<f32>>();
    let mic_signals = signals.clone();
    let mic_shutdown_signals = signals.clone();
    thread_pool.spawn_blocking(move || {
        processing::microphone_listener::main(mic_signals, mic_tx)
            .map_err(|e| mic_shutdown_signals.set_shutdown(Some(e)))
            .ok();

        println!("Microphone listener shutting down");
    });

    let (chunker_tx, chunker_rx) = channel::<Vec<f32>>();
    thread_pool.spawn(async move {
        processing::vad_chunker::main(mic_rx, chunker_tx);
        println!("VAD chunker shutting down");
    });

    let (detector_tx, detector_rx) = channel::<Vec<f32>>();
    let detector_signals = signals.clone();
    thread_pool.spawn(async move {
        processing::wake_word_detector::main(chunker_rx, detector_tx)
            .map_err(|e| detector_signals.set_shutdown(Some(e)))
            .ok();
        println!("Wake word detector shutting down");
    });

    let (recognizer_tx, recognizer_rx) = channel::<String>();
    thread_pool.spawn(async move {
        processing::recognizer::main(detector_rx, recognizer_tx);
        println!("Speech recognizer shutting down");
    });

    let (classifier_tx, classifier_rx) = channel::<ClassifierOutput>();
    let classifier_signals = signals.clone();
    thread_pool.spawn_blocking(move || {
        processing::classifier::main(recognizer_rx, classifier_tx)
            .map_err(|e| classifier_signals.set_shutdown(Some(e)))
            .ok();
        println!("Classifier shutting down");
    });

    let (executor_tx, executor_rx) = channel::<ClassifierOutput>();
    thread_pool.spawn(async move {
        processing::intent_executor::main(classifier_rx, executor_tx);
        println!("Command executor shutting down");
    });

    let (feedback_tx, feedback_rx) = channel::<String>();
    let feedback_signals = signals.clone();
    thread_pool.spawn_blocking(move || {
        processing::feedback_generator::main(executor_rx, feedback_tx)
            .map_err(|e| feedback_signals.set_shutdown(Some(e)))
            .ok();
        println!("Feedback generator shutting down");
    });

    let speech_signals = signals.clone();
    let speech_shutdown_signals = signals.clone();
    thread_pool.spawn_blocking(move || {
        processing::speech_synthesizer::main(speech_signals, feedback_rx)
            .map_err(|e| speech_shutdown_signals.set_shutdown(Some(e)))
            .ok();

        println!("Speech synthesiser shutting down");
    });

    let shutdown_signal = signals.clone();
    thread_pool.spawn(async move {
        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    shutdown_signal.set_shutdown(None);
                    break;
                },
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    if shutdown_signal.is_shutdown() {
                        break;
                    }
                }
            }
        }
    });

    println!("Listening for speech");

    while !signals.is_shutdown() {
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Terminating auxiliary threads...");

    while let Some(_) = thread_pool.join_next().await {}

    println!("\nAux threads terminated. Exiting...");
}