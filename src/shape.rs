use glam::{Quat, Vec3, Vec3A};
use static_assertions::const_assert_eq;
use std::mem;
use std::pin::Pin;

use crate::base::*;
use crate::consts::{DEFAULT_CONVEX_RADIUS, MAX_CONVEX_RADIUS, MIN_CONVEX_RADIUS};
use crate::error::{JoltError, JoltResult};

#[cxx::bridge()]
pub mod ffi {
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

    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type Shape = crate::base::ffi::Shape;
        type Vec3 = crate::base::ffi::Vec3;
        type Quat = crate::base::ffi::Quat;
        type AABox = crate::base::ffi::AABox;

        type ShapeType;
        type ShapeSubType;

        type SphereSettings;
        type BoxSettings;
        type CapsuleSettings;
        type TaperedCapsuleSettings;
        type CylinderSettings;
        type TaperedCylinderSettings;
        type ConvexHullSettings;
        type TriangleSettings;
        type PlaneSettings;
        type MeshSettings;
        type HeightFieldSettings;
        type EmptySettings;

        type ScaledSettings;
        type RotatedTranslatedSettings;
        type OffsetCenterOfMassSettings;

        #[allow(dead_code)]
        type SubShapeSettings = crate::shape::SubShapeSettings;
        type StaticCompoundSettings;
        type MutableCompoundSettings;
        type CompoundShapeSubShape;

        fn CreateShapeSphere(settings: &SphereSettings) -> *mut Shape;
        fn CreateShapeBox(settings: &BoxSettings) -> *mut Shape;
        fn CreateShapeCapsule(settings: &CapsuleSettings) -> *mut Shape;
        fn CreateShapeTaperedCapsule(settings: &TaperedCapsuleSettings) -> *mut Shape;
        fn CreateShapeCylinder(settings: &CylinderSettings) -> *mut Shape;
        fn CreateShapeTaperedCylinder(settings: &TaperedCylinderSettings) -> *mut Shape;
        fn CreateShapeConvexHull(settings: &ConvexHullSettings) -> *mut Shape;
        fn CreateShapeTriangle(settings: &TriangleSettings) -> *mut Shape;
        fn CreateShapePlane(settings: &PlaneSettings) -> *mut Shape;
        fn CreateShapeMesh(settings: &MeshSettings) -> *mut Shape;
        fn CreateShapeHeightField(settings: &HeightFieldSettings) -> *mut Shape;
        fn CreateShapeEmpty(settings: &EmptySettings) -> *mut Shape;

        fn CreateShapeScaled(settings: &ScaledSettings) -> *mut Shape;
        fn CreateShapeRotatedTranslated(settings: &RotatedTranslatedSettings) -> *mut Shape;
        fn CreateShapeOffsetCenterOfMass(settings: &OffsetCenterOfMassSettings) -> *mut Shape;

        fn CreateShapeStaticCompound(settings: &StaticCompoundSettings) -> *mut Shape;
        fn CreateShapeMutableCompound(settings: &MutableCompoundSettings) -> *mut Shape;

        fn GetType(self: &Shape) -> ShapeType;
        fn GetSubType(self: &Shape) -> ShapeSubType;
        fn GetUserData(self: &Shape) -> u64;
        fn SetUserData(self: Pin<&mut Shape>, data: u64);
        fn GetCenterOfMass(self: &Shape) -> Vec3;
        fn MustBeStatic(self: &Shape) -> bool;
        fn GetLocalBounds(self: &Shape) -> AABox;
        fn GetInnerRadius(self: &Shape) -> f32;
        fn GetVolume(self: &Shape) -> f32;
        fn IsValidScale(self: &Shape, scale: Vec3) -> bool;
        fn MakeScaleValid(self: &Shape, scale: Vec3) -> Vec3;

        type StaticCompoundShape;
        fn GetNumSubShapes(self: &StaticCompoundShape) -> u32;
        unsafe fn GetSubShape(self: &StaticCompoundShape, index: u32) -> &CompoundShapeSubShape;
        fn GetCompoundUserData(self: &StaticCompoundShape, idx: u32) -> u32;
        fn SetCompoundUserData(self: Pin<&mut StaticCompoundShape>, idx: u32, data: u32);

        type MutableCompoundShape;
        fn GetNumSubShapes(self: &MutableCompoundShape) -> u32;
        unsafe fn GetSubShape(self: &MutableCompoundShape, index: u32) -> &CompoundShapeSubShape;
        fn GetCompoundUserData(self: &MutableCompoundShape, idx: u32) -> u32;
        fn SetCompoundUserData(self: Pin<&mut MutableCompoundShape>, idx: u32, data: u32);
        unsafe fn AddShape(
            self: Pin<&mut MutableCompoundShape>,
            position: Vec3,
            rotation: Quat,
            shape: *const Shape,
            user_data: u32,
        ) -> u32;
        fn RemoveShape(self: Pin<&mut MutableCompoundShape>, index: u32);
        fn ModifyShape(self: Pin<&mut MutableCompoundShape>, index: u32, position: Vec3, rotation: Quat);
        #[rust_name = "ModifyShapeEx"]
        unsafe fn ModifyShape(
            self: Pin<&mut MutableCompoundShape>,
            index: u32,
            position: Vec3,
            rotation: Quat,
            shape: *const Shape,
        );
        unsafe fn ModifyShapes(
            self: Pin<&mut MutableCompoundShape>,
            start_idx: u32,
            count: u32,
            position: *const Vec3,
            rotation: *const Quat,
            position_stride: u32,
            rotation_stride: u32,
        );
        fn AdjustCenterOfMass(self: Pin<&mut MutableCompoundShape>);
    }
}

pub type ShapeType = ffi::ShapeType;
pub type ShapeSubType = ffi::ShapeSubType;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SphereSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub radius: f32,
}
const_assert_eq!(std::mem::size_of::<SphereSettings>(), 24);

impl Default for SphereSettings {
    fn default() -> SphereSettings {
        SphereSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            radius: 0.5,
        }
    }
}

impl SphereSettings {
    pub fn new(radius: f32) -> SphereSettings {
        SphereSettings {
            radius,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoxSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub half_x: f32,
    pub half_y: f32,
    pub half_z: f32,
    pub convex_radius: f32,
}
const_assert_eq!(std::mem::size_of::<BoxSettings>(), 40);

impl Default for BoxSettings {
    fn default() -> BoxSettings {
        BoxSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            half_x: 0.0,
            half_y: 0.0,
            half_z: 0.0,
            convex_radius: DEFAULT_CONVEX_RADIUS,
        }
    }
}

impl BoxSettings {
    pub fn new(half_x: f32, half_y: f32, half_z: f32) -> BoxSettings {
        let min = half_x.min(half_y.min(half_z));
        BoxSettings {
            half_x,
            half_y,
            half_z,
            convex_radius: (min / 10.0).clamp(MIN_CONVEX_RADIUS, MAX_CONVEX_RADIUS),
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CapsuleSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub half_height: f32,
    pub radius: f32,
}
const_assert_eq!(std::mem::size_of::<CapsuleSettings>(), 32);

impl Default for CapsuleSettings {
    fn default() -> CapsuleSettings {
        CapsuleSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            radius: 0.0,
            half_height: 0.0,
        }
    }
}

impl CapsuleSettings {
    pub fn new(half_height: f32, radius: f32) -> CapsuleSettings {
        CapsuleSettings {
            half_height,
            radius,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TaperedCapsuleSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub half_height: f32,
    pub top_radius: f32,
    pub bottom_radius: f32,
}
const_assert_eq!(std::mem::size_of::<TaperedCapsuleSettings>(), 32);

impl Default for TaperedCapsuleSettings {
    fn default() -> TaperedCapsuleSettings {
        TaperedCapsuleSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            half_height: 0.0,
            top_radius: 0.0,
            bottom_radius: 0.0,
        }
    }
}

impl TaperedCapsuleSettings {
    pub fn new(half_height: f32, top_radius: f32, bottom_radius: f32) -> TaperedCapsuleSettings {
        TaperedCapsuleSettings {
            top_radius,
            bottom_radius,
            half_height,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CylinderSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub half_height: f32,
    pub radius: f32,
    pub convex_radius: f32,
}
const_assert_eq!(std::mem::size_of::<CylinderSettings>(), 32);

impl Default for CylinderSettings {
    fn default() -> CylinderSettings {
        CylinderSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            half_height: 0.0,
            radius: 0.0,
            convex_radius: DEFAULT_CONVEX_RADIUS,
        }
    }
}

impl CylinderSettings {
    pub fn new(half_height: f32, radius: f32) -> CylinderSettings {
        CylinderSettings {
            half_height,
            radius,
            convex_radius: (half_height / 10.0).clamp(MIN_CONVEX_RADIUS, MAX_CONVEX_RADIUS),
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TaperedCylinderSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub half_height: f32,
    pub top_radius: f32,
    pub bottom_radius: f32,
    pub convex_radius: f32,
}
const_assert_eq!(std::mem::size_of::<TaperedCylinderSettings>(), 40);

impl Default for TaperedCylinderSettings {
    fn default() -> TaperedCylinderSettings {
        TaperedCylinderSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            half_height: 0.0,
            top_radius: 0.0,
            bottom_radius: 0.0,
            convex_radius: DEFAULT_CONVEX_RADIUS,
        }
    }
}

impl TaperedCylinderSettings {
    pub fn new(half_height: f32, top_radius: f32, bottom_radius: f32) -> TaperedCylinderSettings {
        TaperedCylinderSettings {
            half_height,
            top_radius,
            bottom_radius,
            convex_radius: (half_height / 10.0).clamp(MIN_CONVEX_RADIUS, MAX_CONVEX_RADIUS),
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConvexHullSettings<'t> {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub points: &'t [Vec3A],
    pub max_convex_radius: f32,
    pub max_error_convex_radius: f32,
    pub hull_tolerance: f32,
}
const_assert_eq!(std::mem::size_of::<ConvexHullSettings>(), 56);

impl<'t> Default for ConvexHullSettings<'t> {
    fn default() -> ConvexHullSettings<'t> {
        ConvexHullSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            points: &[],
            max_convex_radius: 0.05,
            max_error_convex_radius: 0.05,
            hull_tolerance: 1.0e-3,
        }
    }
}

impl<'t> ConvexHullSettings<'t> {
    pub fn new(points: &'t [Vec3A]) -> ConvexHullSettings<'t> {
        ConvexHullSettings {
            points,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TriangleSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub density: f32,
    pub convex_radius: f32,
    pub v1: Vec3A,
    pub v2: Vec3A,
    pub v3: Vec3A,
}
const_assert_eq!(std::mem::size_of::<TriangleSettings>(), 80);

impl Default for TriangleSettings {
    fn default() -> TriangleSettings {
        TriangleSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            convex_radius: 0.05,
            v1: Vec3A::ZERO,
            v2: Vec3A::ZERO,
            v3: Vec3A::ZERO,
        }
    }
}

impl TriangleSettings {
    pub fn new(v1: Vec3A, v2: Vec3A, v3: Vec3A) -> TriangleSettings {
        TriangleSettings {
            v1,
            v2,
            v3,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PlaneSettings {
    pub user_data: u64,
    pub material: Option<RefPhysicsMaterial>,
    pub plane: Plane,
    pub half_extent: f32,
}
const_assert_eq!(std::mem::size_of::<PlaneSettings>(), 48);

impl Default for PlaneSettings {
    fn default() -> PlaneSettings {
        PlaneSettings {
            user_data: 0,
            material: None,
            plane: Plane::new(Vec3::Y, 0.0),
            half_extent: 1000.0,
        }
    }
}

impl PlaneSettings {
    pub fn new(plane: Plane, half_extent: f32) -> PlaneSettings {
        PlaneSettings {
            plane,
            half_extent,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshSettings<'t> {
    pub user_data: u64,
    pub triangle_vertices: &'t [Vec3],
    pub indexed_triangles: &'t [IndexedTriangle],
    pub materials: &'t [RefPhysicsMaterial],
    pub max_triangles_per_leaf: u32,
    pub active_edge_cos_threshold_angle: f32,
}
const_assert_eq!(std::mem::size_of::<MeshSettings>(), 64);

impl<'t> Default for MeshSettings<'t> {
    fn default() -> MeshSettings<'t> {
        MeshSettings {
            user_data: 0,
            triangle_vertices: &[],
            indexed_triangles: &[],
            materials: &[],
            max_triangles_per_leaf: 8,
            active_edge_cos_threshold_angle: 0.996195, // cos(5)
        }
    }
}

impl<'t> MeshSettings<'t> {
    pub fn new(triangle_vertices: &'t [Vec3], indexed_triangles: &'t [IndexedTriangle]) -> MeshSettings<'t> {
        MeshSettings {
            triangle_vertices,
            indexed_triangles,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HeightFieldSettings<'t> {
    pub user_data: u64,
    pub offset: Vec3A,
    pub scale: Vec3A,
    pub sample_count: u32,
    pub min_height_value: f32,
    pub max_height_value: f32,
    pub block_size: u32,
    pub bits_per_sample: u32,
    pub height_samples: &'t [f32],
    pub material_indices: &'t [u8],
    pub materials: &'t [RefPhysicsMaterial],
    pub active_edge_cos_threshold_angle: f32,
}
const_assert_eq!(std::mem::size_of::<HeightFieldSettings>(), 128);

impl<'t> Default for HeightFieldSettings<'t> {
    fn default() -> HeightFieldSettings<'t> {
        HeightFieldSettings {
            user_data: 0,
            offset: Vec3A::ZERO,
            scale: Vec3A::ONE,
            sample_count: 0,
            min_height_value: f32::MAX,
            max_height_value: f32::MIN,
            block_size: 2,
            bits_per_sample: 8,
            height_samples: &[],
            material_indices: &[],
            materials: &[],
            active_edge_cos_threshold_angle: 0.996195, // cos(5)
        }
    }
}

impl<'t> HeightFieldSettings<'t> {
    pub fn new(height_samples: &'t [f32], sample_count: u32) -> HeightFieldSettings<'t> {
        HeightFieldSettings {
            height_samples,
            sample_count,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EmptySettings {
    pub user_data: u64,
    pub center_of_mass: Vec3A,
}
const_assert_eq!(std::mem::size_of::<EmptySettings>(), 32);

impl Default for EmptySettings {
    fn default() -> EmptySettings {
        EmptySettings {
            user_data: 0,
            center_of_mass: Vec3A::ZERO,
        }
    }
}

impl EmptySettings {
    pub fn new(center_of_mass: Vec3A) -> EmptySettings {
        EmptySettings {
            center_of_mass,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScaledSettings {
    pub user_data: u64,
    pub inner_shape: Option<RefShape>,
    pub scale: Vec3A,
}
const_assert_eq!(std::mem::size_of::<ScaledSettings>(), 32);

impl Default for ScaledSettings {
    fn default() -> ScaledSettings {
        ScaledSettings {
            user_data: 0,
            inner_shape: None,
            scale: Vec3A::ONE,
        }
    }
}

impl ScaledSettings {
    pub fn new(inner_shape: RefShape, scale: Vec3A) -> ScaledSettings {
        ScaledSettings {
            user_data: 0,
            inner_shape: Some(inner_shape),
            scale,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RotatedTranslatedSettings {
    pub user_data: u64,
    pub inner_shape: Option<RefShape>,
    pub position: Vec3A,
    pub rotation: Quat,
}
const_assert_eq!(std::mem::size_of::<RotatedTranslatedSettings>(), 48);

impl Default for RotatedTranslatedSettings {
    fn default() -> RotatedTranslatedSettings {
        RotatedTranslatedSettings {
            user_data: 0,
            inner_shape: None,
            position: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

impl RotatedTranslatedSettings {
    pub fn new(inner_shape: RefShape, position: Vec3A, rotation: Quat) -> RotatedTranslatedSettings {
        RotatedTranslatedSettings {
            user_data: 0,
            inner_shape: Some(inner_shape),
            position,
            rotation,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct OffsetCenterOfMassSettings {
    pub user_data: u64,
    pub inner_shape: Option<RefShape>,
    pub offset: Vec3A,
}
const_assert_eq!(std::mem::size_of::<OffsetCenterOfMassSettings>(), 32);

impl Default for OffsetCenterOfMassSettings {
    fn default() -> OffsetCenterOfMassSettings {
        OffsetCenterOfMassSettings {
            user_data: 0,
            inner_shape: None,
            offset: Vec3A::ZERO,
        }
    }
}

impl OffsetCenterOfMassSettings {
    pub fn new(inner_shape: RefShape, offset: Vec3A) -> OffsetCenterOfMassSettings {
        OffsetCenterOfMassSettings {
            user_data: 0,
            inner_shape: Some(inner_shape),
            offset,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SubShapeSettings {
    _shape: *const (),
    pub shape: Option<RefShape>,
    pub position: Vec3A,
    pub rotation: Quat,
    pub user_data: u32,
}
const_assert_eq!(std::mem::size_of::<SubShapeSettings>(), 64);

unsafe impl cxx::ExternType for SubShapeSettings {
    type Id = cxx::type_id!("SubShapeSettings");
    type Kind = cxx::kind::Trivial;
}

impl Default for SubShapeSettings {
    fn default() -> SubShapeSettings {
        SubShapeSettings {
            _shape: std::ptr::null(),
            shape: None,
            position: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
            user_data: 0,
        }
    }
}

impl SubShapeSettings {
    pub fn new(shape: RefShape, position: Vec3A, rotation: Quat) -> SubShapeSettings {
        SubShapeSettings {
            _shape: std::ptr::null(),
            shape: Some(shape),
            position,
            rotation,
            user_data: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SubShape {
    pub shape: RefShape,
    pub position: Vec3,
    rotation: Vec3, // X, Y, Z of rotation quaternion
    pub user_data: u32,
    pub is_rotation_identity: bool,
}
const_assert_eq!(std::mem::size_of::<SubShape>(), 40);

impl SubShape {
    #[inline]
    pub fn rotation(&self) -> Quat {
        if self.is_rotation_identity {
            Quat::IDENTITY
        } else {
            let w = (1.0 - self.rotation.length_squared()).max(0.0).sqrt();
            Quat::from_xyzw(self.rotation.x, self.rotation.y, self.rotation.z, w)
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct StaticCompoundSettings<'t> {
    pub user_data: u64,
    pub sub_shapes: &'t [SubShapeSettings],
}

impl<'t> Default for StaticCompoundSettings<'t> {
    fn default() -> StaticCompoundSettings<'t> {
        StaticCompoundSettings {
            user_data: 0,
            sub_shapes: &[],
        }
    }
}

impl StaticCompoundSettings<'_> {
    pub fn new(sub_shapes: &[SubShapeSettings]) -> StaticCompoundSettings<'_> {
        StaticCompoundSettings {
            user_data: 0,
            sub_shapes,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MutableCompoundSettings<'t> {
    pub user_data: u64,
    pub sub_shapes: &'t [SubShapeSettings],
}

impl<'t> Default for MutableCompoundSettings<'t> {
    fn default() -> MutableCompoundSettings<'t> {
        MutableCompoundSettings {
            user_data: 0,
            sub_shapes: &[],
        }
    }
}

impl MutableCompoundSettings<'_> {
    pub fn new(sub_shapes: &[SubShapeSettings]) -> MutableCompoundSettings<'_> {
        MutableCompoundSettings {
            user_data: 0,
            sub_shapes,
        }
    }
}

pub fn create_sphere_shape(settings: &SphereSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeSphere(mem::transmute::<&SphereSettings, &ffi::SphereSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_box_shape(settings: &BoxSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeBox(mem::transmute::<&BoxSettings, &ffi::BoxSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_capsule_shape(settings: &CapsuleSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeCapsule(mem::transmute::<&CapsuleSettings, &ffi::CapsuleSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_tapered_capsule_shape(settings: &TaperedCapsuleSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeTaperedCapsule(
            mem::transmute::<&TaperedCapsuleSettings, &ffi::TaperedCapsuleSettings>(settings),
        );
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_cylinder_shape(settings: &CylinderSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeCylinder(mem::transmute::<&CylinderSettings, &ffi::CylinderSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_tapered_cylinder_shape(settings: &TaperedCylinderSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeTaperedCylinder(mem::transmute::<
            &TaperedCylinderSettings,
            &ffi::TaperedCylinderSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_convex_hull_shape(settings: &ConvexHullSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeConvexHull(mem::transmute::<&ConvexHullSettings, &ffi::ConvexHullSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_triangle_shape(settings: &TriangleSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeTriangle(mem::transmute::<&TriangleSettings, &ffi::TriangleSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_plane_shape(settings: &PlaneSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapePlane(mem::transmute::<&PlaneSettings, &ffi::PlaneSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_mesh_shape(settings: &MeshSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeMesh(mem::transmute::<&MeshSettings, &ffi::MeshSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_height_field_shape(settings: &HeightFieldSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeHeightField(mem::transmute::<&HeightFieldSettings, &ffi::HeightFieldSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_empty_shape(settings: &EmptySettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeEmpty(mem::transmute::<&EmptySettings, &ffi::EmptySettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_scaled_shape(settings: &ScaledSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeScaled(mem::transmute::<&ScaledSettings, &ffi::ScaledSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_rotated_translated_shape(settings: &RotatedTranslatedSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeRotatedTranslated(mem::transmute::<
            &RotatedTranslatedSettings,
            &ffi::RotatedTranslatedSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_offset_center_of_mass_shape(settings: &OffsetCenterOfMassSettings) -> JoltResult<RefShape> {
    unsafe {
        let ptr = ffi::CreateShapeOffsetCenterOfMass(mem::transmute::<
            &OffsetCenterOfMassSettings,
            &ffi::OffsetCenterOfMassSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefShape::new(ptr))
    }
}

pub fn create_static_compound_shape(settings: &StaticCompoundSettings) -> JoltResult<RefStaticCompoundShape> {
    if settings.sub_shapes.len() < 2 {
        return Err(JoltError::TooLessSubShape);
    }
    unsafe {
        let ptr = ffi::CreateShapeStaticCompound(
            mem::transmute::<&StaticCompoundSettings, &ffi::StaticCompoundSettings>(settings),
        );
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefStaticCompoundShape(RefShape::new(ptr)))
    }
}

pub fn create_mutable_compound_shape(settings: &MutableCompoundSettings) -> JoltResult<RefMutableCompoundShape> {
    if settings.sub_shapes.len() < 2 {
        return Err(JoltError::TooLessSubShape);
    }
    unsafe {
        let ptr = ffi::CreateShapeMutableCompound(mem::transmute::<
            &MutableCompoundSettings,
            &ffi::MutableCompoundSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(RefMutableCompoundShape(RefShape::new(ptr)))
    }
}

macro_rules! shape_methods {
    ($type:ty, $ref:path, $mut:path) => {
        impl $type {
            #[inline]
            pub fn get_type(&self) -> ShapeType {
                $ref(self).GetType()
            }

            #[inline]
            pub fn get_sub_type(&self) -> ShapeSubType {
                $ref(self).GetSubType()
            }

            #[inline]
            pub fn get_user_data(&self) -> u64 {
                $ref(self).GetUserData()
            }

            #[inline]
            pub unsafe fn set_user_data(&mut self, data: u64) {
                $mut(self).SetUserData(data);
            }

            #[inline]
            pub fn get_center_of_mass(&self) -> Vec3A {
                $ref(self).GetCenterOfMass().0
            }

            #[inline]
            pub fn must_be_static(&self) -> bool {
                $ref(self).MustBeStatic()
            }

            #[inline]
            pub fn get_local_bounds(&self) -> AABox {
                $ref(self).GetLocalBounds()
            }

            #[inline]
            pub fn get_inner_radius(&self) -> f32 {
                $ref(self).GetInnerRadius()
            }

            #[inline]
            pub fn get_volume(&self) -> f32 {
                $ref(self).GetVolume()
            }

            #[inline]
            pub fn is_valid_scale(&self, scale: Vec3A) -> bool {
                $ref(self).IsValidScale(scale.into())
            }

            #[inline]
            pub fn make_scale_valid(&self, scale: Vec3A) -> Vec3A {
                $ref(self).MakeScaleValid(scale.into()).0
            }
        }
    };
}

shape_methods!(RefShape, RefShape::as_ref, RefShape::as_mut);

/// In C++ code, Shape* is actually a smart pointer with a reference count.
/// Currently, we don't have a perfect representation of this in Rust.
/// So we're marking all `&mut self` functions as unsafe for now.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefStaticCompoundShape(RefShape);

impl From<RefStaticCompoundShape> for RefShape {
    #[inline]
    fn from(shape: RefStaticCompoundShape) -> RefShape {
        shape.0
    }
}

impl RefStaticCompoundShape {
    #[inline]
    fn as_shape_ref(&self) -> &ffi::Shape {
        self.0.as_ref()
    }

    #[inline]
    fn as_shape_mut(&mut self) -> Pin<&mut ffi::Shape> {
        self.0.as_mut()
    }
}

shape_methods!(
    RefStaticCompoundShape,
    RefStaticCompoundShape::as_shape_ref,
    RefStaticCompoundShape::as_shape_mut
);

impl RefStaticCompoundShape {
    #[inline]
    fn as_ref(&self) -> &ffi::StaticCompoundShape {
        unsafe { self.0.as_ref_t::<ffi::StaticCompoundShape>() }
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::StaticCompoundShape> {
        unsafe { self.0.as_mut_t::<ffi::StaticCompoundShape>() }
    }

    #[inline]
    pub fn get_num_sub_shapes(&self) -> u32 {
        self.as_ref().GetNumSubShapes()
    }

    #[inline]
    pub fn get_sub_shape(&self, idx: u32) -> &SubShape {
        unsafe { mem::transmute(self.as_ref().GetSubShape(idx)) }
    }

    #[inline]
    pub fn get_compound_user_data(&self, idx: u32) -> u32 {
        self.as_ref().GetCompoundUserData(idx)
    }

    #[inline]
    pub unsafe fn set_compound_user_data(&mut self, idx: u32, data: u32) {
        self.as_mut().SetCompoundUserData(idx, data);
    }
}

/// In C++ code, Shape* is actually a smart pointer with a reference count.
/// Currently, we don't have a perfect representation of this in Rust.
/// So we're marking all `&mut self` functions as unsafe for now.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefMutableCompoundShape(RefShape);

impl From<RefMutableCompoundShape> for RefShape {
    #[inline]
    fn from(shape: RefMutableCompoundShape) -> RefShape {
        shape.0
    }
}

impl RefMutableCompoundShape {
    #[inline]
    fn as_shape_ref(&self) -> &ffi::Shape {
        self.0.as_ref()
    }

    #[inline]
    fn as_shape_mut(&mut self) -> Pin<&mut ffi::Shape> {
        self.0.as_mut()
    }
}

shape_methods!(
    RefMutableCompoundShape,
    RefMutableCompoundShape::as_shape_ref,
    RefMutableCompoundShape::as_shape_mut
);

impl RefMutableCompoundShape {
    #[inline]
    fn as_ref(&self) -> &ffi::MutableCompoundShape {
        unsafe { self.0.as_ref_t::<ffi::MutableCompoundShape>() }
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::MutableCompoundShape> {
        unsafe { self.0.as_mut_t::<ffi::MutableCompoundShape>() }
    }

    #[inline]
    pub fn get_num_sub_shapes(&self) -> u32 {
        self.as_ref().GetNumSubShapes()
    }

    #[inline]
    pub fn get_sub_shape(&self, idx: u32) -> &SubShape {
        unsafe { mem::transmute(self.as_ref().GetSubShape(idx)) }
    }

    #[inline]
    pub fn get_compound_user_data(&self, idx: u32) -> u32 {
        self.as_ref().GetCompoundUserData(idx)
    }

    #[inline]
    pub unsafe fn set_compound_user_data(&mut self, idx: u32, data: u32) {
        self.as_mut().SetCompoundUserData(idx, data);
    }

    #[inline]
    pub unsafe fn add_shape(&mut self, position: Vec3A, rotation: Quat, shape: &RefShape, user_data: u32) -> u32 {
        unsafe {
            self.as_mut()
                .AddShape(position.into(), rotation.into(), shape.as_ptr(), user_data)
        }
    }

    #[inline]
    pub unsafe fn remove_shape(&mut self, index: u32) {
        self.as_mut().RemoveShape(index);
    }

    #[inline]
    pub unsafe fn modify_shape(&mut self, index: u32, position: Vec3A, rotation: Quat) {
        self.as_mut().ModifyShape(index, position.into(), rotation.into());
    }

    #[inline]
    pub unsafe fn modify_shape_ex(&mut self, index: u32, position: Vec3A, rotation: Quat, shape: &RefShape) {
        unsafe {
            self.as_mut()
                .ModifyShapeEx(index, position.into(), rotation.into(), shape.as_ptr())
        }
    }

    #[inline]
    pub unsafe fn adjust_center_of_mass(&mut self) {
        self.as_mut().AdjustCenterOfMass();
    }

    #[inline]
    pub unsafe fn modify_shapes(&mut self, sub_shape_start_idx: u32, position: &[Vec3A], rotation: &[Quat]) {
        let count = usize::min(position.len(), rotation.len()) as u32;
        unsafe {
            self.as_mut().ModifyShapes(
                sub_shape_start_idx,
                count,
                position.as_ptr() as *const XVec3,
                rotation.as_ptr() as *const XQuat,
                mem::size_of::<Vec3A>() as u32,
                mem::size_of::<Quat>() as u32,
            );
        }
    }

    #[inline]
    pub unsafe fn modify_shapes_ex(
        &mut self,
        sub_shape_start_idx: u32,
        position_ptr: *const Vec3A,
        rotation_ptr: *const Quat,
        count: u32,
        position_stride: u32,
        rotation_stride: u32,
    ) {
        self.as_mut().ModifyShapes(
            sub_shape_start_idx,
            count,
            position_ptr as *const XVec3,
            rotation_ptr as *const XQuat,
            position_stride,
            rotation_stride,
        );
    }
}
