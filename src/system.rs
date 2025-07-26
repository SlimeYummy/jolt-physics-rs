use cxx::{kind, type_id, ExternType};
use glam::{Mat4, Quat, Vec3A};
use jolt_macros::vtable;
use static_assertions::const_assert_eq;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fmt, mem, ptr};

use crate::base::{
    AABox, BodyID, BodyType, BroadPhaseLayer, JQuat, JRef, JRefTarget, JVec3, MotionQuality, MotionType, ObjectLayer,
    StaticArray, SubShapeID, ValidateResult,
};
use crate::body::{Body, BodyCreationSettings};
use crate::error::{JoltError, JoltResult};
use crate::shape::Shape;
use crate::vtable::{VBox, VPair};

#[cxx::bridge()]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type BodyType = crate::base::ffi::BodyType;
        type MotionType = crate::base::ffi::MotionType;
        type MotionQuality = crate::base::ffi::MotionQuality;
        type Activation = crate::base::ffi::Activation;

        type Vec3 = crate::base::ffi::Vec3;
        type Quat = crate::base::ffi::Quat;
        type Mat44 = crate::base::ffi::Mat44;
        type AABox = crate::base::ffi::AABox;
        type BodyID = crate::base::ffi::BodyID;
        type Shape = crate::shape::ffi::Shape;

        type PhysicsSettings = crate::system::PhysicsSettings;
        type XBodyStats = crate::system::BodyStats;
        #[allow(dead_code)]
        type CollideShapeResult = crate::system::CollideShapeResult;
        #[allow(dead_code)]
        type ContactManifold = crate::system::ContactManifold;
        #[allow(dead_code)]
        type ContactSettings = crate::system::ContactSettings;
        #[allow(dead_code)]
        type SubShapeIDPair = crate::system::SubShapeIDPair;

        type BroadPhaseLayerInterface;
        type ObjectVsBroadPhaseLayerFilter;
        type ObjectLayerPairFilter;
        type BodyActivationListener;
        type ContactListener;

        fn GlobalInitialize();
        fn GlobalFinalize();

        type XPhysicsSystem;
        unsafe fn CreatePhysicSystem(
            clean_up: fn(zelf: Pin<&mut XPhysicsSystem>),
            bpli: *const BroadPhaseLayerInterface,
            obplf: *const ObjectVsBroadPhaseLayerFilter,
            olpf: *const ObjectLayerPairFilter,
        ) -> *mut XPhysicsSystem;
        unsafe fn DropXPhysicsSystem(system: *mut XPhysicsSystem);
        unsafe fn CloneXPhysicsSystem(system: *mut XPhysicsSystem) -> *mut XPhysicsSystem;
        unsafe fn CountRefXPhysicsSystem(system: *const XPhysicsSystem) -> u32;
        fn GetPhysicsSystem(self: Pin<&mut XPhysicsSystem>) -> *mut PhysicsSystem;
        unsafe fn GetBodyInterface(self: Pin<&mut XPhysicsSystem>, lock: bool) -> *mut XBodyInterface;
        unsafe fn GetBroadPhaseLayerInterface(self: &XPhysicsSystem) -> *const BroadPhaseLayerInterface;
        unsafe fn GetObjectVsBroadPhaseLayerFilter(self: &XPhysicsSystem) -> *const ObjectVsBroadPhaseLayerFilter;
        unsafe fn GetObjectLayerPairFilter(self: &XPhysicsSystem) -> *const ObjectLayerPairFilter;
        fn Update(self: Pin<&mut XPhysicsSystem>, delta: f32) -> u32;
        fn GetBodies(self: &XPhysicsSystem, bodies: &mut Vec<BodyID>);
        fn GetActiveBodies(self: &XPhysicsSystem, body_type: BodyType, bodies: &mut Vec<BodyID>);

        type PhysicsSystem;
        unsafe fn SetBodyActivationListener(self: Pin<&mut PhysicsSystem>, listener: *mut BodyActivationListener);
        unsafe fn GetBodyActivationListener(self: &PhysicsSystem) -> *mut BodyActivationListener;
        unsafe fn SetContactListener(self: Pin<&mut PhysicsSystem>, inListener: *mut ContactListener);
        unsafe fn GetContactListener(self: &PhysicsSystem) -> *mut ContactListener;
        // void SetSoftBodyContactListener(SoftBodyContactListener *inListener);
        // SoftBodyContactListener* GetSoftBodyContactListener() const;
        // void SetCombineFriction(ContactConstraintManager::CombineFunction inCombineFriction);
        // ContactConstraintManager::CombineFunction GetCombineFriction() const;
        // void SetCombineRestitution(ContactConstraintManager::CombineFunction inCombineRestition);
        // ContactConstraintManager::CombineFunction GetCombineRestitution() const;
        fn SetPhysicsSettings(self: Pin<&mut PhysicsSystem>, settings: &PhysicsSettings);
        fn GetPhysicsSettings(self: &PhysicsSystem) -> &PhysicsSettings;
        // const BodyInterface& GetBodyInterface() const;
        // BodyInterface& GetBodyInterface();
        // const BodyInterface& GetBodyInterfaceNoLock() const;
        // BodyInterface& GetBodyInterfaceNoLock();
        // inline const BodyLockInterfaceNoLock& GetBodyLockInterfaceNoLock() const;
        // inline const BodyLockInterfaceLocking& GetBodyLockInterface() const;
        // const BroadPhaseQuery & GetBroadPhaseQuery() const;
        // const NarrowPhaseQuery & GetNarrowPhaseQuery() const;
        // const NarrowPhaseQuery & GetNarrowPhaseQueryNoLock() const;
        fn OptimizeBroadPhase(self: Pin<&mut PhysicsSystem>);
        fn GetGravity(self: &PhysicsSystem) -> Vec3;
        fn SetGravity(self: Pin<&mut PhysicsSystem>, gravity: Vec3);
        // void AddStepListener(PhysicsStepListener *inListener);
        // void RemoveStepListener(PhysicsStepListener *inListener);
        fn GetNumBodies(self: &PhysicsSystem) -> u32;
        fn GetNumActiveBodies(self: &PhysicsSystem, body_type: BodyType) -> u32;
        fn GetMaxBodies(self: &PhysicsSystem) -> u32;
        fn GetBodyStats(self: &PhysicsSystem) -> XBodyStats;
        fn WereBodiesInContact(self: &PhysicsSystem, body1: &BodyID, body2: &BodyID) -> bool;
        fn GetBounds(self: &PhysicsSystem) -> AABox;

        type XBodyInterface;
        type BodyCreationSettings;

        fn CreateBody(self: Pin<&mut XBodyInterface>, settings: &BodyCreationSettings) -> BodyID;
        fn CreateBodyWithID(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            settings: &BodyCreationSettings,
        ) -> BodyID;
        fn CreateAddBody(self: Pin<&mut XBodyInterface>, settings: &BodyCreationSettings, active: Activation)
            -> BodyID;
        fn DestroyBody(self: Pin<&mut XBodyInterface>, body_id: &BodyID);
        fn AddBody(self: Pin<&mut XBodyInterface>, body_id: &BodyID, active: Activation);
        fn RemoveBody(self: Pin<&mut XBodyInterface>, body_id: &BodyID);
        fn IsAdded(self: &XBodyInterface, body_id: &BodyID) -> bool;

        fn ActivateBody(self: Pin<&mut XBodyInterface>, body_id: &BodyID);
        unsafe fn ActivateBodies(self: Pin<&mut XBodyInterface>, body_ids: *const BodyID, count: i32);
        fn DeactivateBody(self: Pin<&mut XBodyInterface>, body_id: &BodyID);
        unsafe fn DeactivateBodies(self: Pin<&mut XBodyInterface>, body_ids: *const BodyID, count: i32);
        fn IsActive(self: &XBodyInterface, body_id: &BodyID) -> bool;
        fn ResetSleepTimer(self: Pin<&mut XBodyInterface>, body_id: &BodyID);

        unsafe fn SetShape(
            self: &XBodyInterface,
            body_id: &BodyID,
            shape: *const Shape,
            update_mass_properties: bool,
            activation: Activation,
        );
        fn NotifyShapeChanged(
            self: &XBodyInterface,
            body_id: &BodyID,
            previous_center_of_mass: Vec3,
            update_mass_properties: bool,
            activation: Activation,
        );

        fn SetObjectLayer(self: Pin<&mut XBodyInterface>, body_id: &BodyID, layer: u32);
        fn GetObjectLayer(self: &XBodyInterface, body_id: &BodyID) -> u32;

        fn SetPositionAndRotation(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            position: Vec3,
            rotation: Quat,
            active: Activation,
        );
        fn SetPositionAndRotationWhenChanged(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            position: Vec3,
            rotation: Quat,
            active: Activation,
        );
        fn GetPositionAndRotation(self: &XBodyInterface, body_id: &BodyID, position: &mut Vec3, rotation: &mut Quat);
        fn SetPosition(self: Pin<&mut XBodyInterface>, body_id: &BodyID, position: Vec3, active: Activation);
        fn GetPosition(self: &XBodyInterface, body_id: &BodyID) -> Vec3;
        fn GetCenterOfMassPosition(self: &XBodyInterface, body_id: &BodyID) -> Vec3;
        fn SetRotation(self: Pin<&mut XBodyInterface>, body_id: &BodyID, rotation: Quat, active: Activation);
        fn GetRotation(self: &XBodyInterface, body_id: &BodyID) -> Quat;
        fn GetWorldTransform(self: &XBodyInterface, body_id: &BodyID) -> Mat44;
        fn GetCenterOfMassTransform(self: &XBodyInterface, body_id: &BodyID) -> Mat44;

        fn MoveKinematic(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            target_position: Vec3,
            target_rotation: Quat,
            delta_time: f32,
        );

        fn SetLinearAndAngularVelocity(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            linear_velocity: Vec3,
            angular_velocity: Vec3,
        );
        fn GetLinearAndAngularVelocity(
            self: &XBodyInterface,
            body_id: &BodyID,
            linear_velocity: &mut Vec3,
            angular_velocity: &mut Vec3,
        );
        fn SetLinearVelocity(self: Pin<&mut XBodyInterface>, body_id: &BodyID, linear_velocity: Vec3);
        fn GetLinearVelocity(self: &XBodyInterface, body_id: &BodyID) -> Vec3;
        fn AddLinearVelocity(self: Pin<&mut XBodyInterface>, body_id: &BodyID, linear_velocity: Vec3);
        fn AddLinearAndAngularVelocity(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            linear_velocity: Vec3,
            angular_velocity: Vec3,
        );
        fn SetAngularVelocity(self: Pin<&mut XBodyInterface>, body_id: &BodyID, angular_velocity: Vec3);
        fn GetAngularVelocity(self: &XBodyInterface, body_id: &BodyID) -> Vec3;
        fn GetPointVelocity(self: &XBodyInterface, body_id: &BodyID, point: Vec3) -> Vec3;
        fn SetPositionRotationAndVelocity(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            position: Vec3,
            rotation: Quat,
            linear_velocity: Vec3,
            angular_velocity: Vec3,
        );

        fn AddForce(self: Pin<&mut XBodyInterface>, body_id: &BodyID, force: Vec3, activation: Activation);
        #[rust_name = "AddForceEx"]
        fn AddForce(self: Pin<&mut XBodyInterface>, body_id: &BodyID, force: Vec3, point: Vec3, activation: Activation);
        fn AddTorque(self: Pin<&mut XBodyInterface>, body_id: &BodyID, torque: Vec3, activation: Activation);
        fn AddForceAndTorque(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            force: Vec3,
            torque: Vec3,
            activation: Activation,
        );

        fn AddImpulse(self: Pin<&mut XBodyInterface>, body_id: &BodyID, impulse: Vec3);
        #[rust_name = "AddImpulseEx"]
        fn AddImpulse(self: Pin<&mut XBodyInterface>, body_id: &BodyID, impulse: Vec3, point: Vec3);
        fn AddAngularImpulse(self: Pin<&mut XBodyInterface>, body_id: &BodyID, impulse: Vec3);
        fn ApplyBuoyancyImpulse(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            surface_position: Vec3,
            surface_normal: Vec3,
            buoyancy: f32,
            linear_drag: f32,
            angular_drag: f32,
            fluid_velocity: Vec3,
            gravity: Vec3,
            delta_time: f32,
        ) -> bool;

        fn GetBodyType(self: &XBodyInterface, body_id: &BodyID) -> BodyType;
        fn SetMotionType(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            motion_type: MotionType,
            activation: Activation,
        );
        fn GetMotionType(self: &XBodyInterface, body_id: &BodyID) -> MotionType;
        fn SetMotionQuality(self: Pin<&mut XBodyInterface>, body_id: &BodyID, motion_quality: MotionQuality);
        fn GetMotionQuality(self: &XBodyInterface, body_id: &BodyID) -> MotionQuality;
        fn GetInverseInertia(self: &XBodyInterface, body_id: &BodyID) -> Mat44;
        fn SetRestitution(self: Pin<&mut XBodyInterface>, body_id: &BodyID, restitution: f32);
        fn GetRestitution(self: &XBodyInterface, body_id: &BodyID) -> f32;
        fn SetFriction(self: Pin<&mut XBodyInterface>, body_id: &BodyID, friction: f32);
        fn GetFriction(self: &XBodyInterface, body_id: &BodyID) -> f32;
        fn SetGravityFactor(self: Pin<&mut XBodyInterface>, body_id: &BodyID, gravity_factor: f32);
        fn GetGravityFactor(self: &XBodyInterface, body_id: &BodyID) -> f32;
        fn SetUseManifoldReduction(self: Pin<&mut XBodyInterface>, body_id: &BodyID, use_reduction: bool);
        fn GetUseManifoldReduction(self: &XBodyInterface, body_id: &BodyID) -> bool;
        fn GetUserData(self: &XBodyInterface, body_id: &BodyID) -> u64;
        fn SetUserData(self: &XBodyInterface, body_id: &BodyID, user_data: u64);
        fn InvalidateContactCache(self: Pin<&mut XBodyInterface>, body_id: &BodyID);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PhysicsSettings {
    pub max_in_flight_body_pairs: i32,
    pub step_listeners_batch_size: i32,
    pub step_listener_batches_per_job: i32,
    pub baumgarte: f32,
    pub speculative_contact_distance: f32,
    pub penetration_slop: f32,
    pub linear_cast_threshold: f32,
    pub linear_cast_max_penetration: f32,
    pub manifold_tolerance_sq: f32,
    pub max_penetration_distance: f32,
    pub body_pair_cache_max_delta_position_sq: f32,
    pub body_pair_cache_cos_max_delta_rotation_div2: f32,
    pub contact_normal_cos_max_delta_rotation: f32,
    pub contact_point_preserve_lambda_max_dist_sq: f32,
    pub num_velocity_steps: u32,
    pub num_position_steps: u32,
    pub min_velocity_for_restitution: f32,
    pub time_before_sleep: f32,
    pub point_velocity_sleep_threshold: f32,
    pub deterministic_simulation: bool,
    pub constraint_warm_start: bool,
    pub use_body_pair_contact_cache: bool,
    pub use_manifold_reduction: bool,
    pub use_large_island_splitter: bool,
    pub allow_sleeping: bool,
    pub check_active_edges: bool,
}
const_assert_eq!(mem::size_of::<PhysicsSettings>(), 84);

unsafe impl ExternType for PhysicsSettings {
    type Id = type_id!("PhysicsSettings");
    type Kind = kind::Trivial;
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        PhysicsSettings {
            max_in_flight_body_pairs: 16384,
            step_listeners_batch_size: 8,
            step_listener_batches_per_job: 1,
            baumgarte: 0.2,
            speculative_contact_distance: 0.02,
            penetration_slop: 0.02,
            linear_cast_threshold: 0.75,
            linear_cast_max_penetration: 0.25,
            manifold_tolerance_sq: 1.0e-6,
            max_penetration_distance: 0.2,
            body_pair_cache_max_delta_position_sq: 0.001 * 0.001,
            body_pair_cache_cos_max_delta_rotation_div2: 0.99984769515639123915701155881391, // cos(2 degrees / 2)
            contact_normal_cos_max_delta_rotation: 0.99619469809174553229501040247389,       // cos(5 degree)
            contact_point_preserve_lambda_max_dist_sq: 0.01 * 0.01,
            num_velocity_steps: 10,
            num_position_steps: 2,
            min_velocity_for_restitution: 1.0,
            time_before_sleep: 0.5,
            point_velocity_sleep_threshold: 0.03,
            deterministic_simulation: true,
            constraint_warm_start: true,
            use_body_pair_contact_cache: true,
            use_manifold_reduction: true,
            use_large_island_splitter: true,
            allow_sleeping: true,
            check_active_edges: true,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct BodyStats {
    pub num_bodies: u32,
    pub max_bodies: u32,
    pub num_bodies_static: u32,
    pub num_bodies_dynamic: u32,
    pub num_bodies_active_dynamic: u32,
    pub num_bodies_kinematic: u32,
    pub num_bodies_active_kinematic: u32,
    pub num_bodies_soft: u32,
    pub num_bodies_active_soft: u32,
}
const_assert_eq!(mem::size_of::<BodyStats>(), 36);

unsafe impl ExternType for BodyStats {
    type Id = type_id!("XBodyStats");
    type Kind = kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CollideShapeResult {
    pub contact_point1: Vec3A,
    pub contact_point2: Vec3A,
    pub penetration_axis: Vec3A,
    pub penetration_depth: f32,
    pub sub_shape_id1: SubShapeID,
    pub sub_shape_id2: SubShapeID,
    pub body_id2: BodyID,
    pub shape1_face: StaticArray<Vec3A, 32>,
    pub shape2_face: StaticArray<Vec3A, 32>,
}
const_assert_eq!(mem::size_of::<CollideShapeResult>(), 1120);

unsafe impl ExternType for CollideShapeResult {
    type Id = type_id!("CollideShapeResult");
    type Kind = kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ContactManifold {
    pub base_offset: Vec3A,
    pub world_space_normal: Vec3A,
    pub penetration_depth: f32,
    pub sub_shape_id1: SubShapeID,
    pub sub_shape_id2: SubShapeID,
    pub relative_contact_points_on1: StaticArray<Vec3A, 64>,
    pub relative_contact_points_on2: StaticArray<Vec3A, 64>,
}
const_assert_eq!(mem::size_of::<ContactManifold>(), 2128);

unsafe impl ExternType for ContactManifold {
    type Id = type_id!("ContactManifold");
    type Kind = kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ContactSettings {
    pub combined_friction: f32,
    pub combined_restitution: f32,
    pub inv_mass_scale1: f32,
    pub inv_inertia_scale1: f32,
    pub inv_mass_scale2: f32,
    pub inv_inertia_scale2: f32,
    pub is_sensor: bool,
    pub relative_linear_surface_velocity: Vec3A,
    pub relative_angular_surface_velocity: Vec3A,
}
const_assert_eq!(mem::size_of::<ContactSettings>(), 64);

unsafe impl ExternType for ContactSettings {
    type Id = type_id!("ContactSettings");
    type Kind = kind::Trivial;
}

impl Default for ContactSettings {
    fn default() -> Self {
        Self {
            combined_friction: 0.0,
            combined_restitution: 0.0,
            inv_mass_scale1: 1.0,
            inv_inertia_scale1: 1.0,
            inv_mass_scale2: 1.0,
            inv_inertia_scale2: 1.0,
            is_sensor: false,
            relative_linear_surface_velocity: Vec3A::ZERO,
            relative_angular_surface_velocity: Vec3A::ZERO,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SubShapeIDPair {
    pub body1_id: BodyID,
    pub body2_id: BodyID,
    pub sub_shape_id1: SubShapeID,
    pub sub_shape_id2: SubShapeID,
}
const_assert_eq!(mem::size_of::<SubShapeIDPair>(), 16);

unsafe impl ExternType for SubShapeIDPair {
    type Id = type_id!("SubShapeIDPair");
    type Kind = kind::Trivial;
}

//
// Global Init
//

#[inline]
pub fn global_initialize() {
    static JOLT_INITED: AtomicBool = AtomicBool::new(false);
    if JOLT_INITED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        ffi::GlobalInitialize();
    }
}

#[inline]
pub fn global_finalize() {
    ffi::GlobalFinalize();
}

//
// PhysicsSystem
//

pub struct PhysicsSystem<CL: ContactListener = (), BAL: BodyActivationListener = ()> {
    inner: Box<PhysicsSystemInner>,
    cl_phantom: PhantomData<CL>,
    bal_phantom: PhantomData<BAL>,
}

struct PhysicsSystemInner {
    x_system: NonNull<ffi::XPhysicsSystem>,
    raw_system: NonNull<ffi::PhysicsSystem>,
}

impl<CL: ContactListener, BAL: BodyActivationListener> fmt::Debug for PhysicsSystem<CL, BAL> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysicsSystem").finish()
    }
}

impl<CL: ContactListener, BAL: BodyActivationListener> Drop for PhysicsSystem<CL, BAL> {
    fn drop(&mut self) {
        #[cfg(feature = "debug-print")]
        println!(
            "PhysicsSystem::drop {:?} {}",
            self.inner.x_system,
            unsafe { ffi::CountRefXPhysicsSystem(self.as_x_ptr()) } - 1
        );

        unsafe {
            let cl = self.as_raw_ref().GetContactListener();
            if !cl.is_null() {
                let _ = VBox::<CL, ContactListenerVTable>::from_raw(cl as *mut _);
            }
            self.as_raw_mut().SetContactListener(ptr::null_mut());

            let bal = self.as_raw_ref().GetBodyActivationListener();
            if !bal.is_null() {
                let _ = VBox::<BAL, BodyActivationListenerVTable>::from_raw(bal as *mut _);
            }
            self.as_raw_mut().SetBodyActivationListener(ptr::null_mut());

            ffi::DropXPhysicsSystem(self.as_x_ptr());
        }
    }
}

impl<CL: ContactListener, BAL: BodyActivationListener> PhysicsSystem<CL, BAL> {
    pub fn new<BPLI: BroadPhaseLayerInterface, OBPLF: ObjectVsBroadPhaseLayerFilter, OLPF: ObjectLayerPairFilter>(
        bpli: VBox<BPLI, BroadPhaseLayerInterfaceVTable>,
        obplf: VBox<OBPLF, ObjectVsBroadPhaseLayerFilterVTable>,
        olpf: VBox<OLPF, ObjectLayerPairFilterVTable>,
    ) -> PhysicsSystem<CL, BAL> {
        unsafe {
            let bpli_ptr = Box::into_raw(bpli) as *mut u8;
            let obplf_ptr = Box::into_raw(obplf) as *mut u8;
            let olpf_ptr = Box::into_raw(olpf) as *mut u8;

            let mut x_system = NonNull::new_unchecked(ffi::CreatePhysicSystem(
                Self::clean_up::<BPLI, OBPLF, OLPF>,
                bpli_ptr as *const _,
                obplf_ptr as *const _,
                olpf_ptr as *const _,
            ));
            let raw_system = NonNull::new_unchecked(Pin::new_unchecked(x_system.as_mut()).GetPhysicsSystem());

            PhysicsSystem {
                inner: Box::new(PhysicsSystemInner { x_system, raw_system }),
                cl_phantom: PhantomData,
                bal_phantom: PhantomData,
            }
        }
    }

    fn clean_up<BPLI: BroadPhaseLayerInterface, OBPLF: ObjectVsBroadPhaseLayerFilter, OLPF: ObjectLayerPairFilter>(
        zelf: Pin<&mut ffi::XPhysicsSystem>,
    ) {
        unsafe {
            let bpli = zelf.GetBroadPhaseLayerInterface();
            if !bpli.is_null() {
                let _ = VBox::<BPLI, BroadPhaseLayerInterfaceVTable>::from_raw(bpli as *mut _);
            }

            let obplf = zelf.GetObjectVsBroadPhaseLayerFilter();
            if !obplf.is_null() {
                let _ = VBox::<OBPLF, ObjectVsBroadPhaseLayerFilterVTable>::from_raw(obplf as *mut _);
            }

            let olpf = zelf.GetObjectLayerPairFilter();
            if !olpf.is_null() {
                let _ = VBox::<OLPF, ObjectLayerPairFilterVTable>::from_raw(olpf as *mut _);
            }
        }

        #[cfg(feature = "debug-print")]
        println!("PhysicsSystem::clean_up called");
    }

    #[inline]
    pub fn count_ref(&self) -> u32 {
        unsafe { ffi::CountRefXPhysicsSystem(self.inner.x_system.as_ptr()) }
    }

    #[inline]
    fn as_x_ref(&self) -> &ffi::XPhysicsSystem {
        unsafe { self.inner.x_system.as_ref() }
    }

    #[inline]
    fn as_x_mut(&mut self) -> Pin<&mut ffi::XPhysicsSystem> {
        unsafe { Pin::new_unchecked(self.inner.x_system.as_mut()) }
    }

    #[inline]
    pub(crate) fn as_x_ptr(&self) -> *mut ffi::XPhysicsSystem {
        self.inner.x_system.as_ptr()
    }

    #[inline]
    fn as_raw_ref(&self) -> &ffi::PhysicsSystem {
        unsafe { self.inner.raw_system.as_ref() }
    }

    #[inline]
    fn as_raw_mut(&mut self) -> Pin<&mut ffi::PhysicsSystem> {
        unsafe { Pin::new_unchecked(self.inner.raw_system.as_mut()) }
    }

    #[inline]
    pub unsafe fn cpp_physics_system(&self) -> *mut u8 {
        self.as_x_ptr() as *mut u8
    }

    #[inline]
    pub fn get_body_interface(&mut self) -> &mut BodyInterface {
        unsafe { &mut *(self.as_x_mut().GetBodyInterface(false) as *mut _) }
    }

    #[inline]
    pub fn body_itf(&mut self) -> &mut BodyInterface {
        self.get_body_interface()
    }

    #[inline]
    pub unsafe fn steal_body_interface(&mut self) -> JRef<BodyInterface> {
        unsafe {
            JRef(BorrowedBodyInterface {
                body_itf: NonNull::new_unchecked(self.as_x_mut().GetBodyInterface(false) as *mut _),
                phy_system: ffi::CloneXPhysicsSystem(self.as_x_ptr()),
            })
        }
    }

    #[inline]
    pub unsafe fn steal_body_itf(&mut self) -> JRef<BodyInterface> {
        self.steal_body_interface()
    }

    #[inline]
    pub fn set_body_activation_listener(&mut self, listener: Option<VBox<BAL, BodyActivationListenerVTable>>) {
        unsafe {
            let old = self.as_raw_ref().GetBodyActivationListener() as *mut u8;
            if !old.is_null() {
                let _ = VBox::<BAL, BodyActivationListenerVTable>::from_raw(old as *mut _);
            }
            if let Some(listener) = listener {
                self.as_raw_mut()
                    .SetBodyActivationListener(VBox::<BAL, BodyActivationListenerVTable>::into_raw(listener) as *mut _);
            } else {
                self.as_raw_mut().SetBodyActivationListener(ptr::null_mut());
            }
        };
    }

    #[inline]
    pub fn get_body_activation_listener(&self) -> Option<&VPair<BAL, BodyActivationListenerVTable>> {
        unsafe {
            let current = self.as_raw_ref().GetBodyActivationListener() as *const u8;
            match current.is_null() {
                true => None,
                false => Some(&*(current as *const _)),
            }
        }
    }

    #[inline]
    pub fn set_contact_listener(&mut self, listener: Option<VBox<CL, ContactListenerVTable>>) {
        unsafe {
            let old = self.as_raw_ref().GetContactListener() as *mut u8;
            if !old.is_null() {
                let _ = VBox::<CL, ContactListenerVTable>::from_raw(old as *mut _);
            }
            if let Some(listener) = listener {
                self.as_raw_mut()
                    .SetContactListener(VBox::<CL, ContactListenerVTable>::into_raw(listener) as *mut _);
            } else {
                self.as_raw_mut().SetContactListener(ptr::null_mut());
            }
        };
    }

    #[inline]
    pub fn get_contact_listener(&self) -> Option<&VPair<CL, ContactListenerVTable>> {
        unsafe {
            let current = self.as_raw_ref().GetContactListener() as *const u8;
            match current.is_null() {
                true => None,
                false => Some(&*(current as *const _)),
            }
        }
    }

    #[inline]
    pub fn set_physics_settings(&mut self, settings: &PhysicsSettings) {
        self.as_raw_mut()
            .SetPhysicsSettings(unsafe { mem::transmute::<&PhysicsSettings, &ffi::PhysicsSettings>(settings) });
    }

    #[inline]
    pub fn get_physics_settings(&self) -> &PhysicsSettings {
        unsafe { mem::transmute::<&ffi::PhysicsSettings, &PhysicsSettings>(self.as_raw_ref().GetPhysicsSettings()) }
    }

    #[inline]
    pub fn optimize_broad_phase(&mut self) {
        self.as_raw_mut().OptimizeBroadPhase();
    }

    #[inline]
    pub fn get_gravity(&self) -> Vec3A {
        self.as_raw_ref().GetGravity().into()
    }

    #[inline]
    pub fn set_gravity(&mut self, gravity: Vec3A) {
        self.as_raw_mut().SetGravity(gravity.into());
    }

    #[inline]
    pub fn get_num_bodies(&self) -> u32 {
        self.as_raw_ref().GetNumBodies()
    }

    #[inline]
    pub fn get_num_active_bodies(&self, body_type: BodyType) -> u32 {
        self.as_raw_ref().GetNumActiveBodies(body_type)
    }

    #[inline]
    pub fn get_max_bodies(&self) -> u32 {
        self.as_raw_ref().GetMaxBodies()
    }

    #[inline]
    pub fn get_body_stats(&self) -> BodyStats {
        self.as_raw_ref().GetBodyStats()
    }

    #[inline]
    pub fn were_bodies_in_contact(&self, body1: &BodyID, body2: &BodyID) -> bool {
        self.as_raw_ref().WereBodiesInContact(body1, body2)
    }

    #[inline]
    pub fn get_bounds(&self) -> AABox {
        self.as_raw_ref().GetBounds()
    }

    #[inline]
    pub fn update(&mut self, delta: f32) -> u32 {
        self.as_x_mut().Update(delta)
    }

    #[inline]
    pub fn get_bodies(&self) -> Vec<BodyID> {
        let mut bodies = Vec::new();
        self.as_x_ref().GetBodies(&mut bodies);
        bodies
    }

    #[inline]
    pub fn get_active_bodies(&self, body_type: BodyType) -> Vec<BodyID> {
        let mut bodies = Vec::new();
        self.as_x_ref().GetActiveBodies(body_type, &mut bodies);
        bodies
    }
}

//
// BodyInterface
//

pub struct BodyInterface(pub(crate) ffi::XBodyInterface);

impl fmt::Debug for BodyInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BodyInterface").finish()
    }
}

impl BodyInterface {
    #[inline]
    fn as_ref(&self) -> &ffi::XBodyInterface {
        &self.0
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::XBodyInterface> {
        unsafe { Pin::new_unchecked(&mut self.0) }
    }

    pub fn create_body(&mut self, settings: &BodyCreationSettings) -> JoltResult<BodyID> {
        let body_id = self
            .as_mut()
            .CreateBody(unsafe { mem::transmute::<&BodyCreationSettings, &ffi::BodyCreationSettings>(settings) });
        if body_id.is_invalid() {
            return Err(JoltError::CreateBody);
        }
        Ok(body_id)
    }

    pub fn create_body_with_id(&mut self, body_id: BodyID, settings: &BodyCreationSettings) -> JoltResult<BodyID> {
        let res_body_id = self.as_mut().CreateBodyWithID(&body_id, unsafe {
            mem::transmute::<&BodyCreationSettings, &ffi::BodyCreationSettings>(settings)
        });
        if res_body_id.is_invalid() {
            return Err(JoltError::CreateBody);
        }
        Ok(res_body_id)
    }

    pub fn create_add_body(&mut self, settings: &BodyCreationSettings, active: bool) -> JoltResult<BodyID> {
        let body_id = self.as_mut().CreateAddBody(
            unsafe { mem::transmute::<&BodyCreationSettings, &ffi::BodyCreationSettings>(settings) },
            active.into(),
        );
        if body_id.is_invalid() {
            return Err(JoltError::CreateBody);
        }
        Ok(body_id)
    }

    #[inline]
    pub fn destroy_body(&mut self, body_id: BodyID) {
        self.as_mut().DestroyBody(&body_id);
    }

    #[inline]
    pub fn add_body(&mut self, body_id: BodyID, active: bool) {
        self.as_mut().AddBody(&body_id, active.into())
    }

    #[inline]
    pub fn remove_body(&mut self, body_id: BodyID) {
        self.as_mut().RemoveBody(&body_id)
    }

    #[inline]
    pub fn is_added(&self, body_id: BodyID) -> bool {
        self.as_ref().IsAdded(&body_id)
    }

    #[inline]
    pub fn activate_body(&mut self, body_id: BodyID) {
        self.as_mut().ActivateBody(&body_id)
    }

    #[inline]
    pub fn activate_bodies(&mut self, body_ids: &[BodyID]) {
        unsafe { self.as_mut().ActivateBodies(body_ids.as_ptr(), body_ids.len() as i32) }
    }

    #[inline]
    pub fn deactivate_body(&mut self, body_id: BodyID) {
        self.as_mut().DeactivateBody(&body_id)
    }

    #[inline]
    pub fn deactivate_bodies(&mut self, body_ids: &[BodyID]) {
        unsafe { self.as_mut().DeactivateBodies(body_ids.as_ptr(), body_ids.len() as i32) }
    }

    #[inline]
    pub fn is_active(&self, body_id: BodyID) -> bool {
        self.as_ref().IsActive(&body_id)
    }

    #[inline]
    pub fn reset_sleep_timer(&mut self, body_id: BodyID) {
        self.as_mut().ResetSleepTimer(&body_id)
    }

    #[inline]
    pub fn set_shape(&mut self, body_id: BodyID, shape: &Shape, update_mass_properties: bool, active: bool) {
        unsafe {
            self.as_mut()
                .SetShape(&body_id, &shape.0, update_mass_properties, active.into())
        }
    }

    #[inline]
    pub fn notify_shape_changed(
        &mut self,
        body_id: BodyID,
        previous_center_of_mass: Vec3A,
        update_mass_properties: bool,
        active: bool,
    ) {
        self.as_mut().NotifyShapeChanged(
            &body_id,
            previous_center_of_mass.into(),
            update_mass_properties,
            active.into(),
        )
    }

    #[inline]
    pub fn set_object_layer(&mut self, body_id: BodyID, layer: ObjectLayer) {
        self.as_mut().SetObjectLayer(&body_id, layer)
    }

    #[inline]
    pub fn get_object_layer(&self, body_id: BodyID) -> ObjectLayer {
        self.as_ref().GetObjectLayer(&body_id)
    }

    #[inline]
    pub fn set_position_rotation(&mut self, body_id: BodyID, position: Vec3A, rotation: Quat, active: bool) {
        self.as_mut()
            .SetPositionAndRotation(&body_id, position.into(), rotation.into(), active.into())
    }

    #[inline]
    pub fn set_position_rotation_when_changed(
        &mut self,
        body_id: BodyID,
        position: Vec3A,
        rotation: Quat,
        active: bool,
    ) {
        self.as_mut()
            .SetPositionAndRotationWhenChanged(&body_id, position.into(), rotation.into(), active.into())
    }

    #[inline]
    pub fn get_position_rotation(&self, body_id: BodyID) -> (Vec3A, Quat) {
        let mut position = JVec3::default();
        let mut rotation = JQuat::default();
        self.as_ref()
            .GetPositionAndRotation(&body_id, &mut position, &mut rotation);
        (position.into(), rotation.into())
    }

    #[inline]
    pub fn set_position(&mut self, body_id: BodyID, position: Vec3A, active: bool) {
        self.as_mut().SetPosition(&body_id, position.into(), active.into())
    }

    #[inline]
    pub fn get_position(&self, body_id: BodyID) -> Vec3A {
        self.as_ref().GetPosition(&body_id).into()
    }

    #[inline]
    pub fn get_center_of_mass_position(&self, body_id: BodyID) -> Vec3A {
        self.as_ref().GetCenterOfMassPosition(&body_id).into()
    }

    #[inline]
    pub fn set_rotation(&mut self, body_id: BodyID, rotation: Quat, active: bool) {
        self.as_mut().SetRotation(&body_id, rotation.into(), active.into())
    }

    #[inline]
    pub fn get_rotation(&self, body_id: BodyID) -> Quat {
        self.as_ref().GetRotation(&body_id).into()
    }

    #[inline]
    pub fn get_world_transform(&self, body_id: BodyID) -> Mat4 {
        self.as_ref().GetWorldTransform(&body_id).into()
    }

    #[inline]
    pub fn get_center_of_mass_transform(&self, body_id: BodyID) -> Mat4 {
        self.as_ref().GetCenterOfMassTransform(&body_id).into()
    }

    #[inline]
    pub fn move_kinematic(&mut self, body_id: BodyID, target_position: Vec3A, target_rotation: Quat, delta_time: f32) {
        self.as_mut()
            .MoveKinematic(&body_id, target_position.into(), target_rotation.into(), delta_time)
    }

    #[inline]
    pub fn set_linear_and_angular_velocity(
        &mut self,
        body_id: BodyID,
        linear_velocity: Vec3A,
        angular_velocity: Vec3A,
    ) {
        self.as_mut()
            .SetLinearAndAngularVelocity(&body_id, linear_velocity.into(), angular_velocity.into())
    }

    #[inline]
    pub fn get_linear_and_angular_velocity(&self, body_id: BodyID) -> (Vec3A, Vec3A) {
        let mut linear_velocity = JVec3::default();
        let mut angular_velocity = JVec3::default();
        self.as_ref()
            .GetLinearAndAngularVelocity(&body_id, &mut linear_velocity, &mut angular_velocity);
        (linear_velocity.into(), angular_velocity.into())
    }

    #[inline]
    pub fn set_linear_velocity(&mut self, body_id: BodyID, velocity: Vec3A) {
        self.as_mut().SetLinearVelocity(&body_id, velocity.into())
    }

    #[inline]
    pub fn get_linear_velocity(&self, body_id: BodyID) -> Vec3A {
        self.as_ref().GetLinearVelocity(&body_id).into()
    }

    #[inline]
    pub fn add_linear_velocity(&mut self, body_id: BodyID, velocity: Vec3A) {
        self.as_mut().AddLinearVelocity(&body_id, velocity.into())
    }

    #[inline]
    pub fn add_linear_and_angular_velocity(
        &mut self,
        body_id: BodyID,
        linear_velocity: Vec3A,
        angular_velocity: Vec3A,
    ) {
        self.as_mut()
            .AddLinearAndAngularVelocity(&body_id, linear_velocity.into(), angular_velocity.into())
    }

    #[inline]
    pub fn set_angular_velocity(&mut self, body_id: BodyID, velocity: Vec3A) {
        self.as_mut().SetAngularVelocity(&body_id, velocity.into())
    }

    #[inline]
    pub fn get_angular_velocity(&self, body_id: BodyID) -> Vec3A {
        self.as_ref().GetAngularVelocity(&body_id).into()
    }

    #[inline]
    pub fn get_point_velocity(&self, body_id: BodyID, point: Vec3A) -> Vec3A {
        self.as_ref().GetPointVelocity(&body_id, point.into()).into()
    }

    #[inline]
    pub fn set_position_rotation_and_velocity(
        &mut self,
        body_id: BodyID,
        position: Vec3A,
        rotation: Quat,
        linear_velocity: Vec3A,
        angular_velocity: Vec3A,
    ) {
        self.as_mut().SetPositionRotationAndVelocity(
            &body_id,
            position.into(),
            rotation.into(),
            linear_velocity.into(),
            angular_velocity.into(),
        )
    }

    #[inline]
    pub fn add_force(&mut self, body_id: BodyID, force: Vec3A, active: bool) {
        self.as_mut().AddForce(&body_id, force.into(), active.into())
    }

    #[inline]
    pub fn add_force_ex(&mut self, body_id: BodyID, force: Vec3A, point: Vec3A, active: bool) {
        self.as_mut()
            .AddForceEx(&body_id, force.into(), point.into(), active.into())
    }

    #[inline]
    pub fn add_torque(&mut self, body_id: BodyID, torque: Vec3A, active: bool) {
        self.as_mut().AddTorque(&body_id, torque.into(), active.into())
    }

    #[inline]
    pub fn add_force_and_torque(&mut self, body_id: BodyID, force: Vec3A, torque: Vec3A, active: bool) {
        self.as_mut()
            .AddForceAndTorque(&body_id, force.into(), torque.into(), active.into())
    }

    #[inline]
    pub fn add_impulse(&mut self, body_id: BodyID, impulse: Vec3A) {
        self.as_mut().AddImpulse(&body_id, impulse.into())
    }

    #[inline]
    pub fn add_impulse_ex(&mut self, body_id: BodyID, impulse: Vec3A, point: Vec3A) {
        self.as_mut().AddImpulseEx(&body_id, impulse.into(), point.into())
    }

    #[inline]
    pub fn add_angular_impulse(&mut self, body_id: BodyID, impulse: Vec3A) {
        self.as_mut().AddAngularImpulse(&body_id, impulse.into())
    }

    #[inline]
    pub fn apply_buoyancy_impulse(
        &mut self,
        body_id: &BodyID,
        surface_position: Vec3A,
        surface_normal: Vec3A,
        buoyancy: f32,
        linear_drag: f32,
        angular_drag: f32,
        fluid_velocity: Vec3A,
        gravity: Vec3A,
        delta_time: f32,
    ) -> bool {
        self.as_mut().ApplyBuoyancyImpulse(
            body_id,
            surface_position.into(),
            surface_normal.into(),
            buoyancy,
            linear_drag,
            angular_drag,
            fluid_velocity.into(),
            gravity.into(),
            delta_time,
        )
    }

    #[inline]
    pub fn get_body_type(&self, body_id: BodyID) -> BodyType {
        self.as_ref().GetBodyType(&body_id)
    }

    #[inline]
    pub fn set_motion_type(&mut self, body_id: BodyID, motion_type: MotionType, active: bool) {
        self.as_mut().SetMotionType(&body_id, motion_type, active.into())
    }

    #[inline]
    pub fn get_motion_type(&self, body_id: BodyID) -> MotionType {
        self.as_ref().GetMotionType(&body_id)
    }

    #[inline]
    pub fn set_motion_quality(&mut self, body_id: BodyID, motion_quality: MotionQuality) {
        self.as_mut().SetMotionQuality(&body_id, motion_quality)
    }

    #[inline]
    pub fn get_motion_quality(&self, body_id: BodyID) -> MotionQuality {
        self.as_ref().GetMotionQuality(&body_id)
    }

    #[inline]
    pub fn get_inverse_inertia(&self, body_id: BodyID) -> Mat4 {
        self.as_ref().GetInverseInertia(&body_id).into()
    }

    #[inline]
    pub fn set_restitution(&mut self, body_id: BodyID, restitution: f32) {
        self.as_mut().SetRestitution(&body_id, restitution)
    }

    #[inline]
    pub fn get_restitution(&self, body_id: BodyID) -> f32 {
        self.as_ref().GetRestitution(&body_id)
    }

    #[inline]
    pub fn set_friction(&mut self, body_id: BodyID, friction: f32) {
        self.as_mut().SetFriction(&body_id, friction)
    }

    #[inline]
    pub fn get_friction(&self, body_id: BodyID) -> f32 {
        self.as_ref().GetFriction(&body_id)
    }

    #[inline]
    pub fn set_gravity_factor(&mut self, body_id: BodyID, gravity_factor: f32) {
        self.as_mut().SetGravityFactor(&body_id, gravity_factor)
    }

    #[inline]
    pub fn get_gravity_factor(&self, body_id: BodyID) -> f32 {
        self.as_ref().GetGravityFactor(&body_id)
    }

    #[inline]
    pub fn set_use_manifold_reduction(&mut self, body_id: BodyID, use_reduction: bool) {
        self.as_mut().SetUseManifoldReduction(&body_id, use_reduction)
    }

    #[inline]
    pub fn get_use_manifold_reduction(&self, body_id: BodyID) -> bool {
        self.as_ref().GetUseManifoldReduction(&body_id)
    }

    #[inline]
    pub fn get_user_data(&self, body_id: BodyID) -> u64 {
        self.as_ref().GetUserData(&body_id)
    }

    #[inline]
    pub fn set_user_data(&self, body_id: BodyID, user_data: u64) {
        self.as_ref().SetUserData(&body_id, user_data)
    }

    #[inline]
    pub fn invalidate_contact_cache(&mut self, body_id: BodyID) {
        self.as_mut().InvalidateContactCache(&body_id)
    }
}

#[derive(Debug)]
pub struct BorrowedBodyInterface {
    body_itf: NonNull<BodyInterface>,
    phy_system: *mut ffi::XPhysicsSystem,
}

unsafe impl JRefTarget for BodyInterface {
    type JRaw = BorrowedBodyInterface;

    #[inline]
    fn name() -> &'static str {
        "Character"
    }

    #[inline]
    unsafe fn make_ref(raw: &Self::JRaw) -> &Self {
        unsafe { raw.body_itf.as_ref() }
    }

    #[inline]
    unsafe fn clone_raw(raw: &Self::JRaw) -> Self::JRaw {
        BorrowedBodyInterface {
            body_itf: raw.body_itf,
            phy_system: unsafe { ffi::CloneXPhysicsSystem(raw.phy_system as *mut _) },
        }
    }

    #[inline]
    unsafe fn drop_raw(raw: &mut Self::JRaw) {
        ffi::DropXPhysicsSystem(raw.phy_system);
    }

    #[inline]
    unsafe fn count_ref(raw: &Self::JRaw) -> u32 {
        unsafe { ffi::CountRefXPhysicsSystem(raw.phy_system) }
    }
}

#[cfg(not(feature = "profile"))]
#[vtable]
#[repr(C)]
pub struct BroadPhaseLayerInterfaceVTable {
    pub drop: extern "C" fn(*mut u8),
    pub get_num_broad_phase_layers: extern "C" fn(*const u8) -> u32,
    pub get_broad_phase_layer: extern "C" fn(*const u8, layer: ObjectLayer) -> BroadPhaseLayer,
}

#[cfg(feature = "profile")]
#[vtable]
#[repr(C)]
pub struct BroadPhaseLayerInterfaceVTable {
    pub drop: extern "C" fn(*mut u8),
    pub get_num_broad_phase_layers: extern "C" fn(*const u8) -> u32,
    pub get_broad_phase_layer: extern "C" fn(*const u8, layer: ObjectLayer) -> BroadPhaseLayer,
    pub get_broad_phase_layer_name: extern "C" fn(*const u8, layer: ObjectLayer) -> *const char,
}

#[vtable]
#[repr(C)]
pub struct ObjectVsBroadPhaseLayerFilterVTable {
    pub drop: extern "C" fn(*mut u8),
    pub should_collide: extern "C" fn(*const u8, layer1: ObjectLayer, layer2: BroadPhaseLayer) -> bool,
}

#[vtable]
#[repr(C)]
pub struct ObjectLayerPairFilterVTable {
    pub drop: extern "C" fn(*mut u8),
    pub should_collide: extern "C" fn(*const u8, layer1: ObjectLayer, layer2: ObjectLayer) -> bool,
}

#[vtable(allow_empty)]
#[repr(C)]
pub struct BodyActivationListenerVTable {
    pub drop: extern "C" fn(*mut u8),
    pub on_body_activated: extern "C" fn(*mut u8, body: &BodyID, user_data: u64),
    pub on_body_deactivated: extern "C" fn(*mut u8, body: &BodyID, user_data: u64),
}

#[vtable(allow_empty)]
#[repr(C)]
pub struct ContactListenerVTable {
    pub drop: extern "C" fn(*mut u8),
    pub on_contact_validate: extern "C" fn(
        *mut u8,
        body1: &Body,
        body2: &Body,
        base_offset: JVec3,
        collision_result: &CollideShapeResult,
    ) -> ValidateResult,
    pub on_contact_added:
        extern "C" fn(*mut u8, body1: &Body, body2: &Body, manifold: &ContactManifold, settings: &ContactSettings),
    pub on_contact_persisted:
        extern "C" fn(*mut u8, body1: &Body, body2: &Body, manifold: &ContactManifold, settings: &ContactSettings),
    pub on_contact_removed: extern "C" fn(*mut u8, sub_shape_pair: &SubShapeIDPair),
}
