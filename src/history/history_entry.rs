//! # === history_entry ===
//!
//! distincts user input from program output

use std::fmt;

/// Defines Entry type
#[derive(Debug)]
pub enum HistoryEntry  {
    CommandPrompt(String),
    OutputAnswer(String),
}

impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_sep = "==============================".repeat(3);
        match self {
            HistoryEntry::CommandPrompt(txt) => write!(f,
                "{sep}\nCommand: {}\n{sep}",
                txt, sep=line_sep
            ),
            HistoryEntry::OutputAnswer(txt) => write!(f,
                "{sep}\nOutput: {}\n{sep}",
                txt, sep=line_sep
            ),
        }
    }
}

impl HistoryEntry {
    /// creates entry instance from given type-String and text
    pub fn from(typing: &str, text: &str) -> Self {
        match typing {
            "CommandPrompt" => HistoryEntry::CommandPrompt(String::from(text)),
            "OutputAnswer" => HistoryEntry::OutputAnswer(String::from(text)),
            _ => panic!("Something went wrong! HistoryEntry::from"),
        }
    }
}
