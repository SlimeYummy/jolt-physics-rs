use glam::{Quat, Vec3, Vec3A};
use jolt_physics_rs::debug::*;
use jolt_physics_rs::keys::*;
use jolt_physics_rs::*;
use std::f32::consts::PI;

// const FPS: f32 = 60.0;
const FPS: f32 = 120.0;

struct JoltDemo {
    system: PhysicsSystem,
    duration: f32,
    character: Option<JMut<CharacterVirtual<CharacterContactListenerImpl>>>,
    mutable_object: Option<(JMut<MutableCompoundShape>, BodyID)>,
    cv_desired_velocity: Vec3A,
    cv_player_body_id: BodyID,
}

impl DebugApp for JoltDemo {
    fn cpp_physics_system(&mut self) -> *mut u8 {
        unsafe { self.system.cpp_physics_system() }
    }

    fn update_frame(
        &mut self,
        delta: f32,
        camera: &CameraState,
        mouse: &mut DebugMouse,
        keyboard: &mut DebugKeyboard,
    ) -> bool {
        self.update(delta, camera, mouse, keyboard);
        true
    }

    fn get_initial_camera(&mut self, state: &mut CameraState) {
        state.pos = Vec3A::ZERO;
        state.forward = Vec3A::new(10.0, -2.0, 0.0).normalize();
    }

    fn get_camera_pivot(&mut self, heading: f32, pitch: f32) -> Vec3A {
        let fwd = Vec3A::new(pitch.cos() * heading.cos(), pitch.sin(), pitch.cos() * heading.sin());
        if let Some(chara) = &self.character {
            let pos = chara.get_position();
            let ret = Vec3A::new(pos.x, pos.y + 1.0, pos.z) - 5.0 * fwd;
            return ret;
        }
        Vec3A::new(0.0, 10.0, 0.0)
    }
}

impl JoltDemo {
    pub fn new() -> Box<dyn DebugApp> {
        let system = PhysicsSystem::<(), ()>::new(
            BroadPhaseLayerInterfaceImpl::new_vbox(BroadPhaseLayerInterfaceImpl),
            ObjectVsBroadPhaseLayerFilterImpl::new_vbox(ObjectVsBroadPhaseLayerFilterImpl),
            ObjectLayerPairFilterImpl::new_vbox(ObjectLayerPairFilterImpl),
        );
        let mut app = Box::new(Self {
            system,
            duration: 0.0,
            character: None,
            mutable_object: None,
            cv_desired_velocity: Vec3A::ZERO,
            cv_player_body_id: BodyID::INVALID,
        });

        app.create_dyn_cube().unwrap();
        app.create_dyn_sphere().unwrap();
        app.create_dyn_box().unwrap();
        app.create_dyn_tapered_capsule().unwrap();
        app.create_dyn_convex_hull().unwrap();
        app.create_dyn_static_compound().unwrap();
        app.create_dyn_mutable_compound().unwrap();

        app.create_ground().unwrap();
        app.create_mesh_steps().unwrap();
        app.create_height_field().unwrap();

        app.create_sensor_sphere().unwrap();

        let chara_shape = create_capsule_shape(&CapsuleShapeSettings::new(0.5 * 1.35, 0.3)).unwrap();
        let chara_shape = create_rotated_translated_shape(&RotatedTranslatedShapeSettings::new(
            chara_shape.into(),
            Vec3A::new(0.0, 0.5 * 1.35 + 0.3, 0.0),
            Quat::IDENTITY,
        ))
        .unwrap();

        // virtual character
        let mut character = CharacterVirtual::new(
            &mut app.system,
            &CharacterVirtualSettings::new(chara_shape),
            Vec3A::new(4.0, 5.0, 4.0),
            Quat::IDENTITY,
        );
        character.set_listener(Some(CharacterContactListenerImpl::new_vbox(
            CharacterContactListenerImpl {
                allow_sliding: false,
                body_itf: unsafe { app.system.steal_body_itf() },
            },
        )));

        let target_shape = create_capsule_shape(&CapsuleShapeSettings::new(0.5 * 1.2, 0.25)).unwrap();
        let target_shape = create_rotated_translated_shape(&RotatedTranslatedShapeSettings::new(
            target_shape,
            Vec3A::new(0.0, 0.5 * 1.35 + 0.3, 0.0),
            Quat::IDENTITY,
        ))
        .unwrap();
        app.cv_player_body_id = app
            .system
            .body_itf()
            .create_add_body(
                &BodyCreationSettings::new(
                    target_shape,
                    LAYER_PLAYER,
                    MotionType::Kinematic,
                    Vec3A::new(4.0, 5.0, 4.0),
                    Quat::IDENTITY,
                ),
                false,
            )
            .unwrap();

        app.system.optimize_broad_phase();
        app.character = Some(character);
        app
    }

    fn update(&mut self, delta: f32, camera: &CameraState, _mouse: &mut DebugMouse, keyboard: &mut DebugKeyboard) {
        self.duration += delta;

        let jump = keyboard.is_key_pressed(DIK_SPACE);
        let mut move_dir = Vec3A::ZERO;
        if keyboard.is_key_pressed(DIK_W) {
            move_dir.x += 1.0;
        }
        if keyboard.is_key_pressed(DIK_S) {
            move_dir.x += -1.0;
        }
        if keyboard.is_key_pressed(DIK_A) {
            move_dir.z += -1.0;
        }
        if keyboard.is_key_pressed(DIK_D) {
            move_dir.z += 1.0;
        }
        move_dir = move_dir.normalize_or_zero();

        let mut cam_fwd = camera.forward;
        cam_fwd.y = 0.0;
        cam_fwd = cam_fwd.normalize_or_zero();
        if cam_fwd == Vec3A::ZERO {
            cam_fwd = Vec3A::X;
        }
        let rotation = Quat::from_rotation_arc(Vec3::X, cam_fwd.into());
        move_dir = rotation * move_dir;

        if let Some(chara) = &mut self.character {
            chara.get_listener_mut().unwrap().allow_sliding = !move_dir.abs_diff_eq(Vec3A::ZERO, 1e-6);

            let mut new_velocity;
            chara.update_ground_velocity();
            let ground_velocity = chara.get_ground_velocity();
            let linear_velocity = chara.get_linear_velocity();
            let moving_towards_ground = (linear_velocity.y - ground_velocity.y) < 0.1;
            if chara.get_ground_state() == GroundState::OnGround && moving_towards_ground {
                new_velocity = ground_velocity;
                if jump {
                    // ...
                }
            } else {
                new_velocity = Vec3A::new(0.0, linear_velocity.y, 0.0);
            }
            new_velocity += self.system.get_gravity() * (1.0 / FPS); // Gravity
                                                                     // println!("new_velocity => {:?}", new_velocity);
            if chara.is_supported() {
                self.cv_desired_velocity = 0.25 * 5.0 * move_dir + 0.75 * self.cv_desired_velocity;
                new_velocity += self.cv_desired_velocity;
            } else {
                let horizontal_velocity = linear_velocity - Vec3A::new(0.0, linear_velocity.y, 0.0);
                new_velocity += horizontal_velocity;
            }
            chara.set_linear_velocity(new_velocity);

            chara.extended_update(
                LAYER_PLAYER,
                1.0 / FPS,
                self.system.get_gravity(),
                &ExtendedUpdateSettings::default(),
            );

            if self.cv_player_body_id.is_valid() {
                self.system
                    .body_itf()
                    .set_position(self.cv_player_body_id, chara.get_position(), true);
            }
        }

        self.update_dyn_mutable_compound();

        self.system.update(1.0 / FPS);
    }

    fn create_ground(&mut self) -> JoltResult<BodyID> {
        println!("create_ground");
        let ground = create_plane_shape(&PlaneShapeSettings::new(Plane::new(Vec3::Y, 0.0), 50.0))?;
        self.system.body_itf().create_add_body(
            &BodyCreationSettings::new_static(ground, LAYER_STATIC, Vec3A::new(0.0, 0.0, 50.0), Quat::IDENTITY),
            false,
        )
    }

    fn create_dyn_cube(&mut self) -> JoltResult<BodyID> {
        println!("create_dyn_cube");
        let boxx = create_box_shape(&BoxShapeSettings::new(0.5, 0.5, 0.5))?;
        println!("{}", boxx.count_ref());
        let mut bs = BodyCreationSettings::new(
            boxx,
            LAYER_DYNAMIC,
            MotionType::Dynamic,
            Vec3A::new(8.0, 15.0, 8.0),
            Quat::IDENTITY,
        );
        bs.override_mass_properties = OverrideMassProperties::CalculateInertia;
        bs.mass_properties.mass = 10.0;
        self.system.body_itf().create_add_body(&bs, true)
    }

    fn create_dyn_sphere(&mut self) -> JoltResult<BodyID> {
        println!("create_dyn_sphere");
        let sphere = create_sphere_shape(&SphereShapeSettings::new(0.8))?;
        let mut bs = BodyCreationSettings::new(
            sphere,
            LAYER_DYNAMIC,
            MotionType::Dynamic,
            Vec3A::new(10.0, 20.0, 10.0),
            Quat::IDENTITY,
        );
        bs.override_mass_properties = OverrideMassProperties::CalculateInertia;
        bs.mass_properties.mass = 25.0;
        self.system.body_itf().create_add_body(&bs, true)
    }

    fn create_dyn_box(&mut self) -> JoltResult<BodyID> {
        println!("create_dyn_box");
        let long_box = create_box_shape(&BoxShapeSettings::new(0.5, 1.0, 0.5))?;
        let mut bs = BodyCreationSettings::new(
            long_box,
            LAYER_DYNAMIC,
            MotionType::Dynamic,
            Vec3A::new(2.0, 20.0, 10.0),
            Quat::IDENTITY,
        );
        bs.override_mass_properties = OverrideMassProperties::CalculateInertia;
        bs.mass_properties.mass = 70.0;
        self.system.body_itf().create_add_body(&bs, true)
    }

    fn create_dyn_tapered_capsule(&mut self) -> JoltResult<BodyID> {
        println!("create_dyn_tapered_capsule");
        let obj = create_tapered_capsule_shape(&TaperedCapsuleShapeSettings::new(1.0, 1.0, 0.3))?;
        self.system.body_itf().create_add_body(
            &BodyCreationSettings::new(
                obj,
                LAYER_DYNAMIC,
                MotionType::Dynamic,
                Vec3A::new(8.0, 30.0, 16.0),
                Quat::IDENTITY,
            ),
            true,
        )
    }

    fn create_dyn_convex_hull(&mut self) -> JoltResult<BodyID> {
        println!("create_dyn_convex_hull");
        let convex = create_convex_hull_shape(&ConvexHullShapeSettings::new(&[
            Vec3A::new(1.0, 1.0, 1.0),
            Vec3A::new(1.0, -1.0, -1.0),
            Vec3A::new(-1.0, -1.0, 1.0),
            Vec3A::new(-1.0, 1.0, -1.0),
        ]))?;
        self.system.body_itf().create_add_body(
            &BodyCreationSettings::new(
                convex,
                LAYER_DYNAMIC,
                MotionType::Dynamic,
                Vec3A::new(-4.0, 30.0, 10.0),
                Quat::IDENTITY,
            ),
            true,
        )
    }

    fn create_dyn_static_compound(&mut self) -> JoltResult<BodyID> {
        println!("create_dyn_static_compound");
        let capsule = create_capsule_shape(&CapsuleShapeSettings::new(0.25, 0.5))?;
        let boxx = create_box_shape(&BoxShapeSettings::new(0.1, 0.1, 1.0))?;
        let sub_shapes = vec![
            SubShapeSettings::new(capsule, Vec3A::new(0.0, 0.25, 0.0), Quat::IDENTITY),
            SubShapeSettings::new(boxx.clone(), Vec3A::new(0.0, 0.0, 0.0), Quat::IDENTITY),
            SubShapeSettings::new(boxx.clone(), Vec3A::new(0.0, 0.0, 0.0), Quat::from_rotation_y(PI / 2.0)),
        ];
        let static_compound = create_static_compound_shape(&StaticCompoundShapeSettings::new(&sub_shapes))?;
        self.system.body_itf().create_add_body(
            &BodyCreationSettings::new(
                static_compound.into(),
                LAYER_DYNAMIC,
                MotionType::Dynamic,
                Vec3A::new(7.0, 0.0, 15.0),
                Quat::IDENTITY,
            ),
            true,
        )
    }

    fn create_dyn_mutable_compound(&mut self) -> JoltResult<BodyID> {
        println!("create_dyn_mutable_compound");
        let sphere = create_sphere_shape(&SphereShapeSettings::new(0.75))?;
        let boxx = create_box_shape(&BoxShapeSettings::new(0.1, 0.1, 1.0))?;
        let sub_shapes = vec![
            SubShapeSettings::new(sphere, Vec3A::new(0.0, 0.0, 0.0), Quat::IDENTITY),
            SubShapeSettings::new(boxx, Vec3A::new(0.0, 0.0, 0.0), Quat::IDENTITY),
        ];
        let mut mutable_compound = create_mutable_compound_shape_mut(&MutableCompoundShapeSettings::new(&sub_shapes))?;
        let body_id = self.system.body_itf().create_add_body(
            &BodyCreationSettings::new(
                unsafe { mutable_compound.steal_ref().into() },
                LAYER_DYNAMIC,
                MotionType::Dynamic,
                Vec3A::new(7.0, 3.0, -7.0),
                Quat::IDENTITY,
            ),
            true,
        )?;
        self.mutable_object = Some((mutable_compound, body_id));
        Ok(body_id)
    }

    fn update_dyn_mutable_compound(&mut self) {
        let (mutable_compound, body_id) = match self.mutable_object.as_mut() {
            Some(v) => (&mut v.0, v.1.clone()),
            None => return,
        };
        let previous_center_of_mass = mutable_compound.get_center_of_mass();
        mutable_compound.modify_shapes(
            0,
            &[Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, 0.0)],
            &[Quat::IDENTITY, Quat::from_rotation_x(self.duration * PI / 4.0)],
        );
        self.system
            .body_itf()
            .notify_shape_changed(body_id, previous_center_of_mass, false, true);
    }

    fn create_mesh_steps(&mut self) -> JoltResult<BodyID> {
        println!("create_mesh_steps");
        let mut vertices = Vec::new();
        let mut indexes = Vec::new();
        for idx in 0..15 {
            let step_height = 0.2;
            let near_z = 15.0 * step_height;
            let base = Vec3::new(0.0, step_height * (idx as f32), step_height * (idx as f32));

            // left side
            let b1 = base + Vec3::new(3.0, 0.0, 0.0);
            let s1 = b1 + Vec3::new(0.0, step_height, 0.0);
            let p1 = s1 + Vec3::new(0.0, 0.0, step_height);

            // right side
            let width = Vec3::new(-6.0, 0.0, 0.0);
            let b2 = b1 + width;
            let s2 = s1 + width;
            let p2 = p1 + width;
            vertices.extend_from_slice(&[s1, b1, s2]);
            indexes.push(IndexedTriangle::new(idx * 18 + 0, idx * 18 + 1, idx * 18 + 2, 0));
            vertices.extend_from_slice(&[b1, b2, s2]);
            indexes.push(IndexedTriangle::new(idx * 18 + 3, idx * 18 + 4, idx * 18 + 5, 0));
            vertices.extend_from_slice(&[s1, p2, p1]);
            indexes.push(IndexedTriangle::new(idx * 18 + 6, idx * 18 + 7, idx * 18 + 8, 0));
            vertices.extend_from_slice(&[s1, s2, p2]);
            indexes.push(IndexedTriangle::new(idx * 18 + 9, idx * 18 + 10, idx * 18 + 11, 0));

            // side of stairs
            let mut rb2 = b2;
            rb2.z = near_z;
            let mut rs2 = p2;
            rs2.z = near_z;
            vertices.extend_from_slice(&[s2, b2, rs2]);
            indexes.push(IndexedTriangle::new(idx * 18 + 12, idx * 18 + 13, idx * 18 + 14, 0));
            vertices.extend_from_slice(&[rs2, b2, rb2]);
            indexes.push(IndexedTriangle::new(idx * 18 + 15, idx * 18 + 16, idx * 18 + 17, 0));
        }
        let settings = MeshShapeSettings::new(&vertices, &indexes);
        let mesh = create_mesh_shape(&settings)?;
        self.system.body_itf().create_add_body(
            &BodyCreationSettings::new_static(mesh, LAYER_STATIC, Vec3A::new(2.0, 0.0, 15.0), Quat::IDENTITY),
            false,
        )
    }

    fn create_height_field(&mut self) -> JoltResult<BodyID> {
        println!("create_height_field");
        let mut samples = Vec::new();
        for x in 1..=32 {
            for y in 1..=32 {
                let z = if x == 32 {
                    0.0
                } else {
                    let r = (((10007 + 6961 * x) ^ (8623 + 1187 * y)) & 0xFF) as f32 / 256.0;
                    let xy = (((x + 4) % 8) * ((y + 4) % 8)) as f32 / 32.0;
                    f32::sqrt(r + xy) - 0.5
                };
                samples.push(z);
            }
        }
        let mut settings = HeightFieldShapeSettings::new(&samples, 32);
        settings.offset = Vec3A::new(0.0, 0.0, 0.0);
        settings.scale = Vec3A::new(1.0, 1.0, 1.0);
        let height_field = create_height_field_shape(&settings)?;
        self.system.body_itf().create_add_body(
            &BodyCreationSettings::new_static(
                height_field,
                LAYER_STATIC,
                Vec3A::new(-16.0, 0.0, -31.0),
                Quat::IDENTITY,
            ),
            false,
        )
    }

    fn create_sensor_sphere(&mut self) -> JoltResult<BodyID> {
        println!("create_sensor_sphere");
        let sphere = create_sphere_shape(&SphereShapeSettings::new(4.0))?;
        self.system.body_itf().create_add_body(
            &BodyCreationSettings::new_sensor(
                sphere,
                LAYER_STATIC,
                MotionType::Static,
                Vec3A::new(-10.0, 1.0, 10.0),
                Quat::IDENTITY,
            ),
            true,
        )
    }
}

pub const LAYER_STATIC: u32 = 0x1;
pub const LAYER_DYNAMIC: u32 = 0x2;
pub const LAYER_PLAYER: u32 = 0x3;

const BP_LAYER_STATIC: u8 = 0x0;
const BP_LAYER_MOVE: u8 = 0x1;

#[vdata(BroadPhaseLayerInterfaceVTable)]
struct BroadPhaseLayerInterfaceImpl;

impl BroadPhaseLayerInterface for BroadPhaseLayerInterfaceImpl {
    fn get_num_broad_phase_layers(&self) -> u32 {
        2
    }

    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer {
        match layer {
            LAYER_STATIC => BP_LAYER_STATIC,
            LAYER_DYNAMIC => BP_LAYER_MOVE,
            LAYER_PLAYER => BP_LAYER_MOVE,
            _ => BP_LAYER_STATIC,
        }
    }
}

#[vdata(ObjectVsBroadPhaseLayerFilterVTable)]
struct ObjectVsBroadPhaseLayerFilterImpl;

impl ObjectVsBroadPhaseLayerFilter for ObjectVsBroadPhaseLayerFilterImpl {
    fn should_collide(&self, layer: ObjectLayer, bp_layer: BroadPhaseLayer) -> bool {
        match layer {
            LAYER_STATIC => bp_layer != BP_LAYER_STATIC,
            LAYER_DYNAMIC => true,
            LAYER_PLAYER => true,
            _ => false,
        }
    }
}

#[vdata(ObjectLayerPairFilterVTable)]
struct ObjectLayerPairFilterImpl;

impl ObjectLayerPairFilter for ObjectLayerPairFilterImpl {
    fn should_collide(&self, layer1: ObjectLayer, layer2: ObjectLayer) -> bool {
        match layer1 {
            LAYER_STATIC => layer2 != LAYER_STATIC,
            LAYER_DYNAMIC => true,
            LAYER_PLAYER => layer2 != LAYER_PLAYER,
            _ => false,
        }
    }
}

#[vdata(CharacterContactListenerVTable)]
struct CharacterContactListenerImpl {
    allow_sliding: bool,
    body_itf: JRef<BodyInterface>,
}

impl CharacterContactListener for CharacterContactListenerImpl {
    fn on_adjust_body_velocity(
        &mut self,
        _character: &CharacterVirtual,
        _body2: &Body,
        _linear_velocity: &mut Vec3A,
        _angular_velocity: &mut Vec3A,
    ) {
    }

    fn on_contact_validate(&mut self, _character: &CharacterVirtual, _body2: &BodyID, _subshape2: &SubShapeID) -> bool {
        true
    }

    fn on_character_contact_validate(
        &mut self,
        _character: &CharacterVirtual,
        _other_character: &CharacterVirtual,
        _subshape2: &SubShapeID,
    ) -> bool {
        true
    }

    fn on_contact_added(
        &mut self,
        _character: &CharacterVirtual,
        body2: &BodyID,
        _subshape2: &SubShapeID,
        _contact_position: JVec3,
        _contact_normal: JVec3,
        settings: &mut CharacterContactSettings,
    ) {
        if settings.can_push_character && self.body_itf.get_motion_type(*body2) != MotionType::Static {
            self.allow_sliding = true;
        }
    }

    fn on_character_contact_added(
        &mut self,
        _character: &CharacterVirtual,
        _other_character: &CharacterVirtual,
        _subshape2: &SubShapeID,
        _contact_position: JVec3,
        _contact_normal: JVec3,
        settings: &mut CharacterContactSettings,
    ) {
        if settings.can_push_character {
            self.allow_sliding = true;
        }
    }

    fn on_contact_solve(
        &mut self,
        character: &CharacterVirtual,
        _body2: &BodyID,
        _subshape2: &SubShapeID,
        _contact_position: JVec3,
        contact_normal: JVec3,
        contact_velocity: JVec3,
        _material: &PhysicsMaterial,
        _character_velocity: JVec3,
        new_character_velocity: &mut Vec3A,
    ) {
        let contact_normal: Vec3A = contact_normal.into();
        let contact_velocity: Vec3A = contact_velocity.into();
        if !self.allow_sliding
            && contact_velocity.abs_diff_eq(Vec3A::ZERO, 1e-6)
            && !character.is_slope_too_steep(contact_normal)
        {
            *new_character_velocity = Vec3A::ZERO;
        }
    }

    fn on_character_contact_solve(
        &mut self,
        _character: &CharacterVirtual,
        _other_character: &CharacterVirtual,
        _subshape2: &SubShapeID,
        _contact_position: JVec3,
        _contact_normal: JVec3,
        _contact_velocity: JVec3,
        _material: &PhysicsMaterial,
        _character_velocity: JVec3,
        _new_character_velocity: &mut Vec3A,
    ) {
    }
}

fn main() {
    global_initialize();
    let demo_app = JoltDemo::new();
    run_debug_application(demo_app);
}
