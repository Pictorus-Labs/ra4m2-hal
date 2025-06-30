use std::env;
use std::fs;
use std::path::PathBuf;

// The memory.x file for the RA4M2 needs to be linked. This script copies the existing 
// memory.x file to the OUT_DIR and lets the linker know where to find it. 
// Embassy generates this file automatically, while we are just copying it for now.
// https://github.com/embassy-rs/embassy/blob/main/embassy-stm32/build.rs

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Assume memory.x is at the root of this crate
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("memory.x");
    let dest = out.join("memory.x");

    fs::copy(&source, &dest).expect("Could not copy memory.x");

    // Tell the linker to look in this OUT_DIR — this *will* propagate
    println!("cargo:rustc-link-search={}", out.display());

    // Rebuild if memory.x changes
    println!("cargo:rerun-if-changed={}", source.display());
}