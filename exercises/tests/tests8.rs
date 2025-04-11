fn main() {
    // We will instruct Cargo to enable the "pass" feature
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:feature=pass");
}
