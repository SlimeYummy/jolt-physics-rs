use cxx::{type_id, ExternType};
use glam::Vec3A;
use static_assertions::const_assert_eq;
use std::mem;

use crate::base::*;

#[cxx::bridge()]
pub(crate) mod ffi {
    extern "Rust" {
        type XDebugApplication;

        #[cxx_name = "Initialize"]
        unsafe fn initialize(self: &mut XDebugApplication) -> *mut XPhysicsSystem;
        #[cxx_name = "RenderFrame"]
        unsafe fn render_frame(self: &mut XDebugApplication, delta: f32, camera: &CameraState, keyboard: *mut Keyboard) -> bool;
        #[cxx_name = "GetCameraPivot"]
        fn get_camera_pivot(self: &XDebugApplication, heading: f32, pitch: f32) -> Vec3;
    }

    extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type Vec3 = crate::base::ffi::Vec3;

        type XPhysicsSystem = crate::system::ffi::XPhysicsSystem;

        type CameraState = crate::debug::CameraState;

        type Keyboard;
        unsafe fn IsKeyPressed(self: &Keyboard, inKey: i32) -> bool;

        unsafe fn RunDebugApplication(rs_app: Box<XDebugApplication>);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraState {
    pub pos: Vec3A,
    pub forward: Vec3A,
    pub up: Vec3A,
    pub fov_y: f32,
    pub far_plane: f32,
}
const_assert_eq!(mem::size_of::<CameraState>(), 64);

unsafe impl ExternType for CameraState {
    type Id = type_id!("CameraState");
    type Kind = cxx::kind::Trivial;
}

#[derive(PartialEq)]
pub struct DebugKeyboard(pub(crate) *mut ffi::Keyboard);

impl DebugKeyboard {
    pub fn is_key_pressed(&self, key: i32) -> bool {
        return unsafe { (&*self.0).IsKeyPressed(key) };
    }
}

pub struct XDebugApplication {
    creator: fn() -> Box<dyn DebugApplication>,
    app: Option<Box<dyn DebugApplication>>,
}

impl XDebugApplication {
    pub fn new(creator: fn() -> Box<dyn DebugApplication>) -> Self {
        return Self { creator, app: None };
    }

    pub fn initialize(&mut self) -> *mut ffi::XPhysicsSystem {
        self.app = Some((self.creator)());
        return unsafe { self.app.as_mut().unwrap().get_ref_system().ptr() };
    }

    fn render_frame(&mut self, delta: f32, camera: &CameraState, keyboard: *mut ffi::Keyboard) -> bool {
        if let Some(app) = &mut self.app {
            let mut keyboard = DebugKeyboard(keyboard);
            return app.render_frame(delta, &mut keyboard, camera);
        }
        return false;
    }

    fn get_camera_pivot(&self, heading: f32, pitch: f32) -> XVec3 {
        if let Some(app) = &self.app {
            return app.get_camera_pivot(heading, pitch).into();
        }
        return Vec3A::new(0.0, 0.0, 0.0).into();
    }
}

pub trait DebugApplication {
    fn get_ref_system(&mut self) -> RefPhysicsSystem;
    fn render_frame(&mut self, delta: f32, keyboard: &mut DebugKeyboard, camera: &CameraState) -> bool;
    fn get_camera_pivot(&self, heading: f32, pitch: f32) -> Vec3A;
}

pub fn run_debug_application(creator: fn() -> Box<dyn DebugApplication>) {
    let rs_app = Box::new(XDebugApplication { creator, app: None });
    unsafe { ffi::RunDebugApplication(rs_app) };
}
