use glam::{Mat4, Quat, Vec3A};
use static_assertions::const_assert_eq;
use std::mem;
use std::pin::Pin;
use std::ptr::NonNull;

use crate::base::*;
use crate::error::{JoltError, JoltResult};

#[cxx::bridge()]
pub(crate) mod ffi {
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum RsEventType {
        Start,
        Stop,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct RsHitEvent {
        event: RsEventType,
        body_id: BodyID,
        hit_id: BodyID,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct RsSensorEvent {
        event: RsEventType,
        sensor_id: BodyID,
        object_id: BodyID,
    }

    extern "Rust" {
        type XContactCollector;
        fn start_hit_event(self: &mut XContactCollector, body_id: BodyID, hit_id: BodyID);
        fn stop_hit_event(self: &mut XContactCollector, body_id: BodyID, hit_id: BodyID);
        fn start_sensor_event(self: &mut XContactCollector, sensor_id: BodyID, object_id: BodyID);
        fn stop_sensor_event(self: &mut XContactCollector, sensor_id: BodyID, object_id: BodyID);
    }

    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type Vec3 = crate::base::ffi::Vec3;
        type Quat = crate::base::ffi::Quat;
        type Mat44 = crate::base::ffi::Mat44;
        type BodyID = crate::base::ffi::BodyID;
        type Shape = crate::base::ffi::Shape;

        type BodyType;
        type MotionType;
        type MotionQuality;
        type AllowedDOFs;
        type OverrideMassProperties;
        type Activation;

        fn GlobalInitialize();
        fn GlobalFinalize();

        type XPhysicsSystem = crate::base::ffi::XPhysicsSystem;
        unsafe fn CreatePhysicSystem(contacts: *mut XContactCollector) -> *mut XPhysicsSystem;
        fn Prepare(self: Pin<&mut XPhysicsSystem>);
        fn Update(self: Pin<&mut XPhysicsSystem>, delta: f32) -> u32;
        fn GetGravity(self: &XPhysicsSystem) -> Vec3;

        type XBodyInterface;
        unsafe fn CreateBodyInterface(system: *mut XPhysicsSystem, lock: bool) -> *mut XBodyInterface;
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

        fn SetObjectLayer(self: Pin<&mut XBodyInterface>, body_id: &BodyID, layer: u16);
        fn GetObjectLayer(self: &XBodyInterface, body_id: &BodyID) -> u16;

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

pub type BodyType = ffi::BodyType;
pub type MotionType = ffi::MotionType;
pub type MotionQuality = ffi::MotionQuality;
pub type AllowedDOFs = ffi::AllowedDOFs;
pub type OverrideMassProperties = ffi::OverrideMassProperties;

impl From<bool> for ffi::Activation {
    #[inline]
    fn from(value: bool) -> ffi::Activation {
        if value {
            ffi::Activation::Activate
        } else {
            ffi::Activation::DontActivate
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CollisionGroup {
    _group_filter: usize,
    _group_id: u32,
    _sub_group_id: u32,
}
const_assert_eq!(mem::size_of::<CollisionGroup>(), 16);

impl Default for CollisionGroup {
    fn default() -> CollisionGroup {
        CollisionGroup {
            _group_filter: 0,
            _group_id: 0xFFFF_FFFF,
            _sub_group_id: 0xFFFF_FFFF,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct MassProperties {
    pub mass: f32,
    pub inertia: Mat4,
}
const_assert_eq!(mem::size_of::<MassProperties>(), 80);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BodySettings {
    pub position: Vec3A,
    pub rotation: Quat,
    pub linear_velocity: Vec3A,
    pub angular_velocity: Vec3A,
    pub user_data: u64,
    pub object_layer: u16,
    _collision_group: CollisionGroup,
    pub motion_type: MotionType,
    pub allowed_dofs: AllowedDOFs,
    pub allow_dynamic_kinematic: bool,
    pub is_sensor: bool,
    pub collide_kinematic_vs_non_dynamic: bool,
    pub use_manifold_reduction: bool,
    pub apply_gyroscopic_force: bool,
    pub motion_quality: MotionQuality,
    pub enhance_internal_edge_removal: bool,
    pub allow_sleeping: bool,
    pub friction: f32,
    pub restitution: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub max_linear_velocity: f32,
    pub max_angular_velocity: f32,
    pub gravity_factor: f32,
    pub num_velocity_steps_override: u32,
    pub num_position_steps_override: u32,
    pub override_mass_properties: OverrideMassProperties,
    pub inertia_multiplier: f32,
    pub mass_properties: MassProperties,
    _shape_settings: usize,
    pub shape: Option<RefShape>,
}
const_assert_eq!(mem::size_of::<BodySettings>(), 256);

impl Default for BodySettings {
    fn default() -> BodySettings {
        BodySettings {
            position: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
            linear_velocity: Vec3A::ZERO,
            angular_velocity: Vec3A::ZERO,
            user_data: 0,
            object_layer: 0,
            _collision_group: CollisionGroup::default(),
            motion_type: MotionType::Dynamic,
            allowed_dofs: AllowedDOFs::All,
            allow_dynamic_kinematic: false,
            is_sensor: false,
            collide_kinematic_vs_non_dynamic: false,
            use_manifold_reduction: true,
            apply_gyroscopic_force: false,
            motion_quality: MotionQuality::Discrete,
            enhance_internal_edge_removal: false,
            allow_sleeping: true,
            friction: 0.2,
            restitution: 0.0,
            linear_damping: 0.05,
            angular_damping: 0.05,
            max_linear_velocity: 500.0,
            max_angular_velocity: 0.25 * std::f32::consts::PI * 60.0,
            gravity_factor: 1.0,
            num_velocity_steps_override: 0,
            num_position_steps_override: 0,
            override_mass_properties: OverrideMassProperties::CalculateMassAndInertia,
            inertia_multiplier: 1.0,
            mass_properties: MassProperties::default(),
            _shape_settings: 0,
            shape: None,
        }
    }
}

impl BodySettings {
    pub fn new(shape: RefShape, layer: u16, motion_type: MotionType, position: Vec3A, rotation: Quat) -> BodySettings {
        BodySettings {
            position,
            rotation,
            object_layer: layer,
            motion_type,
            shape: Some(shape),
            ..Default::default()
        }
    }

    pub fn new_static(shape: RefShape, layer: u16, position: Vec3A, rotation: Quat) -> BodySettings {
        BodySettings {
            position,
            rotation,
            object_layer: layer,
            motion_type: MotionType::Static,
            shape: Some(shape),
            ..Default::default()
        }
    }

    pub fn new_sensor(
        shape: RefShape,
        layer: u16,
        motion_type: MotionType,
        position: Vec3A,
        rotation: Quat,
    ) -> BodySettings {
        BodySettings {
            position,
            rotation,
            object_layer: layer,
            motion_type,
            is_sensor: true,
            shape: Some(shape),
            ..Default::default()
        }
    }
}

#[inline]
pub fn global_initialize() {
    ffi::GlobalInitialize();
}

#[inline]
pub fn global_finalize() {
    ffi::GlobalFinalize();
}

//
// PhysicsSystem
//

pub struct PhysicsSystem {
    contacts: XContactCollector,
    system: RefPhysicsSystem,
}

impl PhysicsSystem {
    pub fn new() -> Box<PhysicsSystem> {
        let mut system = Box::new(PhysicsSystem {
            contacts: XContactCollector::new(256, 128),
            system: unsafe { mem::transmute::<usize, RefPhysicsSystem>(usize::MAX) },
        });
        unsafe { system.system.0 = NonNull::new_unchecked(ffi::CreatePhysicSystem(&mut system.contacts)) };
        system
    }

    #[inline]
    pub fn inner_ref(&self) -> &RefPhysicsSystem {
        &self.system
    }

    #[inline]
    fn system(&self) -> &ffi::XPhysicsSystem {
        self.system.as_ref()
    }

    #[inline]
    fn system_mut(&mut self) -> Pin<&mut ffi::XPhysicsSystem> {
        unsafe { Pin::new_unchecked(self.system.as_mut()) }
    }

    #[inline]
    pub(crate) fn system_ptr(&mut self) -> *mut ffi::XPhysicsSystem {
        self.system.as_mut_ptr()
    }

    #[inline]
    pub fn body_interface(&mut self, lock: bool) -> BodyInterface {
        BodyInterface::new(self, lock)
    }

    #[inline]
    pub fn prepare(&mut self) {
        self.system_mut().Prepare()
    }

    #[inline]
    pub fn update(&mut self, delta: f32) -> u32 {
        self.contacts.clear();
        self.system_mut().Update(delta)
    }

    #[inline]
    pub fn get_gravity(&self) -> Vec3A {
        self.system().GetGravity().0
    }

    #[inline]
    pub fn hit_events(&self) -> &Vec<HitEvent> {
        &self.contacts.hit_events
    }

    #[inline]
    pub fn sensor_events(&self) -> &Vec<SensorEvent> {
        &self.contacts.sensor_events
    }
}

//
// EventCollector
//

pub type EventType = ffi::RsEventType;
pub type HitEvent = ffi::RsHitEvent;
pub type SensorEvent = ffi::RsSensorEvent;

pub struct XContactCollector {
    hit_events: Vec<HitEvent>,
    sensor_events: Vec<SensorEvent>,
}

impl XContactCollector {
    fn new(hit_cap: usize, sensor_cap: usize) -> XContactCollector {
        XContactCollector {
            hit_events: Vec::with_capacity(hit_cap),
            sensor_events: Vec::with_capacity(sensor_cap),
        }
    }

    fn clear(&mut self) {
        self.hit_events.clear();
        self.sensor_events.clear();
    }

    fn start_hit_event(&mut self, body_id: BodyID, hit_id: BodyID) {
        // println!("start_hit_event: {:?} {:?}", body_id, hit_id);
        self.hit_events.push(HitEvent {
            event: EventType::Start,
            body_id,
            hit_id,
        });
    }

    fn stop_hit_event(&mut self, body_id: BodyID, hit_id: BodyID) {
        // println!("stop_hit_event: {:?} {:?}", body_id, hit_id);
        self.hit_events.push(HitEvent {
            event: EventType::Stop,
            body_id,
            hit_id,
        });
    }

    fn start_sensor_event(&mut self, sensor_id: BodyID, object_id: BodyID) {
        self.sensor_events.push(SensorEvent {
            event: EventType::Start,
            sensor_id,
            object_id,
        });
    }

    fn stop_sensor_event(&mut self, sensor_id: BodyID, object_id: BodyID) {
        self.sensor_events.push(SensorEvent {
            event: EventType::Stop,
            sensor_id,
            object_id,
        });
    }
}

//
// BodyInterface
//

#[derive(Debug, Clone)]
pub struct BodyInterface {
    body_itf: *mut ffi::XBodyInterface,
    _system: RefPhysicsSystem,
}

impl BodyInterface {
    pub fn new(system: &mut PhysicsSystem, lock: bool) -> BodyInterface {
        BodyInterface {
            body_itf: unsafe { ffi::CreateBodyInterface(system.system_ptr(), lock) },
            _system: system.system.clone(),
        }
    }

    #[inline]
    fn as_ref(&self) -> &ffi::XBodyInterface {
        unsafe { &*self.body_itf }
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::XBodyInterface> {
        unsafe { Pin::new_unchecked(&mut *self.body_itf) }
    }

    pub fn create_body(&mut self, settings: &BodySettings) -> JoltResult<BodyID> {
        let body_id = self
            .as_mut()
            .CreateBody(unsafe { mem::transmute::<&BodySettings, &ffi::BodyCreationSettings>(settings) });
        if body_id.is_invalid() {
            return Err(JoltError::CreateBody);
        }
        Ok(body_id)
    }

    pub fn create_body_with_id(&mut self, body_id: BodyID, settings: &BodySettings) -> JoltResult<BodyID> {
        let res_body_id = self.as_mut().CreateBodyWithID(&body_id, unsafe {
            mem::transmute::<&BodySettings, &ffi::BodyCreationSettings>(settings)
        });
        if res_body_id.is_invalid() {
            return Err(JoltError::CreateBody);
        }
        Ok(res_body_id)
    }

    pub fn create_add_body(&mut self, settings: &BodySettings, active: bool) -> JoltResult<BodyID> {
        let body_id = self.as_mut().CreateAddBody(
            unsafe { mem::transmute::<&BodySettings, &ffi::BodyCreationSettings>(settings) },
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
    pub fn set_shape(&mut self, body_id: BodyID, shape: &RefShape, update_mass_properties: bool, active: bool) {
        unsafe {
            self.as_mut()
                .SetShape(&body_id, shape.as_ptr(), update_mass_properties, active.into())
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
    pub fn set_object_layer(&mut self, body_id: BodyID, layer: u16) {
        self.as_mut().SetObjectLayer(&body_id, layer)
    }

    #[inline]
    pub fn get_object_layer(&self, body_id: BodyID) -> u16 {
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
        let mut position = XVec3::default();
        let mut rotation = XQuat::default();
        self.as_ref()
            .GetPositionAndRotation(&body_id, &mut position, &mut rotation);
        (position.0, rotation.0)
    }

    #[inline]
    pub fn set_position(&mut self, body_id: BodyID, position: Vec3A, active: bool) {
        self.as_mut().SetPosition(&body_id, position.into(), active.into())
    }

    #[inline]
    pub fn get_position(&self, body_id: BodyID) -> Vec3A {
        self.as_ref().GetPosition(&body_id).0
    }

    #[inline]
    pub fn get_center_of_mass_position(&self, body_id: BodyID) -> Vec3A {
        self.as_ref().GetCenterOfMassPosition(&body_id).0
    }

    #[inline]
    pub fn set_rotation(&mut self, body_id: BodyID, rotation: Quat, active: bool) {
        self.as_mut().SetRotation(&body_id, rotation.into(), active.into())
    }

    #[inline]
    pub fn get_rotation(&self, body_id: BodyID) -> Quat {
        self.as_ref().GetRotation(&body_id).0
    }

    #[inline]
    pub fn get_world_transform(&self, body_id: BodyID) -> Mat4 {
        self.as_ref().GetWorldTransform(&body_id).0
    }

    #[inline]
    pub fn get_center_of_mass_transform(&self, body_id: BodyID) -> Mat4 {
        self.as_ref().GetCenterOfMassTransform(&body_id).0
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
        let mut linear_velocity = XVec3::default();
        let mut angular_velocity = XVec3::default();
        self.as_ref()
            .GetLinearAndAngularVelocity(&body_id, &mut linear_velocity, &mut angular_velocity);
        (linear_velocity.0, angular_velocity.0)
    }

    #[inline]
    pub fn set_linear_velocity(&mut self, body_id: BodyID, velocity: Vec3A) {
        self.as_mut().SetLinearVelocity(&body_id, velocity.into())
    }

    #[inline]
    pub fn get_linear_velocity(&self, body_id: BodyID) -> Vec3A {
        self.as_ref().GetLinearVelocity(&body_id).0
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
        self.as_ref().GetAngularVelocity(&body_id).0
    }

    #[inline]
    pub fn get_point_velocity(&self, body_id: BodyID, point: Vec3A) -> Vec3A {
        self.as_ref().GetPointVelocity(&body_id, point.into()).0
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
        self.as_ref().GetInverseInertia(&body_id).0
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
