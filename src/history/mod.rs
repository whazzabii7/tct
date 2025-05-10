pub mod history_entry;
use history_entry::*;
use std::ptr;

pub struct History {
    name: String,
    date: String,
    pub entries: Vec<HistoryEntry>
}

impl History {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            date: String::new(),
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, text: &str, typing: &str) {
       self.entries.push(HistoryEntry::from(typing, text));
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_date(&mut self) {
       self.date = get_date(); 
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

}

#[cfg(unix)]
fn get_date() -> String {
    use libc::{localtime, time, time_t, tm};

    unsafe {
        let mut t: time_t = time(ptr::null_mut());
        let tm: *mut tm = localtime(&mut t);
        let tm_ref: tm = *tm;

        format!(
            "{:04}-{:02}-{:02}",
            tm_ref.tm_year + 1900,
            tm_ref.tm_mon + 1,
            tm_ref.tm_mday
        )
    }
}

#[cfg(windows)]
fn get_date() {
    use libc::{localtime, time, time_t, tm}; // Works with `libc` on Windows too

    unsafe {
        let mut t: time_t = time(ptr::null_mut());
        let tm: *mut tm = localtime(&mut t);
        let tm_ref: tm = *tm;

        format!(
            "Date: {:04}-{:02}-{:02}",
            tm_ref.tm_year + 1900,
            tm_ref.tm_mon + 1,
            tm_ref.tm_mday
        )
    }
}

