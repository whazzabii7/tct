pub enum EntryType {
    CommandPrompt,
    OutputAnswer
}

pub struct HistoryEntry {
    typing: EntryType,
    text: String
}
