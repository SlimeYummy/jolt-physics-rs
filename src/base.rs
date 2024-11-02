#![allow(dead_code)]

use cxx::{type_id, ExternType};
use glam::{IVec3, IVec4, Mat4, Quat, Vec3, Vec3A, Vec4};
use serde::{Deserialize, Serialize};
use static_assertions::const_assert_eq;
use std::mem;

#[cxx::bridge()]
pub mod ffi {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct XRefShape {
        ptr: *mut u8,
    }
    impl Vec<XRefShape> {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct XRefPhysicsMaterial {
        ptr: *mut u8,
    }
    impl Vec<XRefPhysicsMaterial> {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct XRefPhysicsSystem {
        ptr: *mut u8,
    }

    impl Vec<Vec3> {}
    impl Vec<Float3> {}
    impl Vec<Int3> {}
    impl Vec<IndexedTriangle> {}

    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type Vec3 = crate::base::XVec3;
        type Vec4 = crate::base::XVec4;
        type Quat = crate::base::XQuat;
        type Mat44 = crate::base::XMat4;
        type Isometry = crate::base::Isometry;
        type Transform = crate::base::Transform;
        type Float3 = crate::base::XFloat3;
        type Int3 = crate::base::XInt3;
        type Plane = crate::base::Plane;
        type IndexedTriangle = crate::base::IndexedTriangle;
        type BodyID = crate::base::BodyID;

        type Shape;
        fn DropRefShape(shape: XRefShape);
        fn CloneRefShape(shape: XRefShape) -> XRefShape;
        fn CountRefShape(shape: XRefShape) -> u32;

        type PhysicsMaterial;
        fn DropRefPhysicsMaterial(material: XRefPhysicsMaterial);
        fn CloneRefPhysicsMaterial(material: XRefPhysicsMaterial) -> XRefPhysicsMaterial;
        fn CountRefPhysicsMaterial(material: XRefPhysicsMaterial) -> u32;

        type XPhysicsSystem;
        fn DropRefPhysicsSystem(system: XRefPhysicsSystem);
        fn CloneRefPhysicsSystem(system: XRefPhysicsSystem) -> XRefPhysicsSystem;
        fn CountRefPhysicsSystem(system: XRefPhysicsSystem) -> u32;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XVec3(pub Vec3A);
const_assert_eq!(mem::size_of::<XVec3>(), 16);

unsafe impl ExternType for XVec3 {
    type Id = type_id!("Vec3");
    type Kind = cxx::kind::Trivial;
}

impl From<Vec3A> for XVec3 {
    fn from(v: Vec3A) -> XVec3 {
        return XVec3(v);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XVec4(pub Vec4);
const_assert_eq!(mem::size_of::<XVec4>(), 16);

unsafe impl ExternType for XVec4 {
    type Id = type_id!("Vec4");
    type Kind = cxx::kind::Trivial;
}

impl From<Vec4> for XVec4 {
    fn from(v: Vec4) -> XVec4 {
        return XVec4(v);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XQuat(pub Quat);
const_assert_eq!(mem::size_of::<XQuat>(), 16);

unsafe impl ExternType for XQuat {
    type Id = type_id!("Quat");
    type Kind = cxx::kind::Trivial;
}

impl From<Quat> for XQuat {
    fn from(q: Quat) -> XQuat {
        return XQuat(q);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XMat4(pub Mat4);
const_assert_eq!(mem::size_of::<XMat4>(), 64);

unsafe impl ExternType for XMat4 {
    type Id = type_id!("Mat44");
    type Kind = cxx::kind::Trivial;
}

impl From<Mat4> for XMat4 {
    fn from(m: Mat4) -> XMat4 {
        return XMat4(m);
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Isometry {
    pub position: Vec3A,
    pub rotation: Quat,
}
const_assert_eq!(mem::size_of::<Isometry>(), 32);

unsafe impl ExternType for Isometry {
    type Id = type_id!("Isometry");
    type Kind = cxx::kind::Trivial;
}

impl Isometry {
    pub fn new(position: Vec3A, rotation: Quat) -> Isometry {
        return Isometry { position, rotation };
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3A,
    pub rotation: Quat,
    pub scale: Vec3A,
}
const_assert_eq!(mem::size_of::<Transform>(), 48);

unsafe impl ExternType for Transform {
    type Id = type_id!("Transform");
    type Kind = cxx::kind::Trivial;
}

impl Transform {
    pub fn new(position: Vec3A, rotation: Quat, scale: Vec3A) -> Transform {
        return Transform { position, rotation, scale };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XFloat3(Vec3);
const_assert_eq!(mem::size_of::<XFloat3>(), 12);

unsafe impl ExternType for XFloat3 {
    type Id = type_id!("Float3");
    type Kind = cxx::kind::Trivial;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XInt3(IVec3);
const_assert_eq!(mem::size_of::<XInt3>(), 12);

unsafe impl ExternType for XInt3 {
    type Id = type_id!("Int3");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}
const_assert_eq!(mem::size_of::<Plane>(), 16);

unsafe impl ExternType for Plane {
    type Id = type_id!("Plane");
    type Kind = cxx::kind::Trivial;
}

impl Plane {
    pub fn new(normal: Vec3, distance: f32) -> Plane {
        return Plane { normal, distance };
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IndexedTriangle {
    pub idx: [u32; 3],
    pub material_index: u32,
}
const_assert_eq!(mem::size_of::<IndexedTriangle>(), 16);

unsafe impl ExternType for IndexedTriangle {
    type Id = type_id!("IndexedTriangle");
    type Kind = cxx::kind::Trivial;
}

impl IndexedTriangle {
    pub fn new(idx1: u32, idx2: u32, idx3: u32, material_index: u32) -> IndexedTriangle {
        return IndexedTriangle {
            idx: [idx1, idx2, idx3],
            material_index,
        };
    }
}

impl Serialize for IndexedTriangle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return IVec4::new(
            self.idx[0] as i32,
            self.idx[1] as i32,
            self.idx[2] as i32,
            self.material_index as i32,
        )
        .serialize(serializer);
    }
}

impl<'de> Deserialize<'de> for IndexedTriangle {
    fn deserialize<D>(deserializer: D) -> Result<IndexedTriangle, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = IVec4::deserialize(deserializer)?;
        return Ok(IndexedTriangle::new(v.x as u32, v.y as u32, v.z as u32, v.w as u32));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyID(u32);
const_assert_eq!(mem::size_of::<BodyID>(), 4);

unsafe impl ExternType for BodyID {
    type Id = type_id!("BodyID");
    type Kind = cxx::kind::Trivial;
}

impl BodyID {
    pub fn invalid() -> BodyID {
        return BodyID(0xFFFF_FFFF);
    }

    pub fn is_valid(&self) -> bool {
        return self.0 != 0xFFFF_FFFF;
    }

    pub fn is_invalid(&self) -> bool {
        return self.0 == 0xFFFF_FFFF;
    }
}

const_assert_eq!(mem::size_of::<ffi::XRefShape>(), mem::size_of::<usize>());

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RefShape(pub(crate) ffi::XRefShape);

impl Default for RefShape {
    fn default() -> RefShape {
        return RefShape(ffi::XRefShape { ptr: std::ptr::null_mut() });
    }
}

impl Clone for RefShape {
    fn clone(&self) -> RefShape {
        return RefShape(ffi::CloneRefShape(self.0));
    }
}

impl Drop for RefShape {
    fn drop(&mut self) {
        #[cfg(feature = "debug-print")]
        if !self.0.ptr.is_null() {
            println!("DropRefShape::drop {:?} {}", self.0.ptr, self.ref_count() - 1);
        }
        ffi::DropRefShape(self.0);
        self.0.ptr = std::ptr::null_mut();
    }
}

impl RefShape {
    pub fn ref_count(&self) -> u32 {
        return ffi::CountRefShape(self.0);
    }

    pub fn as_ref(&self) -> Option<&ffi::Shape> {
        if self.0.ptr.is_null() {
            return None;
        }
        return Some(unsafe { &*(self.0.ptr as *const ffi::Shape) });
    }

    pub fn as_mut(&mut self) -> Option<&mut ffi::Shape> {
        if self.0.ptr.is_null() {
            return None;
        }
        return Some(unsafe { &mut *(self.0.ptr as *mut ffi::Shape) });
    }

    pub fn as_usize(&self) -> usize {
        return self.0.ptr as usize;
    }

    pub unsafe fn ptr(&mut self) -> *mut ffi::Shape {
        return self.0.ptr as *mut ffi::Shape;
    }
}

const_assert_eq!(mem::size_of::<ffi::XRefPhysicsMaterial>(), mem::size_of::<usize>());

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RefPhysicsMaterial(pub(crate) ffi::XRefPhysicsMaterial);

impl Default for RefPhysicsMaterial {
    fn default() -> RefPhysicsMaterial {
        return RefPhysicsMaterial(ffi::XRefPhysicsMaterial { ptr: std::ptr::null_mut() });
    }
}

impl Clone for RefPhysicsMaterial {
    fn clone(&self) -> RefPhysicsMaterial {
        return RefPhysicsMaterial(ffi::CloneRefPhysicsMaterial(self.0));
    }
}

impl Drop for RefPhysicsMaterial {
    fn drop(&mut self) {
        #[cfg(feature = "debug-print")]
        if !self.0.ptr.is_null() {
            println!("DropRefPhysicsMaterial::drop {:?} {}", self.0.ptr, self.ref_count() - 1);
        }
        ffi::DropRefPhysicsMaterial(self.0);
        self.0.ptr = std::ptr::null_mut();
    }
}

impl RefPhysicsMaterial {
    pub fn ref_count(&self) -> u32 {
        return ffi::CountRefPhysicsMaterial(self.0);
    }

    pub fn as_ref(&self) -> Option<&ffi::PhysicsMaterial> {
        if self.0.ptr.is_null() {
            return None;
        }
        return Some(unsafe { &*(self.0.ptr as *const ffi::PhysicsMaterial) });
    }

    pub fn as_mut(&mut self) -> Option<&mut ffi::PhysicsMaterial> {
        if self.0.ptr.is_null() {
            return None;
        }
        return Some(unsafe { &mut *(self.0.ptr as *mut ffi::PhysicsMaterial) });
    }

    pub fn as_usize(&self) -> usize {
        return self.0.ptr as usize;
    }

    pub unsafe fn ptr(&mut self) -> *mut ffi::PhysicsMaterial {
        return self.0.ptr as *mut ffi::PhysicsMaterial;
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RefPhysicsSystem(pub(crate) ffi::XRefPhysicsSystem);

impl Default for RefPhysicsSystem {
    fn default() -> RefPhysicsSystem {
        return RefPhysicsSystem(ffi::XRefPhysicsSystem { ptr: std::ptr::null_mut() });
    }
}

impl Clone for RefPhysicsSystem {
    fn clone(&self) -> RefPhysicsSystem {
        return RefPhysicsSystem(ffi::CloneRefPhysicsSystem(self.0));
    }
}

impl Drop for RefPhysicsSystem {
    fn drop(&mut self) {
        #[cfg(feature = "debug-print")]
        if !self.0.ptr.is_null() {
            println!("DropRefPhysicsSystem::drop {:?} {}", self.0.ptr, self.ref_count() - 1);
        }
        ffi::DropRefPhysicsSystem(self.0);
        self.0.ptr = std::ptr::null_mut();
    }
}

impl RefPhysicsSystem {
    pub fn ref_count(&self) -> u32 {
        return ffi::CountRefPhysicsSystem(self.0);
    }

    pub fn as_ref(&self) -> Option<&ffi::XPhysicsSystem> {
        if self.0.ptr.is_null() {
            return None;
        }
        return Some(unsafe { &*(self.0.ptr as *const ffi::XPhysicsSystem) });
    }

    pub fn as_mut(&mut self) -> Option<&mut ffi::XPhysicsSystem> {
        if self.0.ptr.is_null() {
            return None;
        }
        return Some(unsafe { &mut *(self.0.ptr as *mut ffi::XPhysicsSystem) });
    }

    pub unsafe fn ptr(&mut self) -> *mut ffi::XPhysicsSystem {
        return self.0.ptr as *mut ffi::XPhysicsSystem;
    }
}
