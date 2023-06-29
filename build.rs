fn main() {
    println!("cargo:rustc-link-lib=static=wrapper");
    println!("cargo:rustc-link-search={}/wrapper/build/lib", std::env::var("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rustc-link-lib=dylib=stdc++");
}