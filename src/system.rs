use glam::{Mat4, Quat, Vec3A};
use static_assertions::const_assert_eq;
use std::mem;
use std::pin::Pin;

use crate::base::*;
use crate::error::{JoltError, JoltResult};

#[cxx::bridge()]
pub(crate) mod ffi {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum MotionType {
        Static = 0,
        Kinematic = 1,
        Dynamic = 2,
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
        type Isometry = crate::base::ffi::Isometry;
        type BodyID = crate::base::ffi::BodyID;
        type XRefPhysicsSystem = crate::base::ffi::XRefPhysicsSystem;

        type MotionType;
        type MotionQuality;
        type AllowedDOFs;
        type OverrideMassProperties;
        type Activation;

        fn GlobalInitialize();
        fn GlobalFinalize();

        type XPhysicsSystem = crate::base::ffi::XPhysicsSystem;
        unsafe fn CreatePhysicSystem(contacts: *mut XContactCollector) -> XRefPhysicsSystem;
        fn Prepare(self: Pin<&mut XPhysicsSystem>);
        fn Update(self: Pin<&mut XPhysicsSystem>, delta: f32) -> u32;
        fn GetGravity(self: &XPhysicsSystem) -> Vec3;

        type XBodyInterface;
        unsafe fn CreateBodyInterface(system: *mut XPhysicsSystem, lock: bool) -> *mut XBodyInterface;
        type BodyCreationSettings;
        fn CreateBody(self: Pin<&mut XBodyInterface>, settings: &BodyCreationSettings) -> BodyID;
        fn CreateAddBody(self: Pin<&mut XBodyInterface>, settings: &BodyCreationSettings, active: Activation) -> BodyID;
        fn AddBody(self: Pin<&mut XBodyInterface>, body_id: &BodyID, active: Activation);
        fn SetObjectLayer(self: Pin<&mut XBodyInterface>, body_id: &BodyID, layer: u16);
        fn GetObjectLayer(self: &XBodyInterface, body_id: &BodyID) -> u16;
        fn SetPositionAndRotation(self: Pin<&mut XBodyInterface>, body_id: &BodyID, position: Vec3, rotation: Quat, active: Activation);
        fn SetPositionAndRotationWhenChanged(
            self: Pin<&mut XBodyInterface>,
            body_id: &BodyID,
            position: Vec3,
            rotation: Quat,
            active: Activation,
        );
        fn GetPositionAndRotation(self: &XBodyInterface, body_id: &BodyID) -> Isometry;
        fn SetPosition(self: Pin<&mut XBodyInterface>, body_id: &BodyID, position: Vec3, active: Activation);
        fn GetPosition(self: &XBodyInterface, body_id: &BodyID) -> Vec3;
        fn GetCenterOfMassPosition(self: &XBodyInterface, body_id: &BodyID) -> Vec3;
        fn SetRotation(self: Pin<&mut XBodyInterface>, body_id: &BodyID, rotation: Quat, active: Activation);
        fn GetRotation(self: &XBodyInterface, body_id: &BodyID) -> Quat;
        fn GetWorldTransform(self: &XBodyInterface, body_id: &BodyID) -> Mat44;
        fn GetCenterOfMassTransform(self: &XBodyInterface, body_id: &BodyID) -> Mat44;
    }
}

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
    pub shape: RefShape,
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
            shape: RefShape::invalid(),
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
            shape,
            ..Default::default()
        }
    }

    pub fn new_static(shape: RefShape, layer: u16, position: Vec3A, rotation: Quat) -> BodySettings {
        BodySettings {
            position,
            rotation,
            object_layer: layer,
            motion_type: MotionType::Static,
            shape,
            ..Default::default()
        }
    }

    pub fn new_sensor(shape: RefShape, layer: u16, motion_type: MotionType, position: Vec3A, rotation: Quat) -> BodySettings {
        BodySettings {
            position,
            rotation,
            object_layer: layer,
            motion_type,
            is_sensor: true,
            shape,
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
            system: RefPhysicsSystem::invalid(),
        });
        system.system = RefPhysicsSystem(unsafe { ffi::CreatePhysicSystem(&mut system.contacts) });
        system
    }

    #[inline]
    pub fn inner_ref(&self) -> &RefPhysicsSystem {
        &self.system
    }

    #[inline]
    fn system(&self) -> &ffi::XPhysicsSystem {
        return self.system.as_ref().unwrap();
    }

    #[inline]
    fn system_mut(&mut self) -> Pin<&mut ffi::XPhysicsSystem> {
        return unsafe { Pin::new_unchecked(self.system.as_mut().unwrap()) };
    }

    #[inline]
    pub(crate) fn system_ptr(&mut self) -> *mut ffi::XPhysicsSystem {
        unsafe { self.system.ptr() }
    }

    #[inline]
    pub fn body_interface(&mut self, lock: bool) -> BodyInterface {
        BodyInterface::new(self, lock)
    }

    #[inline]
    pub fn prepare(&mut self) {
        return self.system_mut().Prepare();
    }

    #[inline]
    pub fn update(&mut self, delta: f32) -> u32 {
        self.contacts.clear();
        return self.system_mut().Update(delta);
    }

    #[inline]
    pub fn get_gravity(&self) -> Vec3A {
        return self.system().GetGravity().0;
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
    pub fn add_body(&mut self, body_id: BodyID, active: bool) {
        return self.as_mut().AddBody(&body_id, active.into());
    }

    #[inline]
    pub fn set_object_layer(&mut self, body_id: BodyID, layer: u16) {
        return self.as_mut().SetObjectLayer(&body_id, layer);
    }

    #[inline]
    pub fn get_object_layer(&self, body_id: BodyID) -> u16 {
        return self.as_ref().GetObjectLayer(&body_id);
    }

    #[inline]
    pub fn set_position_rotation(&mut self, body_id: BodyID, position: Vec3A, rotation: Quat, active: bool) {
        return self
            .as_mut()
            .SetPositionAndRotation(&body_id, position.into(), rotation.into(), active.into());
    }

    #[inline]
    pub fn set_position_rotation_when_changed(&mut self, body_id: BodyID, position: Vec3A, rotation: Quat, active: bool) {
        return self
            .as_mut()
            .SetPositionAndRotationWhenChanged(&body_id, position.into(), rotation.into(), active.into());
    }

    #[inline]
    pub fn get_position_rotation(&self, body_id: BodyID) -> (Vec3A, Quat) {
        let isometry = self.as_ref().GetPositionAndRotation(&body_id);
        (isometry.position, isometry.rotation)
    }

    #[inline]
    pub fn set_position(&mut self, body_id: BodyID, position: Vec3A, active: bool) {
        return self.as_mut().SetPosition(&body_id, position.into(), active.into());
    }

    #[inline]
    pub fn get_position(&self, body_id: BodyID) -> Vec3A {
        return self.as_ref().GetPosition(&body_id).0;
    }

    #[inline]
    pub fn get_center_of_mass_position(&self, body_id: BodyID) -> Vec3A {
        return self.as_ref().GetCenterOfMassPosition(&body_id).0;
    }

    #[inline]
    pub fn set_rotation(&mut self, body_id: BodyID, rotation: Quat, active: bool) {
        return self.as_mut().SetRotation(&body_id, rotation.into(), active.into());
    }

    #[inline]
    pub fn get_rotation(&self, body_id: BodyID) -> Quat {
        return self.as_ref().GetRotation(&body_id).0;
    }

    #[inline]
    pub fn get_world_transform(&self, body_id: BodyID) -> Mat4 {
        return self.as_ref().GetWorldTransform(&body_id).0;
    }

    #[inline]
    pub fn get_center_of_mass_transform(&self, body_id: BodyID) -> Mat4 {
        return self.as_ref().GetCenterOfMassTransform(&body_id).0;
    }
}
