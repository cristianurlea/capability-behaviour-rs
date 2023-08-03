// use std::env;

fn main() {

    let library_path = std::path::Path::new(
        "/builder/cheri/output/rootfs-morello-purecap/usr/include"
    );
    
    // let inc_path = std::path::Path::new(
    //     "/builder/cheribsd/lib"
    // );   

    cc::Build::new()
    .opt_level(0)
    .file("src/c/lib.c")
    .compiler("/builder/cheri/output/morello-sdk/bin/clang")
    .target("aarch64-unknown-freebsd-purecap")
    .flag("-march=morello+c64")
    .flag("-mabi=purecap")
    // .include(inc_path)
    .include(library_path)
    .compile("containercclib");
}