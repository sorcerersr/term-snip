use std::{io, usize};

use console::Term;


pub struct TermSnip {
    term: Term,
    limit: usize, 
    lines: Vec<String>,
    
}


impl TermSnip{
    pub fn new(limit: usize) -> TermSnip {
        TermSnip{ term: Term::stdout(), limit, lines: Vec::new()}
    }

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




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
