use std::path::Path;
use std::{env, fs};

fn main() {
    // let is_debug = env::var("PROFILE").map(|p| p == "debug").unwrap_or(true);
    let is_windows = env::var_os("CARGO_CFG_WINDOWS").is_some();
    let is_unix = env::var_os("CARGO_CFG_UNIX").is_some();
    let is_clang = env::var("CC")
        .map(|cc| cc == "clang" || cc == "clang-cl")
        .unwrap_or(false);
    let is_deterministic = env::var("CARGO_FEATURE_DETERMINISTIC").is_ok();
    let is_profile = env::var("CARGO_FEATURE_PROFILE").is_ok();
    let is_debug_renderer = env::var("CARGO_FEATURE_DEBUG_RENDERER").is_ok();
    let is_debug_print = env::var("CARGO_FEATURE_DEBUG_PRINT").is_ok();

    let mut rs_file = vec![
        "src/base.rs",
        "src/layer.rs",
        "src/shape.rs",
        "src/body.rs",
        "src/system.rs",
        "src/character.rs",
        "src/test_callback.rs",
    ];

    let mut cpp_file = vec![
        "src/shape.cpp",
        "src/body.cpp",
        "src/system.cpp",
        "src/character.cpp",
        "src/test_callback.cpp",
    ];

    println!("cargo:rerun-if-changed=src/ffi.h");
    println!("cargo:rerun-if-changed=src/shape.cpp");
    println!("cargo:rerun-if-changed=src/body.cpp");
    println!("cargo:rerun-if-changed=src/system.cpp");
    println!("cargo:rerun-if-changed=src/character.cpp");
    println!("cargo:rerun-if-changed=src/test_callback.cpp");

    if is_windows && is_debug_renderer {
        rs_file.push("src/debug.rs");
        cpp_file.push("src/debug.cpp");
        println!("cargo:rerun-if-changed=src/debug.h");
        println!("cargo:rerun-if-changed=src/debug.cpp");
    }

    let mut cxx = cxx_build::bridges(&rs_file);
    if is_clang {
        cxx.compiler("clang-cl");
    }

    cxx.cpp(true)
        .std("c++17")
        .static_crt(false)
        .files(&cpp_file)
        .include("./JoltPhysics")
        .files(list_source_files("./JoltPhysics/Jolt"))
        .define("RUST_CXX_NO_EXCEPTIONS", None)
        .define("JPH_DISABLE_CUSTOM_ALLOCATOR", "1")
        .define("JPH_OBJECT_LAYER_BITS", "32")
        .define("NDEBUG", "1")
        .define("JPH_USE_SSE4_1", "1")
        .define("JPH_USE_SSE4_2", "1")
        .define("JPH_USE_LZCNT", "1")
        .define("JPH_USE_TZCNT", "1");
    // .define("JPH_USE_F16C", "1")

    if is_profile {
        cxx.define("JPH_PROFILE_ENABLED", "1");
    }

    if is_debug_print {
        cxx.define("JPH_DEBUG_PRINT", "1");
    }

    if is_windows && is_debug_renderer {
        cxx.include("./JoltPhysics/TestFramework")
            .files(list_source_files("./JoltPhysics/TestFramework"))
            .define("JPH_DEBUG_RENDERER", "1");
    }

    if is_windows && !is_clang {
        #[cfg(windows)]
        cxx.includes(vcvars::Vcvars::new().get("INCLUDE").unwrap().split(';'))
            .define("JPH_COMPILER_MSVC", "1")
            .flag_if_supported("/Zc:__cplusplus")
            .flag_if_supported("/Gm-")
            .flag_if_supported("/nologo")
            .flag_if_supported("/diagnostics:classic")
            .flag_if_supported("/fp:except-")
            .flag_if_supported("/Zc:inline")
            .flag_if_supported("/GR-")
            .flag_if_supported("/wd4577")
            .flag_if_supported("/arch:SSE2")
            .flag_if_supported("/arch:SSE4.2");

        // if is_debug {
        //     cxx.flag_if_supported("/GS")
        //         .flag_if_supported("/Od")
        //         .flag_if_supported("/Ob0")
        //         .flag_if_supported("/RTC1");
        // } else {
        //     cxx.flag_if_supported("/GS-")
        //         .flag_if_supported("/O2")
        //         .flag_if_supported("/Oi")
        //         .flag_if_supported("/Ot");
        // }

        if is_deterministic {
            cxx.flag_if_supported("/fp:precise");
        } else {
            cxx.flag_if_supported("/fp:fast");
        }

        println!("cargo:rustc-link-lib=User32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=Shcore");
        println!("cargo:rustc-link-lib=DXGI");
        println!("cargo:rustc-link-lib=D3D12");
        println!("cargo:rustc-link-lib=d3dcompiler");
        println!("cargo:rustc-link-lib=Dinput8");
        println!("cargo:rustc-link-lib=dxguid");
    }

    if is_unix || is_clang {
        cxx.flag_if_supported("-Wall")
            .flag_if_supported("-Werror")
            .flag_if_supported("-fno-rtti")
            .flag_if_supported("-fno-exceptions")
            .flag_if_supported("-Wno-stringop-overflow")
            .flag_if_supported("-mbmi")
            .flag_if_supported("-mbmi")
            .flag_if_supported("-mbmi")
            .flag_if_supported("-mbmi")
            .flag_if_supported("-mbmi")
            .flag_if_supported("-mbmi")
            .flag_if_supported("-mpopcnt")
            .flag_if_supported("-mlzcnt")
            .flag_if_supported("-msse2")
            .flag_if_supported("-msse4.1")
            .flag_if_supported("-msse4.2")
            .flag_if_supported("-mno-avx2")
            .flag_if_supported("-mno-avx512")
            .flag_if_supported("-flto=thin");
        // .flag_if_supported("-mf16c")

        if is_deterministic {
            cxx.flag_if_supported("-ffp-model=precise")
                .flag_if_supported("-ffp-contract=off");
        }
    }

    replace_code_in_cpp(
        "./JoltPhysics/Jolt/Physics/Collision/BroadPhase/BroadPhaseLayer.h",
        "virtual BroadPhaseLayer			GetBroadPhaseLayer(ObjectLayer inLayer) const = 0;",
        "virtual BroadPhaseLayer::Type		GetBroadPhaseLayer(ObjectLayer inLayer) const = 0;",
    );

    replace_code_in_cpp(
        "./JoltPhysics/Jolt/Physics/Body/BodyManager.h",
        "ioBody.mBroadPhaseLayer = mBroadPhaseLayerInterface->GetBroadPhaseLayer(inLayer);",
        "ioBody.mBroadPhaseLayer = (BroadPhaseLayer)mBroadPhaseLayerInterface->GetBroadPhaseLayer(inLayer);",
    );

    cxx.compile("jolt-physics-rs");
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
    files
}

fn replace_code_in_cpp(path: &str, from: &str, to: &str) {
    let mut content = fs::read_to_string(path).unwrap();
    content = content.replace(from, to);
    fs::write(path, content).unwrap();
}
