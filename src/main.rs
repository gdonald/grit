use grit::run;
use std::env;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdout = io::stdout();

    if let Err(code) = run(&args, &mut stdout) {
        process::exit(code);
    }
}
