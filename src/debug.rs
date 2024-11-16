use cxx::{type_id, ExternType};
use glam::Vec3A;
use static_assertions::{assert_cfg, const_assert_eq};
use std::mem;

use crate::base::*;

assert_cfg!(windows, "Debug rendering is only supported on Windows");

#[cxx::bridge()]
pub(crate) mod ffi {
    extern "Rust" {
        type XDebugApp;

        #[cxx_name = "GetPhysicsSystem"]
        unsafe fn get_physics_system(self: &mut XDebugApp) -> *mut XPhysicsSystem;
        #[cxx_name = "UpdateFrame"]
        unsafe fn update_frame(self: &mut XDebugApp, delta: f32, camera: &CameraState, mouse: *mut Mouse, keyboard: *mut Keyboard) -> bool;
        #[cxx_name = "GetInitialCamera"]
        unsafe fn get_initial_camera(self: &mut XDebugApp, state: *mut CameraState);
        #[cxx_name = "GetCameraPivot"]
        fn get_camera_pivot(self: &XDebugApp, heading: f32, pitch: f32) -> Vec3;
    }

    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type Vec3 = crate::base::ffi::Vec3;

        type XPhysicsSystem = crate::system::ffi::XPhysicsSystem;

        type CameraState = crate::debug::CameraState;

        type Keyboard;
        fn IsKeyPressed(self: &Keyboard, key: i32) -> bool;

        type Mouse;
        fn GetX(self: &Mouse) -> i32;
        fn GetY(self: &Mouse) -> i32;
        fn GetDX(self: &Mouse) -> i32;
        fn GetDY(self: &Mouse) -> i32;
        fn IsLeftPressed(self: &Mouse) -> bool;
        fn IsRightPressed(self: &Mouse) -> bool;
        fn IsMiddlePressed(self: &Mouse) -> bool;

        fn RunDebugApplication(rs_app: Box<XDebugApp>);
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

#[derive(PartialEq)]
pub struct DebugMouse(pub(crate) *mut ffi::Mouse);

impl DebugMouse {
    pub fn get_x(&self) -> i32 {
        return unsafe { (&*self.0).GetX() };
    }

    pub fn get_y(&self) -> i32 {
        return unsafe { (&*self.0).GetY() };
    }

    pub fn get_dx(&self) -> i32 {
        return unsafe { (&*self.0).GetDX() };
    }

    pub fn get_dy(&self) -> i32 {
        return unsafe { (&*self.0).GetDY() };
    }

    pub fn is_left_pressed(&self) -> bool {
        return unsafe { (&*self.0).IsLeftPressed() };
    }

    pub fn is_right_pressed(&self) -> bool {
        return unsafe { (&*self.0).IsRightPressed() };
    }

    pub fn is_middle_pressed(&self) -> bool {
        return unsafe { (&*self.0).IsMiddlePressed() };
    }
}

pub struct XDebugApp(Box<dyn DebugApplication>);

impl XDebugApp {
    pub fn get_physics_system(&mut self) -> *mut ffi::XPhysicsSystem {
        return unsafe { self.0.as_mut().get_physics_system().ptr() };
    }

    fn update_frame(&mut self, delta: f32, camera: &CameraState, mouse: *mut ffi::Mouse, keyboard: *mut ffi::Keyboard) -> bool {
        let mut mouse = DebugMouse(mouse);
        let mut keyboard = DebugKeyboard(keyboard);
        return self.0.update_frame(delta, camera, &mut mouse, &mut keyboard);
    }

    fn get_initial_camera(&self, state: *mut ffi::CameraState) {
        unsafe { self.0.get_initial_camera(&mut *state) };
    }

    fn get_camera_pivot(&self, heading: f32, pitch: f32) -> XVec3 {
        return self.0.get_camera_pivot(heading, pitch).into();
    }
}

pub trait DebugApplication {
    fn get_physics_system(&mut self) -> RefPhysicsSystem;
    fn update_frame(&mut self, delta: f32, camera: &CameraState, mouse: &mut DebugMouse, keyboard: &mut DebugKeyboard) -> bool;
    fn get_initial_camera(&self, state: &mut CameraState);
    fn get_camera_pivot(&self, heading: f32, pitch: f32) -> Vec3A;
}

pub fn run_debug_application(dbg_app: Box<dyn DebugApplication>) {
    let rs_app = Box::new(XDebugApp(dbg_app));
    ffi::RunDebugApplication(rs_app);
}
