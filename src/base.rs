#![allow(dead_code)]

use cxx::{type_id, ExternType};
use glam::{IVec3, IVec4, Mat4, Quat, Vec3, Vec3A, Vec4};
use serde::{Deserialize, Serialize};
use static_assertions::const_assert_eq;
use std::mem;
use std::pin::Pin;

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
    impl Vec<Plane> {}
    impl Vec<AABox> {}
    impl Vec<IndexedTriangle> {}

    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type Vec3 = crate::base::XVec3;
        type Vec4 = crate::base::XVec4;
        type Quat = crate::base::XQuat;
        type Mat44 = crate::base::XMat4;
        type Float3 = crate::base::XFloat3;
        type Int3 = crate::base::XInt3;
        type Plane = crate::base::Plane;
        type AABox = crate::base::AABox;
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
    #[inline]
    fn from(v: Vec3A) -> XVec3 {
        XVec3(v)
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
    #[inline]
    fn from(v: Vec4) -> XVec4 {
        XVec4(v)
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
    #[inline]
    fn from(q: Quat) -> XQuat {
        XQuat(q)
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
    #[inline]
    fn from(m: Mat4) -> XMat4 {
        XMat4(m)
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

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}
const_assert_eq!(mem::size_of::<Plane>(), 16);

unsafe impl ExternType for Plane {
    type Id = type_id!("Plane");
    type Kind = cxx::kind::Trivial;
}

impl Default for Plane {
    #[inline]
    fn default() -> Self {
        Plane::new(Vec3::Y, 0.0)
    }
}

impl Plane {
    #[inline]
    pub fn new(normal: Vec3, distance: f32) -> Plane {
        Plane { normal, distance }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AABox {
    pub min: Vec3A,
    pub max: Vec3A,
}
const_assert_eq!(mem::size_of::<AABox>(), 32);

unsafe impl ExternType for AABox {
    type Id = type_id!("AABox");
    type Kind = cxx::kind::Trivial;
}

impl AABox {
    #[inline]
    pub fn new(min: Vec3A, max: Vec3A) -> AABox {
        AABox { min, max }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IndexedTriangle {
    pub idx: [u32; 3],
    pub material_index: u32,
    pub user_data: u32,
}
const_assert_eq!(mem::size_of::<IndexedTriangle>(), 20);

unsafe impl ExternType for IndexedTriangle {
    type Id = type_id!("IndexedTriangle");
    type Kind = cxx::kind::Trivial;
}

impl IndexedTriangle {
    #[inline]
    pub fn new(idx1: u32, idx2: u32, idx3: u32, material_index: u32) -> IndexedTriangle {
        IndexedTriangle {
            idx: [idx1, idx2, idx3],
            material_index,
            user_data: 0,
        }
    }
}

impl Serialize for IndexedTriangle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        IVec4::new(
            self.idx[0] as i32,
            self.idx[1] as i32,
            self.idx[2] as i32,
            self.material_index as i32,
        )
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IndexedTriangle {
    fn deserialize<D>(deserializer: D) -> Result<IndexedTriangle, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = IVec4::deserialize(deserializer)?;
        Ok(IndexedTriangle::new(v.x as u32, v.y as u32, v.z as u32, v.w as u32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyID(pub u32);
const_assert_eq!(mem::size_of::<BodyID>(), 4);

unsafe impl ExternType for BodyID {
    type Id = type_id!("BodyID");
    type Kind = cxx::kind::Trivial;
}

impl BodyID {
    #[inline]
    pub fn new(id: u32) -> BodyID {
        BodyID(id)
    }

    #[inline]
    pub fn invalid() -> BodyID {
        BodyID(0xFFFF_FFFF)
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 != 0xFFFF_FFFF
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.0 == 0xFFFF_FFFF
    }
}

impl From<u32> for BodyID {
    #[inline]
    fn from(id: u32) -> BodyID {
        BodyID(id)
    }
}

impl From<BodyID> for u32 {
    #[inline]
    fn from(id: BodyID) -> u32 {
        id.0
    }
}

const_assert_eq!(mem::size_of::<ffi::XRefShape>(), mem::size_of::<usize>());

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RefShape(pub(crate) ffi::XRefShape);

impl Clone for RefShape {
    #[inline]
    fn clone(&self) -> RefShape {
        RefShape(ffi::CloneRefShape(self.0))
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
    #[inline]
    pub fn invalid() -> RefShape {
        RefShape(ffi::XRefShape {
            ptr: std::ptr::null_mut(),
        })
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.0.ptr.is_null()
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.0.ptr.is_null()
    }

    #[inline]
    pub fn ref_count(&self) -> u32 {
        ffi::CountRefShape(self.0)
    }

    #[inline]
    pub(crate) fn as_ref(&self) -> &ffi::Shape {
        unsafe { &*(self.0.ptr as *const ffi::Shape) }
    }

    #[inline]
    pub(crate) fn as_mut(&mut self) -> Pin<&mut ffi::Shape> {
        unsafe { Pin::new_unchecked(&mut *(self.0.ptr as *mut ffi::Shape)) }
    }

    #[inline]
    pub(crate) unsafe fn as_ref_t<T>(&self) -> &T {
        unsafe { &*(self.0.ptr as *const T) }
    }

    #[inline]
    pub(crate) unsafe fn as_mut_t<T>(&mut self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.0.ptr as *mut T)) }
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::Shape {
        self.0.ptr as *const ffi::Shape
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::Shape {
        self.0.ptr as *mut ffi::Shape
    }
}

const_assert_eq!(mem::size_of::<ffi::XRefPhysicsMaterial>(), mem::size_of::<usize>());

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RefPhysicsMaterial(pub(crate) ffi::XRefPhysicsMaterial);

impl Clone for RefPhysicsMaterial {
    #[inline]
    fn clone(&self) -> RefPhysicsMaterial {
        RefPhysicsMaterial(ffi::CloneRefPhysicsMaterial(self.0))
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
    #[inline]
    pub fn invalid() -> RefPhysicsMaterial {
        RefPhysicsMaterial(ffi::XRefPhysicsMaterial {
            ptr: std::ptr::null_mut(),
        })
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.0.ptr.is_null()
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.0.ptr.is_null()
    }

    #[inline]
    pub fn ref_count(&self) -> u32 {
        ffi::CountRefPhysicsMaterial(self.0)
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&ffi::PhysicsMaterial> {
        if self.0.ptr.is_null() {
            return None;
        }
        Some(unsafe { &*(self.0.ptr as *const ffi::PhysicsMaterial) })
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut ffi::PhysicsMaterial> {
        if self.0.ptr.is_null() {
            return None;
        }
        Some(unsafe { &mut *(self.0.ptr as *mut ffi::PhysicsMaterial) })
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0.ptr as usize
    }

    /// # Safety
    /// JoltPhysics underlying PhysicsMaterial object pointer
    #[inline]
    pub unsafe fn ptr(&mut self) -> *mut ffi::PhysicsMaterial {
        self.0.ptr as *mut ffi::PhysicsMaterial
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RefPhysicsSystem(pub(crate) ffi::XRefPhysicsSystem);

impl Clone for RefPhysicsSystem {
    #[inline]
    fn clone(&self) -> RefPhysicsSystem {
        RefPhysicsSystem(ffi::CloneRefPhysicsSystem(self.0))
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
    #[inline]
    pub fn invalid() -> RefPhysicsSystem {
        RefPhysicsSystem(ffi::XRefPhysicsSystem {
            ptr: std::ptr::null_mut(),
        })
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.0.ptr.is_null()
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.0.ptr.is_null()
    }

    #[inline]
    pub fn ref_count(&self) -> u32 {
        ffi::CountRefPhysicsSystem(self.0)
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&ffi::XPhysicsSystem> {
        if self.0.ptr.is_null() {
            return None;
        }
        Some(unsafe { &*(self.0.ptr as *const ffi::XPhysicsSystem) })
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut ffi::XPhysicsSystem> {
        if self.0.ptr.is_null() {
            return None;
        }
        Some(unsafe { &mut *(self.0.ptr as *mut ffi::XPhysicsSystem) })
    }

    /// # Safety
    /// JoltPhysics underlying XPhysicsSystem object pointer
    #[inline]
    pub unsafe fn ptr(&mut self) -> *mut ffi::XPhysicsSystem {
        self.0.ptr as *mut ffi::XPhysicsSystem
    }
}
