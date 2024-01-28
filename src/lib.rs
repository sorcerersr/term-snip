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

use std::{collections::VecDeque, io, usize};

mod termwrap;
use termwrap::{ConsoleTermWrap, TermWrap};

/// Representation of a terminal.
pub struct TermSnip<'a> {
    term: Box<dyn TermWrap + 'a>,
    limit: usize,
    lines: VecDeque<String>,
}

impl<'a> TermSnip<'a> {
    /// Creates a TermSnip wich limits output lines to the given limit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use term_snip::TermSnip;
    /// let mut term = TermSnip::new(5);
    /// ```
    ///
    pub fn new(limit: usize) -> TermSnip<'a> {
        TermSnip {
            term: Box::new(ConsoleTermWrap::new()),
            limit,
            lines: VecDeque::new(),
        }
    }

    /// Writes a line to the terminal (stdout).
    pub fn write_line(&mut self, text: &str) -> io::Result<()> {
        // split text into multiple text when it is longer than a line
        let line_len: usize = self.term.size().1.into();
        // if the char count of the text is less than the line length
        // just write it and return
        if text.chars().count() < line_len {
            self.term_write_line(text)?;
            return Ok(());
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
        self.lines.push_back(text.to_string());
        if self.lines.len() > self.limit {
            self.lines.pop_front();
            self.clear_lines()?;
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
        self.term.clear_last_lines(self.lines.len())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    #[test]
    fn test_simple_one_line() {
        let mut termwrap_mock = termwrap::MockTermWrap::new();
        termwrap_mock.expect_size().returning(|| (20, 20));
        termwrap_mock
            .expect_write_line()
            .with(eq("test"))
            .times(1)
            .returning(|_x| Ok(()));

        let mut term_snip = TermSnip {
            term: Box::new(termwrap_mock),
            limit: 5,
            lines: VecDeque::new(),
        };

        term_snip.write_line("test").unwrap();
    }

    #[test]
    fn test_six_lines_with_limit_5() {
        let mut termwrap_mock = termwrap::MockTermWrap::new();
        termwrap_mock.expect_size().returning(|| (20, 20));
        // 10 calls to write_line are expected.
        // what happens is:
        // * lines 0,1,2,3,4 are written to terminal, limit of 5 is reached
        // * next line is about to be written, after limit, so clear_lines is called
        // * lines 1,2,3,4 are written again, so it looks like they are moving
        //   up one line
        // * line 5 is written
        // in total 10 calls to write_line
        termwrap_mock
            .expect_write_line()
            .times(10)
            .returning(|_x| Ok(()));
        termwrap_mock
            .expect_clear_last_lines()
            .with(eq(5))
            .times(1)
            .returning(|_x| Ok(()));

        let mut term_snip = TermSnip {
            term: Box::new(termwrap_mock),
            limit: 5,
            lines: VecDeque::new(),
        };

        // write six lines
        for _n in 0..6 {
            term_snip.write_line("test").unwrap();
        }
    }

    #[test]
    fn test_long_line_split() {
        let mut termwrap_mock = termwrap::MockTermWrap::new();
        // line length of terminal is mocked to 6
        termwrap_mock.expect_size().returning(|| (20, 6));
        // testcase is one long line that needs to be splitted to three lines
        // due to its length
        termwrap_mock
            .expect_write_line()
            .times(3)
            .returning(|_x| Ok(()));

        let mut term_snip = TermSnip {
            term: Box::new(termwrap_mock),
            limit: 5,
            lines: VecDeque::new(),
        };

        // sample text is 17 chars long which should lead to
        // three lines as line length is 6
        let testline = "Lorem ipsum dolor";
        term_snip.write_line(testline).unwrap();
    }
}
