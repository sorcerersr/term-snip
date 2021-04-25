use term_snip::TermSnip;

use std::{thread, time};

/// Writting some lines using TermSnip and then clearing them.
fn main() {
    // note: sleeps are just to slow down for demonstration 
    let mut term = TermSnip::new(5);
    for n in 1..6 {
        term.write_line(&format!("{} - line number {}", n, n)).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
     
     thread::sleep(time::Duration::from_secs(1));

     term.clear_lines().unwrap();
     println!("cleared...");
     
     thread::sleep(time::Duration::from_secs(2));
}