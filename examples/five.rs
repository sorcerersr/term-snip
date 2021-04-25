use term_snip::TermSnip;

use std::{thread, time};

/// A simple example writing 15 lines to stdout but only showing
/// a maximum of five lines. 
fn main() {
    let half_sec = time::Duration::from_millis(500);
    
    let mut term = TermSnip::new(5);
    for n in 1..15 {
        term.write_line(&format!("{} - line number {}", n, n)).unwrap();
        
        // just to slow down for demonstration 
        thread::sleep(half_sec);
    }
}