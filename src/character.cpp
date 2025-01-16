#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/shape.rs.h"
#include "jolt-physics-rs/src/system.rs.h"
#include "jolt-physics-rs/src/character.rs.h"

static_assert(sizeof(CharacterVirtual::ExtendedUpdateSettings) == 64, "ExtendedUpdateSettings size");

//
// XCharacter
//

XCharacter::XCharacter(
	Ref<XPhysicsSystem> system,
	const CharacterSettings* settings,
	Vec3 position,
	Quat rotation,
	uint64 userData
):
	Character(settings, position, rotation, userData, &system->PhySys()),
	_system(system) {
	RENDERER_ONLY(_system->AddRenderable(this));
}

XCharacter::~XCharacter() {
	RENDERER_ONLY(_system->RemoveRenderable(this));
	Character::RemoveFromPhysicsSystem();
	PRINT_ONLY(printf("~XCharacter %d system %d\n", GetRefCount(), _system->GetRefCount() - 1));
}

#if defined(JPH_DEBUG_RENDERER)
void XCharacter::Render(DebugRenderer* render) const {}
#endif

struct XCharacterSettings {
	Vec3 up;
	Plane supportingVolume;
	float maxSlopeAngle;
	RefConst<Shape> shape;
	uint16_t layer;
	float mass;
	float friction;
	float gravityFactor;
};
static_assert(sizeof(XCharacterSettings) == 64, "XCharacterSettings size");

XCharacter* CreateCharacter(
	XPhysicsSystem* system,
	const XCharacterSettings& st,
	Vec3 position,
	Quat rotation,
	uint64 userData
) {
	CharacterSettings settings;
	settings.mUp = st.up;
	settings.mSupportingVolume = st.supportingVolume;
	settings.mMaxSlopeAngle = st.maxSlopeAngle;
	settings.mShape = st.shape;
	settings.mLayer = st.layer;
	settings.mMass = st.mass;
	settings.mFriction = st.friction;
	settings.mGravityFactor = st.gravityFactor;
	Ref<XCharacter> character = Ref(new XCharacter(Ref(system), &settings, position, rotation, userData));
	return LeakRefT<XCharacter>(character);
}

XCharacter* CreateAddCharacter(
	XPhysicsSystem* system,
	const XCharacterSettings& settings,
	Vec3 position,
	Quat rotation,
	uint64 userData,
	EActivation activation,
	bool lock
) {
	auto character = CreateCharacter(system, settings, position, rotation, userData);
	character->AddToPhysicsSystem(activation, lock);
	return character;
}

//
// XCharacterVirtual
//

XCharacterVirtual::XCharacterVirtual(
	rust::Fn<void (XCharacterVirtual&)> rustCleanUp,
	Ref<XPhysicsSystem> system,
	const CharacterVirtualSettings* settings,
	Vec3 position,
	Quat rotation
):
	CharacterVirtual(settings, position, rotation, &system->PhySys()),
	_rustCleanUp(rustCleanUp),
	_system(system) {
	RENDERER_ONLY(_system->AddRenderable(this));
}

XCharacterVirtual::~XCharacterVirtual() {
	_rustCleanUp(*this);
	RENDERER_ONLY(_system->RemoveRenderable(this));
	PRINT_ONLY(printf("~XCharacterVirtual %d system %d\n", GetRefCount(), _system->GetRefCount() - 1));
}

void XCharacterVirtual::Update(ObjectLayer chara_layer, float deltaTime, Vec3 gravity) {
	CharacterVirtual::Update(
		deltaTime,
		gravity,
		_system->PhySys().GetDefaultBroadPhaseLayerFilter(chara_layer),
		_system->PhySys().GetDefaultLayerFilter(chara_layer),
		{},
		{},
		_system->Allocator()
	);
}

bool XCharacterVirtual::WalkStairs(
	ObjectLayer chara_layer,
	float deltaTime,
	Vec3 stepUp,
	Vec3 stepForward,
	Vec3 stepForwardTest,
	Vec3 stepDownExtra
) {
	return CharacterVirtual::WalkStairs(
		deltaTime,
		stepUp,
		stepForward,
		stepForwardTest,
		stepDownExtra,
		_system->PhySys().GetDefaultBroadPhaseLayerFilter(chara_layer),
		_system->PhySys().GetDefaultLayerFilter(chara_layer),
		{},
		{},
		_system->Allocator()
	);
}

bool XCharacterVirtual::StickToFloor(ObjectLayer chara_layer, Vec3 stepDown) {
	return CharacterVirtual::StickToFloor(
		stepDown,
		_system->PhySys().GetDefaultBroadPhaseLayerFilter(chara_layer),
		_system->PhySys().GetDefaultLayerFilter(chara_layer),
		{},
		{},
		_system->Allocator()
	);
}

void XCharacterVirtual::ExtendedUpdate(ObjectLayer chara_layer, float deltaTime, Vec3 gravity, const ExtendedUpdateSettings& settings) {
	return CharacterVirtual::ExtendedUpdate(
		deltaTime,
		gravity,
		settings,
		_system->PhySys().GetDefaultBroadPhaseLayerFilter(chara_layer),
		_system->PhySys().GetDefaultLayerFilter(chara_layer),
		{},
		{},
		_system->Allocator()
	);
}

void XCharacterVirtual::RefreshContacts(ObjectLayer chara_layer) {
	CharacterVirtual::RefreshContacts(
		_system->PhySys().GetDefaultBroadPhaseLayerFilter(chara_layer),
		_system->PhySys().GetDefaultLayerFilter(chara_layer),
		{},
		{},
		_system->Allocator()
	);
}

bool XCharacterVirtual::SetShape(ObjectLayer chara_layer, const Shape* shape, float maxPenetrationDepth) {
	return CharacterVirtual::SetShape(
		shape,
		maxPenetrationDepth,
		_system->PhySys().GetDefaultBroadPhaseLayerFilter(chara_layer),
		_system->PhySys().GetDefaultLayerFilter(chara_layer),
		{},
		{},
		_system->Allocator()
	);
}

#if defined(JPH_DEBUG_RENDERER)
void XCharacterVirtual::Render(DebugRenderer* debugRenderer) const {
	const CharacterVirtual* chara = dynamic_cast<const CharacterVirtual*>(this);
	Mat44 com = chara->GetCenterOfMassTransform();
	chara->GetShape()->Draw(debugRenderer, com, Vec3::sReplicate(1.0f), Color::sGreen, false, true);
}
#endif

// void XCharacterVirtual::CheckCollision(ObjectLayer chara_layer, RsVec3 position, RsQuat rotation, RsVec3 movementDirection, float maxPenetrationDepth, Shape* shape, RsVec3 baseOffset) const {
// 	CharacterVirtual::CheckCollision(
// 		AsVec3(position),
// 		AsQuat(rotation),
// 		AsVec3(movementDirection),
// 		maxPenetrationDepth,
// 		AsRefConst<Shape>(shape),
// 		AsVec3(baseOffset),
// 		,
// 		_system->PhySys().GetDefaultBroadPhaseLayerFilter(chara_layer),
// 		_system->PhySys().GetDefaultLayerFilter(chara_layer),
// 		{},
// 		{}
// 	);
// }

struct XCharacterVirtualSettings {
	Vec3 up;
	Plane supportingVolume;
	float maxSlopeAngle;
	RefConst<Shape> shape;
	float mass;
	float maxStrength;
	Vec3 shapeOffset;
	BackFaceMode backFaceMode;
	float predictiveContactDistance;
	uint32_t maxCollisionIterations;
	uint32_t maxConstraintIterations;
	float minTimeRemaining;
	float collisionTolerance;
	float characterPadding;
	uint32_t maxNumHits;
	float hitReductionCosMaxAngle;
	float penetrationRecoverySpeed;
};
static_assert(sizeof(XCharacterVirtualSettings) == 128, "XCharacterVirtualSettings size");

XCharacterVirtual* CreateCharacterVirtual(
	rust::Fn<void (XCharacterVirtual&)> rustCleanUp,
	XPhysicsSystem* system,
	const XCharacterVirtualSettings& st,
	Vec3 position,
	Quat rotation
) {
	JPH::CharacterVirtualSettings settings;
	settings.mUp = st.up;
	settings.mSupportingVolume = st.supportingVolume;
	settings.mMaxSlopeAngle = st.maxSlopeAngle;
	settings.mShape = st.shape;
	settings.mMass = st.mass;
	settings.mMaxStrength = st.maxStrength;
	settings.mShapeOffset = st.shapeOffset;
	settings.mBackFaceMode = st.backFaceMode;
	settings.mPredictiveContactDistance = st.predictiveContactDistance;
	settings.mMaxCollisionIterations = st.maxCollisionIterations;
	settings.mMaxConstraintIterations = st.maxConstraintIterations;
	settings.mMinTimeRemaining = st.minTimeRemaining;
	settings.mCollisionTolerance = st.collisionTolerance;
	settings.mCharacterPadding = st.characterPadding;
	settings.mMaxNumHits = st.maxNumHits;
	settings.mHitReductionCosMaxAngle = st.hitReductionCosMaxAngle;
	settings.mPenetrationRecoverySpeed = st.penetrationRecoverySpeed;
	Ref<XCharacterVirtual> character = Ref(new XCharacterVirtual(rustCleanUp, Ref(system), &settings, position, rotation));
	return LeakRefT<XCharacterVirtual>(character);
}
