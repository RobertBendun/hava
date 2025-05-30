pub mod raw;

use crate::raw::class::Class;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let _ = args.next();

    let Some(filename) = args.next() else {
        eprintln!("please provide class file as a program argument");
        std::process::exit(2)
    };

    let Ok(file) = File::open(filename) else {
        eprintln!("Couldn't open Hello.class file");
        std::process::exit(1)
    };
    let mut class = Class::from(file)?;
    class.resolve_attributes()?;

    class.disassemble();

    Ok(())
}
