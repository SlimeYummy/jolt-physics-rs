use glam::{Quat, Vec3, Vec3A};
use static_assertions::const_assert_eq;
use std::mem;

use crate::base::*;

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
        type IsometrySettings;
        type ScaledSettings;
        type ConvexHullSettings;
        type MeshSettings;
        type HeightFieldSettings;

        fn CreateShapeBox(settings: &BoxSettings) -> XRefShape;
        fn CreateShapeSphere(settings: &SphereSettings) -> XRefShape;
        fn CreateShapeCapsule(settings: &CapsuleSettings) -> XRefShape;
        fn CreateShapeTaperedCapsule(settings: &TaperedCapsuleSettings) -> XRefShape;
        fn CreateShapeCylinder(settings: &CylinderSettings) -> XRefShape;
        fn CreateShapeIsometry(settings: &IsometrySettings) -> XRefShape;
        fn CreateShapeScaled(settings: &ScaledSettings) -> XRefShape;
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
        return BoxSettings {
            user_data: 0,
            material: RefPhysicsMaterial::default(),
            density: 1000.0,
            half_x: 0.0,
            half_y: 0.0,
            half_z: 0.0,
            convex_radius: 0.05,
        };
    }
}

impl BoxSettings {
    pub fn new(half_x: f32, half_y: f32, half_z: f32) -> BoxSettings {
        return BoxSettings {
            half_x,
            half_y,
            half_z,
            ..Default::default()
        };
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
        return SphereSettings {
            user_data: 0,
            material: RefPhysicsMaterial::default(),
            density: 1000.0,
            radius: 0.5,
        };
    }
}

impl SphereSettings {
    pub fn new(radius: f32) -> SphereSettings {
        return SphereSettings { radius, ..Default::default() };
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
        return CapsuleSettings {
            user_data: 0,
            material: RefPhysicsMaterial::default(),
            density: 1000.0,
            radius: 0.0,
            half_height: 0.0,
        };
    }
}

impl CapsuleSettings {
    pub fn new(half_height: f32, radius: f32) -> CapsuleSettings {
        return CapsuleSettings {
            half_height,
            radius,
            ..Default::default()
        };
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
        return TaperedCapsuleSettings {
            user_data: 0,
            material: RefPhysicsMaterial::default(),
            density: 1000.0,
            half_height: 0.0,
            top_radius: 0.0,
            bottom_radius: 0.0,
        };
    }
}

impl TaperedCapsuleSettings {
    pub fn new(half_height: f32, top_radius: f32, bottom_radius: f32) -> TaperedCapsuleSettings {
        return TaperedCapsuleSettings {
            top_radius,
            bottom_radius,
            half_height,
            ..Default::default()
        };
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
        return CylinderSettings {
            user_data: 0,
            material: RefPhysicsMaterial::default(),
            density: 1000.0,
            half_height: 0.0,
            radius: 0.0,
            convex_radius: 0.05,
        };
    }
}

impl CylinderSettings {
    pub fn new(half_height: f32, radius: f32) -> CylinderSettings {
        return CylinderSettings {
            half_height,
            radius,
            ..Default::default()
        };
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct IsometrySettings {
    pub user_data: u64,
    pub inner_shape: RefShape,
    pub position: Vec3A,
    pub rotation: Quat,
}
const_assert_eq!(std::mem::size_of::<IsometrySettings>(), 48);

impl Default for IsometrySettings {
    fn default() -> IsometrySettings {
        return IsometrySettings {
            user_data: 0,
            inner_shape: RefShape::default(),
            position: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
        };
    }
}

impl IsometrySettings {
    pub fn new(inner_shape: RefShape, position: Vec3A, rotation: Quat) -> IsometrySettings {
        return IsometrySettings {
            user_data: 0,
            inner_shape: inner_shape,
            position,
            rotation,
        };
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
        return ScaledSettings {
            user_data: 0,
            inner_shape: RefShape::default(),
            scale: Vec3A::ONE,
        };
    }
}

impl ScaledSettings {
    pub fn new(inner_shape: RefShape, scale: Vec3A) -> ScaledSettings {
        return ScaledSettings {
            user_data: 0,
            inner_shape: inner_shape,
            scale,
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConvexHullSettings {
    pub user_data: u64,
    pub material: RefPhysicsMaterial,
    pub density: f32,
    pub points: Vec<Vec3A>,
    pub max_convex_radius: f32,
    pub max_error_convex_radius: f32,
    pub hull_tolerance: f32,
}
const_assert_eq!(std::mem::size_of::<ConvexHullSettings>(), 64);

impl Default for ConvexHullSettings {
    fn default() -> ConvexHullSettings {
        return ConvexHullSettings {
            user_data: 0,
            material: RefPhysicsMaterial::default(),
            density: 1000.0,
            points: Vec::new(),
            max_convex_radius: 0.05,
            max_error_convex_radius: 0.05,
            hull_tolerance: 1.0e-3,
        };
    }
}

impl ConvexHullSettings {
    pub fn new(points: Vec<Vec3A>) -> ConvexHullSettings {
        return ConvexHullSettings { points, ..Default::default() };
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshSettings {
    pub user_data: u64,
    pub triangle_vertices: Vec<Vec3>,
    pub indexed_triangles: Vec<IndexedTriangle>,
    pub materials: Vec<RefPhysicsMaterial>,
    pub max_triangles_per_leaf: u32,
    pub active_edge_cos_threshold_angle: f32,
}
const_assert_eq!(std::mem::size_of::<MeshSettings>(), 88);

impl Default for MeshSettings {
    fn default() -> MeshSettings {
        return MeshSettings {
            user_data: 0,
            triangle_vertices: Vec::new(),
            indexed_triangles: Vec::new(),
            materials: Vec::new(),
            max_triangles_per_leaf: 8,
            active_edge_cos_threshold_angle: 0.996195, // cos(5)
        };
    }
}

impl MeshSettings {
    pub fn new(triangle_vertices: Vec<Vec3>, indexed_triangles: Vec<IndexedTriangle>) -> MeshSettings {
        return MeshSettings {
            triangle_vertices,
            indexed_triangles,
            ..Default::default()
        };
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HeightFieldSettings {
    pub user_data: u64,
    pub offset: Vec3A,
    pub scale: Vec3A,
    pub sample_count: u32,
    pub min_height_value: f32,
    pub max_height_value: f32,
    pub block_size: u32,
    pub bits_per_sample: u32,
    pub height_samples: Vec<f32>,
    pub material_indices: Vec<u8>,
    pub materials: Vec<RefPhysicsMaterial>,
    pub active_edge_cos_threshold_angle: f32,
}
const_assert_eq!(std::mem::size_of::<HeightFieldSettings>(), 160);

impl Default for HeightFieldSettings {
    fn default() -> HeightFieldSettings {
        return HeightFieldSettings {
            user_data: 0,
            offset: Vec3A::ZERO,
            scale: Vec3A::ONE,
            sample_count: 0,
            min_height_value: f32::MAX,
            max_height_value: f32::MIN,
            block_size: 2,
            bits_per_sample: 8,
            height_samples: Vec::new(),
            material_indices: Vec::new(),
            materials: Vec::new(),
            active_edge_cos_threshold_angle: 0.996195, // cos(5)
        };
    }
}

impl HeightFieldSettings {
    pub fn new(height_samples: Vec<f32>, sample_count: u32) -> HeightFieldSettings {
        return HeightFieldSettings {
            height_samples,
            sample_count,
            ..Default::default()
        };
    }
}

pub fn create_shape_box(settings: &BoxSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeBox(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

pub fn create_shape_sphere(settings: &SphereSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeSphere(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

pub fn create_shape_capsule(settings: &CapsuleSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeCapsule(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

pub fn create_shape_tapered_capsule(settings: &TaperedCapsuleSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeTaperedCapsule(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

pub fn create_shape_cylinder(settings: &CylinderSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeCylinder(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

pub fn create_shape_convex_hull(settings: &ConvexHullSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeConvexHull(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

pub fn create_shape_mesh(settings: &MeshSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeMesh(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

pub fn create_shape_height_field(settings: &HeightFieldSettings, transform: Option<&Transform>) -> RefShape {
    let shape = RefShape(ffi::CreateShapeHeightField(unsafe { mem::transmute(settings) }));
    return apply_shape_transform(&shape, transform);
}

fn apply_shape_transform(inner: &RefShape, transform: Option<&Transform>) -> RefShape {
    if let Some(t) = transform {
        let mut shape = inner.clone();
        if t.scale != Vec3A::ONE {
            let settings = ScaledSettings::new(shape, t.scale);
            shape = RefShape(ffi::CreateShapeScaled(unsafe { mem::transmute(&settings) }));
        }
        if t.position != Vec3A::ZERO || !(t.rotation.xyz() == Vec3::ZERO && t.rotation.w.abs() == 1.0) {
            let settings = IsometrySettings::new(shape, t.position, t.rotation);
            shape = RefShape(ffi::CreateShapeIsometry(unsafe { mem::transmute(&settings) }));
        }
        return shape;
    } else {
        return inner.clone();
    }
}
