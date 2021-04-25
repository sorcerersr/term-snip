use term_snip::TermSnip;

use std::{thread, time};

/// Example showing text larger than one line splitted to use multiple lines.
///
/// Sample text is 155 chars long, so use a Terminal width less than 155 
/// with this example.
fn main() {
    let half_sec = time::Duration::from_millis(500);
    
    let mut term = TermSnip::new(5);
    for n in 1..15 {
        let some_text = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";
        term.write_line(&format!("{} - line number {} {}", n, n, some_text)).unwrap();
        
        // just to slow down for demonstration 
        thread::sleep(half_sec);
    }
}