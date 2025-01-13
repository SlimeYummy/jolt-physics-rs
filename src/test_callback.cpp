#include "jolt-physics-rs/src/ffi.h"

const char* TestBroadPhaseLayerInterface(const BroadPhaseLayerInterface* itf) {
	uint num = itf->GetNumBroadPhaseLayers();
	if (num != 123456) {
		return "GetNumBroadPhaseLayers failed";
	}

	BroadPhaseLayer::Type bpLayer = itf->GetBroadPhaseLayer(2233);
	if (bpLayer != 43) {
		return "GetBroadPhaseLayer failed";
	}

	return nullptr;
}

const char* TestObjectVsBroadPhaseLayerFilter(const ObjectVsBroadPhaseLayerFilter* filter) {
	bool res = filter->ShouldCollide(1234000, BroadPhaseLayer(44));
	if (res != true) {
		return "ShouldCollide failed";
	}
	return nullptr;
}

const char* TestObjectLayerPairFilter(const ObjectLayerPairFilter* filter) {
	bool res = filter->ShouldCollide(5556000, 989898);
	if (res != false) {
		return "ShouldCollide failed";
	}
	return nullptr;
}

const char* TestBodyActivationListener(BodyActivationListener* listener) {
	listener->OnBodyActivated(BodyID(123456), 99999);
	listener->OnBodyDeactivated(BodyID(654321), 88888);
	return nullptr;
}

const char* TestContactListener(ContactListener* listener, XPhysicsSystem* system) {
	BodyInterface &bodyItf = system->BodyItf(false);
	BodyCreationSettings settings1(new SphereShape(0.5f), RVec3(13.0f, 3.0f, 0.3f), Quat::sIdentity(), EMotionType::Dynamic, 0);
	Body *body1 = bodyItf.CreateBody(settings1);
	BodyCreationSettings settings2(new SphereShape(0.5f), RVec3(17.0f, 7.0f, 0.7f), Quat::sIdentity(), EMotionType::Dynamic, 0);
	Body *body2 = bodyItf.CreateBody(settings2);

	{
		CollideShapeResult csr;
		csr.mPenetrationDepth = 0.073f;
		JPH::ValidateResult res = listener->OnContactValidate(*body1, *body2, RVec3(4.3f, 5.4f, 0.82f), csr);
		if (res != JPH::ValidateResult::RejectContact) {
			return "OnContactValidate failed";
		}
	}

	{
		ContactManifold cm;
		cm.mPenetrationDepth = 0.028;
		ContactSettings cs;
		cs.mRelativeAngularSurfaceVelocity = RVec3(0.1f, 0.2f, 0.3f);
		listener->OnContactAdded(*body1, *body2, cm, cs);
	}

	{
		ContactManifold cm;
		cm.mPenetrationDepth = 0.103;
		ContactSettings cs;
		cs.mRelativeLinearSurfaceVelocity = RVec3(1.1f, 2.2f, 3.3f);
		listener->OnContactPersisted(*body1, *body2, cm, cs);
	}

	{
		SubShapeIDPair pair;
		listener->OnContactRemoved(pair);
	}
	return nullptr;
}

const char* TestCharacterContactListener(
	CharacterContactListener* listener,
	XPhysicsSystem* system,
	XCharacterVirtual* chara1,
	XCharacterVirtual* chara2
) {
	BodyInterface &bodyItf = system->BodyItf(false);
	BodyCreationSettings settings(new SphereShape(0.5f), RVec3(13.0f, 3.0f, 0.3f), Quat::sIdentity(), EMotionType::Dynamic, 0);
	Body *body = bodyItf.CreateBody(settings);

	{
		Vec3 linearVelocity(2.0f, 3.0f, 4.0f);
		Vec3 angularVelocity(0.5f, 0.6f, 0.7f);
		listener->OnAdjustBodyVelocity(chara1, *body, linearVelocity, angularVelocity);
	}

	{
		SubShapeID subshape;
		subshape.SetValue(999888);
		if (false != listener->OnContactValidate(chara1, BodyID(777666), subshape)) {
			return "OnContactValidate failed";
		}
	}

	{
		SubShapeID subshape;
		subshape.SetValue(12345678);
		if (true != listener->OnCharacterContactValidate(chara1, chara2, subshape)) {
			return "OnCharacterContactValidate failed";
		}
	}
	
	{
		SubShapeID subshape;
		subshape.SetValue(8888);
		CharacterContactSettings csc;
		csc.mCanPushCharacter = true;
		csc.mCanReceiveImpulses = true;
		listener->OnContactAdded(chara1, BodyID(999999), subshape, RVec3(7.0f, 7.0f, 7.0f), RVec3(6.0f, 6.0f, 6.0f), csc);
		if (csc.mCanPushCharacter != false || csc.mCanReceiveImpulses != true) {
			return "OnContactAdded failed";
		}
	}

	{
		SubShapeID subshape;
		subshape.SetValue(1111);
		CharacterContactSettings csc;
		csc.mCanPushCharacter = false;
		csc.mCanReceiveImpulses = false;
		listener->OnCharacterContactAdded(chara1, chara2, subshape, RVec3(5.0f, 5.0f, 5.0f), RVec3(4.0f, 4.0f, 4.0f), csc);
		if (csc.mCanPushCharacter != false || csc.mCanReceiveImpulses != true) {
			return "OnCharacterContactAdded failed";
		}
	}

	{
		SubShapeID subshape;
		subshape.SetValue(55566677);
		Vec3 newVelocity(0.0f, 0.0f, 0.0f);
		listener->OnContactSolve(
			chara1,
			BodyID(22233344),
			subshape,
			RVec3(0.1f, 0.1f, 0.1f),
			RVec3(0.2f, 0.2f, 0.2f),
			RVec3(0.3f, 0.3f, 0.3f),
			nullptr,
			RVec3(0.4f, 0.4f, 0.4f),
			newVelocity
		);
		if (newVelocity != RVec3(9.8f, 8.7f, 7.6f)) {
			return "OnContactSolve failed";
		}
	}

	{
		SubShapeID subshape;
		subshape.SetValue(4000000);
		Vec3 newVelocity(9.9f, 9.9f, 9.9f);
		listener->OnCharacterContactSolve(
			chara1,
			chara2,
			subshape,
			RVec3(0.9f, 0.9f, 0.9f),
			RVec3(0.8f, 0.8f, 0.8f),
			RVec3(0.7f, 0.7f, 0.7f),
			nullptr,
			RVec3(0.6f, 0.6f, 0.6f),
			newVelocity
		);
		if (newVelocity != RVec3(1.2f, 2.3f, 3.4f)) {
			return "OnCharacterContactSolve failed";
		}
	}

	return nullptr;
}
