use glam::{Quat, Vec3, Vec3A};
use static_assertions::const_assert_eq;
use std::mem;

use crate::base::*;
use crate::consts::{DEFAULT_CONVEX_RADIUS, MAX_CONVEX_RADIUS, MIN_CONVEX_RADIUS};
use crate::error::{JoltError, JoltResult};

#[cxx::bridge()]
pub mod ffi {
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type XRefShape = crate::base::ffi::XRefShape;

        type BoxSettings;
        type SphereSettings;
        type CapsuleSettings;
        type TaperedCapsuleSettings;
        type CylinderSettings;
        type TaperedCylinderSettings;
        type OffsetCenterOfMassSettings;
        type RotatedTranslatedSettings;
        type ScaledSettings;
        type ConvexHullSettings;
        type MeshSettings;
        type HeightFieldSettings;

        fn CreateShapeBox(settings: &BoxSettings) -> XRefShape;
        fn CreateShapeSphere(settings: &SphereSettings) -> XRefShape;
        fn CreateShapeCapsule(settings: &CapsuleSettings) -> XRefShape;
        fn CreateShapeTaperedCapsule(settings: &TaperedCapsuleSettings) -> XRefShape;
        fn CreateShapeCylinder(settings: &CylinderSettings) -> XRefShape;
        fn CreateShapeTaperedCylinder(settings: &CylinderSettings) -> XRefShape;
        fn CreateShapeRotatedTranslated(settings: &RotatedTranslatedSettings) -> XRefShape;
        fn CreateShapeScaled(settings: &ScaledSettings) -> XRefShape;
        fn CreateShapeOffsetCenterOfMass(settings: &OffsetCenterOfMassSettings) -> XRefShape;
        fn CreateShapeConvexHull(settings: &ConvexHullSettings) -> XRefShape;
        fn CreateShapeMesh(settings: &MeshSettings) -> XRefShape;
        fn CreateShapeHeightField(settings: &HeightFieldSettings) -> XRefShape;
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoxSettings {
    pub user_data: u64,
    pub material: RefPhysicsMaterial,
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
            material: RefPhysicsMaterial::invalid(),
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
pub struct SphereSettings {
    pub user_data: u64,
    pub material: RefPhysicsMaterial,
    pub density: f32,
    pub radius: f32,
}
const_assert_eq!(std::mem::size_of::<SphereSettings>(), 24);

impl Default for SphereSettings {
    fn default() -> SphereSettings {
        SphereSettings {
            user_data: 0,
            material: RefPhysicsMaterial::invalid(),
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
pub struct CapsuleSettings {
    pub user_data: u64,
    pub material: RefPhysicsMaterial,
    pub density: f32,
    pub half_height: f32,
    pub radius: f32,
}
const_assert_eq!(std::mem::size_of::<CapsuleSettings>(), 32);

impl Default for CapsuleSettings {
    fn default() -> CapsuleSettings {
        CapsuleSettings {
            user_data: 0,
            material: RefPhysicsMaterial::invalid(),
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
    pub material: RefPhysicsMaterial,
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
            material: RefPhysicsMaterial::invalid(),
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
    pub material: RefPhysicsMaterial,
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
            material: RefPhysicsMaterial::invalid(),
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
    pub material: RefPhysicsMaterial,
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
            material: RefPhysicsMaterial::invalid(),
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
#[derive(Debug, Clone)]
pub struct RotatedTranslatedSettings {
    pub user_data: u64,
    pub inner_shape: RefShape,
    pub position: Vec3A,
    pub rotation: Quat,
}
const_assert_eq!(std::mem::size_of::<RotatedTranslatedSettings>(), 48);

impl Default for RotatedTranslatedSettings {
    fn default() -> RotatedTranslatedSettings {
        RotatedTranslatedSettings {
            user_data: 0,
            inner_shape: RefShape::invalid(),
            position: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

impl RotatedTranslatedSettings {
    pub fn new(inner_shape: RefShape, position: Vec3A, rotation: Quat) -> RotatedTranslatedSettings {
        RotatedTranslatedSettings {
            user_data: 0,
            inner_shape,
            position,
            rotation,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScaledSettings {
    pub user_data: u64,
    pub inner_shape: RefShape,
    pub scale: Vec3A,
}
const_assert_eq!(std::mem::size_of::<ScaledSettings>(), 32);

impl Default for ScaledSettings {
    fn default() -> ScaledSettings {
        ScaledSettings {
            user_data: 0,
            inner_shape: RefShape::invalid(),
            scale: Vec3A::ONE,
        }
    }
}

impl ScaledSettings {
    pub fn new(inner_shape: RefShape, scale: Vec3A) -> ScaledSettings {
        ScaledSettings {
            user_data: 0,
            inner_shape,
            scale,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct OffsetCenterOfMassSettings {
    pub user_data: u64,
    pub inner_shape: RefShape,
    pub offset: Vec3A,
}
const_assert_eq!(std::mem::size_of::<OffsetCenterOfMassSettings>(), 32);

impl Default for OffsetCenterOfMassSettings {
    fn default() -> OffsetCenterOfMassSettings {
        OffsetCenterOfMassSettings {
            user_data: 0,
            inner_shape: RefShape::invalid(),
            offset: Vec3A::ZERO,
        }
    }
}

impl OffsetCenterOfMassSettings {
    pub fn new(inner_shape: RefShape, offset: Vec3A) -> OffsetCenterOfMassSettings {
        OffsetCenterOfMassSettings {
            user_data: 0,
            inner_shape,
            offset,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConvexHullSettings<'t> {
    pub user_data: u64,
    pub material: RefPhysicsMaterial,
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
            material: RefPhysicsMaterial::invalid(),
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

pub fn create_shape_box(settings: &BoxSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeBox(unsafe {
        mem::transmute::<&BoxSettings, &ffi::BoxSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_sphere(settings: &SphereSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeSphere(unsafe {
        mem::transmute::<&SphereSettings, &ffi::SphereSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_capsule(settings: &CapsuleSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeCapsule(unsafe {
        mem::transmute::<&CapsuleSettings, &ffi::CapsuleSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_tapered_capsule(settings: &TaperedCapsuleSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeTaperedCapsule(unsafe {
        mem::transmute::<&TaperedCapsuleSettings, &ffi::TaperedCapsuleSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_cylinder(settings: &CylinderSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeCylinder(unsafe {
        mem::transmute::<&CylinderSettings, &ffi::CylinderSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_rotated_translated(settings: &RotatedTranslatedSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeRotatedTranslated(unsafe {
        mem::transmute::<&RotatedTranslatedSettings, &ffi::RotatedTranslatedSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_scaled(settings: &ScaledSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeScaled(unsafe {
        mem::transmute::<&&ScaledSettings, &ffi::ScaledSettings>(&settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_offset_center_of_mass(settings: &OffsetCenterOfMassSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeOffsetCenterOfMass(unsafe {
        mem::transmute::<&OffsetCenterOfMassSettings, &ffi::OffsetCenterOfMassSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_convex_hull(settings: &ConvexHullSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeConvexHull(unsafe {
        mem::transmute::<&ConvexHullSettings<'_>, &ffi::ConvexHullSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_mesh(settings: &MeshSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeMesh(unsafe {
        mem::transmute::<&MeshSettings<'_>, &ffi::MeshSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

pub fn create_shape_height_field(settings: &HeightFieldSettings) -> JoltResult<RefShape> {
    let shape = RefShape(ffi::CreateShapeHeightField(unsafe {
        mem::transmute::<&HeightFieldSettings<'_>, &ffi::HeightFieldSettings>(settings)
    }));
    if shape.is_invalid() {
        return Err(JoltError::CreateShape);
    }
    Ok(shape)
}

// fn apply_shape_transform(inner: &RefShape, transform: Option<&Transform>) -> RefShape {
//     if let Some(t) = transform {
//         let mut shape = inner.clone();
//         if t.scale != Vec3A::ONE {
//             let settings = ScaledSettings::new(shape, t.scale);
//             shape = RefShape(ffi::CreateShapeScaled(unsafe { mem::transmute(&settings) }));
//         }
//         if t.position != Vec3A::ZERO || !(t.rotation.xyz() == Vec3::ZERO && t.rotation.w.abs() == 1.0) {
//             let settings = RotatedTranslatedSettings::new(shape, t.position, t.rotation);
//             shape = RefShape(ffi::CreateShapeRotatedTranslated(unsafe { mem::transmute(&settings) }));
//         }
//         return shape;
//     } else {
//         return inner.clone();
//     }
// }
