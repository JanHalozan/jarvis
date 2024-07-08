use std::sync::atomic::AtomicBool;

use anyhow::Error;

pub struct JarvisSignals {
    speaker_active: AtomicBool,
    shutdown: AtomicBool
}

impl JarvisSignals {
    pub fn new() -> Self {
        JarvisSignals {
            speaker_active: AtomicBool::new(false),
            shutdown: AtomicBool::new(false)
        }
    }

    pub fn is_speaker_active(&self) -> bool {
        self.speaker_active.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn set_speaker_active(&self, active: bool) {
        self.speaker_active.store(active, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_shutdown(&self, reason: Option<Error>) {
        if let Some(error) = reason {
            eprintln!("Terminating due to an unexpected error: {:?}", error);
        }

        self.shutdown.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}