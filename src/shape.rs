use core::fmt;
use cxx::{kind, type_id, ExternType};
use glam::{Quat, Vec3, Vec3A};
#[cfg(feature = "glam-ext")]
use glam_ext::{Isometry3A, Transform3A};
use static_assertions::const_assert_eq;
use std::mem;
use std::pin::Pin;
use std::ptr::NonNull;

use crate::base::{AABox, IndexedTriangle, JMut, JQuat, JRef, JRefTarget, JVec3, Plane, ShapeSubType, ShapeType};
use crate::consts::{DEFAULT_CONVEX_RADIUS, DEFAULT_ERROR_CONVEX_RADIUS, MAX_CONVEX_RADIUS, MIN_CONVEX_RADIUS};
use crate::error::{JoltError, JoltResult};
use crate::JMutTarget;

#[cxx::bridge()]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type ShapeType = crate::base::ffi::ShapeType;
        type ShapeSubType = crate::base::ffi::ShapeSubType;

        type Vec3 = crate::base::ffi::Vec3;
        type Quat = crate::base::ffi::Quat;
        type AABox = crate::base::ffi::AABox;

        type XSphereShapeSettings = crate::shape::SphereShapeSettings;
        type XBoxShapeSettings = crate::shape::BoxShapeSettings;
        type XCapsuleShapeSettings = crate::shape::CapsuleShapeSettings;
        type XTaperedCapsuleShapeSettings = crate::shape::TaperedCapsuleShapeSettings;
        type XCylinderShapeSettings = crate::shape::CylinderShapeSettings;
        type XTaperedCylinderShapeSettings = crate::shape::TaperedCylinderShapeSettings;
        type XConvexHullShapeSettings = crate::shape::ConvexHullShapeSettings<'static>;
        type XTriangleShapeSettings = crate::shape::TriangleShapeSettings;
        type XPlaneShapeSettings = crate::shape::PlaneShapeSettings;
        type XMeshShapeSettings = crate::shape::MeshShapeSettings<'static>;
        type XHeightFieldShapeSettings = crate::shape::HeightFieldShapeSettings<'static>;
        type XEmptyShapeSettings = crate::shape::EmptyShapeSettings;

        type XScaledShapeSettings = crate::shape::ScaledShapeSettings;
        type XRotatedTranslatedShapeSettings = crate::shape::RotatedTranslatedShapeSettings;
        type XOffsetCenterOfMassShapeSettings = crate::shape::OffsetCenterOfMassShapeSettings;

        #[allow(dead_code)]
        type XSubShapeSettings = crate::shape::SubShapeSettings;
        type XStaticCompoundShapeSettings = crate::shape::StaticCompoundShapeSettings<'static>;
        type XMutableCompoundShapeSettings = crate::shape::MutableCompoundShapeSettings<'static>;
        type XCompoundSubShape = crate::shape::CompoundSubShape;

        type PhysicsMaterial;
        unsafe fn DropPhysicsMaterial(material: *mut PhysicsMaterial);
        unsafe fn ClonePhysicsMaterial(material: *mut PhysicsMaterial) -> *mut PhysicsMaterial;
        unsafe fn CountRefPhysicsMaterial(material: *const PhysicsMaterial) -> u32;

        type Shape;
        unsafe fn DropShape(shape: *mut Shape);
        unsafe fn CloneShape(shape: *mut Shape) -> *mut Shape;
        unsafe fn CountRefShape(shape: *const Shape) -> u32;

        fn CreateSphereShape(settings: &XSphereShapeSettings) -> *mut Shape;
        fn CreateBoxShape(settings: &XBoxShapeSettings) -> *mut Shape;
        fn CreateCapsuleShape(settings: &XCapsuleShapeSettings) -> *mut Shape;
        fn CreateTaperedCapsuleShape(settings: &XTaperedCapsuleShapeSettings) -> *mut Shape;
        fn CreateCylinderShape(settings: &XCylinderShapeSettings) -> *mut Shape;
        fn CreateTaperedCylinderShape(settings: &XTaperedCylinderShapeSettings) -> *mut Shape;
        fn CreateConvexHullShape(settings: &XConvexHullShapeSettings) -> *mut Shape;
        fn CreateTriangleShape(settings: &XTriangleShapeSettings) -> *mut Shape;
        fn CreatePlaneShape(settings: &XPlaneShapeSettings) -> *mut Shape;
        fn CreateMeshShape(settings: &XMeshShapeSettings) -> *mut Shape;
        fn CreateHeightFieldShape(settings: &XHeightFieldShapeSettings) -> *mut Shape;
        fn CreateEmptyShape(settings: &XEmptyShapeSettings) -> *mut Shape;

        fn CreateScaledShape(settings: &XScaledShapeSettings) -> *mut Shape;
        fn CreateRotatedTranslatedShape(settings: &XRotatedTranslatedShapeSettings) -> *mut Shape;
        fn CreateOffsetCenterOfMassShape(settings: &XOffsetCenterOfMassShapeSettings) -> *mut Shape;

        fn CreateStaticCompoundShape(settings: &XStaticCompoundShapeSettings) -> *mut StaticCompoundShape;
        fn CreateMutableCompoundShape(settings: &XMutableCompoundShapeSettings) -> *mut MutableCompoundShape;

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
        unsafe fn DropStaticCompoundShape(shape: *mut StaticCompoundShape);
        unsafe fn CloneStaticCompoundShape(shape: *mut StaticCompoundShape) -> *mut StaticCompoundShape;
        unsafe fn CountRefStaticCompoundShape(shape: *const StaticCompoundShape) -> u32;

        fn GetType(self: &StaticCompoundShape) -> ShapeType;
        fn GetSubType(self: &StaticCompoundShape) -> ShapeSubType;
        fn GetUserData(self: &StaticCompoundShape) -> u64;
        fn SetUserData(self: Pin<&mut StaticCompoundShape>, data: u64);
        fn GetCenterOfMass(self: &StaticCompoundShape) -> Vec3;
        fn MustBeStatic(self: &StaticCompoundShape) -> bool;
        fn GetLocalBounds(self: &StaticCompoundShape) -> AABox;
        fn GetInnerRadius(self: &StaticCompoundShape) -> f32;
        fn GetVolume(self: &StaticCompoundShape) -> f32;
        fn IsValidScale(self: &StaticCompoundShape, scale: Vec3) -> bool;
        fn MakeScaleValid(self: &StaticCompoundShape, scale: Vec3) -> Vec3;
        fn GetNumSubShapes(self: &StaticCompoundShape) -> u32;
        unsafe fn GetSubShape(self: &StaticCompoundShape, index: u32) -> &XCompoundSubShape;
        fn GetCompoundUserData(self: &StaticCompoundShape, idx: u32) -> u32;
        fn SetCompoundUserData(self: Pin<&mut StaticCompoundShape>, idx: u32, data: u32);

        type MutableCompoundShape;
        unsafe fn DropMutableCompoundShape(shape: *mut MutableCompoundShape);
        unsafe fn CloneMutableCompoundShape(shape: *mut MutableCompoundShape) -> *mut MutableCompoundShape;
        unsafe fn CountRefMutableCompoundShape(shape: *const MutableCompoundShape) -> u32;

        fn GetType(self: &MutableCompoundShape) -> ShapeType;
        fn GetSubType(self: &MutableCompoundShape) -> ShapeSubType;
        fn GetUserData(self: &MutableCompoundShape) -> u64;
        fn SetUserData(self: Pin<&mut MutableCompoundShape>, data: u64);
        fn GetCenterOfMass(self: &MutableCompoundShape) -> Vec3;
        fn MustBeStatic(self: &MutableCompoundShape) -> bool;
        fn GetLocalBounds(self: &MutableCompoundShape) -> AABox;
        fn GetInnerRadius(self: &MutableCompoundShape) -> f32;
        fn GetVolume(self: &MutableCompoundShape) -> f32;
        fn IsValidScale(self: &MutableCompoundShape, scale: Vec3) -> bool;
        fn MakeScaleValid(self: &MutableCompoundShape, scale: Vec3) -> Vec3;
        fn GetNumSubShapes(self: &MutableCompoundShape) -> u32;
        unsafe fn GetSubShape(self: &MutableCompoundShape, index: u32) -> &XCompoundSubShape;
        fn GetCompoundUserData(self: &MutableCompoundShape, idx: u32) -> u32;
        fn SetCompoundUserData(self: Pin<&mut MutableCompoundShape>, idx: u32, data: u32);
        unsafe fn AddShape(
            self: Pin<&mut MutableCompoundShape>,
            position: Vec3,
            rotation: Quat,
            shape: *const Shape,
            user_data: u32,
            index: u32,
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SphereShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub radius: f32,
}
const_assert_eq!(std::mem::size_of::<SphereShapeSettings>(), 24);

unsafe impl ExternType for SphereShapeSettings {
    type Id = type_id!("XSphereShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for SphereShapeSettings {
    fn default() -> SphereShapeSettings {
        SphereShapeSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            radius: 0.5,
        }
    }
}

impl SphereShapeSettings {
    pub fn new(radius: f32) -> SphereShapeSettings {
        SphereShapeSettings {
            radius,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoxShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub half_x: f32,
    pub half_y: f32,
    pub half_z: f32,
    pub convex_radius: f32,
}
const_assert_eq!(std::mem::size_of::<BoxShapeSettings>(), 40);

unsafe impl ExternType for BoxShapeSettings {
    type Id = type_id!("XBoxShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for BoxShapeSettings {
    fn default() -> BoxShapeSettings {
        BoxShapeSettings {
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

impl BoxShapeSettings {
    pub fn new(half_x: f32, half_y: f32, half_z: f32) -> BoxShapeSettings {
        let min = half_x.min(half_y.min(half_z));
        BoxShapeSettings {
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
pub struct CapsuleShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub half_height: f32,
    pub radius: f32,
}
const_assert_eq!(std::mem::size_of::<CapsuleShapeSettings>(), 32);

unsafe impl ExternType for CapsuleShapeSettings {
    type Id = type_id!("XCapsuleShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for CapsuleShapeSettings {
    fn default() -> CapsuleShapeSettings {
        CapsuleShapeSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            radius: 0.0,
            half_height: 0.0,
        }
    }
}

impl CapsuleShapeSettings {
    pub fn new(half_height: f32, radius: f32) -> CapsuleShapeSettings {
        CapsuleShapeSettings {
            half_height,
            radius,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TaperedCapsuleShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub half_height: f32,
    pub top_radius: f32,
    pub bottom_radius: f32,
}
const_assert_eq!(std::mem::size_of::<TaperedCapsuleShapeSettings>(), 32);

unsafe impl ExternType for TaperedCapsuleShapeSettings {
    type Id = type_id!("XTaperedCapsuleShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for TaperedCapsuleShapeSettings {
    fn default() -> TaperedCapsuleShapeSettings {
        TaperedCapsuleShapeSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            half_height: 0.0,
            top_radius: 0.0,
            bottom_radius: 0.0,
        }
    }
}

impl TaperedCapsuleShapeSettings {
    pub fn new(half_height: f32, top_radius: f32, bottom_radius: f32) -> TaperedCapsuleShapeSettings {
        TaperedCapsuleShapeSettings {
            top_radius,
            bottom_radius,
            half_height,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CylinderShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub half_height: f32,
    pub radius: f32,
    pub convex_radius: f32,
}
const_assert_eq!(std::mem::size_of::<CylinderShapeSettings>(), 32);

unsafe impl ExternType for CylinderShapeSettings {
    type Id = type_id!("XCylinderShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for CylinderShapeSettings {
    fn default() -> CylinderShapeSettings {
        CylinderShapeSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            half_height: 0.0,
            radius: 0.0,
            convex_radius: DEFAULT_CONVEX_RADIUS,
        }
    }
}

impl CylinderShapeSettings {
    pub fn new(half_height: f32, radius: f32) -> CylinderShapeSettings {
        CylinderShapeSettings {
            half_height,
            radius,
            convex_radius: (half_height / 10.0).clamp(MIN_CONVEX_RADIUS, MAX_CONVEX_RADIUS),
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TaperedCylinderShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub half_height: f32,
    pub top_radius: f32,
    pub bottom_radius: f32,
    pub convex_radius: f32,
}
const_assert_eq!(std::mem::size_of::<TaperedCylinderShapeSettings>(), 40);

unsafe impl ExternType for TaperedCylinderShapeSettings {
    type Id = type_id!("XTaperedCylinderShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for TaperedCylinderShapeSettings {
    fn default() -> TaperedCylinderShapeSettings {
        TaperedCylinderShapeSettings {
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

impl TaperedCylinderShapeSettings {
    pub fn new(half_height: f32, top_radius: f32, bottom_radius: f32) -> TaperedCylinderShapeSettings {
        TaperedCylinderShapeSettings {
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
pub struct ConvexHullShapeSettings<'t> {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub points: &'t [Vec3A],
    pub max_convex_radius: f32,
    pub max_error_convex_radius: f32,
    pub hull_tolerance: f32,
}
const_assert_eq!(std::mem::size_of::<ConvexHullShapeSettings>(), 56);

unsafe impl ExternType for ConvexHullShapeSettings<'_> {
    type Id = type_id!("XConvexHullShapeSettings");
    type Kind = kind::Trivial;
}

impl<'t> Default for ConvexHullShapeSettings<'t> {
    fn default() -> ConvexHullShapeSettings<'t> {
        ConvexHullShapeSettings {
            user_data: 0,
            material: None,
            density: 1000.0,
            points: &[],
            max_convex_radius: DEFAULT_CONVEX_RADIUS,
            max_error_convex_radius: DEFAULT_ERROR_CONVEX_RADIUS,
            hull_tolerance: 1.0e-3,
        }
    }
}

impl<'t> ConvexHullShapeSettings<'t> {
    pub fn new(points: &'t [Vec3A]) -> ConvexHullShapeSettings<'t> {
        ConvexHullShapeSettings {
            points,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TriangleShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub density: f32,
    pub convex_radius: f32,
    pub v1: Vec3A,
    pub v2: Vec3A,
    pub v3: Vec3A,
}
const_assert_eq!(std::mem::size_of::<TriangleShapeSettings>(), 80);

unsafe impl ExternType for TriangleShapeSettings {
    type Id = type_id!("XTriangleShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for TriangleShapeSettings {
    fn default() -> TriangleShapeSettings {
        TriangleShapeSettings {
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

impl TriangleShapeSettings {
    pub fn new(v1: Vec3A, v2: Vec3A, v3: Vec3A) -> TriangleShapeSettings {
        TriangleShapeSettings {
            v1,
            v2,
            v3,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PlaneShapeSettings {
    pub user_data: u64,
    pub material: Option<JRef<PhysicsMaterial>>,
    pub plane: Plane,
    pub half_extent: f32,
}
const_assert_eq!(std::mem::size_of::<PlaneShapeSettings>(), 48);

unsafe impl ExternType for PlaneShapeSettings {
    type Id = type_id!("XPlaneShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for PlaneShapeSettings {
    fn default() -> PlaneShapeSettings {
        PlaneShapeSettings {
            user_data: 0,
            material: None,
            plane: Plane::new(Vec3::Y, 0.0),
            half_extent: 1000.0,
        }
    }
}

impl PlaneShapeSettings {
    pub fn new(plane: Plane, half_extent: f32) -> PlaneShapeSettings {
        PlaneShapeSettings {
            plane,
            half_extent,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshShapeSettings<'t> {
    pub user_data: u64,
    pub triangle_vertices: &'t [Vec3],
    pub indexed_triangles: &'t [IndexedTriangle],
    pub materials: &'t [JRef<PhysicsMaterial>],
    pub max_triangles_per_leaf: u32,
    pub active_edge_cos_threshold_angle: f32,
}
const_assert_eq!(std::mem::size_of::<MeshShapeSettings>(), 64);

unsafe impl ExternType for MeshShapeSettings<'_> {
    type Id = type_id!("XMeshShapeSettings");
    type Kind = kind::Trivial;
}

impl<'t> Default for MeshShapeSettings<'t> {
    fn default() -> MeshShapeSettings<'t> {
        MeshShapeSettings {
            user_data: 0,
            triangle_vertices: &[],
            indexed_triangles: &[],
            materials: &[],
            max_triangles_per_leaf: 8,
            active_edge_cos_threshold_angle: 0.996195, // cos(5)
        }
    }
}

impl<'t> MeshShapeSettings<'t> {
    pub fn new(triangle_vertices: &'t [Vec3], indexed_triangles: &'t [IndexedTriangle]) -> MeshShapeSettings<'t> {
        MeshShapeSettings {
            triangle_vertices,
            indexed_triangles,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HeightFieldShapeSettings<'t> {
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
    pub materials: &'t [JRef<PhysicsMaterial>],
    pub active_edge_cos_threshold_angle: f32,
}
const_assert_eq!(std::mem::size_of::<HeightFieldShapeSettings>(), 128);

unsafe impl ExternType for HeightFieldShapeSettings<'_> {
    type Id = type_id!("XHeightFieldShapeSettings");
    type Kind = kind::Trivial;
}

impl<'t> Default for HeightFieldShapeSettings<'t> {
    fn default() -> HeightFieldShapeSettings<'t> {
        HeightFieldShapeSettings {
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

impl<'t> HeightFieldShapeSettings<'t> {
    pub fn new(height_samples: &'t [f32], sample_count: u32) -> HeightFieldShapeSettings<'t> {
        HeightFieldShapeSettings {
            height_samples,
            sample_count,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EmptyShapeSettings {
    pub user_data: u64,
    pub center_of_mass: Vec3A,
}
const_assert_eq!(std::mem::size_of::<EmptyShapeSettings>(), 32);

unsafe impl ExternType for EmptyShapeSettings {
    type Id = type_id!("XEmptyShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for EmptyShapeSettings {
    fn default() -> EmptyShapeSettings {
        EmptyShapeSettings {
            user_data: 0,
            center_of_mass: Vec3A::ZERO,
        }
    }
}

impl EmptyShapeSettings {
    pub fn new(center_of_mass: Vec3A) -> EmptyShapeSettings {
        EmptyShapeSettings {
            center_of_mass,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScaledShapeSettings {
    pub user_data: u64,
    pub inner_shape: Option<JRef<Shape>>,
    pub scale: Vec3A,
}
const_assert_eq!(std::mem::size_of::<ScaledShapeSettings>(), 32);

unsafe impl ExternType for ScaledShapeSettings {
    type Id = type_id!("XScaledShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for ScaledShapeSettings {
    fn default() -> ScaledShapeSettings {
        ScaledShapeSettings {
            user_data: 0,
            inner_shape: None,
            scale: Vec3A::ONE,
        }
    }
}

impl ScaledShapeSettings {
    pub fn new(inner_shape: JRef<Shape>, scale: Vec3A) -> ScaledShapeSettings {
        ScaledShapeSettings {
            user_data: 0,
            inner_shape: Some(inner_shape),
            scale,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RotatedTranslatedShapeSettings {
    pub user_data: u64,
    pub inner_shape: Option<JRef<Shape>>,
    pub position: Vec3A,
    pub rotation: Quat,
}
const_assert_eq!(std::mem::size_of::<RotatedTranslatedShapeSettings>(), 48);

unsafe impl ExternType for RotatedTranslatedShapeSettings {
    type Id = type_id!("XRotatedTranslatedShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for RotatedTranslatedShapeSettings {
    fn default() -> RotatedTranslatedShapeSettings {
        RotatedTranslatedShapeSettings {
            user_data: 0,
            inner_shape: None,
            position: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

impl RotatedTranslatedShapeSettings {
    pub fn new(inner_shape: JRef<Shape>, position: Vec3A, rotation: Quat) -> RotatedTranslatedShapeSettings {
        RotatedTranslatedShapeSettings {
            user_data: 0,
            inner_shape: Some(inner_shape),
            position,
            rotation,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct OffsetCenterOfMassShapeSettings {
    pub user_data: u64,
    pub inner_shape: Option<JRef<Shape>>,
    pub offset: Vec3A,
}
const_assert_eq!(std::mem::size_of::<OffsetCenterOfMassShapeSettings>(), 32);

unsafe impl ExternType for OffsetCenterOfMassShapeSettings {
    type Id = type_id!("XOffsetCenterOfMassShapeSettings");
    type Kind = kind::Trivial;
}

impl Default for OffsetCenterOfMassShapeSettings {
    fn default() -> OffsetCenterOfMassShapeSettings {
        OffsetCenterOfMassShapeSettings {
            user_data: 0,
            inner_shape: None,
            offset: Vec3A::ZERO,
        }
    }
}

impl OffsetCenterOfMassShapeSettings {
    pub fn new(inner_shape: JRef<Shape>, offset: Vec3A) -> OffsetCenterOfMassShapeSettings {
        OffsetCenterOfMassShapeSettings {
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
    pub shape: Option<JRef<Shape>>,
    pub position: Vec3A,
    pub rotation: Quat,
    pub user_data: u32,
}
const_assert_eq!(std::mem::size_of::<SubShapeSettings>(), 64);

unsafe impl ExternType for SubShapeSettings {
    type Id = type_id!("XSubShapeSettings");
    type Kind = kind::Trivial;
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
    pub fn new(shape: JRef<Shape>, position: Vec3A, rotation: Quat) -> SubShapeSettings {
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
#[derive(Debug, Clone, Default)]
pub struct StaticCompoundShapeSettings<'t> {
    pub user_data: u64,
    pub sub_shapes: &'t [SubShapeSettings],
}
const_assert_eq!(std::mem::size_of::<StaticCompoundShapeSettings>(), 24);

unsafe impl ExternType for StaticCompoundShapeSettings<'_> {
    type Id = type_id!("XStaticCompoundShapeSettings");
    type Kind = kind::Trivial;
}

impl StaticCompoundShapeSettings<'_> {
    pub fn new(sub_shapes: &[SubShapeSettings]) -> StaticCompoundShapeSettings<'_> {
        StaticCompoundShapeSettings {
            user_data: 0,
            sub_shapes,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MutableCompoundShapeSettings<'t> {
    pub user_data: u64,
    pub sub_shapes: &'t [SubShapeSettings],
}
const_assert_eq!(std::mem::size_of::<MutableCompoundShapeSettings>(), 24);

unsafe impl ExternType for MutableCompoundShapeSettings<'_> {
    type Id = type_id!("XMutableCompoundShapeSettings");
    type Kind = kind::Trivial;
}

impl MutableCompoundShapeSettings<'_> {
    pub fn new(sub_shapes: &[SubShapeSettings]) -> MutableCompoundShapeSettings<'_> {
        MutableCompoundShapeSettings {
            user_data: 0,
            sub_shapes,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CompoundSubShape {
    pub shape: JRef<Shape>,
    pub position: Vec3,
    rotation: Vec3, // X, Y, Z of rotation quaternion
    pub user_data: u32,
    pub is_rotation_identity: bool,
}
const_assert_eq!(std::mem::size_of::<CompoundSubShape>(), 40);

unsafe impl ExternType for CompoundSubShape {
    type Id = type_id!("XCompoundSubShape");
    type Kind = kind::Trivial;
}

impl CompoundSubShape {
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

#[inline]
pub fn create_sphere_shape(settings: &SphereShapeSettings) -> JoltResult<JRef<Shape>> {
    create_sphere_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_sphere_shape_mut(settings: &SphereShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateSphereShape(mem::transmute::<&SphereShapeSettings, &ffi::XSphereShapeSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_box_shape(settings: &BoxShapeSettings) -> JoltResult<JRef<Shape>> {
    create_box_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_box_shape_mut(settings: &BoxShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateBoxShape(mem::transmute::<&BoxShapeSettings, &ffi::XBoxShapeSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_capsule_shape(settings: &CapsuleShapeSettings) -> JoltResult<JRef<Shape>> {
    create_capsule_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_capsule_shape_mut(settings: &CapsuleShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateCapsuleShape(mem::transmute::<&CapsuleShapeSettings, &ffi::XCapsuleShapeSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_tapered_capsule_shape(settings: &TaperedCapsuleShapeSettings) -> JoltResult<JRef<Shape>> {
    create_tapered_capsule_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_tapered_capsule_shape_mut(settings: &TaperedCapsuleShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateTaperedCapsuleShape(mem::transmute::<
            &TaperedCapsuleShapeSettings,
            &ffi::XTaperedCapsuleShapeSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_cylinder_shape(settings: &CylinderShapeSettings) -> JoltResult<JRef<Shape>> {
    create_cylinder_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_cylinder_shape_mut(settings: &CylinderShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateCylinderShape(mem::transmute::<&CylinderShapeSettings, &ffi::XCylinderShapeSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_tapered_cylinder_shape(settings: &TaperedCylinderShapeSettings) -> JoltResult<JRef<Shape>> {
    create_tapered_cylinder_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_tapered_cylinder_shape_mut(settings: &TaperedCylinderShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateTaperedCylinderShape(mem::transmute::<
            &TaperedCylinderShapeSettings,
            &ffi::XTaperedCylinderShapeSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_convex_hull_shape(settings: &ConvexHullShapeSettings) -> JoltResult<JRef<Shape>> {
    create_convex_hull_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_convex_hull_shape_mut(settings: &ConvexHullShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateConvexHullShape(
            mem::transmute::<&ConvexHullShapeSettings, &ffi::XConvexHullShapeSettings>(settings),
        );
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_triangle_shape(settings: &TriangleShapeSettings) -> JoltResult<JRef<Shape>> {
    create_triangle_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_triangle_shape_mut(settings: &TriangleShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateTriangleShape(mem::transmute::<&TriangleShapeSettings, &ffi::XTriangleShapeSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_plane_shape(settings: &PlaneShapeSettings) -> JoltResult<JRef<Shape>> {
    create_plane_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_plane_shape_mut(settings: &PlaneShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreatePlaneShape(mem::transmute::<&PlaneShapeSettings, &ffi::XPlaneShapeSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_mesh_shape(settings: &MeshShapeSettings) -> JoltResult<JRef<Shape>> {
    create_mesh_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_mesh_shape_mut(settings: &MeshShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateMeshShape(mem::transmute::<&MeshShapeSettings, &ffi::XMeshShapeSettings>(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_height_field_shape(settings: &HeightFieldShapeSettings) -> JoltResult<JRef<Shape>> {
    create_height_field_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_height_field_shape_mut(settings: &HeightFieldShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateHeightFieldShape(mem::transmute::<
            &HeightFieldShapeSettings,
            &ffi::XHeightFieldShapeSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_empty_shape(settings: &EmptyShapeSettings) -> JoltResult<JRef<Shape>> {
    create_empty_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_empty_shape_mut(settings: &EmptyShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateEmptyShape(mem::transmute::<&EmptyShapeSettings, &ffi::XEmptyShapeSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_scaled_shape(settings: &ScaledShapeSettings) -> JoltResult<JRef<Shape>> {
    create_scaled_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_scaled_shape_mut(settings: &ScaledShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateScaledShape(mem::transmute::<&ScaledShapeSettings, &ffi::XScaledShapeSettings>(
            settings,
        ));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_rotated_translated_shape(settings: &RotatedTranslatedShapeSettings) -> JoltResult<JRef<Shape>> {
    create_rotated_translated_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_rotated_translated_shape_mut(settings: &RotatedTranslatedShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateRotatedTranslatedShape(mem::transmute::<
            &RotatedTranslatedShapeSettings,
            &ffi::XRotatedTranslatedShapeSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_offset_center_of_mass_shape(settings: &OffsetCenterOfMassShapeSettings) -> JoltResult<JRef<Shape>> {
    create_offset_center_of_mass_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_offset_center_of_mass_shape_mut(settings: &OffsetCenterOfMassShapeSettings) -> JoltResult<JMut<Shape>> {
    unsafe {
        let ptr = ffi::CreateOffsetCenterOfMassShape(mem::transmute::<
            &OffsetCenterOfMassShapeSettings,
            &ffi::XOffsetCenterOfMassShapeSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<Shape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_static_compound_shape(settings: &StaticCompoundShapeSettings) -> JoltResult<JRef<StaticCompoundShape>> {
    create_static_compound_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_static_compound_shape_mut(
    settings: &StaticCompoundShapeSettings,
) -> JoltResult<JMut<StaticCompoundShape>> {
    if settings.sub_shapes.len() < 2 {
        return Err(JoltError::TooLessSubShape);
    }
    unsafe {
        let ptr = ffi::CreateStaticCompoundShape(mem::transmute::<
            &StaticCompoundShapeSettings,
            &ffi::XStaticCompoundShapeSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<StaticCompoundShape>::new_unchecked(ptr))
    }
}

#[inline]
pub fn create_mutable_compound_shape(
    settings: &MutableCompoundShapeSettings,
) -> JoltResult<JRef<MutableCompoundShape>> {
    create_mutable_compound_shape_mut(settings).map(|s| s.into())
}

#[inline]
pub fn create_mutable_compound_shape_mut(
    settings: &MutableCompoundShapeSettings,
) -> JoltResult<JMut<MutableCompoundShape>> {
    if settings.sub_shapes.len() < 2 {
        return Err(JoltError::TooLessSubShape);
    }
    unsafe {
        let ptr = ffi::CreateMutableCompoundShape(mem::transmute::<
            &MutableCompoundShapeSettings,
            &ffi::XMutableCompoundShapeSettings,
        >(settings));
        if ptr.is_null() {
            return Err(JoltError::CreateShape);
        }
        Ok(JMut::<MutableCompoundShape>::new_unchecked(ptr))
    }
}

pub struct PhysicsMaterial(ffi::PhysicsMaterial);

const_assert_eq!(mem::size_of::<JRef<PhysicsMaterial>>(), mem::size_of::<usize>());
const_assert_eq!(mem::size_of::<Option<JRef<PhysicsMaterial>>>(), 8);
const_assert_eq!(
    unsafe { mem::transmute::<Option<JRef<PhysicsMaterial>>, usize>(None) },
    0
);

impl fmt::Debug for PhysicsMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysicsMaterial").finish()
    }
}

unsafe impl JRefTarget for PhysicsMaterial {
    type JRaw = NonNull<PhysicsMaterial>;

    #[inline]
    fn name() -> &'static str {
        "PhysicsMaterial"
    }

    #[inline]
    unsafe fn make_ref(raw: &Self::JRaw) -> &Self {
        unsafe { raw.as_ref() }
    }

    #[inline]
    unsafe fn clone_raw(raw: &Self::JRaw) -> Self::JRaw {
        NonNull::new_unchecked(ffi::ClonePhysicsMaterial(raw.as_ptr() as *mut _) as *mut _)
    }

    #[inline]
    unsafe fn drop_raw(raw: &mut Self::JRaw) {
        ffi::DropPhysicsMaterial(raw.as_ptr() as *mut _);
    }

    #[inline]
    unsafe fn count_ref(raw: &Self::JRaw) -> u32 {
        unsafe { ffi::CountRefPhysicsMaterial(raw.as_ptr() as *const _) }
    }
}

impl PhysicsMaterial {
    #[inline]
    pub(crate) unsafe fn cast_ptr(p: *const ffi::PhysicsMaterial) -> *const PhysicsMaterial {
        p as *const PhysicsMaterial
    }
}

pub struct Shape(pub(crate) ffi::Shape);

const_assert_eq!(mem::size_of::<JRef<Shape>>(), mem::size_of::<usize>());
const_assert_eq!(mem::size_of::<Option<JRef<Shape>>>(), 8);
const_assert_eq!(unsafe { mem::transmute::<Option<JRef<Shape>>, usize>(None) }, 0);

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shape")
            .field("type", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("user_data", &self.get_user_data())
            .finish()
    }
}

unsafe impl JRefTarget for Shape {
    type JRaw = NonNull<Shape>;

    #[inline]
    fn name() -> &'static str {
        "Shape"
    }

    #[inline]
    unsafe fn make_ref(raw: &Self::JRaw) -> &Self {
        unsafe { raw.as_ref() }
    }

    #[inline]
    unsafe fn clone_raw(raw: &Self::JRaw) -> Self::JRaw {
        NonNull::new_unchecked(ffi::CloneShape(raw.as_ptr() as *mut _) as *mut _)
    }

    #[inline]
    unsafe fn drop_raw(raw: &mut Self::JRaw) {
        ffi::DropShape(raw.as_ptr() as *mut _);
    }

    #[inline]
    unsafe fn count_ref(raw: &Self::JRaw) -> u32 {
        unsafe { ffi::CountRefShape(raw.as_ptr() as *mut _) }
    }
}

unsafe impl JMutTarget for Shape {
    #[inline]
    unsafe fn make_mut(raw: &mut Self::JRaw) -> &mut Self {
        unsafe { raw.as_mut() }
    }

    #[inline]
    unsafe fn steal_raw(raw: &Self::JRaw) -> Self::JRaw {
        *raw
    }
}

impl JRef<Shape> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut ffi::Shape) -> JRef<Shape> {
        JRef(unsafe { NonNull::new_unchecked(raw as *mut _) })
    }
}

impl JMut<Shape> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut ffi::Shape) -> JMut<Shape> {
        JMut(unsafe { NonNull::new_unchecked(raw as *mut _) })
    }
}

macro_rules! shape_methods {
    ($type:ty, $ref:ty) => {
        impl $type {
            #[inline]
            fn as_ref(&self) -> &$ref {
                &self.0
            }

            #[inline]
            fn as_mut(&mut self) -> Pin<&mut $ref> {
                unsafe { Pin::new_unchecked(&mut self.0) }
            }

            #[inline]
            pub fn get_type(&self) -> ShapeType {
                self.as_ref().GetType()
            }

            #[inline]
            pub fn get_sub_type(&self) -> ShapeSubType {
                self.as_ref().GetSubType()
            }

            #[inline]
            pub fn get_user_data(&self) -> u64 {
                self.as_ref().GetUserData()
            }

            #[inline]
            pub fn set_user_data(&mut self, data: u64) {
                self.as_mut().SetUserData(data);
            }

            #[inline]
            pub fn get_center_of_mass(&self) -> Vec3A {
                self.as_ref().GetCenterOfMass().into()
            }

            #[inline]
            pub fn must_be_static(&self) -> bool {
                self.as_ref().MustBeStatic()
            }

            #[inline]
            pub fn get_local_bounds(&self) -> AABox {
                self.as_ref().GetLocalBounds()
            }

            #[inline]
            pub fn get_inner_radius(&self) -> f32 {
                self.as_ref().GetInnerRadius()
            }

            #[inline]
            pub fn get_volume(&self) -> f32 {
                self.as_ref().GetVolume()
            }

            #[inline]
            pub fn is_valid_scale(&self, scale: Vec3A) -> bool {
                self.as_ref().IsValidScale(scale.into())
            }

            #[inline]
            pub fn make_scale_valid(&self, scale: Vec3A) -> Vec3A {
                self.as_ref().MakeScaleValid(scale.into()).into()
            }
        }
    };
}

shape_methods!(Shape, ffi::Shape);

impl Shape {
    #[inline]
    pub(crate) unsafe fn cast_ptr(p: *const ffi::Shape) -> *const Shape {
        p as *const Shape
    }
}

pub struct StaticCompoundShape(pub(crate) ffi::StaticCompoundShape);

impl fmt::Debug for StaticCompoundShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StaticCompoundShape")
            .field("type", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("num_sub_shapes", &self.get_num_sub_shapes())
            .field("user_data", &self.get_user_data())
            .finish()
    }
}

unsafe impl JRefTarget for StaticCompoundShape {
    type JRaw = NonNull<StaticCompoundShape>;

    #[inline]
    fn name() -> &'static str {
        "StaticCompoundShape"
    }

    #[inline]
    unsafe fn make_ref(raw: &Self::JRaw) -> &Self {
        unsafe { mem::transmute::<&Self::JRaw, &Self>(raw) }
    }

    #[inline]
    unsafe fn clone_raw(raw: &Self::JRaw) -> Self::JRaw {
        NonNull::new_unchecked(ffi::CloneStaticCompoundShape(raw.as_ptr() as *mut _) as *mut _)
    }

    #[inline]
    unsafe fn drop_raw(raw: &mut Self::JRaw) {
        ffi::DropStaticCompoundShape(raw.as_ptr() as *mut _);
    }

    #[inline]
    unsafe fn count_ref(raw: &Self::JRaw) -> u32 {
        unsafe { ffi::CountRefStaticCompoundShape(raw.as_ptr() as *const _) }
    }
}

unsafe impl JMutTarget for StaticCompoundShape {
    #[inline]
    unsafe fn make_mut(raw: &mut Self::JRaw) -> &mut Self {
        unsafe { raw.as_mut() }
    }

    #[inline]
    unsafe fn steal_raw(raw: &Self::JRaw) -> Self::JRaw {
        *raw
    }
}

impl JMut<StaticCompoundShape> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut ffi::StaticCompoundShape) -> JMut<StaticCompoundShape> {
        JMut(unsafe { NonNull::new_unchecked(raw as *mut _) })
    }
}

impl From<JMut<StaticCompoundShape>> for JMut<Shape> {
    #[inline]
    fn from(compund: JMut<StaticCompoundShape>) -> JMut<Shape> {
        let shape = unsafe { JMut::<Shape>::new_unchecked(compund.0.as_ptr() as *mut _) };
        mem::forget(compund);
        shape
    }
}

impl From<JRef<StaticCompoundShape>> for JRef<Shape> {
    #[inline]
    fn from(compund: JRef<StaticCompoundShape>) -> JRef<Shape> {
        let shape = unsafe { JRef::<Shape>::new_unchecked(compund.0.as_ptr() as *mut _) };
        mem::forget(compund);
        shape
    }
}

shape_methods!(StaticCompoundShape, ffi::StaticCompoundShape);

impl StaticCompoundShape {
    #[inline]
    pub fn get_num_sub_shapes(&self) -> u32 {
        self.as_ref().GetNumSubShapes()
    }

    #[inline]
    pub fn get_sub_shape(&self, idx: u32) -> &CompoundSubShape {
        unsafe { self.as_ref().GetSubShape(idx) }
    }

    #[inline]
    pub fn get_compound_user_data(&self, idx: u32) -> u32 {
        self.as_ref().GetCompoundUserData(idx)
    }

    #[inline]
    pub fn set_compound_user_data(&mut self, idx: u32, data: u32) {
        self.as_mut().SetCompoundUserData(idx, data);
    }
}

pub struct MutableCompoundShape(pub(crate) ffi::MutableCompoundShape);

impl fmt::Debug for MutableCompoundShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MutableCompoundShape")
            .field("type", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("num_sub_shapes", &self.get_num_sub_shapes())
            .field("user_data", &self.get_user_data())
            .finish()
    }
}

unsafe impl JRefTarget for MutableCompoundShape {
    type JRaw = NonNull<MutableCompoundShape>;

    #[inline]
    fn name() -> &'static str {
        "MutableCompoundShape"
    }

    #[inline]
    unsafe fn make_ref(raw: &Self::JRaw) -> &Self {
        unsafe { raw.as_ref() }
    }

    #[inline]
    unsafe fn clone_raw(raw: &Self::JRaw) -> Self::JRaw {
        NonNull::new_unchecked(ffi::CloneMutableCompoundShape(raw.as_ptr() as *mut _) as *mut _)
    }

    #[inline]
    unsafe fn drop_raw(raw: &mut Self::JRaw) {
        ffi::DropMutableCompoundShape(raw.as_ptr() as *mut _);
    }

    #[inline]
    unsafe fn count_ref(raw: &Self::JRaw) -> u32 {
        unsafe { ffi::CountRefMutableCompoundShape(raw.as_ptr() as *const _) }
    }
}

unsafe impl JMutTarget for MutableCompoundShape {
    #[inline]
    unsafe fn make_mut(raw: &mut Self::JRaw) -> &mut Self {
        unsafe { raw.as_mut() }
    }

    #[inline]
    unsafe fn steal_raw(raw: &Self::JRaw) -> Self::JRaw {
        *raw
    }
}

impl JMut<MutableCompoundShape> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut ffi::MutableCompoundShape) -> JMut<MutableCompoundShape> {
        JMut(unsafe { NonNull::new_unchecked(raw as *mut _) })
    }
}

impl From<JMut<MutableCompoundShape>> for JMut<Shape> {
    #[inline]
    fn from(compund: JMut<MutableCompoundShape>) -> JMut<Shape> {
        let shape = unsafe { JMut::<Shape>::new_unchecked(compund.0.as_ptr() as *mut _) };
        mem::forget(compund);
        shape
    }
}

impl From<JRef<MutableCompoundShape>> for JRef<Shape> {
    #[inline]
    fn from(compund: JRef<MutableCompoundShape>) -> JRef<Shape> {
        let shape = unsafe { JRef::<Shape>::new_unchecked(compund.0.as_ptr() as *mut _) };
        mem::forget(compund);
        shape
    }
}

shape_methods!(MutableCompoundShape, ffi::MutableCompoundShape);

impl MutableCompoundShape {
    #[inline]
    pub fn get_num_sub_shapes(&self) -> u32 {
        self.as_ref().GetNumSubShapes()
    }

    #[inline]
    pub fn get_sub_shape(&self, idx: u32) -> &CompoundSubShape {
        unsafe { self.as_ref().GetSubShape(idx) }
    }

    #[inline]
    pub fn get_compound_user_data(&self, idx: u32) -> u32 {
        self.as_ref().GetCompoundUserData(idx)
    }

    #[inline]
    pub fn set_compound_user_data(&mut self, idx: u32, data: u32) {
        self.as_mut().SetCompoundUserData(idx, data);
    }

    #[inline]
    pub fn add_shape(&mut self, position: Vec3A, rotation: Quat, shape: &Shape, user_data: u32, index: u32) -> u32 {
        unsafe {
            self.as_mut()
                .AddShape(position.into(), rotation.into(), &shape.0, user_data, index)
        }
    }

    #[inline]
    pub fn remove_shape(&mut self, index: u32) {
        self.as_mut().RemoveShape(index);
    }

    #[inline]
    pub fn modify_shape(&mut self, index: u32, position: Vec3A, rotation: Quat) {
        self.as_mut().ModifyShape(index, position.into(), rotation.into());
    }

    #[inline]
    pub fn modify_shape_ex(&mut self, index: u32, position: Vec3A, rotation: Quat, shape: &Shape) {
        unsafe {
            self.as_mut()
                .ModifyShapeEx(index, position.into(), rotation.into(), &shape.0)
        }
    }

    #[inline]
    pub fn adjust_center_of_mass(&mut self) {
        self.as_mut().AdjustCenterOfMass();
    }

    #[inline]
    pub fn modify_shapes(&mut self, sub_shape_start_idx: u32, position: &[Vec3A], rotation: &[Quat]) {
        let count = usize::min(position.len(), rotation.len()) as u32;
        unsafe {
            self.as_mut().ModifyShapes(
                sub_shape_start_idx,
                count,
                position.as_ptr() as *const JVec3,
                rotation.as_ptr() as *const JQuat,
                mem::size_of::<Vec3A>() as u32,
                mem::size_of::<Quat>() as u32,
            );
        }
    }

    #[cfg(feature = "glam-ext")]
    #[inline]
    pub fn modify_shapes_by_isometry(&mut self, sub_shape_start_idx: u32, transform: &[Isometry3A]) {
        unsafe {
            self.as_mut().ModifyShapes(
                sub_shape_start_idx,
                transform.len() as u32,
                (&transform[0].translation as *const _) as *const JVec3,
                (&transform[0].rotation as *const _) as *const JQuat,
                mem::size_of::<Isometry3A>() as u32,
                mem::size_of::<Isometry3A>() as u32,
            );
        }
    }

    #[cfg(feature = "glam-ext")]
    #[inline]
    pub fn modify_shapes_by_transform(&mut self, sub_shape_start_idx: u32, transform: &[Transform3A]) {
        unsafe {
            self.as_mut().ModifyShapes(
                sub_shape_start_idx,
                transform.len() as u32,
                (&transform[0].translation as *const _) as *const JVec3,
                (&transform[0].rotation as *const _) as *const JQuat,
                mem::size_of::<Transform3A>() as u32,
                mem::size_of::<Transform3A>() as u32,
            );
        }
    }

    #[inline]
    pub unsafe fn modify_shapes_unsafe(
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
            position_ptr as *const JVec3,
            rotation_ptr as *const JQuat,
            position_stride,
            rotation_stride,
        );
    }
}
