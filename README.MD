## Description

This repo is a partial wrapper for JoltPhysics physics engine.

**Please notice**. Currently, this is not a completely wrapper for JoltPhysics.

I create this repo beacuse I can't find any usable rust wrapper for JoltPhysics. But JoltPhysics is a full functional physics engine. I have no time to wrapper all APIs. So I only provide the API I need. For the above reasons, this repo will not be published to crates.io.

If anyone is interested in this project, you are welcome to contribute. I'd love to work with you to implement the completely JoltPhysics wrapper.

## How to ...

1. Follow [JoltPhysics documment](https://github.com/jrouwe/JoltPhysics/tree/master/Build) to build the project.
    - Add `add_compile_definitions(JPH_DISABLE_CUSTOM_ALLOCATOR)` at the end of `<your-dir>/JoltPhysics/Build/CMakeLists.txt` in JoltPhysics to disable customer allocator, which will conflict with cxx generated code.
    - On windows, pass `-DUSE_STATIC_MSVC_RUNTIME_LIBRARY=OFF` to cmake scripts to use dll version crt.
2. Set following environment variables:
    - JOLT_PHYSICS_INCLUDE: JoltPhysics's include path, such as `<your-dir>/JoltPhysics/`.
    - JOLT_PHYSICS_LIBRARY: JoltPhysics's library path, such as `<your-dir>/JoltPhysics/Build/VS2022_CL` (for MSVC).
3. `cargo build`
