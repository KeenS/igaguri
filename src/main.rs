extern crate igaguri;

use igaguri::parse::Reader;
use igaguri::ast::{Input, Ast};
use igaguri::igaguri::Igaguri;

fn main() {
    // `()` can be used when no completer is required

    let mut igaguri = Igaguri::new();
    igaguri.repl();
}
