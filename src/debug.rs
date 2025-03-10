use cxx::{kind, type_id, ExternType};
use glam::Vec3A;
use static_assertions::{assert_cfg, const_assert_eq};
use std::mem;

use crate::base::JVec3;

assert_cfg!(windows, "Debug rendering is only supported on Windows");

#[cxx::bridge()]
pub(crate) mod ffi {
    extern "Rust" {
        type RustDebugApp;

        #[cxx_name = "GetPhysicsSystem"]
        unsafe fn get_physics_system(self: &mut RustDebugApp) -> *mut XPhysicsSystem;
        #[cxx_name = "UpdateFrame"]
        unsafe fn update_frame(
            self: &mut RustDebugApp,
            delta: f32,
            camera: &CameraState,
            mouse: *mut Mouse,
            keyboard: *mut Keyboard,
        ) -> bool;
        #[cxx_name = "GetInitialCamera"]
        unsafe fn get_initial_camera(self: &mut RustDebugApp, state: *mut CameraState);
        #[cxx_name = "GetCameraPivot"]
        fn get_camera_pivot(self: &mut RustDebugApp, heading: f32, pitch: f32) -> Vec3;
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

        fn RunDebugApplication(rs_app: Box<RustDebugApp>);
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
    type Kind = kind::Trivial;
}

#[derive(PartialEq)]
pub struct DebugKeyboard(pub(crate) *mut ffi::Keyboard);

impl DebugKeyboard {
    pub fn is_key_pressed(&self, key: i32) -> bool {
        unsafe { (&*self.0).IsKeyPressed(key) }
    }
}

#[derive(PartialEq)]
pub struct DebugMouse(pub(crate) *mut ffi::Mouse);

impl DebugMouse {
    pub fn get_x(&self) -> i32 {
        unsafe { (&*self.0).GetX() }
    }

    pub fn get_y(&self) -> i32 {
        unsafe { (&*self.0).GetY() }
    }

    pub fn get_dx(&self) -> i32 {
        unsafe { (&*self.0).GetDX() }
    }

    pub fn get_dy(&self) -> i32 {
        unsafe { (&*self.0).GetDY() }
    }

    pub fn is_left_pressed(&self) -> bool {
        unsafe { (&*self.0).IsLeftPressed() }
    }

    pub fn is_right_pressed(&self) -> bool {
        unsafe { (&*self.0).IsRightPressed() }
    }

    pub fn is_middle_pressed(&self) -> bool {
        unsafe { (&*self.0).IsMiddlePressed() }
    }
}

pub struct RustDebugApp {
    dbg_app: Box<dyn DebugApp>,
    x_physics_system: *mut ffi::XPhysicsSystem,
}

impl RustDebugApp {
    pub fn get_physics_system(&mut self) -> *mut ffi::XPhysicsSystem {
        self.x_physics_system
    }

    fn update_frame(
        &mut self,
        delta: f32,
        camera: &CameraState,
        mouse: *mut ffi::Mouse,
        keyboard: *mut ffi::Keyboard,
    ) -> bool {
        let mut mouse = DebugMouse(mouse);
        let mut keyboard = DebugKeyboard(keyboard);
        self.dbg_app.update_frame(delta, camera, &mut mouse, &mut keyboard)
    }

    fn get_initial_camera(&mut self, state: *mut ffi::CameraState) {
        unsafe { self.dbg_app.get_initial_camera(&mut *state) };
    }

    fn get_camera_pivot(&mut self, heading: f32, pitch: f32) -> JVec3 {
        self.dbg_app.get_camera_pivot(heading, pitch).into()
    }
}

pub trait DebugApp {
    fn cpp_physics_system(&mut self) -> *mut u8;
    fn update_frame(
        &mut self,
        delta: f32,
        camera: &CameraState,
        mouse: &mut DebugMouse,
        keyboard: &mut DebugKeyboard,
    ) -> bool;
    fn get_initial_camera(&mut self, state: &mut CameraState);
    fn get_camera_pivot(&mut self, heading: f32, pitch: f32) -> Vec3A;
}

pub fn run_debug_application(mut dbg_app: Box<dyn DebugApp>) {
    let x_physics_system = dbg_app.cpp_physics_system() as *mut ffi::XPhysicsSystem;
    let rs_dbg_app = Box::new(RustDebugApp {
        dbg_app,
        x_physics_system,
    });
    ffi::RunDebugApplication(rs_dbg_app);
}
