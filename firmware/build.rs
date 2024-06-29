use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let version: Vec<&str> = env!("CARGO_PKG_VERSION").split_terminator('.').collect();
    let version = format!(
        "
pub const MAJOR: u8 = {:};
pub const MINOR: u8 = {:};
pub const BUGFIX: u8 = {:};
pub const BUILD_TS: u32 = {:};",
        version[0], version[1], version[2], since_the_epoch
    );
    let out = &PathBuf::from("src");
    File::create(out.join("version.rs"))
        .unwrap()
        .write_all(version.as_bytes())
        .unwrap();
}
