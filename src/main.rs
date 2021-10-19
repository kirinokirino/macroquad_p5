#[macro_use] extern crate log;
extern crate env_logger;

use log::Level;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    debug!("this is a debug {}", "message");
    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
