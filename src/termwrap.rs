//! Wraps external dependency console::Term in order to be replaced
//! by mock when running tests.
//!
//!
use std::io;

use console::Term;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait TermWrap {
    fn write_line(&self, s: &str) -> io::Result<()>;
    fn clear_last_lines(&self, n: usize) -> io::Result<()>;
    fn size(&self) -> (u16, u16);
}

pub struct ConsoleTermWrap {
    term: Term,
}

impl TermWrap for ConsoleTermWrap {
    fn write_line(&self, s: &str) -> io::Result<()> {
        return self.term.write_line(s);
    }

    fn clear_last_lines(&self, n: usize) -> io::Result<()> {
        return self.term.clear_last_lines(n);
    }

    fn size(&self) -> (u16, u16) {
        return self.term.size();
    }
}

impl ConsoleTermWrap {
    pub fn new() -> ConsoleTermWrap {
        ConsoleTermWrap {
            term: Term::stdout(),
        }
    }
}
