mod flags;
mod to_telex;

use flags::VietTelex;
pub use to_telex::viet_telex;

use std::{io, io::Write};

pub fn main() -> io::Result<()> {
    let flags = VietTelex::from_env_or_exit();

    let content = std::fs::read_to_string(flags.viet)?;

    let output = viet_telex(&content);

    let mut output_file = std::fs::OpenOptions::new()
        .create_new(!flags.overwrite)
        .create(flags.overwrite)
        .truncate(flags.overwrite)
        .write(true)
        .open(flags.out)?;

    output_file.write_all(output.as_bytes())
}
