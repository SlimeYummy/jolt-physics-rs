use std::path::Path;
use std::{env, fs};
use vcvars::Vcvars;

fn main() {
    let debug = match env::var("PROFILE").unwrap().as_str() {
        "debug" => true,
        "release" => false,
        _ => unreachable!(),
    };

    let mut cxx = cxx_build::bridges([
        "src/base.rs",
        "src/layer.rs",
        "src/shape.rs",
        "src/system.rs",
        "src/character.rs",
        "src/debug.rs",
    ]);

    cxx.cpp(true)
        .std("c++17")
        .static_crt(false)
        .files(&["src/shape.cpp", "src/system.cpp", "src/character.cpp", "src/debug.cpp"])
        .include("./JoltPhysics")
        .files(&list_source_files("./JoltPhysics/Jolt"))
        .include("./JoltPhysics/TestFramework")
        .files(&list_source_files("./JoltPhysics/TestFramework"))
        .define("JPH_DISABLE_CUSTOM_ALLOCATOR", "1")
        .define("NDEBUG", "1")
        .define("JPH_USE_AVX2", "1")
        .define("JPH_USE_AVX", "1")
        .define("JPH_USE_SSE4_1", "1")
        .define("JPH_USE_SSE4_2", "1")
        .define("JPH_USE_LZCNT", "1")
        .define("JPH_USE_TZCNT", "1")
        .define("JPH_USE_F16C", "1")
        .flag_if_supported("-march=native");

    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");
    if windows {
        cxx.includes(Vcvars::new().get("INCLUDE").unwrap().split(';'))
            .define("JPH_COMPILER_MSVC", "1");
    }

    if env::var("CARGO_FEATURE_DETERMINISTIC").is_ok() {
        cxx.flag_if_supported("/fp:precise")
            .flag_if_supported("-ffp-model=precise")
            .flag_if_supported("-ffp-contract=off")
            .define("JPH_CROSS_PLATFORM_DETERMINISTIC", "1");
    }

    if debug {
        cxx.define("JPH_PROFILE_ENABLED", "1").define("JPH_DEBUG_RENDERER", "1");
    } else {
        cxx.define("CRITICAL_POINT_RELEASE", "1");
    }
    cxx.compile("jolt-physics-rs");

    if windows {
        println!("cargo:rustc-link-lib=User32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=Shcore");
        println!("cargo:rustc-link-lib=DXGI");
        println!("cargo:rustc-link-lib=D3D12");
        println!("cargo:rustc-link-lib=d3dcompiler");
        println!("cargo:rustc-link-lib=Dinput8");
        println!("cargo:rustc-link-lib=dxguid");
    }

    // println!("cargo:rerun-if-changed=src/*");
}

fn list_source_files<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path = path.to_str().unwrap().to_string();
            if path.ends_with(".cpp") || path.ends_with(".cc") {
                files.push(path);
            }
        } else if path.is_dir() {
            files.extend(list_source_files(path));
        }
    }
    return files;
}
