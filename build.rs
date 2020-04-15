fn main() {
    if let Some(datastax_dir) = option_env!("CASSANDRA_SYS_LIB_PATH") {
        for p in datastax_dir.split(";") {
            println!("cargo:rustc-link-search={}", p);
        }
    }

    println!("cargo:rustc-flags=-l static=cassandra_static");
    println!("cargo:rustc-flags=-l static=z");
    println!("cargo:rustc-flags=-l static=crypto");
    println!("cargo:rustc-flags=-l static=ssl");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-flags=-l dylib=c++");
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=static-nobundle=stdc++");
    println!("cargo:rustc-flags=-l static=uv");
    println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search={}", "/usr/local/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search={}", "/usr/local/lib64");
    println!("cargo:rustc-link-search={}", "/usr/local/lib");
    println!("cargo:rustc-link-search={}", "/usr/lib64/");
    println!("cargo:rustc-link-search={}", "/usr/lib/");
    println!("cargo:rustc-link-search={}", "/usr/local/opt/openssl/lib");
}
