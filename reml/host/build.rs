//! Build script for reml-host
//!
//! This compiles the guest program to an ELF binary that can be run in SP1

fn main() {
    sp1_build::build_program("../guest");
}
