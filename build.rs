fn main() {
    cc::Build::new()
        .file("asm/plc.S")
        .flag("-m64")
        .compile("plc");

    println!("cargo:rerun-if-changed=asm/plc.S");
    println!("cargo:rerun-if-changed=include/plc.h");
}
