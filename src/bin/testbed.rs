use glam::{Quat, Vec3, Vec3A};
use jolt_physics_rs::*;

// const FPS: f32 = 60.0;
const FPS: f32 = 144.0;

const KEY_UP: i32 = 0xC8;
const KEY_LEFT: i32 = 0xCB;
const KEY_DOWN: i32 = 0xD0;
const KEY_RIGHT: i32 = 0xCD;
const KEY_SPACE: i32 = 0x39;

struct DebugApplicationImpl {
    system: Box<PhysicsSystem>,
    body_itf: BodyInterface,
    chara_common: Option<CharacterCommon>,
    chara_virtual: Option<CharacterVirtual>,
    cv_desired_velocity: Vec3A,
    cv_player_body_id: BodyID,
}

impl DebugApplication for DebugApplicationImpl {
    fn get_ref_system(&mut self) -> RefPhysicsSystem {
        return self.system.inner_ref().clone();
    }

    fn render_frame(&mut self, delta: f32, keyboard: &mut DebugKeyboard, camera: &CameraState) -> bool {
        self.update(delta, keyboard, camera);
        return true;
    }

    fn get_camera_pivot(&self, heading: f32, pitch: f32) -> Vec3A {
        let fwd = Vec3A::new(pitch.cos() * heading.cos(), pitch.sin(), pitch.cos() * heading.sin());
        // if let Some(chara) = &self.chara_common {
        //     let pos = chara.get_position(false);
        //     let ret = Vec3A::new(pos.x, pos.y + 1.0, pos.z) - 5.0 * fwd;
        //     return ret;
        // }
        if let Some(chara) = &self.chara_virtual {
            let pos = chara.get_position();
            let ret = Vec3A::new(pos.x, pos.y + 1.0, pos.z) - 5.0 * fwd;
            return ret;
        }
        return Vec3A::new(0.0, 10.0, 0.0);
    }
}

impl DebugApplicationImpl {
    fn create_floor(&mut self) -> BodyID {
        let floor = create_shape_box(&BoxSettings::new(100.0, 2.0, 50.0));
        let floor = create_shape_rotated_translated(&RotatedTranslatedSettings::new(floor, Vec3A::new(0.0, -1.0, 0.0), Quat::IDENTITY));
        return self
            .body_itf
            .create_add_body(
                &BodySettings::new_static(
                    floor,
                    PHY_LAYER_STATIC,
                    Vec3A::new(0.0, 0.0, 50.0),
                    Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                ),
                false,
            )
            .unwrap();
    }

    fn create_dynamic_cube(&mut self) -> BodyID {
        let boxx = create_shape_box(&BoxSettings::new(0.5, 0.5, 0.5));
        let mut bs = BodySettings::new(
            boxx,
            PHY_LAYER_DYNAMIC,
            MotionType::Dynamic,
            Vec3A::new(8.0, 15.0, 8.0),
            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
        );
        bs.override_mass_properties = OverrideMassProperties::CalculateInertia;
        bs.mass_properties.mass = 10.0;
        return self.body_itf.create_add_body(&bs, true).unwrap();
    }

    fn create_dynamic_sphere(&mut self) -> BodyID {
        let sphere = create_shape_sphere(&SphereSettings::new(0.8));
        let mut bs = BodySettings::new(
            sphere,
            PHY_LAYER_DYNAMIC,
            MotionType::Dynamic,
            Vec3A::new(10.0, 20.0, 10.0),
            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
        );
        bs.override_mass_properties = OverrideMassProperties::CalculateInertia;
        bs.mass_properties.mass = 25.0;
        return self.body_itf.create_add_body(&bs, true).unwrap();
    }

    fn create_dynamic_box(&mut self) -> BodyID {
        let long_box = create_shape_box(&BoxSettings::new(0.5, 1.0, 0.5));
        let mut bs = BodySettings::new(
            long_box,
            PHY_LAYER_DYNAMIC,
            MotionType::Dynamic,
            Vec3A::new(2.0, 20.0, 10.0),
            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
        );
        bs.override_mass_properties = OverrideMassProperties::CalculateInertia;
        bs.mass_properties.mass = 70.0;
        return self.body_itf.create_add_body(&bs, true).unwrap();
    }

    fn create_dynamic_convex_hull(&mut self) -> BodyID {
        let convex = create_shape_convex_hull(&ConvexHullSettings::new(&[
            Vec3A::new(1.0, 1.0, 1.0),
            Vec3A::new(1.0, -1.0, -1.0),
            Vec3A::new(-1.0, -1.0, 1.0),
            Vec3A::new(-1.0, 1.0, -1.0),
        ]));
        return self
            .body_itf
            .create_add_body(
                &BodySettings::new(
                    convex,
                    PHY_LAYER_DYNAMIC,
                    MotionType::Dynamic,
                    Vec3A::new(-4.0, 30.0, 10.0),
                    Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                ),
                true,
            )
            .unwrap();
    }

    fn create_mesh_steps(&mut self) -> BodyID {
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
        let settings = MeshSettings::new(&vertices, &indexes);
        let mesh = create_shape_mesh(&settings);
        return self
            .body_itf
            .create_add_body(
                &BodySettings::new_static(
                    mesh,
                    PHY_LAYER_STATIC,
                    Vec3A::new(2.0, 1.0, 15.0),
                    Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                ),
                false,
            )
            .unwrap();
    }

    fn create_height_field(&mut self) -> BodyID {
        let mut samples = Vec::new();
        for x in 0..32 {
            for y in 0..32 {
                let z = ((x as f32) % 16.0 - 8.0).abs();
                samples.push(z);
            }
        }
        let mut settings = HeightFieldSettings::new(&samples, 32);
        settings.offset = Vec3A::new(0.0, 0.0, 0.0);
        settings.scale = Vec3A::new(1.0, 1.0, 1.0);
        let height_field = create_shape_height_field(&settings);
        return self
            .body_itf
            .create_add_body(
                &BodySettings::new_static(
                    height_field,
                    PHY_LAYER_STATIC,
                    Vec3A::new(-16.0, -5.0, -31.0),
                    Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                ),
                false,
            )
            .unwrap();
    }

    fn create_sensor_sphere(&mut self) -> BodyID {
        let sphere = create_shape_sphere(&SphereSettings::new(4.0));
        return self
            .body_itf
            .create_add_body(
                &BodySettings::new_sensor(
                    sphere,
                    PHY_LAYER_STATIC,
                    MotionType::Static,
                    Vec3A::new(-10.0, 1.0, 10.0),
                    Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                ),
                true,
            )
            .unwrap();
    }

    pub fn new() -> Box<dyn DebugApplication> {
        let mut system = PhysicsSystem::new();
        let body_itf = BodyInterface::new(system.as_mut(), false);
        let mut app = Box::new(Self {
            system,
            body_itf,
            chara_common: None,
            chara_virtual: None,
            cv_desired_velocity: Vec3A::ZERO,
            cv_player_body_id: BodyID::invalid(),
        });

        app.create_dynamic_cube();
        app.create_dynamic_sphere();
        app.create_dynamic_box();
        app.create_dynamic_convex_hull();

        app.create_floor();
        app.create_mesh_steps();
        app.create_height_field();

        app.create_sensor_sphere();

        let chara_shape = create_shape_capsule(&CapsuleSettings::new(0.5 * 1.35, 0.3));
        let chara_shape = create_shape_rotated_translated(&RotatedTranslatedSettings::new(
            chara_shape,
            Vec3A::new(0.0, 0.5 * 1.35 + 0.3, 0.0),
            Quat::IDENTITY,
        ));

        // common character
        let mut chara_common = CharacterCommon::new_ex(
            app.system.as_mut(),
            &CharacterCommonSettings::new(chara_shape.clone(), PHY_LAYER_BODY_PLAYER),
            Vec3A::new(0.0, 5.0, 1.0),
            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            0,
            true,
            true,
        );

        // virtual character
        let chara_virtual = CharacterVirtual::new(
            app.system.as_mut(),
            &CharacterVirtualSettings::new(chara_shape),
            Vec3A::new(4.0, 5.0, 4.0),
            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
        );

        let target_shape = create_shape_capsule(&CapsuleSettings::new(0.5 * 1.2, 0.25));
        let target_shape = create_shape_rotated_translated(&RotatedTranslatedSettings::new(
            target_shape,
            Vec3A::new(0.0, 0.5 * 1.35 + 0.3, 0.0),
            Quat::IDENTITY,
        ));
        app.cv_player_body_id = app
            .body_itf
            .create_add_body(
                &BodySettings::new(
                    target_shape,
                    PHY_LAYER_BODY_PLAYER,
                    MotionType::Kinematic,
                    Vec3A::new(4.0, 5.0, 4.0),
                    Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                ),
                false,
            )
            .unwrap();

        app.system.prepare();

        app.chara_common = Some(chara_common);
        app.chara_virtual = Some(chara_virtual);
        return app;
    }

    fn update(&mut self, delta: f32, keyboard: &mut DebugKeyboard, camera: &CameraState) {
        let jump = keyboard.is_key_pressed(KEY_SPACE);

        let mut move_dir = Vec3A::ZERO;
        if keyboard.is_key_pressed(KEY_UP) {
            move_dir.x += 1.0;
        }
        if keyboard.is_key_pressed(KEY_DOWN) {
            move_dir.x += -1.0;
        }
        if keyboard.is_key_pressed(KEY_LEFT) {
            move_dir.z += -1.0;
        }
        if keyboard.is_key_pressed(KEY_RIGHT) {
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

        // if let Some(chara) = &mut self.chara_common {
        //     if chara.is_supported() {
        //         let current_velocity = chara.get_linear_velocity(false);
        //         let mut desired_velocity = 5.0 * move_dir;
        //         desired_velocity.y = current_velocity.y;
        //         let new_velocity = desired_velocity * 0.75 + current_velocity * 0.25;
        //         chara.set_linear_velocity(&new_velocity, false);
        //     }
        // }

        if let Some(chara) = &mut self.chara_virtual {
            let mut new_velocity = Vec3A::ZERO;
            chara.update_ground_velocity();
            let ground_velocity = chara.get_ground_velocity();
            let linear_velocity = chara.get_linear_velocity();
            let moving_towards_ground = (linear_velocity.y - ground_velocity.y) < 0.1;
            // println!("ground_velocity => {:?} {:?}", ground_velocity, linear_velocity);
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
                PHY_LAYER_BODY_PLAYER,
                1.0 / FPS,
                self.system.get_gravity(),
                &ExtendedUpdateSettings::default(),
            );

            self.body_itf.set_position(self.cv_player_body_id, chara.get_position(), true);
        }

        self.system.update(1.0 / FPS);

        // if let Some(chara) = &mut self.chara_common {
        //     chara.post_simulation(0.1, false);
        // }
    }
}

fn main() {
    global_initialize();
    run_debug_application(DebugApplicationImpl::new);
}
