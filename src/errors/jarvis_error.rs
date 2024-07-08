use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug)]
pub enum JarvisErrorReason {
    NoMicrophone
}

pub struct JarvisError {
    reason: JarvisErrorReason
}

impl JarvisError {
    pub fn no_mic() -> Self {
        JarvisError {
            reason: JarvisErrorReason::NoMicrophone
        }
    }
}

impl Display for JarvisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self.reason {
            JarvisErrorReason::NoMicrophone => "No microphone found.",
        };

        write!(f, "{}", message)
    }
}

impl Debug for JarvisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jarvis error {{ reason: {:?}, message: {} }}", self.reason, self.to_string())
    }
}

impl Error for JarvisError {
}