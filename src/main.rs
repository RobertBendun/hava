pub mod raw;

use crate::raw::class::Class;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("Hello.class")?;
    let mut class = Class::from(file)?;
    class.resolve_attributes()?;

    class.disassemble();

    Ok(())
}
