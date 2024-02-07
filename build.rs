fn main() {
    // std::process::Command::new("sh")
    //     .args(&["build.sh"])
    //     .status()
    //     .unwrap();

    println!("cargo:rustc-link-search=native=spike-interfaces");

    println!("cargo:rustc-link-lib=dylib=riscv");
    println!("cargo:rustc-link-lib=dylib=customext");
    println!("cargo:rustc-link-lib=dylib=softfloat");

    println!("cargo:rustc-link-lib=dylib=spike-interfaces");
}
