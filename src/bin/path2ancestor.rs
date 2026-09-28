use std::io;
use std::process::ExitCode;

use std::path::Path;

use rs_path2ancestor::Ancestor;

fn io_path() -> impl Fn() -> String {
    || std::env::var("ENV_PATH").unwrap_or_default()
}

fn io_main() -> impl Fn() -> Result<(), io::Error> {
    move || {
        let s: String = io_path()();
        let pat = Path::new(&s);
        let a: Ancestor = rs_path2ancestor::path2ancestor(pat);
        println!("{a:#?}");
        Ok(())
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

pub fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
