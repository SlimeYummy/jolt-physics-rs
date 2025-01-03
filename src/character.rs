use cxx::UniquePtr;
use glam::{Mat4, Quat, Vec3, Vec3A};
use static_assertions::const_assert_eq;
use std::mem;
use std::pin::Pin;

use crate::base::*;
use crate::system::PhysicsSystem;

#[cxx::bridge()]
pub(crate) mod ffi {
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum GroundState {
        OnGround,
        OnSteepGround,
        NotSupported,
        InAir,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum BackFaceMode {
        IgnoreBackFaces,
        CollideWithBackFaces,
    }

    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type Vec3 = crate::base::ffi::Vec3;
        type Quat = crate::base::ffi::Quat;
        type Mat44 = crate::base::ffi::Mat44;
        type BodyID = crate::base::ffi::BodyID;
        type Activation = crate::system::ffi::Activation;
        type Shape = crate::shape::ffi::Shape;
        type XPhysicsSystem = crate::system::ffi::XPhysicsSystem;

        type GroundState;
        type BackFaceMode;
        type XCharacterCommonSettings;
        type XCharacterVirtualSettings;
        type ExtendedUpdateSettings;

        type XCharacterCommon;
        unsafe fn CreateCharacterCommon(
            system: *mut XPhysicsSystem,
            settings: &XCharacterCommonSettings,
            position: Vec3,
            rotation: Quat,
            user_data: u64,
        ) -> UniquePtr<XCharacterCommon>;
        unsafe fn CreateAddCharacterCommon(
            system: *mut XPhysicsSystem,
            settings: &XCharacterCommonSettings,
            position: Vec3,
            rotation: Quat,
            user_data: u64,
            activation: Activation,
            lock: bool,
        ) -> UniquePtr<XCharacterCommon>;
        fn SetMaxSlopeAngle(self: Pin<&mut XCharacterCommon>, angle: f32);
        fn GetCosMaxSlopeAngle(self: &XCharacterCommon) -> f32;
        fn SetUp(self: Pin<&mut XCharacterCommon>, up: Vec3);
        fn GetUp(self: &XCharacterCommon) -> Vec3;
        fn IsSlopeTooSteep(self: &XCharacterCommon, normal: Vec3) -> bool;
        fn GetGroundState(self: &XCharacterCommon) -> GroundState;
        fn IsSupported(self: &XCharacterCommon) -> bool;
        unsafe fn GetShape(self: &XCharacterCommon) -> *const Shape;
        fn GetGroundPosition(self: &XCharacterCommon) -> Vec3;
        fn GetGroundNormal(self: &XCharacterCommon) -> Vec3;
        fn GetGroundVelocity(self: &XCharacterCommon) -> Vec3;
        fn GetGroundBodyID(self: &XCharacterCommon) -> BodyID;
        fn GetBodyID(self: &XCharacterCommon) -> BodyID;
        fn AddToPhysicsSystem(self: Pin<&mut XCharacterCommon>, activation: Activation, lock: bool);
        fn RemoveFromPhysicsSystem(self: Pin<&mut XCharacterCommon>, lock: bool);
        fn Activate(self: Pin<&mut XCharacterCommon>, lock: bool);
        fn PostSimulation(self: Pin<&mut XCharacterCommon>, max_distance: f32, lock: bool);
        fn SetLinearAndAngularVelocity(self: Pin<&mut XCharacterCommon>, linear: Vec3, angular: Vec3, lock: bool);
        fn GetLinearVelocity(self: &XCharacterCommon, lock: bool) -> Vec3;
        fn SetLinearVelocity(self: Pin<&mut XCharacterCommon>, velocity: Vec3, lock: bool);
        fn AddLinearVelocity(self: Pin<&mut XCharacterCommon>, velocity: Vec3, lock: bool);
        fn AddImpulse(self: Pin<&mut XCharacterCommon>, impulse: Vec3, lock: bool);
        fn GetPositionAndRotation(self: &XCharacterCommon, position: &mut Vec3, rotation: &mut Quat, lock: bool);
        fn SetPositionAndRotation(
            self: &XCharacterCommon,
            position: Vec3,
            rotation: Quat,
            activation: Activation,
            lock: bool,
        );
        fn GetPosition(self: &XCharacterCommon, lock: bool) -> Vec3;
        fn SetPosition(self: Pin<&mut XCharacterCommon>, position: Vec3, activation: Activation, lock: bool);
        fn GetRotation(self: &XCharacterCommon, lock: bool) -> Quat;
        fn SetRotation(self: Pin<&mut XCharacterCommon>, rotation: Quat, activation: Activation, lock: bool);
        fn GetCenterOfMassPosition(self: &XCharacterCommon, lock: bool) -> Vec3;
        fn GetWorldTransform(self: &XCharacterCommon, lock: bool) -> Mat44;
        fn SetLayer(self: Pin<&mut XCharacterCommon>, layer: u16, lock: bool);
        unsafe fn SetShape(
            self: Pin<&mut XCharacterCommon>,
            shape: *const Shape,
            max_penetration_depth: f32,
            lock: bool,
        ) -> bool;

        type XCharacterVirtual;
        unsafe fn CreateCharacterVirtual(
            system: *mut XPhysicsSystem,
            settings: &XCharacterVirtualSettings,
            position: Vec3,
            rotation: Quat,
        ) -> UniquePtr<XCharacterVirtual>;
        fn SetMaxSlopeAngle(self: Pin<&mut XCharacterVirtual>, angle: f32);
        fn GetCosMaxSlopeAngle(self: &XCharacterVirtual) -> f32;
        fn SetUp(self: Pin<&mut XCharacterVirtual>, up: Vec3);
        fn GetUp(self: &XCharacterVirtual) -> Vec3;
        fn IsSlopeTooSteep(self: &XCharacterVirtual, normal: Vec3) -> bool;
        fn GetGroundState(self: &XCharacterVirtual) -> GroundState;
        fn IsSupported(self: &XCharacterVirtual) -> bool;
        fn GetShape(self: &XCharacterVirtual) -> *const Shape;
        fn GetGroundPosition(self: &XCharacterVirtual) -> Vec3;
        fn GetGroundNormal(self: &XCharacterVirtual) -> Vec3;
        fn GetGroundVelocity(self: &XCharacterVirtual) -> Vec3;
        fn GetGroundBodyID(self: &XCharacterVirtual) -> BodyID;
        fn GetLinearVelocity(self: &XCharacterVirtual) -> Vec3;
        fn SetLinearVelocity(self: Pin<&mut XCharacterVirtual>, velocity: Vec3);
        fn GetPosition(self: &XCharacterVirtual) -> Vec3;
        fn SetPosition(self: Pin<&mut XCharacterVirtual>, position: Vec3);
        fn GetRotation(self: &XCharacterVirtual) -> Quat;
        fn SetRotation(self: Pin<&mut XCharacterVirtual>, rotation: Quat);
        fn GetWorldTransform(self: &XCharacterVirtual) -> Mat44;
        fn GetCenterOfMassTransform(self: &XCharacterVirtual) -> Mat44;
        fn GetMass(self: &XCharacterVirtual) -> f32;
        fn SetMass(self: Pin<&mut XCharacterVirtual>, mass: f32);
        fn GetMaxStrength(self: &XCharacterVirtual) -> f32;
        fn SetMaxStrength(self: Pin<&mut XCharacterVirtual>, max_strength: f32);
        fn GetPenetrationRecoverySpeed(self: &XCharacterVirtual) -> f32;
        fn SetPenetrationRecoverySpeed(self: Pin<&mut XCharacterVirtual>, speed: f32);
        fn GetCharacterPadding(self: &XCharacterVirtual) -> f32;
        fn GetMaxNumHits(self: &XCharacterVirtual) -> u32;
        fn SetMaxNumHits(self: Pin<&mut XCharacterVirtual>, max_hits: u32);
        fn GetHitReductionCosMaxAngle(self: &XCharacterVirtual) -> f32;
        fn SetHitReductionCosMaxAngle(self: Pin<&mut XCharacterVirtual>, cos_max_angle: f32);
        fn GetMaxHitsExceeded(self: &XCharacterVirtual) -> bool;
        fn GetShapeOffset(self: &XCharacterVirtual) -> Vec3;
        fn SetShapeOffset(self: Pin<&mut XCharacterVirtual>, offset: Vec3);
        fn CancelVelocityTowardsSteepSlopes(self: &XCharacterVirtual, desired_velocity: Vec3) -> Vec3;
        fn Update(self: Pin<&mut XCharacterVirtual>, chara_layer: u16, delta_time: f32, gravity: Vec3);
        fn CanWalkStairs(self: &XCharacterVirtual, velocity: Vec3) -> bool;
        fn WalkStairs(
            self: Pin<&mut XCharacterVirtual>,
            chara_layer: u16,
            delta_time: f32,
            step_up: Vec3,
            step_forward: Vec3,
            step_forward_test: Vec3,
            step_down_extra: Vec3,
        ) -> bool;
        fn StickToFloor(self: Pin<&mut XCharacterVirtual>, chara_layer: u16, step_down: Vec3) -> bool;
        fn ExtendedUpdate(
            self: Pin<&mut XCharacterVirtual>,
            chara_layer: u16,
            delta_time: f32,
            gravity: Vec3,
            settings: &ExtendedUpdateSettings,
        );
        fn RefreshContacts(self: Pin<&mut XCharacterVirtual>, chara_layer: u16);
        fn UpdateGroundVelocity(self: Pin<&mut XCharacterVirtual>);
        unsafe fn SetShape(
            self: Pin<&mut XCharacterVirtual>,
            chara_layer: u16,
            shape: *const Shape,
            max_penetration_depth: f32,
        ) -> bool;
    }
}

pub type GroundState = ffi::GroundState;
pub type BackFaceMode = ffi::BackFaceMode;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CharacterCommonSettings {
    up: Vec3A,
    supporting_volume: Plane,
    max_slope_angle: f32,
    shape: Option<RefShape>,
    layer: u16,
    mass: f32,
    friction: f32,
    gravity_factor: f32,
}
const_assert_eq!(mem::size_of::<CharacterCommonSettings>(), 64);

impl Default for CharacterCommonSettings {
    fn default() -> CharacterCommonSettings {
        CharacterCommonSettings {
            up: Vec3A::Y,
            supporting_volume: Plane::new(Vec3::Y, -1.0e10),
            max_slope_angle: 50.0 / 180.0 * std::f32::consts::PI,
            shape: None,
            layer: 0,
            mass: 80.0,
            friction: 0.2,
            gravity_factor: 1.0,
        }
    }
}

impl CharacterCommonSettings {
    pub fn new(shape: RefShape, layer: u16) -> CharacterCommonSettings {
        CharacterCommonSettings {
            shape: Some(shape),
            layer,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CharacterVirtualSettings {
    up: Vec3A,
    supporting_volume: Plane,
    max_slope_angle: f32,
    shape: Option<RefShape>,
    mass: f32,
    max_strength: f32,
    shape_offset: Vec3A,
    back_face_mode: BackFaceMode,
    predictive_contact_distance: f32,
    max_collision_iterations: u32,
    max_constraint_iterations: u32,
    min_time_remaining: f32,
    collision_tolerance: f32,
    character_padding: f32,
    max_num_hits: u32,
    hit_reduction_cos_max_angle: f32,
    penetration_recovery_speed: f32,
}
const_assert_eq!(mem::size_of::<CharacterVirtualSettings>(), 128);

impl Default for CharacterVirtualSettings {
    fn default() -> CharacterVirtualSettings {
        CharacterVirtualSettings {
            up: Vec3A::Y,
            supporting_volume: Plane::new(Vec3::Y, -1.0e10),
            max_slope_angle: 50.0 / 180.0 * std::f32::consts::PI,
            shape: None,
            mass: 70.0,
            max_strength: 100.0,
            shape_offset: Vec3A::ZERO,
            back_face_mode: BackFaceMode::CollideWithBackFaces,
            predictive_contact_distance: 0.1,
            max_collision_iterations: 5,
            max_constraint_iterations: 15,
            min_time_remaining: 1.0e-4,
            collision_tolerance: 1.0e-3,
            character_padding: 0.02,
            max_num_hits: 256,
            hit_reduction_cos_max_angle: 0.999,
            penetration_recovery_speed: 1.0,
        }
    }
}

impl CharacterVirtualSettings {
    pub fn new(shape: RefShape) -> CharacterVirtualSettings {
        CharacterVirtualSettings {
            shape: Some(shape),
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtendedUpdateSettings {
    stick_to_floor_step_down: Vec3A,
    walk_stairs_step_up: Vec3A,
    walk_stairs_min_step_forward: f32,
    walk_stairs_step_forward_test: f32,
    walk_stairs_cos_angle_forward_contact: f32,
    walk_stairs_step_down_extra: Vec3A,
}
const_assert_eq!(mem::size_of::<ExtendedUpdateSettings>(), 64);

impl Default for ExtendedUpdateSettings {
    fn default() -> ExtendedUpdateSettings {
        ExtendedUpdateSettings {
            stick_to_floor_step_down: Vec3A::new(0.0, -0.5, 0.0),
            walk_stairs_step_up: Vec3A::new(0.0, 0.4, 0.0),
            walk_stairs_min_step_forward: 0.02,
            walk_stairs_step_forward_test: 0.15,
            walk_stairs_cos_angle_forward_contact: 0.258_819_04, // cos(75°)
            walk_stairs_step_down_extra: Vec3A::new(0.0, 0.0, 0.0),
        }
    }
}

pub struct CharacterCommon {
    chara: UniquePtr<ffi::XCharacterCommon>,
    _system: RefPhysicsSystem,
}

impl Drop for CharacterCommon {
    fn drop(&mut self) {
        self.chara = UniquePtr::null();
    }
}

impl CharacterCommon {
    pub fn new(
        system: &mut PhysicsSystem,
        settings: &CharacterCommonSettings,
        position: Vec3A,
        rotation: Quat,
        user_data: u64,
    ) -> CharacterCommon {
        let chara = unsafe {
            ffi::CreateCharacterCommon(
                system.system_ptr(),
                mem::transmute::<&CharacterCommonSettings, &ffi::XCharacterCommonSettings>(settings),
                position.into(),
                rotation.into(),
                user_data,
            )
        };
        CharacterCommon {
            chara,
            _system: system.inner_ref().clone(),
        }
    }

    pub fn new_ex(
        system: &mut PhysicsSystem,
        settings: &CharacterCommonSettings,
        position: Vec3A,
        rotation: Quat,
        user_data: u64,
        active: bool,
        lock: bool,
    ) -> CharacterCommon {
        let chara = unsafe {
            ffi::CreateAddCharacterCommon(
                system.system_ptr(),
                mem::transmute::<&CharacterCommonSettings, &ffi::XCharacterCommonSettings>(settings),
                position.into(),
                rotation.into(),
                user_data,
                active.into(),
                lock,
            )
        };
        CharacterCommon {
            chara,
            _system: system.inner_ref().clone(),
        }
    }

    #[inline]
    fn as_ref(&self) -> &ffi::XCharacterCommon {
        self.chara.as_ref().unwrap()
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::XCharacterCommon> {
        self.chara.as_mut().unwrap()
    }

    #[inline]
    pub fn set_max_slope_angle(&mut self, angle: f32) {
        self.as_mut().SetMaxSlopeAngle(angle);
    }

    #[inline]
    pub fn get_cos_max_slope_angle(&self) -> f32 {
        self.as_ref().GetCosMaxSlopeAngle()
    }

    #[inline]
    pub fn set_up(&mut self, up: Vec3A) {
        self.as_mut().SetUp(up.into());
    }

    #[inline]
    pub fn get_up(&self) -> Vec3A {
        self.as_ref().GetUp().0
    }

    #[inline]
    pub fn is_slope_too_steep(&self, normal: Vec3A) -> bool {
        self.as_ref().IsSlopeTooSteep(normal.into())
    }

    #[inline]
    pub fn get_ground_state(&self) -> GroundState {
        self.as_ref().GetGroundState()
    }

    #[inline]
    pub fn is_supported(&self) -> bool {
        self.as_ref().IsSupported()
    }

    #[inline]
    pub fn get_shape(&self) -> RefShape {
        unsafe { RefShape::new(self.as_ref().GetShape() as *mut _) }
    }

    #[inline]
    pub fn get_ground_position(&self) -> Vec3A {
        self.as_ref().GetGroundPosition().0
    }

    #[inline]
    pub fn get_ground_normal(&self) -> Vec3A {
        self.as_ref().GetGroundNormal().0
    }

    #[inline]
    pub fn get_ground_velocity(&self) -> Vec3A {
        self.as_ref().GetGroundVelocity().0
    }

    #[inline]
    pub fn get_ground_body_id(&self) -> BodyID {
        self.as_ref().GetGroundBodyID()
    }

    #[inline]
    pub fn get_body_id(&self) -> BodyID {
        self.as_ref().GetBodyID()
    }

    #[inline]
    pub fn add_to_physics_system(&mut self, active: bool, lock: bool) {
        self.as_mut().AddToPhysicsSystem(active.into(), lock);
    }

    #[inline]
    pub fn remove_from_physics_system(&mut self, lock: bool) {
        self.as_mut().RemoveFromPhysicsSystem(lock);
    }

    #[inline]
    pub fn activate(&mut self, lock: bool) {
        self.as_mut().Activate(lock);
    }

    #[inline]
    pub fn post_simulation(&mut self, max_distance: f32, lock: bool) {
        self.as_mut().PostSimulation(max_distance, lock);
    }

    #[inline]
    pub fn set_velocity(&mut self, linear: Vec3A, angular: Vec3A, lock: bool) {
        self.as_mut()
            .SetLinearAndAngularVelocity(linear.into(), angular.into(), lock);
    }

    #[inline]
    pub fn set_linear_velocity(&mut self, velocity: Vec3A, lock: bool) {
        self.as_mut().SetLinearVelocity(velocity.into(), lock);
    }

    #[inline]
    pub fn add_linear_velocity(&mut self, velocity: Vec3A, lock: bool) {
        self.as_mut().AddLinearVelocity(velocity.into(), lock);
    }

    #[inline]
    pub fn add_impulse(&mut self, impulse: Vec3A, lock: bool) {
        self.as_mut().AddImpulse(impulse.into(), lock);
    }

    #[inline]
    pub fn get_position_and_rotation(&self, lock: bool) -> (Vec3A, Quat) {
        let mut position = XVec3::default();
        let mut rotation = XQuat::default();
        self.as_ref().GetPositionAndRotation(&mut position, &mut rotation, lock);
        (position.0, rotation.0)
    }

    #[inline]
    pub fn set_position_and_rotation(&mut self, position: Vec3A, rotation: Quat, active: bool, lock: bool) {
        self.as_mut()
            .SetPositionAndRotation(position.into(), rotation.into(), active.into(), lock);
    }

    #[inline]
    pub fn get_position(&self, lock: bool) -> Vec3A {
        self.as_ref().GetPosition(lock).0
    }

    #[inline]
    pub fn set_position(&mut self, position: Vec3A, active: bool, lock: bool) {
        self.as_mut().SetPosition(position.into(), active.into(), lock);
    }

    #[inline]
    pub fn get_rotation(&self, lock: bool) -> Quat {
        self.as_ref().GetRotation(lock).0
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Quat, active: bool, lock: bool) {
        self.as_mut().SetRotation(rotation.into(), active.into(), lock);
    }

    #[inline]
    pub fn get_linear_velocity(&self, lock: bool) -> Vec3A {
        self.as_ref().GetLinearVelocity(lock).0
    }

    #[inline]
    pub fn get_center_of_mass_position(&self, lock: bool) -> Vec3A {
        self.as_ref().GetCenterOfMassPosition(lock).0
    }

    #[inline]
    pub fn get_world_transform(&self, lock: bool) -> Mat4 {
        self.as_ref().GetWorldTransform(lock).0
    }

    #[inline]
    pub fn set_layer(&mut self, layer: u16, lock: bool) {
        self.as_mut().SetLayer(layer, lock);
    }

    #[inline]
    pub fn set_shape(&mut self, shape: &mut RefShape, max_penetration_depth: f32, lock: bool) -> bool {
        unsafe { self.as_mut().SetShape(shape.as_ptr(), max_penetration_depth, lock) }
    }
}

pub struct CharacterVirtual {
    chara: UniquePtr<ffi::XCharacterVirtual>,
    _system: RefPhysicsSystem,
}

impl Drop for CharacterVirtual {
    fn drop(&mut self) {
        self.chara = UniquePtr::null();
    }
}

impl CharacterVirtual {
    pub fn new(
        system: &mut PhysicsSystem,
        settings: &CharacterVirtualSettings,
        position: Vec3A,
        rotation: Quat,
    ) -> CharacterVirtual {
        let chara = unsafe {
            ffi::CreateCharacterVirtual(
                system.system_ptr(),
                mem::transmute::<&CharacterVirtualSettings, &ffi::XCharacterVirtualSettings>(settings),
                position.into(),
                rotation.into(),
            )
        };
        CharacterVirtual {
            chara,
            _system: system.inner_ref().clone(),
        }
    }

    #[inline]
    fn as_ref(&self) -> &ffi::XCharacterVirtual {
        self.chara.as_ref().unwrap()
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::XCharacterVirtual> {
        self.chara.as_mut().unwrap()
    }

    #[inline]
    pub fn set_max_slope_angle(&mut self, angle: f32) {
        self.as_mut().SetMaxSlopeAngle(angle);
    }

    #[inline]
    pub fn get_cos_max_slope_angle(&self) -> f32 {
        self.as_ref().GetCosMaxSlopeAngle()
    }

    #[inline]
    pub fn set_up(&mut self, up: Vec3A) {
        self.as_mut().SetUp(up.into());
    }

    #[inline]
    pub fn get_up(&self) -> Vec3A {
        self.as_ref().GetUp().0
    }

    #[inline]
    pub fn is_slope_too_steep(&self, normal: Vec3A) -> bool {
        self.as_ref().IsSlopeTooSteep(normal.into())
    }

    #[inline]
    pub fn get_ground_state(&self) -> GroundState {
        self.as_ref().GetGroundState()
    }

    #[inline]
    pub fn is_supported(&self) -> bool {
        self.as_ref().IsSupported()
    }

    #[inline]
    pub fn get_shape(&self) -> RefShape {
        unsafe { RefShape::new(self.as_ref().GetShape() as *mut _) }
    }

    #[inline]
    pub fn get_ground_position(&self) -> Vec3A {
        self.as_ref().GetGroundPosition().0
    }

    #[inline]
    pub fn get_ground_normal(&self) -> Vec3A {
        self.as_ref().GetGroundNormal().0
    }

    #[inline]
    pub fn get_ground_velocity(&self) -> Vec3A {
        self.as_ref().GetGroundVelocity().0
    }

    #[inline]
    pub fn get_ground_body_id(&self) -> BodyID {
        self.as_ref().GetGroundBodyID()
    }

    #[inline]
    pub fn get_linear_velocity(&self) -> Vec3A {
        self.as_ref().GetLinearVelocity().0
    }

    #[inline]
    pub fn set_linear_velocity(&mut self, velocity: Vec3A) {
        self.as_mut().SetLinearVelocity(velocity.into());
    }

    #[inline]
    pub fn get_position(&self) -> Vec3A {
        self.as_ref().GetPosition().0
    }

    #[inline]
    pub fn set_position(&mut self, position: Vec3A) {
        self.as_mut().SetPosition(position.into());
    }

    #[inline]
    pub fn get_rotation(&self) -> Quat {
        self.as_ref().GetRotation().0
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.as_mut().SetRotation(rotation.into());
    }

    #[inline]
    pub fn get_world_transform(&self) -> Mat4 {
        self.as_ref().GetWorldTransform().0
    }

    #[inline]
    pub fn get_center_of_mass_transform(&self) -> Mat4 {
        self.as_ref().GetCenterOfMassTransform().0
    }

    #[inline]
    pub fn get_mass(&self) -> f32 {
        self.as_ref().GetMass()
    }

    #[inline]
    pub fn set_mass(&mut self, mass: f32) {
        self.as_mut().SetMass(mass);
    }

    #[inline]
    pub fn get_max_strength(&self) -> f32 {
        self.as_ref().GetMaxStrength()
    }

    #[inline]
    pub fn set_max_strength(&mut self, max_strength: f32) {
        self.as_mut().SetMaxStrength(max_strength);
    }

    #[inline]
    pub fn get_penetration_recovery_speed(&self) -> f32 {
        self.as_ref().GetPenetrationRecoverySpeed()
    }

    #[inline]
    pub fn set_penetration_recovery_speed(&mut self, speed: f32) {
        self.as_mut().SetPenetrationRecoverySpeed(speed);
    }

    #[inline]
    pub fn get_character_padding(&self) -> f32 {
        self.as_ref().GetCharacterPadding()
    }

    #[inline]
    pub fn get_max_num_hits(&self) -> u32 {
        self.as_ref().GetMaxNumHits()
    }

    #[inline]
    pub fn set_max_num_hits(&mut self, max_hits: u32) {
        self.as_mut().SetMaxNumHits(max_hits);
    }

    #[inline]
    pub fn get_hit_reduction_cos_max_angle(&self) -> f32 {
        self.as_ref().GetHitReductionCosMaxAngle()
    }

    #[inline]
    pub fn set_hit_reduction_cos_max_angle(&mut self, cos_max_angle: f32) {
        self.as_mut().SetHitReductionCosMaxAngle(cos_max_angle);
    }

    #[inline]
    pub fn get_max_hits_exceeded(&self) -> bool {
        self.as_ref().GetMaxHitsExceeded()
    }

    #[inline]
    pub fn get_shape_offset(&self) -> Vec3A {
        self.as_ref().GetShapeOffset().0
    }

    #[inline]
    pub fn set_shape_offset(&mut self, offset: Vec3A) {
        self.as_mut().SetShapeOffset(offset.into());
    }

    #[inline]
    pub fn cancel_velocity_towards_steep_slopes(&self, desired_velocity: Vec3A) -> Vec3A {
        self.as_ref()
            .CancelVelocityTowardsSteepSlopes(desired_velocity.into())
            .0
    }

    #[inline]
    pub fn update(&mut self, chara_layer: u16, delta_time: f32, gravity: Vec3A) {
        self.as_mut().Update(chara_layer, delta_time, gravity.into());
    }

    #[inline]
    pub fn can_walk_stairs(&self, velocity: Vec3A) -> bool {
        self.as_ref().CanWalkStairs(velocity.into())
    }

    #[inline]
    pub fn walk_stairs(
        &mut self,
        chara_layer: u16,
        delta_time: f32,
        step_up: Vec3A,
        step_forward: Vec3A,
        step_forward_test: Vec3A,
        step_down_extra: Vec3A,
    ) -> bool {
        self.as_mut().WalkStairs(
            chara_layer,
            delta_time,
            step_up.into(),
            step_forward.into(),
            step_forward_test.into(),
            step_down_extra.into(),
        )
    }

    #[inline]
    pub fn stick_to_floor(&mut self, chara_layer: u16, step_down: Vec3A) -> bool {
        self.as_mut().StickToFloor(chara_layer, step_down.into())
    }

    #[inline]
    pub fn extended_update(
        &mut self,
        chara_layer: u16,
        delta_time: f32,
        gravity: Vec3A,
        settings: &ExtendedUpdateSettings,
    ) {
        self.as_mut()
            .ExtendedUpdate(chara_layer, delta_time, gravity.into(), unsafe {
                mem::transmute::<&ExtendedUpdateSettings, &ffi::ExtendedUpdateSettings>(settings)
            });
    }

    #[inline]
    pub fn refresh_contacts(&mut self, chara_layer: u16) {
        self.as_mut().RefreshContacts(chara_layer);
    }

    #[inline]
    pub fn update_ground_velocity(&mut self) {
        self.as_mut().UpdateGroundVelocity();
    }

    #[inline]
    pub fn set_shape(&mut self, chara_layer: u16, shape: &mut RefShape, max_penetration_depth: f32) -> bool {
        unsafe {
            self.as_mut()
                .SetShape(chara_layer, shape.as_ptr(), max_penetration_depth)
        }
    }
}
