use std::io;
use std::time::Duration;

use crossterm::event::{poll, read};

mod test {

    #[cfg(test)]
    fn print_events() -> io::Result<bool> {
        loop {
            print!("{:?}", read()?);
        }
    }
}
