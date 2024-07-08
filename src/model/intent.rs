use crate::traits::labelable::Labelable;

use super::command::Command;

#[derive(Debug)]
pub enum Intent {
    Command(Command),
    Question(String)
}

impl Intent {
    pub fn is_label_question(label: &str) -> bool {
        label == "question"
    }
}

impl Labelable for Intent {
    fn from_label(label: &str) -> Self {
        unimplemented!("{}", label);
    }

    fn labels() -> Vec<String> {
        vec!["command".to_string(), "question".to_string()]
    }
}