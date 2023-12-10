use std::env;
use vcvars::Vcvars;

fn main() {
    let debug = match env::var("PROFILE").unwrap().as_str() {
        "debug" => true,
        "release" => false,
        _ => unreachable!(),
    };

    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");

    let include_path = env::var("JOLT_PHYSICS_INCLUDE").unwrap_or("./JoltPhysics".into());
    let library_path = env::var("JOLT_PHYSICS_LIBRARY").unwrap_or("./JoltPhysics/Build/VS2022_CL/Release".into());

    let mut cxx = cxx_build::bridges([
        "src/base.rs",
        "src/layer.rs",
        "src/shape.rs",
        "src/system.rs",
        "src/character.rs",
        "src/debug.rs",
    ]);
    cxx.files(&["src/shape.cpp", "src/system.cpp", "src/character.cpp", "src/debug.cpp"])
        .cpp(true)
        .std("c++17")
        .static_crt(false)
        .include(&include_path)
        .include(format!("{}/TestFramework", &include_path))
        .define("JPH_DISABLE_CUSTOM_ALLOCATOR", "1");

    if windows {
        cxx.includes(Vcvars::new().get("INCLUDE").unwrap().split(';'));
    }

    if env::var("CARGO_FEATURE_DETERMINISTIC").is_ok() {
        cxx.flag_if_supported("/fp:precise")
            .flag_if_supported("-ffp-model=precise")
            .flag_if_supported("-ffp-contract=off");
    }

    if !debug {
        cxx.define("CRITICAL_POINT_RELEASE", "1");
    }
    cxx.compile("jolt-physics-rs");

    println!("cargo:rustc-link-lib={}/Jolt", library_path);
    println!("cargo:rustc-link-lib={}/TestFramework", library_path);

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

    println!("cargo:rerun-if-changed=src/*");
}
