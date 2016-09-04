extern crate igaguri;
extern crate env_logger;

use igaguri::igaguri::Igaguri;

fn main() {
    env_logger::init().unwrap();

    let mut igaguri = Igaguri::new();
    igaguri.repl();
}
