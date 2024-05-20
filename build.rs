use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put linker script in the build directory
    File::create(out_dir.join("riscvp2.x"))?.write_all(include_bytes!("riscvp2.x"))?;

    // put p2 emulation code in the build directory
    File::create(out_dir.join("p2jit.o"))?.write_all(include_bytes!("p2jit.o"))?;

    // done
    Ok(())
}
