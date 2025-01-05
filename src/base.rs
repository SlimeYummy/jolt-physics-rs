use cxx::{kind, type_id, ExternType};
use glam::{IVec3, IVec4, Mat4, Quat, Vec3, Vec3A, Vec4};
use serde::{Deserialize, Serialize};
use static_assertions::const_assert_eq;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

#[allow(dead_code)]
#[cxx::bridge()]
pub(crate) mod ffi {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum ShapeType {
        Convex,
        Compound,
        Decorated,
        Mesh,
        HeightField,
        SoftBody,

        User1,
        User2,
        User3,
        User4,

        Plane,
        Empty,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum ShapeSubType {
        // Convex shapes
        Sphere,
        Box,
        Triangle,
        Capsule,
        TaperedCapsule,
        Cylinder,
        ConvexHull,

        // Compound shapes
        StaticCompound,
        MutableCompound,

        // Decorated shapes
        RotatedTranslated,
        Scaled,
        OffsetCenterOfMass,

        // Other shapes
        Mesh,
        HeightField,
        SoftBody,

        // User defined shapes
        User1,
        User2,
        User3,
        User4,
        User5,
        User6,
        User7,
        User8,

        // User defined convex shapes
        UserConvex1,
        UserConvex2,
        UserConvex3,
        UserConvex4,
        UserConvex5,
        UserConvex6,
        UserConvex7,
        UserConvex8,

        // Other shapes
        Plane,
        TaperedCylinder,
        Empty,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum BodyType {
        RigidBody,
        SoftBody,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum MotionType {
        Static,
        Kinematic,
        Dynamic,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum MotionQuality {
        Discrete,
        LinearCast,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum AllowedDOFs {
        None = 0b000000,
        All = 0b111111,
        TranslationX = 0b000001,
        TranslationY = 0b000010,
        TranslationZ = 0b000100,
        RotationX = 0b001000,
        RotationY = 0b010000,
        RotationZ = 0b100000,
        Plane2D = 0b100011,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum OverrideMassProperties {
        CalculateMassAndInertia,
        CalculateInertia,
        MassAndInertiaProvided,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Activation {
        Activate,
        DontActivate,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum CanSleep {
        CannotSleep,
        CanSleep,
    }

    impl Vec<Vec3> {}
    impl Vec<Vec4> {}
    impl Vec<Quat> {}
    impl Vec<Mat44> {}
    impl Vec<Float3> {}
    impl Vec<Int3> {}
    impl Vec<Plane> {}
    impl Vec<AABox> {}
    impl Vec<IndexedTriangle> {}
    impl Vec<BodyID> {}
    impl Vec<SubShapeID> {}

    struct FatVTablePointer {
        vtable: *const u8,
        data: *const u8,
    }

    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type ShapeType;
        type ShapeSubType;
        type BodyType;
        type MotionType;
        type MotionQuality;
        type AllowedDOFs;
        type OverrideMassProperties;
        type Activation;
        type CanSleep;

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
        type SubShapeID = crate::base::SubShapeID;
    }
}

pub type ShapeType = ffi::ShapeType;
pub type ShapeSubType = ffi::ShapeSubType;
pub type BodyType = ffi::BodyType;
pub type MotionType = ffi::MotionType;
pub type MotionQuality = ffi::MotionQuality;
pub type AllowedDOFs = ffi::AllowedDOFs;
pub type OverrideMassProperties = ffi::OverrideMassProperties;

impl From<bool> for ffi::Activation {
    #[inline]
    fn from(value: bool) -> ffi::Activation {
        match value {
            true => ffi::Activation::Activate,
            false => ffi::Activation::DontActivate,
        }
    }
}

impl From<ffi::Activation> for bool {
    #[inline]
    fn from(value: ffi::Activation) -> bool {
        value == ffi::Activation::Activate
    }
}

impl From<bool> for ffi::CanSleep {
    #[inline]
    fn from(value: bool) -> ffi::CanSleep {
        match value {
            true => ffi::CanSleep::CanSleep,
            false => ffi::CanSleep::CannotSleep,
        }
    }
}

impl From<ffi::CanSleep> for bool {
    #[inline]
    fn from(value: ffi::CanSleep) -> bool {
        value == ffi::CanSleep::CanSleep
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XVec3(pub(crate) Vec3A);
const_assert_eq!(mem::size_of::<XVec3>(), 16);

unsafe impl ExternType for XVec3 {
    type Id = type_id!("Vec3");
    type Kind = kind::Trivial;
}

impl From<Vec3A> for XVec3 {
    #[inline]
    fn from(v: Vec3A) -> XVec3 {
        XVec3(v)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XVec4(pub(crate) Vec4);
const_assert_eq!(mem::size_of::<XVec4>(), 16);

unsafe impl ExternType for XVec4 {
    type Id = type_id!("Vec4");
    type Kind = kind::Trivial;
}

impl From<Vec4> for XVec4 {
    #[inline]
    fn from(v: Vec4) -> XVec4 {
        XVec4(v)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XQuat(pub(crate) Quat);
const_assert_eq!(mem::size_of::<XQuat>(), 16);

unsafe impl ExternType for XQuat {
    type Id = type_id!("Quat");
    type Kind = kind::Trivial;
}

impl From<Quat> for XQuat {
    #[inline]
    fn from(q: Quat) -> XQuat {
        XQuat(q)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XMat4(pub(crate) Mat4);
const_assert_eq!(mem::size_of::<XMat4>(), 64);

unsafe impl ExternType for XMat4 {
    type Id = type_id!("Mat44");
    type Kind = kind::Trivial;
}

impl From<Mat4> for XMat4 {
    #[inline]
    fn from(m: Mat4) -> XMat4 {
        XMat4(m)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XFloat3(pub(crate) Vec3);
const_assert_eq!(mem::size_of::<XFloat3>(), 12);

unsafe impl ExternType for XFloat3 {
    type Id = type_id!("Float3");
    type Kind = kind::Trivial;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct XInt3(pub(crate) IVec3);
const_assert_eq!(mem::size_of::<XInt3>(), 12);

unsafe impl ExternType for XInt3 {
    type Id = type_id!("Int3");
    type Kind = kind::Trivial;
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
    type Kind = kind::Trivial;
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
    type Kind = kind::Trivial;
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
    type Kind = kind::Trivial;
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubShapeID(pub u32);
const_assert_eq!(mem::size_of::<SubShapeID>(), 4);

unsafe impl ExternType for SubShapeID {
    type Id = type_id!("SubShapeID");
    type Kind = kind::Trivial;
}

impl SubShapeID {
    pub const EMPTY: SubShapeID = SubShapeID(0);

    #[inline]
    pub fn new(id: u32) -> SubShapeID {
        SubShapeID(id)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == Self::EMPTY
    }
}

impl From<u32> for SubShapeID {
    #[inline]
    fn from(id: u32) -> SubShapeID {
        SubShapeID(id)
    }
}

impl From<SubShapeID> for u32 {
    #[inline]
    fn from(id: SubShapeID) -> u32 {
        id.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyID(pub u32);
const_assert_eq!(mem::size_of::<BodyID>(), 4);

unsafe impl ExternType for BodyID {
    type Id = type_id!("BodyID");
    type Kind = kind::Trivial;
}

impl BodyID {
    pub const INVALID: BodyID = BodyID(0xFFFF_FFFF);

    #[inline]
    pub fn new(id: u32) -> BodyID {
        BodyID(id)
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        *self != Self::INVALID
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        *self == Self::INVALID
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

pub unsafe trait JRefTarget {
    type JRefRaw;

    fn name() -> &'static str;
    fn from_ptr(raw: *const Self::JRefRaw) -> *const Self;
    fn from_non_null(raw: NonNull<Self::JRefRaw>) -> NonNull<Self>;
    unsafe fn clone_ref(&mut self) -> NonNull<Self>;
    unsafe fn drop_ref(&mut self);
    fn count_ref(&self) -> u32;
}

#[derive(Debug)]
pub struct JMut<T: JRefTarget>(pub(crate) NonNull<T>);

impl<T: JRefTarget> JMut<T> {
    #[inline]
    pub(crate) fn new(raw: NonNull<T::JRefRaw>) -> JMut<T> {
        JMut(T::from_non_null(raw))
    }

    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut T::JRefRaw) -> JMut<T> {
        JMut::new(NonNull::new_unchecked(raw))
    }
    
    #[inline]
    pub fn as_ref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }

    #[inline]
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }

    #[inline]
    pub fn into_ref(self) -> JRef<T> {
        let jref = JRef(self.0);
        mem::forget(self);
        jref
    }

    #[inline]
    pub unsafe fn leak_ref(&mut self) -> JRef<T> {
        JRef(self.clone_ref())
    }
}

impl<T: JRefTarget> Drop for JMut<T> {
    fn drop(&mut self) {
        #[cfg(feature = "debug-print")]
        println!("JMut<{}>::drop {:?} {}", T::name(), self.0, self.count_ref() - 1);
        unsafe { (*self.0.as_ptr()).drop_ref() };
    }
}

impl<T: JRefTarget> Deref for JMut<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: JRefTarget> DerefMut for JMut<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

#[derive(Debug)]
pub struct JRef<T: JRefTarget>(pub(crate) NonNull<T>);

impl<T: JRefTarget> JRef<T> {
    #[inline]
    pub(crate) fn new(raw: NonNull<T::JRefRaw>) -> JRef<T> {
        JRef(T::from_non_null(raw))
    }

    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut T::JRefRaw) -> JRef<T> {
        JRef::new(NonNull::new_unchecked(raw))
    }
    
    #[inline]
    pub fn as_ref(&self) -> &T {
        unsafe { &self.0.as_ref() }
    }
}

impl<T: JRefTarget> Clone for JRef<T> {
    #[inline]
    fn clone(&self) -> JRef<T> {
        unsafe { JRef((*self.0.as_ptr()).clone_ref()) }
    }
}

impl<T: JRefTarget> Drop for JRef<T> {
    fn drop(&mut self) {
        #[cfg(feature = "debug-print")]
        println!("JRef<{}>::drop {:?} {}", T::name(), self.0, self.count_ref() - 1);
        unsafe { (*self.0.as_ptr()).drop_ref() };
    }
}

impl<T: JRefTarget> From<JMut<T>> for JRef<T> {
    #[inline]
    fn from(jmut: JMut<T>) -> JRef<T> {
        let jref = JRef(jmut.0);
        mem::forget(jmut);
        jref
    }
}

impl<T: JRefTarget> Deref for JRef<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &self.0.as_ref() }
    }
}
