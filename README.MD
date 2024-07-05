## Description

This repo is a partial wrapper for JoltPhysics physics engine.

**Please notice**. Currently, this is not a completely wrapper for JoltPhysics.

I create this repo beacuse I can't find any usable rust wrapper for JoltPhysics. But JoltPhysics is a full functional physics engine. I have no time to wrapper all APIs. So I only provide the API I need. For the above reasons, this repo will not be published to crates.io.

If anyone is interested in this project, you are welcome to contribute. I'd love to work with you to implement the completely JoltPhysics wrapper.

## How to use this repo

1. Run `git clone ...` to clone [jolt-physics-rs](https://github.com/SlimeYummy/jolt-physics-rs) from github.
2. Run `git submodule update --init` to fetch [JoltPhysics](https://github.com/jrouwe/JoltPhysics) as submodule.
3. Run `cargo build` to build the project. The first compilation will be a bit slow because we need to fully compile the JoltPhysics locally.
