extern crate cmake;
extern crate pkg_config;

use cmake::Config;


fn main() {
    let result = pkg_config::Config::new()
        .atleast_version("0.8.0")
        .statik(true)
        .probe("libucl");
    if result.is_err() {
        let dst = Config::new("libucl")
            .no_build_target(true)
            .build();
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=ucl");
    }
}
