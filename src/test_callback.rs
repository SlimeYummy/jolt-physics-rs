use glam::{Quat, Vec3A};
use jolt_macros::vdata;
use std::cell::Cell;
use std::ffi::CStr;

use crate::base::{BodyID, BroadPhaseLayer, JVec3, ObjectLayer, SubShapeID, ValidateResult};
use crate::body::Body;
use crate::character::{
    CharacterContactListener, CharacterContactListenerVTable, CharacterContactSettings, CharacterVirtual,
    CharacterVirtualSettings,
};
use crate::shape::PhysicsMaterial;
use crate::system::{global_initialize, PhysicsSystem};
use crate::system::{
    BodyActivationListener, BodyActivationListenerVTable, BroadPhaseLayerInterface, BroadPhaseLayerInterfaceVTable,
    CollideShapeResult, ContactListener, ContactListenerVTable, ContactManifold, ContactSettings,
    ObjectLayerPairFilter, ObjectLayerPairFilterVTable, ObjectVsBroadPhaseLayerFilter,
    ObjectVsBroadPhaseLayerFilterVTable, SubShapeIDPair,
};

use crate as jolt_physics_rs;

fn get_stack_pointer() -> usize {
    let rsp: usize;
    unsafe {
        std::arch::asm!(
            "mov {}, rsp",
            out(reg) rsp,
            options(nomem, nostack)
        );
    }
    rsp
}

#[cxx::bridge()]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("jolt-physics-rs/src/ffi.h");

        type BroadPhaseLayerInterface = crate::system::ffi::BroadPhaseLayerInterface;
        type ObjectVsBroadPhaseLayerFilter = crate::system::ffi::ObjectVsBroadPhaseLayerFilter;
        type ObjectLayerPairFilter = crate::system::ffi::ObjectLayerPairFilter;
        type BodyActivationListener = crate::system::ffi::BodyActivationListener;
        type ContactListener = crate::system::ffi::ContactListener;
        type XPhysicsSystem = crate::system::ffi::XPhysicsSystem;
        type CharacterContactListener = crate::character::ffi::CharacterContactListener;
        type XCharacterVirtual = crate::character::ffi::XCharacterVirtual;

        unsafe fn TestBroadPhaseLayerInterface(itf: *const BroadPhaseLayerInterface) -> *const c_char;
        unsafe fn TestObjectVsBroadPhaseLayerFilter(itf: *const ObjectVsBroadPhaseLayerFilter) -> *const c_char;
        unsafe fn TestObjectLayerPairFilter(itf: *const ObjectLayerPairFilter) -> *const c_char;
        unsafe fn TestBodyActivationListener(listener: *mut BodyActivationListener) -> *const c_char;
        unsafe fn TestContactListener(listener: *mut ContactListener, system: *mut XPhysicsSystem) -> *const c_char;
        unsafe fn TestCharacterContactListener(
            listener: *mut CharacterContactListener,
            system: *mut XPhysicsSystem,
            chara1: *mut XCharacterVirtual,
            chara2: *mut XCharacterVirtual,
        ) -> *const c_char;
    }
}

#[vdata(BroadPhaseLayerInterfaceVTable)]
#[derive(Default)]
struct TestBplInterface {
    str: String,
    num: f32,
    called_get_broad_phase_layer: Cell<bool>,
    called_get_num_broad_phase_layers: Cell<bool>,
}

impl BroadPhaseLayerInterface for TestBplInterface {
    fn get_num_broad_phase_layers(&self) -> u32 {
        assert_eq!(self.str, "TestBplInterface - test");
        assert_eq!(self.num, 77.444);
        self.called_get_num_broad_phase_layers.set(true);
        123456
    }

    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer {
        assert_eq!(self.str, "TestBplInterface - test");
        assert_eq!(self.num, 77.444);
        assert_eq!(layer, 2233);
        self.called_get_broad_phase_layer.set(true);
        43.into()
    }
}

#[test]
fn test_board_phase_layer_interface() {
    let mut itf = TestBplInterface::new_vbox(TestBplInterface {
        str: "TestBplInterface - test".to_string(),
        num: 77.444,
        ..Default::default()
    });

    let stack = get_stack_pointer();
    unsafe {
        let err = ffi::TestBroadPhaseLayerInterface(
            itf.as_mut() as *const _ as *const crate::system::ffi::BroadPhaseLayerInterface
        );
        if !err.is_null() {
            panic!("Test failed in C++: {:?}", CStr::from_ptr(err));
        }
    }
    assert_eq!(stack, get_stack_pointer());
    assert!(itf.called_get_broad_phase_layer.get());
    assert!(itf.called_get_num_broad_phase_layers.get());
}

#[vdata(ObjectVsBroadPhaseLayerFilterVTable)]
#[derive(Default)]
struct TestObplFilter {
    str: String,
    num: f32,
    called_should_collide: Cell<bool>,
}

impl ObjectVsBroadPhaseLayerFilter for TestObplFilter {
    fn should_collide(&self, layer1: ObjectLayer, layer2: BroadPhaseLayer) -> bool {
        assert_eq!(self.str, "TestObplFilter - test");
        assert_eq!(self.num, 77.444);
        assert_eq!(layer1, 1234000);
        assert_eq!(layer2, 44.into());
        self.called_should_collide.set(true);
        true
    }
}

#[test]
fn test_object_vs_broad_phase_layer_filter() {
    let flt = TestObplFilter::new_vbox(TestObplFilter {
        str: "TestObplFilter - test".to_string(),
        num: 77.444,
        ..Default::default()
    });

    let stack = get_stack_pointer();
    unsafe {
        let err = ffi::TestObjectVsBroadPhaseLayerFilter(
            flt.as_ref() as *const _ as *const crate::system::ffi::ObjectVsBroadPhaseLayerFilter
        );
        if !err.is_null() {
            panic!("Test failed in C++: {:?}", CStr::from_ptr(err));
        }
    }
    assert_eq!(stack, get_stack_pointer());
    assert!(flt.called_should_collide.get());
}

#[vdata(ObjectLayerPairFilterVTable)]
#[derive(Default)]
struct TestOlpFilter {
    num: f32,
    str: String,
    called_should_collide: Cell<bool>,
}

impl ObjectLayerPairFilter for TestOlpFilter {
    fn should_collide(&self, layer1: ObjectLayer, layer2: ObjectLayer) -> bool {
        assert_eq!(self.num, 592.53);
        assert_eq!(self.str, "TestOlpFilter - test");
        assert_eq!(layer1, 5556000);
        assert_eq!(layer2, 989898);
        self.called_should_collide.set(true);
        false
    }
}

#[test]
fn test_object_layer_pair_filter() {
    let flt = TestOlpFilter::new_vbox(TestOlpFilter {
        num: 592.53,
        str: "TestOlpFilter - test".to_string(),
        ..Default::default()
    });

    let stack = get_stack_pointer();
    unsafe {
        let err = ffi::TestObjectLayerPairFilter(
            flt.as_ref() as *const _ as *const crate::system::ffi::ObjectLayerPairFilter
        );
        if !err.is_null() {
            panic!("Test failed in C++: {:?}", CStr::from_ptr(err));
        }
    }
    assert_eq!(stack, get_stack_pointer());
    assert!(flt.called_should_collide.get());
}

#[vdata(BodyActivationListenerVTable)]
#[derive(Default)]
struct TestBaListener {
    str: String,
    buf: Vec<(BodyID, u64)>,
    called_on_body_activated: bool,
    called_on_body_deactivated: bool,
}

impl BodyActivationListener for TestBaListener {
    fn on_body_activated(&mut self, body: &BodyID, user_data: u64) {
        assert_eq!(self.str, "TestBaListener - test");
        self.buf.push((*body, user_data));
        self.called_on_body_activated = true;
    }

    fn on_body_deactivated(&mut self, body: &BodyID, user_data: u64) {
        assert_eq!(self.str, "TestBaListener - test");
        self.buf.push((*body, user_data));
        self.called_on_body_deactivated = true;
    }
}

#[test]
fn test_body_activation_listener_filter() {
    let mut listener = TestBaListener::new_vbox(TestBaListener {
        str: "TestBaListener - test".to_string(),
        buf: vec![],
        ..Default::default()
    });

    let stack = get_stack_pointer();
    unsafe {
        let err = ffi::TestBodyActivationListener(
            listener.as_mut() as *mut _ as *mut crate::system::ffi::BodyActivationListener
        );
        if !err.is_null() {
            panic!("Test failed in C++: {:?}", CStr::from_ptr(err));
        }
    }
    assert_eq!(stack, get_stack_pointer());
    assert!(listener.called_on_body_activated);
    assert!(listener.called_on_body_deactivated);
    assert_eq!(listener.buf, vec![(BodyID(123456), 99999), (BodyID(654321), 88888)]);
}

#[vdata(ContactListenerVTable)]
#[derive(Default)]
struct TestContactListener {
    str: String,
    buf: Vec<(BodyID, BodyID)>,
    called_on_contact_validate: bool,
    called_on_contact_added: bool,
    called_on_contact_persisted: bool,
    called_on_contact_removed: bool,
}

impl ContactListener for TestContactListener {
    fn on_contact_validate(
        &mut self,
        body1: &Body,
        body2: &Body,
        base_offset: JVec3,
        collision_result: &CollideShapeResult,
    ) -> ValidateResult {
        assert_eq!(self.str, "TestContactListener - test");
        assert_eq!(body1.get_id().0, 0x1000000);
        assert_eq!(body2.get_id().0, 0x1000001);
        self.buf.push((body1.get_id(), body2.get_id()));
        assert_eq!(base_offset, Vec3A::new(4.3, 5.4, 0.82));
        assert_eq!(collision_result.penetration_depth, 0.073);
        self.called_on_contact_validate = true;
        ValidateResult::RejectContact
    }

    fn on_contact_added(&mut self, body1: &Body, body2: &Body, manifold: &ContactManifold, settings: &ContactSettings) {
        assert_eq!(self.str, "TestContactListener - test");
        assert_eq!(body1.get_id().0, 0x1000000);
        assert_eq!(body2.get_id().0, 0x1000001);
        assert_eq!(manifold.penetration_depth, 0.028);
        assert_eq!(settings.relative_angular_surface_velocity, Vec3A::new(0.1, 0.2, 0.3));
        self.buf.push((body1.get_id(), body2.get_id()));
        self.called_on_contact_added = true;
    }

    fn on_contact_persisted(
        &mut self,
        body1: &Body,
        body2: &Body,
        manifold: &ContactManifold,
        settings: &ContactSettings,
    ) {
        assert_eq!(self.str, "TestContactListener - test");
        assert_eq!(body1.get_id().0, 0x1000000);
        assert_eq!(body2.get_id().0, 0x1000001);
        assert_eq!(manifold.penetration_depth, 0.103);
        assert_eq!(settings.relative_linear_surface_velocity, Vec3A::new(1.1, 2.2, 3.3));
        self.buf.push((body1.get_id(), body2.get_id()));
        self.called_on_contact_persisted = true;
    }

    fn on_contact_removed(&mut self, sub_shape_pair: &SubShapeIDPair) {
        assert_eq!(self.str, "TestContactListener - test");
        assert_eq!(sub_shape_pair.sub_shape_id1.0, 0xFFFFFFFF);
        assert_eq!(sub_shape_pair.sub_shape_id2.0, 0xFFFFFFFF);
        self.called_on_contact_removed = true;
    }
}

#[test]
fn test_contact_listener() {
    global_initialize();
    let system: PhysicsSystem<TestContactListener, TestBaListener> = PhysicsSystem::new(
        EmptyBplInterface::new_vbox(EmptyBplInterface),
        EmptyObplFilter::new_vbox(EmptyObplFilter),
        EmptyOlpFilter::new_vbox(EmptyOlpFilter),
    );

    let mut listener = TestContactListener::new_vbox(TestContactListener {
        str: "TestContactListener - test".to_string(),
        buf: vec![],
        ..Default::default()
    });

    let stack = get_stack_pointer();
    unsafe {
        let err = ffi::TestContactListener(
            listener.as_mut() as *mut _ as *mut crate::system::ffi::ContactListener,
            system.as_x_ptr(),
        );
        if !err.is_null() {
            panic!("Test failed in C++: {:?}", CStr::from_ptr(err));
        }
    }
    assert_eq!(stack, get_stack_pointer());
    assert!(listener.called_on_contact_validate);
    assert!(listener.called_on_contact_added);
    assert!(listener.called_on_contact_persisted);
    assert!(listener.called_on_contact_removed);
    assert_eq!(listener.buf, vec![(BodyID(0x1000000), BodyID(0x1000001)); 3]);
}

#[vdata(CharacterContactListenerVTable)]
#[derive(Default)]
struct TestCclListener {
    num: i32,
    str: String,
    called_on_adjust_body_velocity: bool,
    called_on_contact_validate: bool,
    called_on_character_contact_validate: bool,
    called_on_contact_added: bool,
    called_on_character_contact_added: bool,
    called_on_contact_solve: bool,
    called_on_character_contact_solve: bool,
}

impl CharacterContactListener for TestCclListener {
    fn on_adjust_body_velocity(
        &mut self,
        character: &CharacterVirtual,
        body2: &Body,
        linear_velocity: &mut Vec3A,
        angular_velocity: &mut Vec3A,
    ) {
        assert_eq!(self.num, 999999);
        assert_eq!(self.str, "TestCclListener - test");
        assert_eq!(character.get_position(), Vec3A::new(1.0, 1.0, 1.0));
        assert_eq!(body2.get_position(), Vec3A::new(13.0, 3.0, 0.3));
        assert_eq!(linear_velocity, &Vec3A::new(2.0, 3.0, 4.0));
        assert_eq!(angular_velocity, &Vec3A::new(0.5, 0.6, 0.7));
        self.called_on_adjust_body_velocity = true;
    }

    fn on_contact_validate(&mut self, character: &CharacterVirtual, body2: &BodyID, subshape2: &SubShapeID) -> bool {
        assert_eq!(self.num, 999999);
        assert_eq!(self.str, "TestCclListener - test");
        assert_eq!(character.get_position(), Vec3A::new(1.0, 1.0, 1.0));
        assert_eq!(body2.0, 777666);
        assert_eq!(subshape2.0, 999888);
        self.called_on_contact_validate = true;
        false
    }

    fn on_character_contact_validate(
        &mut self,
        character: &CharacterVirtual,
        other_character: &CharacterVirtual,
        subshape2: &SubShapeID,
    ) -> bool {
        assert_eq!(self.num, 999999);
        assert_eq!(self.str, "TestCclListener - test");
        assert_eq!(character.get_position(), Vec3A::new(1.0, 1.0, 1.0));
        assert_eq!(other_character.get_rotation(), Quat::IDENTITY);
        assert_eq!(subshape2.0, 12345678);
        self.called_on_character_contact_validate = true;
        true
    }

    fn on_contact_added(
        &mut self,
        character: &CharacterVirtual,
        body2: &BodyID,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        settings: &mut CharacterContactSettings,
    ) {
        assert_eq!(self.num, 999999);
        assert_eq!(self.str, "TestCclListener - test");
        assert_eq!(character.get_position(), Vec3A::new(1.0, 1.0, 1.0));
        assert_eq!(body2.0, 999999);
        assert_eq!(subshape2.0, 8888);
        assert_eq!(contact_position, Vec3A::new(7.0, 7.0, 7.0));
        assert_eq!(contact_normal, Vec3A::new(6.0, 6.0, 6.0));
        assert_eq!(settings.can_push_character, true);
        assert_eq!(settings.can_receive_impulses, true);
        settings.can_push_character = false;
        self.called_on_contact_added = true;
    }

    fn on_character_contact_added(
        &mut self,
        character: &CharacterVirtual,
        other_character: &CharacterVirtual,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        settings: &mut CharacterContactSettings,
    ) {
        assert_eq!(self.num, 999999);
        assert_eq!(self.str, "TestCclListener - test");
        assert_eq!(character.get_position(), Vec3A::new(1.0, 1.0, 1.0));
        assert_eq!(other_character.get_rotation(), Quat::IDENTITY);
        assert_eq!(subshape2.0, 1111);
        assert_eq!(contact_position, Vec3A::new(5.0, 5.0, 5.0));
        assert_eq!(contact_normal, Vec3A::new(4.0, 4.0, 4.0));
        assert_eq!(settings.can_push_character, false);
        assert_eq!(settings.can_receive_impulses, false);
        settings.can_receive_impulses = true;
        self.called_on_character_contact_added = true;
    }

    fn on_contact_solve(
        &mut self,
        character: &CharacterVirtual,
        body2: &BodyID,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        contact_velocity: JVec3,
        _material: &PhysicsMaterial,
        character_velocity: JVec3,
        new_character_velocity: &mut Vec3A,
    ) {
        assert_eq!(self.num, 999999);
        assert_eq!(self.str, "TestCclListener - test");
        assert_eq!(character.get_position(), Vec3A::new(1.0, 1.0, 1.0));
        assert_eq!(body2.0, 22233344);
        assert_eq!(subshape2.0, 55566677);
        assert_eq!(contact_position, Vec3A::new(0.1, 0.1, 0.1));
        assert_eq!(contact_normal, Vec3A::new(0.2, 0.2, 0.2));
        assert_eq!(contact_velocity, Vec3A::new(0.3, 0.3, 0.3));
        // assert_eq!(material as *const _, std::ptr::null());
        assert_eq!(character_velocity, Vec3A::new(0.4, 0.4, 0.4));
        assert_eq!(*new_character_velocity, Vec3A::new(0.0, 0.0, 0.0));
        *new_character_velocity = Vec3A::new(9.8, 8.7, 7.6);
        self.called_on_contact_solve = true;
    }

    fn on_character_contact_solve(
        &mut self,
        character: &CharacterVirtual,
        other_character: &CharacterVirtual,
        subshape2: &SubShapeID,
        contact_position: JVec3,
        contact_normal: JVec3,
        contact_velocity: JVec3,
        _material: &PhysicsMaterial,
        character_velocity: JVec3,
        new_character_velocity: &mut Vec3A,
    ) {
        assert_eq!(self.num, 999999);
        assert_eq!(self.str, "TestCclListener - test");
        assert_eq!(character.get_position(), Vec3A::new(1.0, 1.0, 1.0));
        assert_eq!(other_character.get_rotation(), Quat::IDENTITY);
        assert_eq!(subshape2.0, 4000000);
        assert_eq!(contact_position, Vec3A::new(0.9, 0.9, 0.9));
        assert_eq!(contact_normal, Vec3A::new(0.8, 0.8, 0.8));
        assert_eq!(contact_velocity, Vec3A::new(0.7, 0.7, 0.7));
        // assert_eq!(material as *const _, std::ptr::null());
        assert_eq!(character_velocity, Vec3A::new(0.6, 0.6, 0.6));
        assert_eq!(*new_character_velocity, Vec3A::new(9.9, 9.9, 9.9));
        *new_character_velocity = Vec3A::new(1.2, 2.3, 3.4);
        self.called_on_character_contact_solve = true;
    }
}

#[test]
fn test_character_contact_listener() {
    global_initialize();
    let mut system: PhysicsSystem<TestContactListener, TestBaListener> = PhysicsSystem::new(
        EmptyBplInterface::new_vbox(EmptyBplInterface),
        EmptyObplFilter::new_vbox(EmptyObplFilter),
        EmptyOlpFilter::new_vbox(EmptyOlpFilter),
    );
    let mut chara1 = CharacterVirtual::<TestCclListener>::new(
        &mut system,
        &CharacterVirtualSettings::default(),
        Vec3A::new(1.0, 1.0, 1.0),
        Quat::IDENTITY,
    );
    let mut chara2 = CharacterVirtual::<()>::new(
        &mut system,
        &CharacterVirtualSettings::default(),
        Vec3A::new(-1.0, -1.0, -1.0),
        Quat::IDENTITY,
    );

    chara1.set_listener(Some(TestCclListener::new_vbox(TestCclListener::default())));
    chara1.set_listener(Some(TestCclListener::new_vbox(TestCclListener::default())));
    chara1.set_listener(None);

    let mut listener = TestCclListener::new_vbox(TestCclListener::default());
    listener.num = 999999;
    listener.str = "TestCclListener - test".to_string();

    let stack = get_stack_pointer();
    unsafe {
        let err = ffi::TestCharacterContactListener(
            listener.as_mut() as *mut _ as *mut crate::character::ffi::CharacterContactListener,
            system.as_x_ptr(),
            &mut chara1.as_mut().character,
            &mut chara2.as_mut().character,
        );
        if !err.is_null() {
            panic!("Test failed in C++: {:?}", CStr::from_ptr(err));
        }
    };
    assert_eq!(stack, get_stack_pointer());

    assert!(listener.called_on_adjust_body_velocity);
    assert!(listener.called_on_contact_validate);
    assert!(listener.called_on_character_contact_validate);
    assert!(listener.called_on_contact_added);
    assert!(listener.called_on_character_contact_added);
    assert!(listener.called_on_contact_solve);
    assert!(listener.called_on_character_contact_solve);
}

#[vdata(BroadPhaseLayerInterfaceVTable)]
struct EmptyBplInterface;

impl BroadPhaseLayerInterface for EmptyBplInterface {
    fn get_num_broad_phase_layers(&self) -> u32 {
        1
    }

    fn get_broad_phase_layer(&self, _: ObjectLayer) -> BroadPhaseLayer {
        1
    }
}

#[vdata(ObjectVsBroadPhaseLayerFilterVTable)]
struct EmptyObplFilter;

impl ObjectVsBroadPhaseLayerFilter for EmptyObplFilter {
    fn should_collide(&self, _1: ObjectLayer, _2: BroadPhaseLayer) -> bool {
        true
    }
}

#[vdata(ObjectLayerPairFilterVTable)]
struct EmptyOlpFilter;

impl ObjectLayerPairFilter for EmptyOlpFilter {
    fn should_collide(&self, _1: ObjectLayer, _2: ObjectLayer) -> bool {
        true
    }
}
