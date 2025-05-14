//! # === history ===
//!
//! tracks a session of in an output
//! of configured executable.

pub mod history_entry;
use history_entry::*;

use crate::tct_error::TctError;
use std::ptr;

/// stores in and output, name and date as storage informations
pub struct History {
    name: String,
    date: String,
    pub entries: Vec<HistoryEntry>
}

impl History {
    /// default constructor
    pub fn new() -> Self {
        Self {
            name: String::new(),
            date: String::new(),
            entries: Vec::new(),
        }
    }

    /// appends entry to the end of list
    pub fn add_entry(&mut self, text: &str, typing: &str) {
       self.entries.push(HistoryEntry::from(typing, text));
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn set_date(&mut self) -> Result<(), TctError> {
       self.date = get_date()?; 
       Ok(())
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

}

// get_date to keep track of last modified
#[cfg(unix)] // linux variant
fn get_date() -> Result<String, TctError> {
    use libc::{localtime, time, time_t, tm};

    unsafe {
        let mut t: time_t = time(ptr::null_mut());
        let tm: *mut tm = localtime(&mut t);
        let tm_ref: tm = *tm;

        Ok(format!(
                "{:04}-{:02}-{:02}",
                tm_ref.tm_year + 1900,
                tm_ref.tm_mon + 1,
                tm_ref.tm_mday
        ))
    }
}

#[cfg(windows)] // windows variant
fn get_date() -> Result<String, TctError> {
    use libc::{localtime_s, time, time_t, tm}; // Works with `libc` on Windows, but localtime
                                               // intentionally not exists
    use std::mem::MaybeUninit;

    unsafe {
        let mut t: time_t = time(ptr::null_mut());
        let mut tm = MaybeUninit::<tm>::uninit();
        let ret = localtime_s(tm.as_mut_ptr(), &mut t);

        if ret == 0 {
            Ok(format!(
                    "{:04}-{:02}-{:02}",
                    (*tm.as_ptr()).tm_year + 1900,
                    (*tm.as_ptr()).tm_mon + 1,
                    (*tm.as_ptr()).tm_mday
            ))
        } else {
            Err(TctError::TimeNotFound)
        }

    }
}

