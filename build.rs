fn main() {
  std::process::Command::new("sh")
    .args(&["build.sh"])
    .status()
    .unwrap();

  println!("cargo:rustc-link-search=native=libspike/target");
  println!("cargo:rustc-link-search=native=libspike/include");

  println!("cargo:rustc-link-lib=static=riscv");
  println!("cargo:rustc-link-lib=static=softfloat");
  println!("cargo:rustc-link-lib=static=disasm");

  println!("cargo:rustc-link-lib=dylib=stdc++");
}
