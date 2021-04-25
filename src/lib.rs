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
//!
//!Clearing the written lines afterwards (```cargo run --example clear```)
//!
//!![Screenshot of example clear.rs](https://gitlab.com/sorcerersr/term-snip/-/raw/master/screenshot/example_clear.gif)
//!

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

        // split text into multiple text when it is longer than a line
        let line_len:usize = self.term.size().1.into();
        
        // if the char count of the text is less than the line length
        // just write it and return
        if text.chars().count() < line_len {
            self.term_write_line(text)?;
            return Ok(())
        }
        // when this code line is reached the text is larger then the line length
        // and must be splitted to be written as multiple lines

        let mut last_text = text;
        while last_text.chars().count() >= line_len {
            let (first, last) = last_text.split_at(line_len);
            last_text = last;
            self.term_write_line(first)?;
        }
        self.term_write_line(last_text)?;

        Ok(())
    }


    /// Delegates the writing to console::Term and manages the line limit.
    fn term_write_line(&mut self, text: &str) -> io::Result<()> {

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


    /// Clear the lines written with the TermSnip instance
    pub fn clear_lines(&mut self) -> io::Result<()> {
        let len = &self.lines.len();
        self.term.move_cursor_up(*len)?;
        
        for _n in 0..*len {
            self.term.clear_line()?;
            self.term.move_cursor_down(1)?;
        }
        self.term.move_cursor_up(*len+1)?;
        self.lines.clear();
        self.term.move_cursor_down(1)?;
        Ok(())
    }

}
