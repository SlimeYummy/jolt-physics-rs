use core::fmt;
use cxx::{kind, type_id, ExternType};
use glam::{Mat4, Quat, Vec3, Vec3A};
use jolt_macros::vtable;
use static_assertions::const_assert_eq;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::NonNull;
use std::{mem, ptr};

use crate::base::{BodyID, JMut, JQuat, JRef, JRefTarget, JVec3, ObjectLayer, Plane, SubShapeID};
use crate::body::Body;
use crate::shape::{PhysicsMaterial, Shape};
use crate::system::{BodyActivationListener, ContactListener, PhysicsSystem};
use crate::vtable::{VBox, VPair};
use crate::JMutTarget;

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
        type SubShapeID = crate::base::ffi::SubShapeID;
        type Activation = crate::system::ffi::Activation;
        type Shape = crate::shape::ffi::Shape;
        type PhysicsMaterial = crate::shape::ffi::PhysicsMaterial;
        type XPhysicsSystem = crate::system::ffi::XPhysicsSystem;

        type GroundState;
        type BackFaceMode;
        type XCharacterSettings = crate::character::CharacterSettings;
        type XCharacterVirtualSettings = crate::character::CharacterVirtualSettings;
        #[allow(dead_code)]
        type CharacterContactSettings = crate::character::CharacterContactSettings;
        type ExtendedUpdateSettings = crate::character::ExtendedUpdateSettings;
        type CharacterContactListener;

        type XCharacter;
        unsafe fn CreateCharacter(
            system: *mut XPhysicsSystem,
            settings: &XCharacterSettings,
            position: Vec3,
            rotation: Quat,
            user_data: u64,
        ) -> *mut XCharacter;
        unsafe fn CreateAddCharacter(
            system: *mut XPhysicsSystem,
            settings: &XCharacterSettings,
            position: Vec3,
            rotation: Quat,
            user_data: u64,
            activation: Activation,
            lock: bool,
        ) -> *mut XCharacter;
        unsafe fn DropXCharacter(character: *mut XCharacter);
        unsafe fn CloneXCharacter(character: *mut XCharacter) -> *mut XCharacter;
        unsafe fn CountRefXCharacter(character: *const XCharacter) -> u32;

        fn SetMaxSlopeAngle(self: Pin<&mut XCharacter>, angle: f32);
        fn GetCosMaxSlopeAngle(self: &XCharacter) -> f32;
        fn SetUp(self: Pin<&mut XCharacter>, up: Vec3);
        fn GetUp(self: &XCharacter) -> Vec3;
        fn IsSlopeTooSteep(self: &XCharacter, normal: Vec3) -> bool;
        unsafe fn GetShape(self: &XCharacter) -> *const Shape;
        fn GetGroundState(self: &XCharacter) -> GroundState;
        fn IsSupported(self: &XCharacter) -> bool;
        fn GetGroundPosition(self: &XCharacter) -> Vec3;
        fn GetGroundNormal(self: &XCharacter) -> Vec3;
        fn GetGroundVelocity(self: &XCharacter) -> Vec3;
        fn GetGroundMaterial(self: &XCharacter) -> *const PhysicsMaterial;
        fn GetGroundBodyID(self: &XCharacter) -> BodyID;
        fn GetGroundSubShapeID(self: &XCharacter) -> SubShapeID;
        fn GetGroundUserData(self: &XCharacter) -> u64;

        fn AddToPhysicsSystem(self: Pin<&mut XCharacter>, activation: Activation, lock: bool);
        fn RemoveFromPhysicsSystem(self: Pin<&mut XCharacter>, lock: bool);
        fn Activate(self: Pin<&mut XCharacter>, lock: bool);
        fn PostSimulation(self: Pin<&mut XCharacter>, max_distance: f32, lock: bool);
        fn SetLinearAndAngularVelocity(self: Pin<&mut XCharacter>, linear: Vec3, angular: Vec3, lock: bool);
        fn GetLinearVelocity(self: &XCharacter, lock: bool) -> Vec3;
        fn SetLinearVelocity(self: Pin<&mut XCharacter>, velocity: Vec3, lock: bool);
        fn AddLinearVelocity(self: Pin<&mut XCharacter>, velocity: Vec3, lock: bool);
        fn AddImpulse(self: Pin<&mut XCharacter>, impulse: Vec3, lock: bool);
        fn GetBodyID(self: &XCharacter) -> BodyID;
        fn GetPositionAndRotation(self: &XCharacter, position: &mut Vec3, rotation: &mut Quat, lock: bool);
        fn SetPositionAndRotation(
            self: &XCharacter,
            position: Vec3,
            rotation: Quat,
            activation: Activation,
            lock: bool,
        );
        fn GetPosition(self: &XCharacter, lock: bool) -> Vec3;
        fn SetPosition(self: Pin<&mut XCharacter>, position: Vec3, activation: Activation, lock: bool);
        fn GetRotation(self: &XCharacter, lock: bool) -> Quat;
        fn SetRotation(self: Pin<&mut XCharacter>, rotation: Quat, activation: Activation, lock: bool);
        fn GetCenterOfMassPosition(self: &XCharacter, lock: bool) -> Vec3;
        fn GetWorldTransform(self: &XCharacter, lock: bool) -> Mat44;
        fn SetLayer(self: Pin<&mut XCharacter>, layer: u32, lock: bool);
        unsafe fn SetShape(
            self: Pin<&mut XCharacter>,
            shape: *const Shape,
            max_penetration_depth: f32,
            lock: bool,
        ) -> bool;
        // GetTransformedShape
        // CheckCollision

        type XCharacterVirtual;
        unsafe fn CreateCharacterVirtual(
            clean_up: fn (zelf: Pin<&mut XCharacterVirtual>),
            system: *mut XPhysicsSystem,
            settings: &XCharacterVirtualSettings,
            position: Vec3,
            rotation: Quat,
        ) -> *mut XCharacterVirtual;
        unsafe fn DropXCharacterVirtual(character: *mut XCharacterVirtual);
        unsafe fn CloneXCharacterVirtual(character: *mut XCharacterVirtual) -> *mut XCharacterVirtual;
        unsafe fn CountRefXCharacterVirtual(character: *const XCharacterVirtual) -> u32;

        fn SetMaxSlopeAngle(self: Pin<&mut XCharacterVirtual>, angle: f32);
        fn GetCosMaxSlopeAngle(self: &XCharacterVirtual) -> f32;
        fn SetUp(self: Pin<&mut XCharacterVirtual>, up: Vec3);
        fn GetUp(self: &XCharacterVirtual) -> Vec3;
        fn IsSlopeTooSteep(self: &XCharacterVirtual, normal: Vec3) -> bool;
        unsafe fn GetShape(self: &XCharacterVirtual) -> *const Shape;
        fn GetGroundState(self: &XCharacterVirtual) -> GroundState;
        fn IsSupported(self: &XCharacterVirtual) -> bool;
        fn GetGroundPosition(self: &XCharacterVirtual) -> Vec3;
        fn GetGroundNormal(self: &XCharacterVirtual) -> Vec3;
        fn GetGroundVelocity(self: &XCharacterVirtual) -> Vec3;
        fn GetGroundMaterial(self: &XCharacterVirtual) -> *const PhysicsMaterial;
        fn GetGroundBodyID(self: &XCharacterVirtual) -> BodyID;
        fn GetGroundSubShapeID(self: &XCharacterVirtual) -> SubShapeID;
        fn GetGroundUserData(self: &XCharacterVirtual) -> u64;

        unsafe fn SetListener(self: Pin<&mut XCharacterVirtual>, listener: *mut CharacterContactListener);
        unsafe fn GetListener(self: &XCharacterVirtual) -> *mut CharacterContactListener;
        // SetCharacterVsCharacterCollision
        fn GetLinearVelocity(self: &XCharacterVirtual) -> Vec3;
        fn SetLinearVelocity(self: Pin<&mut XCharacterVirtual>, velocity: Vec3);
        fn GetPosition(self: &XCharacterVirtual) -> Vec3;
        fn SetPosition(self: Pin<&mut XCharacterVirtual>, position: Vec3);
        fn GetRotation(self: &XCharacterVirtual) -> Quat;
        fn SetRotation(self: Pin<&mut XCharacterVirtual>, rotation: Quat);
        fn GetCenterOfMassPosition(self: &XCharacterVirtual) -> Vec3;
        fn GetWorldTransform(self: &XCharacterVirtual) -> Mat44;
        fn GetCenterOfMassTransform(self: &XCharacterVirtual) -> Mat44;
        fn GetMass(self: &XCharacterVirtual) -> f32;
        fn SetMass(self: Pin<&mut XCharacterVirtual>, mass: f32);
        fn GetMaxStrength(self: &XCharacterVirtual) -> f32;
        fn SetMaxStrength(self: Pin<&mut XCharacterVirtual>, max_strength: f32);
        fn GetPenetrationRecoverySpeed(self: &XCharacterVirtual) -> f32;
        fn SetPenetrationRecoverySpeed(self: Pin<&mut XCharacterVirtual>, speed: f32);
        fn GetEnhancedInternalEdgeRemoval(self: &XCharacterVirtual) -> bool;
        fn SetEnhancedInternalEdgeRemoval(self: Pin<&mut XCharacterVirtual>, enabled: bool);
        fn GetCharacterPadding(self: &XCharacterVirtual) -> f32;
        fn GetMaxNumHits(self: &XCharacterVirtual) -> u32;
        fn SetMaxNumHits(self: Pin<&mut XCharacterVirtual>, max_hits: u32);
        fn GetHitReductionCosMaxAngle(self: &XCharacterVirtual) -> f32;
        fn SetHitReductionCosMaxAngle(self: Pin<&mut XCharacterVirtual>, cos_max_angle: f32);
        fn GetMaxHitsExceeded(self: &XCharacterVirtual) -> bool;
        fn GetShapeOffset(self: &XCharacterVirtual) -> Vec3;
        fn SetShapeOffset(self: Pin<&mut XCharacterVirtual>, offset: Vec3);
        fn GetUserData(self: &XCharacterVirtual) -> u64;
        fn SetUserData(self: Pin<&mut XCharacterVirtual>, user_data: u64);
        fn GetInnerBodyID(self: &XCharacterVirtual) -> BodyID;
        fn CancelVelocityTowardsSteepSlopes(self: &XCharacterVirtual, desired_velocity: Vec3) -> Vec3;
        fn Update(self: Pin<&mut XCharacterVirtual>, chara_layer: u32, delta_time: f32, gravity: Vec3);
        fn CanWalkStairs(self: &XCharacterVirtual, velocity: Vec3) -> bool;
        fn WalkStairs(
            self: Pin<&mut XCharacterVirtual>,
            chara_layer: u32,
            delta_time: f32,
            step_up: Vec3,
            step_forward: Vec3,
            step_forward_test: Vec3,
            step_down_extra: Vec3,
        ) -> bool;
        fn StickToFloor(self: Pin<&mut XCharacterVirtual>, chara_layer: u32, step_down: Vec3) -> bool;
        fn ExtendedUpdate(
            self: Pin<&mut XCharacterVirtual>,
            chara_layer: u32,
            delta_time: f32,
            gravity: Vec3,
            settings: &ExtendedUpdateSettings,
        );
        fn RefreshContacts(self: Pin<&mut XCharacterVirtual>, chara_layer: u32);
        fn UpdateGroundVelocity(self: Pin<&mut XCharacterVirtual>);
        unsafe fn SetShape(
            self: Pin<&mut XCharacterVirtual>,
            chara_layer: u32,
            shape: *const Shape,
            max_penetration_depth: f32,
        ) -> bool;
        unsafe fn SetInnerBodyShape(self: Pin<&mut XCharacterVirtual>, shape: *const Shape);
        // GetTransformedShape
        // CheckCollision
    }
}

pub type GroundState = ffi::GroundState;
pub type BackFaceMode = ffi::BackFaceMode;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CharacterSettings {
    up: Vec3A,
    supporting_volume: Plane,
    max_slope_angle: f32,
    shape: Option<JRef<Shape>>,
    layer: ObjectLayer,
    mass: f32,
    friction: f32,
    gravity_factor: f32,
}
const_assert_eq!(mem::size_of::<CharacterSettings>(), 64);

unsafe impl ExternType for CharacterSettings {
    type Id = type_id!("XCharacterSettings");
    type Kind = kind::Trivial;
}

impl Default for CharacterSettings {
    fn default() -> CharacterSettings {
        CharacterSettings {
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

impl CharacterSettings {
    pub fn new(shape: JRef<Shape>, layer: ObjectLayer) -> CharacterSettings {
        CharacterSettings {
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
    shape: Option<JRef<Shape>>,
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

unsafe impl ExternType for CharacterVirtualSettings {
    type Id = type_id!("XCharacterVirtualSettings");
    type Kind = kind::Trivial;
}

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
    pub fn new(shape: JRef<Shape>) -> CharacterVirtualSettings {
        CharacterVirtualSettings {
            shape: Some(shape),
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterContactSettings {
    pub can_push_character: bool,
    pub can_receive_impulses: bool,
}
const_assert_eq!(std::mem::size_of::<CharacterContactSettings>(), 2);

unsafe impl ExternType for CharacterContactSettings {
    type Id = type_id!("CharacterContactSettings");
    type Kind = kind::Trivial;
}

impl Default for CharacterContactSettings {
    #[inline]
    fn default() -> CharacterContactSettings {
        CharacterContactSettings {
            can_push_character: true,
            can_receive_impulses: true,
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

unsafe impl ExternType for ExtendedUpdateSettings {
    type Id = type_id!("ExtendedUpdateSettings");
    type Kind = kind::Trivial;
}

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

pub struct Character(pub(crate) ffi::XCharacter);

impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Character")
            .field("body_id", &self.get_body_id())
            .field("position", &self.get_position(false))
            .field("rotation", &self.get_rotation(false))
            .field("up", &self.get_up())
            .field("cos_max_slope_angle", &self.get_cos_max_slope_angle())
            .field("shape", &self.get_shape())
            .finish()
    }
}

unsafe impl JRefTarget for Character {
    type JRaw = NonNull<Character>;

    #[inline]
    fn name() -> &'static str {
        "Character"
    }

    #[inline]
    unsafe fn make_ref(raw: &Self::JRaw) -> &Self {
        unsafe { raw.as_ref() }
    }

    #[inline]
    unsafe fn clone_raw(raw: &Self::JRaw) -> Self::JRaw {
        NonNull::new_unchecked(ffi::CloneXCharacter(raw.as_ptr() as *mut _) as *mut _)
    }

    #[inline]
    unsafe fn drop_raw(raw: &mut Self::JRaw) {
        ffi::DropXCharacter(raw.as_ptr() as *mut _);
    }

    #[inline]
    unsafe fn count_ref(raw: &Self::JRaw) -> u32 {
        unsafe { ffi::CountRefXCharacter(raw.as_ptr() as *const _) }
    }
}

unsafe impl JMutTarget for Character {
    #[inline]
    unsafe fn make_mut(raw: &mut Self::JRaw) -> &mut Self {
        unsafe { raw.as_mut() }
    }

    #[inline]
    unsafe fn steal_raw(raw: &Self::JRaw) -> Self::JRaw {
        *raw
    }
}

impl JMut<Character> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut ffi::XCharacter) -> JMut<Character> {
        JMut(unsafe { NonNull::new_unchecked(raw as *mut _) })
    }
}

impl Character {
    pub fn new<CL: ContactListener, BAL: BodyActivationListener>(
        system: &mut PhysicsSystem<CL, BAL>,
        settings: &CharacterSettings,
        position: Vec3A,
        rotation: Quat,
        user_data: u64,
    ) -> JMut<Character> {
        unsafe {
            JMut::<Character>::new_unchecked(ffi::CreateCharacter(
                system.as_x_ptr(),
                mem::transmute::<&CharacterSettings, &ffi::XCharacterSettings>(settings),
                position.into(),
                rotation.into(),
                user_data,
            ))
        }
    }

    pub fn new_add<CL: ContactListener, BAL: BodyActivationListener>(
        system: &mut PhysicsSystem<CL, BAL>,
        settings: &CharacterSettings,
        position: Vec3A,
        rotation: Quat,
        user_data: u64,
        active: bool,
        lock: bool,
    ) -> JMut<Character> {
        unsafe {
            JMut::<Character>::new_unchecked(ffi::CreateAddCharacter(
                system.as_x_ptr(),
                mem::transmute::<&CharacterSettings, &ffi::XCharacterSettings>(settings),
                position.into(),
                rotation.into(),
                user_data,
                active.into(),
                lock,
            ))
        }
    }

    #[inline]
    fn as_ref(&self) -> &ffi::XCharacter {
        &self.0
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::XCharacter> {
        unsafe { Pin::new_unchecked(&mut self.0) }
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
        self.as_ref().GetUp().into()
    }

    #[inline]
    pub fn is_slope_too_steep(&self, normal: Vec3A) -> bool {
        self.as_ref().IsSlopeTooSteep(normal.into())
    }

    #[inline]
    pub fn get_shape(&self) -> &Shape {
        unsafe { &*Shape::cast_ptr(self.as_ref().GetShape()) }
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
    pub fn get_ground_position(&self) -> Vec3A {
        self.as_ref().GetGroundPosition().into()
    }

    #[inline]
    pub fn get_ground_normal(&self) -> Vec3A {
        self.as_ref().GetGroundNormal().into()
    }

    #[inline]
    pub fn get_ground_velocity(&self) -> Vec3A {
        self.as_ref().GetGroundVelocity().into()
    }

    #[inline]
    pub fn get_ground_material(&self) -> &PhysicsMaterial {
        unsafe { &*PhysicsMaterial::cast_ptr(self.as_ref().GetGroundMaterial()) }
    }

    #[inline]
    pub fn get_ground_body_id(&self) -> BodyID {
        self.as_ref().GetGroundBodyID()
    }

    #[inline]
    pub fn get_ground_sub_shape_id(&self) -> SubShapeID {
        self.as_ref().GetGroundSubShapeID()
    }

    #[inline]
    pub fn get_ground_user_data(&self) -> u64 {
        self.as_ref().GetGroundUserData()
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
    pub fn get_linear_velocity(&self, lock: bool) -> Vec3A {
        self.as_ref().GetLinearVelocity(lock).into()
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
    pub fn get_body_id(&self) -> BodyID {
        self.as_ref().GetBodyID()
    }

    #[inline]
    pub fn get_position_and_rotation(&self, lock: bool) -> (Vec3A, Quat) {
        let mut position = JVec3::default();
        let mut rotation = JQuat::default();
        self.as_ref().GetPositionAndRotation(&mut position, &mut rotation, lock);
        (position.into(), rotation.into())
    }

    #[inline]
    pub fn set_position_and_rotation(&mut self, position: Vec3A, rotation: Quat, active: bool, lock: bool) {
        self.as_mut()
            .SetPositionAndRotation(position.into(), rotation.into(), active.into(), lock);
    }

    #[inline]
    pub fn get_position(&self, lock: bool) -> Vec3A {
        self.as_ref().GetPosition(lock).into()
    }

    #[inline]
    pub fn set_position(&mut self, position: Vec3A, active: bool, lock: bool) {
        self.as_mut().SetPosition(position.into(), active.into(), lock);
    }

    #[inline]
    pub fn get_rotation(&self, lock: bool) -> Quat {
        self.as_ref().GetRotation(lock).into()
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Quat, active: bool, lock: bool) {
        self.as_mut().SetRotation(rotation.into(), active.into(), lock);
    }

    #[inline]
    pub fn get_center_of_mass_position(&self, lock: bool) -> Vec3A {
        self.as_ref().GetCenterOfMassPosition(lock).into()
    }

    #[inline]
    pub fn get_world_transform(&self, lock: bool) -> Mat4 {
        self.as_ref().GetWorldTransform(lock).into()
    }

    #[inline]
    pub fn set_layer(&mut self, layer: ObjectLayer, lock: bool) {
        self.as_mut().SetLayer(layer, lock);
    }

    #[inline]
    pub fn set_shape(&mut self, shape: &Shape, max_penetration_depth: f32, lock: bool) -> bool {
        unsafe { self.as_mut().SetShape(&shape.0, max_penetration_depth, lock) }
    }
}

pub struct CharacterVirtual<CCL: CharacterContactListener = ()> {
    pub(crate) character: ffi::XCharacterVirtual,
    _ccl_phantom: PhantomData<CCL>,
}

impl<CCL: CharacterContactListener> fmt::Debug for CharacterVirtual<CCL> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharacterVirtual")
            .field("position", &self.get_position())
            .field("rotation", &self.get_rotation())
            .field("up", &self.get_up())
            .field("cos_max_slope_angle", &self.get_cos_max_slope_angle())
            .field("shape", &self.get_shape())
            .field("user_data", &self.get_user_data())
            .field("inner_body_id", &self.get_inner_body_id())
            .finish()
    }
}

unsafe impl<CCL: CharacterContactListener> JRefTarget for CharacterVirtual<CCL> {
    type JRaw = NonNull<CharacterVirtual<CCL>>;

    #[inline]
    fn name() -> &'static str {
        "CharacterVirtual"
    }

    #[inline]
    unsafe fn make_ref(raw: &Self::JRaw) -> &Self {
        unsafe { raw.as_ref() }
    }

    #[inline]
    unsafe fn clone_raw(raw: &Self::JRaw) -> Self::JRaw {
        NonNull::new_unchecked(ffi::CloneXCharacterVirtual(raw.as_ptr() as *mut _) as *mut _)
    }

    #[inline]
    unsafe fn drop_raw(raw: &mut Self::JRaw) {
        ffi::DropXCharacterVirtual(raw.as_ptr() as *mut _);
    }

    #[inline]
    unsafe fn count_ref(raw: &Self::JRaw) -> u32 {
        unsafe { ffi::CountRefXCharacterVirtual(raw.as_ptr() as *const _) }
    }
}

unsafe impl<CCL: CharacterContactListener> JMutTarget for CharacterVirtual<CCL> {
    #[inline]
    unsafe fn make_mut(raw: &mut Self::JRaw) -> &mut Self {
        unsafe { raw.as_mut() }
    }

    #[inline]
    unsafe fn steal_raw(raw: &Self::JRaw) -> Self::JRaw {
        *raw
    }
}

impl<CCL: CharacterContactListener> JMut<CharacterVirtual<CCL>> {
    #[inline]
    pub(crate) unsafe fn new_unchecked(raw: *mut ffi::XCharacterVirtual) -> JMut<CharacterVirtual<CCL>> {
        JMut(unsafe { NonNull::new_unchecked(raw as *mut _) })
    }
}

impl<CCL: CharacterContactListener> CharacterVirtual<CCL> {
    pub fn new<CL: ContactListener, BAL: BodyActivationListener>(
        system: &mut PhysicsSystem<CL, BAL>,
        settings: &CharacterVirtualSettings,
        position: Vec3A,
        rotation: Quat,
    ) -> JMut<CharacterVirtual<CCL>> {
        unsafe {
            JMut::<CharacterVirtual<CCL>>::new_unchecked(ffi::CreateCharacterVirtual(
                Self::clean_up,
                system.as_x_ptr(),
                mem::transmute::<&CharacterVirtualSettings, &ffi::XCharacterVirtualSettings>(settings),
                position.into(),
                rotation.into(),
            ))
        }
    }

    fn clean_up(zelf: Pin<&mut ffi::XCharacterVirtual>) {
        unsafe {
            let ptr = zelf.GetListener();
            if !ptr.is_null() {
                let _ = VBox::<CCL, CharacterContactListenerVTable>::from_raw(ptr as *mut _);
            }
            zelf.SetListener(ptr::null_mut());
        }

        #[cfg(feature = "debug-print")]
        println!("CharacterVirtual::clean_up called");
    }

    #[inline]
    fn as_ref(&self) -> &ffi::XCharacterVirtual {
        &self.character
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::XCharacterVirtual> {
        unsafe { Pin::new_unchecked(&mut self.character) }
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
        self.as_ref().GetUp().into()
    }

    #[inline]
    pub fn is_slope_too_steep(&self, normal: Vec3A) -> bool {
        self.as_ref().IsSlopeTooSteep(normal.into())
    }

    #[inline]
    pub fn get_shape(&self) -> &Shape {
        unsafe { &*Shape::cast_ptr(self.as_ref().GetShape()) }
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
    pub fn get_ground_position(&self) -> Vec3A {
        self.as_ref().GetGroundPosition().into()
    }

    #[inline]
    pub fn get_ground_normal(&self) -> Vec3A {
        self.as_ref().GetGroundNormal().into()
    }

    #[inline]
    pub fn get_ground_velocity(&self) -> Vec3A {
        self.as_ref().GetGroundVelocity().into()
    }

    #[inline]
    pub fn get_ground_material(&self) -> &PhysicsMaterial {
        unsafe { &*PhysicsMaterial::cast_ptr(self.as_ref().GetGroundMaterial()) }
    }

    #[inline]
    pub fn get_ground_body_id(&self) -> BodyID {
        self.as_ref().GetGroundBodyID()
    }

    #[inline]
    pub fn get_ground_sub_shape_id(&self) -> SubShapeID {
        self.as_ref().GetGroundSubShapeID()
    }

    #[inline]
    pub fn get_ground_user_data(&self) -> u64 {
        self.as_ref().GetGroundUserData()
    }

    #[inline]
    pub fn set_listener(&mut self, listener: Option<VBox<CCL, CharacterContactListenerVTable>>) {
        unsafe {
            let old = self.as_ref().GetListener() as *mut u8;
            if !old.is_null() {
                let _ = VBox::<CCL, CharacterContactListenerVTable>::from_raw(old as *mut _);
            }
            if let Some(listener) = listener {
                self.as_mut()
                    .SetListener(VBox::<CCL, CharacterContactListenerVTable>::into_raw(listener) as *mut _);
            } else {
                self.as_mut().SetListener(ptr::null_mut());
            }
        };
    }

    #[inline]
    pub fn get_listener(&self) -> Option<&VPair<CCL, CharacterContactListenerVTable>> {
        unsafe {
            let current = self.as_ref().GetListener() as *const u8;
            match current.is_null() {
                true => None,
                false => Some(&*(current as *const _)),
            }
        }
    }

    #[inline]
    pub fn get_listener_mut(&mut self) -> Option<&mut VPair<CCL, CharacterContactListenerVTable>> {
        unsafe {
            let current = self.as_ref().GetListener() as *mut u8;
            match current.is_null() {
                true => None,
                false => Some(&mut *(current as *mut _)),
            }
        }
    }

    #[inline]
    pub fn get_linear_velocity(&self) -> Vec3A {
        self.as_ref().GetLinearVelocity().into()
    }

    #[inline]
    pub fn set_linear_velocity(&mut self, velocity: Vec3A) {
        self.as_mut().SetLinearVelocity(velocity.into());
    }

    #[inline]
    pub fn get_position(&self) -> Vec3A {
        self.as_ref().GetPosition().into()
    }

    #[inline]
    pub fn set_position(&mut self, position: Vec3A) {
        self.as_mut().SetPosition(position.into());
    }

    #[inline]
    pub fn get_rotation(&self) -> Quat {
        self.as_ref().GetRotation().into()
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.as_mut().SetRotation(rotation.into());
    }

    #[inline]
    pub fn get_center_of_mass_position(&self) -> Vec3A {
        self.as_ref().GetCenterOfMassPosition().into()
    }

    #[inline]
    pub fn get_world_transform(&self) -> Mat4 {
        self.as_ref().GetWorldTransform().into()
    }

    #[inline]
    pub fn get_center_of_mass_transform(&self) -> Mat4 {
        self.as_ref().GetCenterOfMassTransform().into()
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
    pub fn get_enhanced_internal_edge_removal(&self) -> bool {
        self.as_ref().GetEnhancedInternalEdgeRemoval()
    }

    #[inline]
    pub fn set_enhanced_internal_edge_removal(&mut self, enabled: bool) {
        self.as_mut().SetEnhancedInternalEdgeRemoval(enabled);
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
        self.as_ref().GetShapeOffset().into()
    }

    #[inline]
    pub fn set_shape_offset(&mut self, offset: Vec3A) {
        self.as_mut().SetShapeOffset(offset.into());
    }

    #[inline]
    pub fn get_user_data(&self) -> u64 {
        self.as_ref().GetUserData()
    }

    #[inline]
    pub fn set_user_data(&mut self, user_data: u64) {
        self.as_mut().SetUserData(user_data);
    }

    #[inline]
    pub fn get_inner_body_id(&self) -> BodyID {
        self.as_ref().GetInnerBodyID()
    }

    #[inline]
    pub fn cancel_velocity_towards_steep_slopes(&self, desired_velocity: Vec3A) -> Vec3A {
        self.as_ref()
            .CancelVelocityTowardsSteepSlopes(desired_velocity.into())
            .into()
    }

    #[inline]
    pub fn update(&mut self, chara_layer: ObjectLayer, delta_time: f32, gravity: Vec3A) {
        self.as_mut().Update(chara_layer, delta_time, gravity.into());
    }

    #[inline]
    pub fn can_walk_stairs(&self, velocity: Vec3A) -> bool {
        self.as_ref().CanWalkStairs(velocity.into())
    }

    #[inline]
    pub fn walk_stairs(
        &mut self,
        chara_layer: ObjectLayer,
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
    pub fn stick_to_floor(&mut self, chara_layer: ObjectLayer, step_down: Vec3A) -> bool {
        self.as_mut().StickToFloor(chara_layer, step_down.into())
    }

    #[inline]
    pub fn extended_update(
        &mut self,
        chara_layer: ObjectLayer,
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
    pub fn refresh_contacts(&mut self, chara_layer: ObjectLayer) {
        self.as_mut().RefreshContacts(chara_layer);
    }

    #[inline]
    pub fn update_ground_velocity(&mut self) {
        self.as_mut().UpdateGroundVelocity();
    }

    #[inline]
    pub fn set_shape(&mut self, chara_layer: ObjectLayer, shape: &Shape, max_penetration_depth: f32) -> bool {
        unsafe { self.as_mut().SetShape(chara_layer, &shape.0, max_penetration_depth) }
    }

    #[inline]
    pub fn set_inner_body_shape(&mut self, shape: &Shape) {
        unsafe { self.as_mut().SetInnerBodyShape(&shape.0) };
    }
}

#[vtable(allow_empty)]
#[repr(C)]
pub struct CharacterContactListenerVTable {
    pub drop: extern "C" fn(*mut u8),
    pub on_adjust_body_velocity: extern "C" fn(
        *mut u8,
        character: &CharacterVirtual<()>,
        body2: &Body,
        linear_velocity: &mut Vec3A,
        angular_velocity: &mut Vec3A,
    ),
    pub on_contact_validate:
        extern "C" fn(*mut u8, character: &CharacterVirtual<()>, body2: &BodyID, subshape2: &SubShapeID) -> bool,
    pub on_character_contact_validate: extern "C" fn(
        *mut u8,
        character: &CharacterVirtual<()>,
        other_character: &CharacterVirtual<()>,
        subshape2: &SubShapeID,
    ) -> bool,
    pub on_contact_added: extern "C" fn(
        *mut u8,
        character: &CharacterVirtual<()>,
        body2: &BodyID,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        settings: &mut CharacterContactSettings,
    ),
    pub on_character_contact_added: extern "C" fn(
        *mut u8,
        character: &CharacterVirtual<()>,
        other_character: &CharacterVirtual<()>,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        settings: &mut CharacterContactSettings,
    ),
    pub on_contact_solve: extern "C" fn(
        *mut u8,
        character: &CharacterVirtual<()>,
        body2: &BodyID,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        contact_velocity: JVec3,
        material: &PhysicsMaterial,
        character_velocity: JVec3,
        new_character_velocity: &mut Vec3A,
    ),
    pub on_character_contact_solve: extern "C" fn(
        *mut u8,
        character: &CharacterVirtual<()>,
        other_character: &CharacterVirtual<()>,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        contact_velocity: JVec3,
        material: &PhysicsMaterial,
        character_velocity: JVec3,
        new_character_velocity: &mut Vec3A,
    ),
}
