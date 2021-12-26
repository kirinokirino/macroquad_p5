#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

#[macro_use]
extern crate log;
use simple_logger::SimpleLogger;

use log::Level;

fn main() {
    SimpleLogger::new().init().unwrap();

    debug!("this is a debug {}", "message");
    if log_enabled!(Level::Info) {
        let x = fastrand::i32(1..20); // expensive computation
        info!("the answer was: {}", x);
    }
}
