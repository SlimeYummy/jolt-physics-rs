use core::fmt;
use glam::{Mat4, Quat, Vec3A};
use static_assertions::const_assert_eq;
use std::mem;
use std::pin::Pin;

use crate::base::{
    AABox, AllowedDOFs, BodyID, BodyType, JRef, JRefTarget, MotionQuality, MotionType, OverrideMassProperties,
    SubShapeID, JVec3,
};
use crate::shape::Shape;

#[cxx::bridge()]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type BodyType = crate::base::ffi::BodyType;
        type MotionType = crate::base::ffi::MotionType;
        type CanSleep = crate::base::ffi::CanSleep;

        type Vec3 = crate::base::ffi::Vec3;
        type Quat = crate::base::ffi::Quat;
        type Mat44 = crate::base::ffi::Mat44;
        type AABox = crate::base::ffi::AABox;
        type BodyID = crate::base::ffi::BodyID;
        type SubShapeID = crate::base::ffi::SubShapeID;
        type Shape = crate::shape::ffi::Shape;

        type Body;
        fn GetID(self: &Body) -> &BodyID;
        fn GetBodyType(self: &Body) -> BodyType;
        fn IsRigidBody(self: &Body) -> bool;
        fn IsSoftBody(self: &Body) -> bool;
        fn IsActive(self: &Body) -> bool;
        fn IsStatic(self: &Body) -> bool;
        fn IsKinematic(self: &Body) -> bool;
        fn IsDynamic(self: &Body) -> bool;
        fn CanBeKinematicOrDynamic(self: &Body) -> bool;
        fn SetIsSensor(self: Pin<&mut Body>, inIsSensor: bool);
        fn SetCollideKinematicVsNonDynamic(self: Pin<&mut Body>, inCollide: bool);
        fn GetCollideKinematicVsNonDynamic(self: &Body) -> bool;
        fn SetUseManifoldReduction(self: Pin<&mut Body>, inUseReduction: bool);
        fn GetUseManifoldReduction(self: &Body) -> bool;
        fn GetUseManifoldReductionWithBody(self: &Body, inBody2: &Body) -> bool;
        fn SetApplyGyroscopicForce(self: Pin<&mut Body>, inApply: bool);
        fn GetApplyGyroscopicForce(self: &Body) -> bool;
        fn SetEnhancedInternalEdgeRemoval(self: Pin<&mut Body>, inApply: bool);
        fn GetEnhancedInternalEdgeRemoval(self: &Body) -> bool;
        fn GetEnhancedInternalEdgeRemovalWithBody(self: &Body, inBody2: &Body) -> bool;
        fn GetMotionType(self: &Body) -> MotionType;
        fn SetMotionType(self: Pin<&mut Body>, inType: MotionType);
        // GetBroadPhaseLayer()
        // GetObjectLayer()
        // GetCollisionGroup
        // GetCollisionGroup
        // SetCollisionGroup
        fn GetAllowSleeping(self: &Body) -> bool;
        fn SetAllowSleeping(self: Pin<&mut Body>, inAllow: bool);
        fn ResetSleepTimer(self: Pin<&mut Body>);
        fn GetFriction(self: &Body) -> f32;
        fn SetFriction(self: Pin<&mut Body>, inFriction: f32);
        fn GetRestitution(self: &Body) -> f32;
        fn SetRestitution(self: Pin<&mut Body>, inRestitution: f32);
        fn GetLinearVelocity(self: &Body) -> Vec3;
        fn SetLinearVelocity(self: Pin<&mut Body>, inLinearVelocity: Vec3);
        fn SetLinearVelocityClamped(self: Pin<&mut Body>, inLinearVelocity: Vec3);
        fn GetAngularVelocity(self: &Body) -> Vec3;
        fn SetAngularVelocity(self: Pin<&mut Body>, inAngularVelocity: Vec3);
        fn SetAngularVelocityClamped(self: Pin<&mut Body>, inAngularVelocity: Vec3);
        fn GetPointVelocityCOM(self: &Body, inPoint: Vec3) -> Vec3;
        fn GetPointVelocity(self: &Body, inPoint: Vec3) -> Vec3;
        fn AddForce(self: Pin<&mut Body>, inForce: Vec3);
        #[rust_name = "AddForceEx"]
        fn AddForce(self: Pin<&mut Body>, inForce: Vec3, inPoint: Vec3);
        fn AddTorque(self: Pin<&mut Body>, inTorque: Vec3);
        fn GetAccumulatedForce(self: &Body) -> Vec3;
        fn GetAccumulatedTorque(self: &Body) -> Vec3;
        fn ResetForce(self: Pin<&mut Body>);
        fn ResetMotion(self: Pin<&mut Body>);
        fn GetInverseInertia(self: &Body) -> Mat44;
        fn AddImpulse(self: Pin<&mut Body>, inImpulse: Vec3);
        #[rust_name = "AddImpulseEx"]
        fn AddImpulse(self: Pin<&mut Body>, inImpulse: Vec3, inPosition: Vec3);
        fn AddAngularImpulse(self: Pin<&mut Body>, inAngularImpulse: Vec3);
        fn MoveKinematic(self: Pin<&mut Body>, inTargetPosition: Vec3, inTargetRotation: Quat, inDeltaTime: f32);
        fn GetSubmergedVolume(
            self: &Body,
            inSurfacePosition: Vec3,
            inSurfaceNormal: Vec3,
            outTotalVolume: &mut f32,
            outSubmergedVolume: &mut f32,
            outRelativeCenterOfBuoyancy: &mut Vec3,
        );
        fn ApplyBuoyancyImpulse(
            self: Pin<&mut Body>,
            inSurfacePosition: Vec3,
            inSurfaceNormal: Vec3,
            inBuoyancy: f32,
            inLinearDrag: f32,
            inAngularDrag: f32,
            inFluidVelocity: Vec3,
            inGravity: Vec3,
            inDeltaTime: f32,
        ) -> bool;
        #[rust_name = "ApplyBuoyancyImpulseEx"]
        fn ApplyBuoyancyImpulse(
            self: Pin<&mut Body>,
            inTotalVolume: f32,
            inSubmergedVolume: f32,
            inRelativeCenterOfBuoyancy: Vec3,
            inBuoyancy: f32,
            inLinearDrag: f32,
            inAngularDrag: f32,
            inFluidVelocity: Vec3,
            inGravity: Vec3,
            inDeltaTime: f32,
        ) -> bool;
        fn IsInBroadPhase(self: &Body) -> bool;
        fn IsCollisionCacheInvalid(self: &Body) -> bool;
        fn GetShape(self: &Body) -> *const Shape;
        fn GetPosition(self: &Body) -> Vec3;
        fn GetRotation(self: &Body) -> Quat;
        fn GetWorldTransform(self: &Body) -> Mat44;
        fn GetCenterOfMassPosition(self: &Body) -> Vec3;
        fn GetCenterOfMassTransform(self: &Body) -> Mat44;
        fn GetInverseCenterOfMassTransform(self: &Body) -> Mat44;
        fn GetWorldSpaceBounds(self: &Body) -> &AABox;
        // GetMotionProperties
        // GetMotionPropertiesUnchecked
        fn GetUserData(self: &Body) -> u64;
        fn SetUserData(self: Pin<&mut Body>, inUserData: u64);
        fn GetWorldSpaceSurfaceNormal(self: &Body, inSubShapeID: &SubShapeID, inPosition: Vec3) -> Vec3;
        // GetTransformedShape
        // GetBodyCreationSettings
        // GetSoftBodyCreationSettings
        // sFindCollidingPairsCanCollide
        fn AddPositionStep(self: Pin<&mut Body>, inLinearVelocityTimesDeltaTime: Vec3);
        fn SubPositionStep(self: Pin<&mut Body>, inLinearVelocityTimesDeltaTime: Vec3);
        fn AddRotationStep(self: Pin<&mut Body>, inAngularVelocityTimesDeltaTime: Vec3);
        fn SubRotationStep(self: Pin<&mut Body>, inAngularVelocityTimesDeltaTime: Vec3);
        fn SetInBroadPhaseInternal(self: Pin<&mut Body>, inInBroadPhase: bool);
        fn InvalidateContactCacheInternal(self: Pin<&mut Body>) -> bool;
        fn ValidateContactCacheInternal(self: Pin<&mut Body>);
        fn CalculateWorldSpaceBoundsInternal(self: Pin<&mut Body>);
        fn SetPositionAndRotationInternal(
            self: Pin<&mut Body>,
            inPosition: Vec3,
            inRotation: Quat,
            inResetSleepTimer: bool,
        );
        fn UpdateCenterOfMassInternal(self: Pin<&mut Body>, inPreviousCenterOfMass: Vec3, inUpdateMassProperties: bool);
        unsafe fn SetShapeInternal(self: Pin<&mut Body>, inShape: *const Shape, inUpdateMassProperties: bool);
        fn GetIndexInActiveBodiesInternal(self: &Body) -> u32;
        fn UpdateSleepStateInternal(
            self: Pin<&mut Body>,
            inDeltaTime: f32,
            inMaxMovement: f32,
            inTimeBeforeSleep: f32,
        ) -> CanSleep;
        // SaveState
        // RestoreState
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
pub struct BodyCreationSettings {
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
    pub shape: Option<JRef<Shape>>,
}
const_assert_eq!(mem::size_of::<BodyCreationSettings>(), 256);

impl Default for BodyCreationSettings {
    fn default() -> BodyCreationSettings {
        BodyCreationSettings {
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

impl BodyCreationSettings {
    pub fn new(
        shape: JRef<Shape>,
        layer: u16,
        motion_type: MotionType,
        position: Vec3A,
        rotation: Quat,
    ) -> BodyCreationSettings {
        BodyCreationSettings {
            position,
            rotation,
            object_layer: layer,
            motion_type,
            shape: Some(shape),
            ..Default::default()
        }
    }

    pub fn new_static(shape: JRef<Shape>, layer: u16, position: Vec3A, rotation: Quat) -> BodyCreationSettings {
        BodyCreationSettings {
            position,
            rotation,
            object_layer: layer,
            motion_type: MotionType::Static,
            shape: Some(shape),
            ..Default::default()
        }
    }

    pub fn new_sensor(
        shape: JRef<Shape>,
        layer: u16,
        motion_type: MotionType,
        position: Vec3A,
        rotation: Quat,
    ) -> BodyCreationSettings {
        BodyCreationSettings {
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

pub struct Body(ffi::Body);

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Body")
            .field("body_id", &self.get_id())
            .field("body_type", &self.get_body_type())
            .field("user_data", &self.get_user_data())
            .finish()
    }
}

impl Body {
    #[inline]
    fn as_ref(&self) -> &ffi::Body {
        &self.0
    }

    #[inline]
    fn as_mut(&mut self) -> Pin<&mut ffi::Body> {
        unsafe { Pin::new_unchecked(&mut self.0) }
    }

    #[inline]
    pub fn get_id(&self) -> BodyID {
        *self.as_ref().GetID()
    }

    #[inline]
    pub fn get_body_type(&self) -> BodyType {
        self.as_ref().GetBodyType()
    }

    #[inline]
    pub fn is_rigid_body(&self) -> bool {
        self.as_ref().IsRigidBody()
    }

    #[inline]
    pub fn is_soft_body(&self) -> bool {
        self.as_ref().IsSoftBody()
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.as_ref().IsActive()
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.as_ref().IsStatic()
    }

    #[inline]
    pub fn is_kinematic(&self) -> bool {
        self.as_ref().IsKinematic()
    }

    #[inline]
    pub fn is_dynamic(&self) -> bool {
        self.as_ref().IsDynamic()
    }

    #[inline]
    pub fn can_be_kinematic_or_dynamic(&self) -> bool {
        self.as_ref().CanBeKinematicOrDynamic()
    }

    #[inline]
    pub fn set_is_sensor(&mut self, is_sensor: bool) {
        self.as_mut().SetIsSensor(is_sensor);
    }

    #[inline]
    pub fn set_collide_kinematic_vs_non_dynamic(&mut self, collide: bool) {
        self.as_mut().SetCollideKinematicVsNonDynamic(collide);
    }

    #[inline]
    pub fn get_collide_kinematic_vs_non_dynamic(&self) -> bool {
        self.as_ref().GetCollideKinematicVsNonDynamic()
    }

    #[inline]
    pub fn set_use_manifold_reduction(&mut self, use_reduction: bool) {
        self.as_mut().SetUseManifoldReduction(use_reduction);
    }

    #[inline]
    pub fn get_use_manifold_reduction(&self) -> bool {
        self.as_ref().GetUseManifoldReduction()
    }

    #[inline]
    pub fn get_use_manifold_reduction_with_body(&self, body2: &Body) -> bool {
        self.as_ref().GetUseManifoldReductionWithBody(body2.as_ref())
    }

    #[inline]
    pub fn set_apply_gyroscopic_force(&mut self, apply: bool) {
        self.as_mut().SetApplyGyroscopicForce(apply);
    }

    #[inline]
    pub fn get_apply_gyroscopic_force(&self) -> bool {
        self.as_ref().GetApplyGyroscopicForce()
    }

    #[inline]
    pub fn set_enhanced_internal_edge_removal(&mut self, apply: bool) {
        self.as_mut().SetEnhancedInternalEdgeRemoval(apply);
    }

    #[inline]
    pub fn get_enhanced_internal_edge_removal(&self) -> bool {
        self.as_ref().GetEnhancedInternalEdgeRemoval()
    }

    #[inline]
    pub fn get_enhanced_internal_edge_removal_with_body(&self, body2: &Body) -> bool {
        self.as_ref().GetEnhancedInternalEdgeRemovalWithBody(body2.as_ref())
    }

    #[inline]
    pub fn get_motion_type(&self) -> MotionType {
        self.as_ref().GetMotionType()
    }

    #[inline]
    pub fn set_motion_type(&mut self, typ: MotionType) {
        self.as_mut().SetMotionType(typ);
    }

    #[inline]
    pub fn get_allow_sleeping(&self) -> bool {
        self.as_ref().GetAllowSleeping()
    }

    #[inline]
    pub fn set_allow_sleeping(&mut self, allow: bool) {
        self.as_mut().SetAllowSleeping(allow);
    }

    #[inline]
    pub fn reset_sleep_timer(&mut self) {
        self.as_mut().ResetSleepTimer();
    }

    #[inline]
    pub fn get_friction(&self) -> f32 {
        self.as_ref().GetFriction()
    }

    #[inline]
    pub fn set_friction(&mut self, friction: f32) {
        self.as_mut().SetFriction(friction);
    }

    #[inline]
    pub fn get_restitution(&self) -> f32 {
        self.as_ref().GetRestitution()
    }

    #[inline]
    pub fn set_restitution(&mut self, restitution: f32) {
        self.as_mut().SetRestitution(restitution);
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
    pub fn set_linear_velocity_clamped(&mut self, velocity: Vec3A) {
        self.as_mut().SetLinearVelocityClamped(velocity.into());
    }

    #[inline]
    pub fn get_angular_velocity(&self) -> Vec3A {
        self.as_ref().GetAngularVelocity().into()
    }

    #[inline]
    pub fn set_angular_velocity(&mut self, velocity: Vec3A) {
        self.as_mut().SetAngularVelocity(velocity.into());
    }

    #[inline]
    pub fn set_angular_velocity_clamped(&mut self, velocity: Vec3A) {
        self.as_mut().SetAngularVelocityClamped(velocity.into());
    }

    #[inline]
    pub fn get_point_velocity_com(&self, point: Vec3A) -> Vec3A {
        self.as_ref().GetPointVelocityCOM(point.into()).into()
    }

    #[inline]
    pub fn get_point_velocity(&self, point: Vec3A) -> Vec3A {
        self.as_ref().GetPointVelocity(point.into()).into()
    }

    #[inline]
    pub fn add_force(&mut self, force: Vec3A) {
        self.as_mut().AddForce(force.into());
    }

    #[inline]
    pub fn add_force_ex(&mut self, force: Vec3A, point: Vec3A) {
        self.as_mut().AddForceEx(force.into(), point.into());
    }

    #[inline]
    pub fn add_torque(&mut self, torque: Vec3A) {
        self.as_mut().AddTorque(torque.into());
    }

    #[inline]
    pub fn get_accumulated_force(&self) -> Vec3A {
        self.as_ref().GetAccumulatedForce().into()
    }

    #[inline]
    pub fn get_accumulated_torque(&self) -> Vec3A {
        self.as_ref().GetAccumulatedTorque().into()
    }

    #[inline]
    pub fn reset_force(&mut self) {
        self.as_mut().ResetForce();
    }

    #[inline]
    pub fn reset_motion(&mut self) {
        self.as_mut().ResetMotion();
    }

    #[inline]
    pub fn get_inverse_inertia(&self) -> Mat4 {
        self.as_ref().GetInverseInertia().into()
    }

    #[inline]
    pub fn add_impulse(&mut self, impulse: Vec3A) {
        self.as_mut().AddImpulse(impulse.into());
    }

    #[inline]
    pub fn add_impulse_ex(&mut self, impulse: Vec3A, point: Vec3A) {
        self.as_mut().AddImpulseEx(impulse.into(), point.into());
    }

    #[inline]
    pub fn add_angular_impulse(&mut self, impulse: Vec3A) {
        self.as_mut().AddAngularImpulse(impulse.into());
    }

    #[inline]
    pub fn move_kinematic(&mut self, target_position: Vec3A, target_rotation: Quat, delta_time: f32) {
        self.as_mut()
            .MoveKinematic(target_position.into(), target_rotation.into(), delta_time);
    }

    #[inline]
    pub fn get_submerged_volume(&self, surface_position: Vec3A, surface_normal: Vec3A) -> (f32, f32, Vec3A) {
        let mut total_volume = 0.0;
        let mut submerged_volume = 0.0;
        let mut relative_center_of_buoyancy = JVec3::default();
        self.as_ref().GetSubmergedVolume(
            surface_position.into(),
            surface_normal.into(),
            &mut total_volume,
            &mut submerged_volume,
            &mut relative_center_of_buoyancy,
        );
        (total_volume, submerged_volume, relative_center_of_buoyancy.into())
    }

    #[inline]
    pub fn apply_buoyancy_impulse(
        &mut self,
        surface_position: Vec3A,
        surface_normal: Vec3A,
        buoyancy: f32,
        linear_drag: f32,
        angular_drag: f32,
        fluid_velocity: Vec3A,
        gravity: Vec3A,
        delta_time: f32,
    ) {
        self.as_mut().ApplyBuoyancyImpulse(
            surface_position.into(),
            surface_normal.into(),
            buoyancy,
            linear_drag,
            angular_drag,
            fluid_velocity.into(),
            gravity.into(),
            delta_time,
        );
    }

    #[inline]
    pub fn apply_buoyancy_impulse_ex(
        &mut self,
        total_volume: f32,
        submerged_volume: f32,
        relative_center_of_buoyancy: Vec3A,
        buoyancy: f32,
        linear_drag: f32,
        angular_drag: f32,
        fluid_velocity: Vec3A,
        gravity: Vec3A,
        delta_time: f32,
    ) {
        self.as_mut().ApplyBuoyancyImpulseEx(
            total_volume,
            submerged_volume,
            relative_center_of_buoyancy.into(),
            buoyancy,
            linear_drag,
            angular_drag,
            fluid_velocity.into(),
            gravity.into(),
            delta_time,
        );
    }

    #[inline]
    pub fn is_in_broad_phase(&self) -> bool {
        self.as_ref().IsInBroadPhase()
    }

    #[inline]
    pub fn is_collision_cache_invalid(&self) -> bool {
        self.as_ref().IsCollisionCacheInvalid()
    }

    #[inline]
    pub fn get_shape(&self) -> &Shape {
        unsafe { &*Shape::from_ptr(self.as_ref().GetShape()) }
    }

    #[inline]
    pub fn get_position(&self) -> Vec3A {
        self.as_ref().GetPosition().into()
    }

    #[inline]
    pub fn get_rotation(&self) -> Quat {
        self.as_ref().GetRotation().into()
    }

    #[inline]
    pub fn get_world_transform(&self) -> Mat4 {
        self.as_ref().GetWorldTransform().into()
    }

    #[inline]
    pub fn get_center_of_mass_position(&self) -> Vec3A {
        self.as_ref().GetCenterOfMassPosition().into()
    }

    #[inline]
    pub fn get_center_of_mass_transform(&self) -> Mat4 {
        self.as_ref().GetCenterOfMassTransform().into()
    }

    #[inline]
    pub fn get_inverse_center_of_mass_transform(&self) -> Mat4 {
        self.as_ref().GetInverseCenterOfMassTransform().into()
    }

    #[inline]
    pub fn get_world_space_bounds(&self) -> AABox {
        *self.as_ref().GetWorldSpaceBounds()
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
    pub fn get_world_space_surface_normal(&self, sub_shape_id: &SubShapeID, position: Vec3A) -> Vec3A {
        self.as_ref()
            .GetWorldSpaceSurfaceNormal(&sub_shape_id, position.into())
            .into()
    }

    #[inline]
    pub fn add_position_step(&mut self, linear_velocity_times_delta_time: Vec3A) {
        self.as_mut().AddPositionStep(linear_velocity_times_delta_time.into());
    }

    #[inline]
    pub fn sub_position_step(&mut self, linear_velocity_times_delta_time: Vec3A) {
        self.as_mut().SubPositionStep(linear_velocity_times_delta_time.into());
    }

    #[inline]
    pub fn add_rotation_step(&mut self, angular_velocity_times_delta_time: Vec3A) {
        self.as_mut().AddRotationStep(angular_velocity_times_delta_time.into());
    }

    #[inline]
    pub fn sub_rotation_step(&mut self, angular_velocity_times_delta_time: Vec3A) {
        self.as_mut().SubRotationStep(angular_velocity_times_delta_time.into());
    }

    #[inline]
    pub fn set_in_broad_phase_internal(&mut self, in_broad_phase: bool) {
        self.as_mut().SetInBroadPhaseInternal(in_broad_phase);
    }

    #[inline]
    pub fn invalidate_contact_cache_internal(&mut self) -> bool {
        self.as_mut().InvalidateContactCacheInternal()
    }

    #[inline]
    pub fn validate_contact_cache_internal(&mut self) {
        self.as_mut().ValidateContactCacheInternal();
    }

    #[inline]
    pub fn calculate_world_space_bounds_internal(&mut self) {
        self.as_mut().CalculateWorldSpaceBoundsInternal();
    }

    #[inline]
    pub fn set_position_and_rotation_internal(&mut self, position: Vec3A, rotation: Quat, reset_sleep_timer: bool) {
        self.as_mut()
            .SetPositionAndRotationInternal(position.into(), rotation.into(), reset_sleep_timer);
    }

    #[inline]
    pub fn update_center_of_mass_internal(&mut self, previous_center_of_mass: Vec3A, update_mass_properties: bool) {
        self.as_mut()
            .UpdateCenterOfMassInternal(previous_center_of_mass.into(), update_mass_properties);
    }

    #[inline]
    pub fn set_shape_internal(&mut self, shape: &Shape, update_mass_properties: bool) {
        unsafe {
            self.as_mut().SetShapeInternal(&shape.0, update_mass_properties);
        }
    }

    #[inline]
    pub fn get_index_in_active_bodies_internal(&self) -> u32 {
        self.as_ref().GetIndexInActiveBodiesInternal()
    }

    #[inline]
    pub fn update_sleep_state_internal(&mut self, delta_time: f32, max_movement: f32, time_before_sleep: f32) -> bool {
        self.as_mut()
            .UpdateSleepStateInternal(delta_time, max_movement, time_before_sleep)
            .into()
    }
}
