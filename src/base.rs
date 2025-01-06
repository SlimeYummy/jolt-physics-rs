use cxx::{kind, type_id, ExternType};
use glam::{IVec3, IVec4, Mat4, Quat, Vec3, Vec3A, Vec4};
use serde::{Deserialize, Serialize};
use static_assertions::const_assert_eq;
use core::fmt;
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

        type Vec3 = crate::base::JVec3;
        type Vec4 = crate::base::JVec4;
        type Quat = crate::base::JQuat;
        type Mat44 = crate::base::JMat4;
        type Float3 = crate::base::Float3;
        type Int3 = crate::base::Int3;
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

#[repr(C)]
#[derive(Clone, Copy)]
pub union JVec3 {
    _jolt: [f32; 4],
    glam: Vec3A,
}
const_assert_eq!(mem::size_of::<JVec3>(), 16);

unsafe impl ExternType for JVec3 {
    type Id = type_id!("Vec3");
    type Kind = kind::Trivial;
}

impl JVec3 {
    #[inline(always)]
    fn glam(&self) -> Vec3A {
        unsafe { self.glam }
    }
}

impl Default for JVec3 {
    #[inline]
    fn default() -> Self {
        JVec3 { glam: Vec3A::ZERO }
    }
}

impl PartialEq for JVec3 {
    #[inline]
    fn eq(&self, other: &JVec3) -> bool {
        self.glam() == other.glam()
    }
}

impl PartialEq<Vec3A> for JVec3 {
    #[inline]
    fn eq(&self, other: &Vec3A) -> bool {
        self.glam() == *other
    }
}

impl fmt::Debug for JVec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.glam().fmt(f)
    }
}

impl From<Vec3A> for JVec3 {
    #[inline]
    fn from(v: Vec3A) -> JVec3 {
        JVec3 { glam: v }
    }
}

impl From<JVec3> for Vec3A {
    #[inline]
    fn from(v: JVec3) -> Vec3A {
        v.glam()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JVec4 {
    _jolt: [f32; 4],
    glam: Vec4,
}
const_assert_eq!(mem::size_of::<JVec4>(), 16);

unsafe impl ExternType for JVec4 {
    type Id = type_id!("Vec4");
    type Kind = kind::Trivial;
}

impl JVec4 {
    #[inline(always)]
    fn glam(&self) -> Vec4 {
        unsafe { self.glam }
    }
}

impl Default for JVec4 {
    #[inline]
    fn default() -> Self {
        JVec4 { glam: Vec4::ZERO }
    }
}

impl PartialEq for JVec4 {
    #[inline]
    fn eq(&self, other: &JVec4) -> bool {
        self.glam() == other.glam()
    }
}

impl PartialEq<Vec4> for JVec4 {
    #[inline]
    fn eq(&self, other: &Vec4) -> bool {
        self.glam() == *other
    }
}

impl fmt::Debug for JVec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.glam().fmt(f)
    }
}

impl From<Vec4> for JVec4 {
    #[inline]
    fn from(v: Vec4) -> JVec4 {
        JVec4 { glam: v }
    }
}

impl From<JVec4> for Vec4 {
    #[inline]
    fn from(v: JVec4) -> Vec4 {
        v.glam()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JQuat {
    _jolt: [f32; 4],
    glam: Quat,
}
const_assert_eq!(mem::size_of::<JQuat>(), 16);

unsafe impl ExternType for JQuat {
    type Id = type_id!("Quat");
    type Kind = kind::Trivial;
}

impl JQuat {
    #[inline(always)]
    fn glam(&self) -> Quat {
        unsafe { self.glam }
    }
}

impl Default for JQuat {
    #[inline]
    fn default() -> Self {
        JQuat { glam: Quat::IDENTITY }
    }
}

impl PartialEq for JQuat {
    #[inline]
    fn eq(&self, other: &JQuat) -> bool {
        self.glam() == other.glam()
    }
}

impl PartialEq<Quat> for JQuat {
    #[inline]
    fn eq(&self, other: &Quat) -> bool {
        self.glam() == *other
    }
}

impl fmt::Debug for JQuat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.glam().fmt(f)
    }
}

impl From<Quat> for JQuat {
    #[inline]
    fn from(q: Quat) -> JQuat {
        JQuat { glam: q }
    }
}

impl From<JQuat> for Quat {
    #[inline]
    fn from(q: JQuat) -> Quat {
        q.glam()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JMat4 {
    _jolt: [f32; 16],
    glam: Mat4,
}
const_assert_eq!(mem::size_of::<JMat4>(), 64);

unsafe impl ExternType for JMat4 {
    type Id = type_id!("Mat44");
    type Kind = kind::Trivial;
}

impl JMat4 {
    #[inline(always)]
    fn glam(&self) -> Mat4 {
        unsafe { self.glam }
    }
}

impl Default for JMat4 {
    #[inline]
    fn default() -> Self {
        JMat4 { glam: Mat4::IDENTITY }
    }
}

impl PartialEq for JMat4 {
    #[inline]
    fn eq(&self, other: &JMat4) -> bool {
        self.glam() == other.glam()
    }
}

impl PartialEq<Mat4> for JMat4 {
    #[inline]
    fn eq(&self, other: &Mat4) -> bool {
        self.glam() == *other
    }
}

impl fmt::Debug for JMat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.glam().fmt(f)
    }
}

impl From<Mat4> for JMat4 {
    #[inline]
    fn from(m: Mat4) -> JMat4 {
        JMat4 { glam: m }
    }
}

impl From<JMat4> for Mat4 {
    #[inline]
    fn from(m: JMat4) -> Mat4 {
        m.glam()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Float3(pub(crate) Vec3);
const_assert_eq!(mem::size_of::<Float3>(), 12);

unsafe impl ExternType for Float3 {
    type Id = type_id!("Float3");
    type Kind = kind::Trivial;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Int3(pub(crate) IVec3);
const_assert_eq!(mem::size_of::<Int3>(), 12);

unsafe impl ExternType for Int3 {
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
