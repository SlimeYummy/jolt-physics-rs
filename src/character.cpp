#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/shape.rs.h"
#include "jolt-physics-rs/src/system.rs.h"
#include "jolt-physics-rs/src/character.rs.h"

static_assert(sizeof(CharacterVirtual::ExtendedUpdateSettings) == 64, "ExtendedUpdateSettings size");

//
// XCharacterCommon
//

XCharacterCommon::XCharacterCommon(
	XPhysicsSystem* system,
	const CharacterSettings* settings,
	Vec3 position,
	Quat rotation,
	uint64 userData
):
	Character(settings, position, rotation, userData, &system->PhySys()),
	_system(system) {
	RENDERER_ONLY(_system->AddRenderable(this));
}

XCharacterCommon::~XCharacterCommon() {
	RENDERER_ONLY(_system->RemoveRenderable(this));
	Character::RemoveFromPhysicsSystem();
	printf("~XCharacterCommon\n");
}

Isometry XCharacterCommon::GetPositionAndRotation(bool lock) const {
	Vec3 position = Vec3::sZero();
	Quat rotation = Quat::sIdentity();
	Character::GetPositionAndRotation(position, rotation, lock);
	return Isometry{position, rotation};
}

bool XCharacterCommon::SetShape(XRefShape shape, float maxPenetrationDepth, bool lock) {
	return Character::SetShape(AsRefConst<Shape>(shape), maxPenetrationDepth, lock);
}

#if defined(JPH_DEBUG_RENDERER)
void XCharacterCommon::Render(DebugRenderer* render) const {}
#endif

struct XCharacterCommonSettings {
	Vec3 up;
	Plane supportingVolume;
	float maxSlopeAngle;
	RefConst<Shape> shape;
	uint16_t layer;
	float mass;
	float friction;
	float gravityFactor;
};
static_assert(sizeof(XCharacterCommonSettings) == 64, "XCharacterCommonSettings size");

unique_ptr<XCharacterCommon> CreateCharacterCommon(
	XPhysicsSystem* system,
	const XCharacterCommonSettings& st,
	Vec3 position,
	Quat rotation,
	uint64 userData
) {
	JPH::CharacterSettings settings;
	settings.mUp = st.up;
	settings.mSupportingVolume = st.supportingVolume;
	settings.mMaxSlopeAngle = st.maxSlopeAngle;
	settings.mShape = st.shape;
	settings.mLayer = st.layer;
	settings.mMass = st.mass;
	settings.mFriction = st.friction;
	settings.mGravityFactor = st.gravityFactor;
	return make_unique<XCharacterCommon>(system, &settings, position, rotation, userData);
}

unique_ptr<XCharacterCommon> CreateAddCharacterCommon(
	XPhysicsSystem* system,
	const XCharacterCommonSettings& settings,
	Vec3 position,
	Quat rotation,
	uint64 userData,
	EActivation activation,
	bool lock
) {
	auto chara = CreateCharacterCommon(system, settings, position, rotation, userData);
	chara->AddToPhysicsSystem(activation, lock);
	return chara;
}

//
// XCharacterVirtual
//

XCharacterVirtual::XCharacterVirtual(
	XPhysicsSystem* system,
	const CharacterVirtualSettings* settings,
	Vec3 position,
	Quat rotation
):
	CharacterVirtual(settings, position, rotation, &system->PhySys()),
	_system(system) {
	RENDERER_ONLY(_system->AddRenderable(this));
}

XCharacterVirtual::~XCharacterVirtual() {
	RENDERER_ONLY(_system->RemoveRenderable(this));
	printf("~XCharacterVirtual %d\n", _system->GetRefCount());
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

bool XCharacterVirtual::SetShape(ObjectLayer chara_layer, XRefShape shape, float maxPenetrationDepth) {
	return CharacterVirtual::SetShape(
		AsRefConst<Shape>(shape),
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
	Mat44 transform = chara->GetCenterOfMassTransform();
	chara->GetShape()->Draw(debugRenderer, transform, Vec3::sReplicate(1.0f), Color::sGreen, false, true);
}
#endif

// void XCharacterVirtual::CheckCollision(ObjectLayer chara_layer, RsVec3 position, RsQuat rotation, RsVec3 movementDirection, float maxPenetrationDepth, XRefShape shape, RsVec3 baseOffset) const {
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

// void XCharacterVirtual::Render(DebugRenderer* render) const {
// 	RMat44 com = this->GetCenterOfMassTransform();
// 	this->GetShape()->Draw(mDebugRenderer, com, Vec3::sReplicate(1.0f), Color::sGreen, false, true);
// }


void XCharacterVirtual::OnAdjustBodyVelocity(const CharacterVirtual* chara, const Body& body2, Vec3& linearVelocity, Vec3& angularVelocity)  {}

bool XCharacterVirtual::OnContactValidate(const CharacterVirtual* chara, const BodyID& body2, const SubShapeID& shape2)  {
	return true;
}

void XCharacterVirtual::OnContactAdded(
	const CharacterVirtual* chara,
	const BodyID& body,
	const SubShapeID& shape2,
	Vec3 contactPosition,
	Vec3 contactNormal,
	CharacterContactSettings& settings
) {

}

void XCharacterVirtual::OnContactSolve(
	const CharacterVirtual* chara,
	const BodyID& body2,
	const SubShapeID& shape2,
	Vec3 contactPosition,
	Vec3 contactNormal,
	Vec3 contactVelocity,
	const PhysicsMaterial* contactMaterial,
	Vec3 charaVelocity,
	Vec3& newCharaVelocity
) {

}

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

unique_ptr<XCharacterVirtual> CreateCharacterVirtual(
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
	auto chara = make_unique<XCharacterVirtual>(system, &settings, position, rotation);
	chara->SetListener(chara.get());
	return chara;
}
