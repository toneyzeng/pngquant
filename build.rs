extern crate gcc;
extern crate pkg_config;

use std::env;

fn probe(lib: &str, ver: &str) -> Option<pkg_config::Library> {
    let statik = cfg!(feature = "static");
    let mut pkg = pkg_config::Config::new();
    pkg.atleast_version(ver);
    pkg.statik(statik);
    match pkg.probe(lib) {
        Ok(lib) => Some(lib),
        Err(pkg_config::Error::Failure{output,..}) => {
            println!("cargo:warning={}", String::from_utf8_lossy(&output.stderr).trim_right().replace("\n", "\ncargo:warning="));
            None
        },
        Err(err) => {
            println!("cargo:warning={:?}", err);
            None
        }
    }
}

fn main() {
    let libpng = probe("libpng", "1.4").unwrap();
    let mut cc = gcc::Config::new();

    // Muahahaha
    cc.define("main", Some("pngquant_main"));

    if cfg!(feature = "lcms2") {
        if let Some(lcms2) = probe("lcms2", "2.0") {
            for p in lcms2.include_paths {
                cc.include(p);
            }
            cc.define("USE_LCMS", Some("1"));
        }
    }

    if env::var("PROFILE").map(|p|p != "debug").unwrap_or(true) {
        cc.define("NDEBUG", Some("1"));
    }

    if cfg!(target_arch="x86_64") || cfg!(feature = "sse") {
        cc.define("USE_SSE", Some("1"));
    }
    cc.file("rwpng.c");
    cc.file("pngquant.c");

    for p in libpng.include_paths {
        cc.include(p);
    }

    cc.compile("libpngquant.a");
}
