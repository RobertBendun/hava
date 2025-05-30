pub mod raw;

use crate::raw::class::Class;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let Ok(file) = File::open("Hello.class") else {
        eprintln!("Couldn't open Hello.class file");
        std::process::exit(1)
    };
    let mut class = Class::from(file)?;
    class.resolve_attributes()?;

    class.disassemble();

    Ok(())
}
