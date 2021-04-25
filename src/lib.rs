//! # term-snip
//!
//! `term-snip` A small utility crate using https://crates.io/crates/console 
//! to write to stdout but limited to a given number of lines. 
//! The oldest line is removed when writing a new line.
//!
//!## Usage
//!
//!From *examples/five.rs*:
//!
//!```rust
//!use term_snip::TermSnip;
//!
//!use std::{thread, time};
//!
//!/// A simple example writing 15 lines to stdout but only showing
//!/// a maximum of five lines. 
//!fn main() {
//!    let half_sec = time::Duration::from_millis(500);
//!    
//!    let mut term = TermSnip::new(5);
//!    for n in 1..15 {
//!        term.write_line(&format!("{} - line number {}", n, n)).unwrap();
//!        
//!        // just to slow down for demonstration 
//!        thread::sleep(half_sec);
//!    }
//!}
//! 
//!```
//!
//!
//!## Screenshot
//!
//!Screenshot showing above example in action
//!
//!![Screenshot of example five.rs](https://gitlab.com/sorcerersr/term-snip/-/raw/master/screenshot/example_five.gif)


use std::{io, usize};

use console::Term;

/// Representation of a terminal.
pub struct TermSnip {
    term: Term,
    limit: usize, 
    lines: Vec<String>,
    
}


impl TermSnip{
    /// Creates a TermSnip wich limits output lines to the given limit.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut term = TermSnip::new(5);
    /// ```
    ///
    pub fn new(limit: usize) -> TermSnip {
        TermSnip{ term: Term::stdout(), limit, lines: Vec::new()}
    }

    /// Writes a line to the terminal (stdout).
    pub fn write_line(&mut self, text: &str) -> io::Result<()> {

        self.lines.push(text.to_string());
        if self.lines.len() > self.limit {
            self.term.move_cursor_up(self.limit)?;
            self.lines.remove(0);
            for line in &self.lines {
                self.term.write_line(line)?;
            }
        } else {
            self.term.write_line(text)?;
        }        
        Ok(())
    }
}
