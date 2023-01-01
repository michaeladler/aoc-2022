fn main() {
    cc::Build::new().file("src/day20/helper.c").compile("day20");
    println!("cargo:rerun-if-changed=src/day20/helper.h");
    println!("cargo:rerun-if-changed=src/day20/helper.c");
    println!("cargo:rerun-if-changed=build.rs");
}
