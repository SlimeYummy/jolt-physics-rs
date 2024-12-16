#pragma once

#ifdef WIN32
#include <Windows.h>
#endif
#include <stdarg.h>
#include <thread>
#include <iostream>

#include "rust/cxx.h"

#include <Jolt/Jolt.h>
#include <Jolt/RegisterTypes.h>
#include <Jolt/Core/Factory.h>
#include <Jolt/Core/TempAllocator.h>
#include <Jolt/Core/JobSystemThreadPool.h>
#include <Jolt/Core/FPException.h>
#include <Jolt/Physics/PhysicsSettings.h>
#include <Jolt/Physics/PhysicsSystem.h>
#include <Jolt/Physics/Collision/Shape/SphereShape.h>
#include <Jolt/Physics/Collision/Shape/BoxShape.h>
#include <Jolt/Physics/Collision/Shape/CapsuleShape.h>
#include <Jolt/Physics/Collision/Shape/TaperedCapsuleShape.h>
#include <Jolt/Physics/Collision/Shape/CylinderShape.h>
#include <Jolt/Physics/Collision/Shape/TaperedCylinderShape.h>
#include <Jolt/Physics/Collision/Shape/ConvexHullShape.h>
#include <Jolt/Physics/Collision/Shape/TriangleShape.h>
#include <Jolt/Physics/Collision/Shape/PlaneShape.h>
#include <Jolt/Physics/Collision/Shape/StaticCompoundShape.h>
#include <Jolt/Physics/Collision/Shape/MutableCompoundShape.h>
#include <Jolt/Physics/Collision/Shape/MeshShape.h>
#include <Jolt/Physics/Collision/Shape/HeightFieldShape.h>
#include <Jolt/Physics/Collision/Shape/EmptyShape.h>
#include <Jolt/Physics/Collision/Shape/ScaledShape.h>
#include <Jolt/Physics/Collision/Shape/RotatedTranslatedShape.h>
#include <Jolt/Physics/Collision/Shape/OffsetCenterOfMassShape.h>
#include <Jolt/Physics/Body/BodyCreationSettings.h>
#include <Jolt/Physics/Body/BodyActivationListener.h>
#include <Jolt/Physics/Character/Character.h>
#include <Jolt/Physics/Character/CharacterVirtual.h>

#ifdef JPH_DEBUG_RENDERER
#include <TestFramework/TestFramework.h>
#include <Application/EntryPoint.h>
#include <Application/Application.h>
#endif

// Disable common warnings triggered by Jolt, you can use JPH_SUPPRESS_WARNING_PUSH / JPH_SUPPRESS_WARNING_POP to store and restore the warning state
JPH_SUPPRESS_WARNINGS

#if defined(JPH_EXTERNAL_PROFILE) || defined(JPH_PROFILE_ENABLED)
#define PROFILE_ONLY(code) code
#else
#define PROFILE_ONLY(code) ;
#endif

#if defined(JPH_DEBUG_RENDERER)
#define RENDERER_ONLY(code) code
#else
#define RENDERER_ONLY(code) ;
#endif

#if defined(JPH_DEBUG_PRINT)
#define PRINT_ONLY(code) code
#else
#define PRINT_ONLY(code) ;
#endif

using namespace JPH;
using namespace std;

constexpr float MARGIN_FACTOR = 0.08f;

//
// base
//

static_assert(sizeof(Vec3) == 16, "Vec3 size");
static_assert(sizeof(Vec4) == 16, "Vec4 size");
static_assert(sizeof(Quat) == 16, "Quat size");
static_assert(sizeof(Mat44) == 64, "Mat44 size");

struct Isometry {
	Vec3 position;
	Quat rotation;
};
static_assert(sizeof(Isometry) == 32, "Isometry size");

struct Transform {
	Vec3 position;
	Quat rotation;
	Vec3 scale;
};
static_assert(sizeof(Transform) == 48, "Transform size");

static_assert(sizeof(Float3) == 12, "Float3 size");
struct Int3 { int32_t x, y, z; };
static_assert(sizeof(Int3) == 12, "Int3 size");
static_assert(sizeof(Plane) == 16, "Plane size");
static_assert(sizeof(IndexedTriangle) == 20, "IndexedTriangle size");

static_assert(sizeof(BodyID) == 4, "BodyID size");
static_assert(sizeof(SubShapeID) == 4, "SubShapeID size");

static_assert(sizeof(Ref<int>) == sizeof(size_t), "RsRef size");
template <class T, class R> R CreateRefT(Ref<T> cxx_ref) {
	R ref = R{};
	reinterpret_cast<Ref<T>*>(&ref)->operator=(cxx_ref);
	return ref;
}
template <class T, class R> void DropRefT(R ref) { if (ref.ptr != nullptr) { reinterpret_cast<Ref<T>*>(&ref)->~Ref(); } }
template <class T, class R> uint32 CountRefT(R ref) { return ref.ptr == nullptr ? 0 : (*reinterpret_cast<Ref<T>*>(&ref))->GetRefCount(); }
template <class T, class R> R CloneRefT(R ref) {
	R rs_ref_new = R{};
	*reinterpret_cast<Ref<T>*>(&rs_ref_new) = *reinterpret_cast<Ref<T>*>(&ref);
	return rs_ref_new;
}
template <class T, class R> Ref<T> AsRefMut(R ref) { return *reinterpret_cast<Ref<T>*>(&ref); }
template <class T, class R> RefConst<T> AsRefConst(R ref) { return *reinterpret_cast<RefConst<T>*>(&ref); }

class XPhysicsSystem;

typedef EShapeType ShapeType;
typedef EShapeSubType ShapeSubType;

#include "jolt-physics-rs/src/base.rs.h"
#include "jolt-physics-rs/src/layer.rs.h"

inline void DropRefShape(XRefShape ref) { DropRefT<Shape>(ref); }
inline XRefShape CloneRefShape(XRefShape ref) { return CloneRefT<Shape>(ref); }
inline uint32 CountRefShape(XRefShape ref) { return CountRefT<Shape>(ref); }

inline void DropRefPhysicsMaterial(XRefPhysicsMaterial ref) { DropRefT<PhysicsMaterial>(ref); }
inline XRefPhysicsMaterial CloneRefPhysicsMaterial(XRefPhysicsMaterial ref) { return CloneRefT<PhysicsMaterial>(ref); }
inline uint32 CountRefPhysicsMaterial(XRefPhysicsMaterial ref) { return CountRefT<PhysicsMaterial>(ref); }

//
// shape
//

struct SphereSettings;
XRefShape CreateShapeSphere(const SphereSettings& settings);
struct BoxSettings;
XRefShape CreateShapeBox(const BoxSettings& settings);
struct CapsuleSettings;
XRefShape CreateShapeCapsule(const CapsuleSettings& settings);
struct TaperedCapsuleSettings;
XRefShape CreateShapeTaperedCapsule(const TaperedCapsuleSettings& settings);
struct CylinderSettings;
XRefShape CreateShapeCylinder(const CylinderSettings& settings);
struct TaperedCylinderSettings;
XRefShape CreateShapeTaperedCylinder(const TaperedCylinderSettings& settings);
struct ConvexHullSettings;
XRefShape CreateShapeConvexHull(const ConvexHullSettings& settings);
struct TriangleSettings;
XRefShape CreateShapeTriangle(const TriangleSettings& settings);
struct PlaneSettings;
XRefShape CreateShapePlane(const PlaneSettings& settings);
struct MeshSettings;
XRefShape CreateShapeMesh(const MeshSettings& settings);
struct HeightFieldSettings;
XRefShape CreateShapeHeightField(const HeightFieldSettings& settings);
struct EmptySettings;
XRefShape CreateShapeEmpty(const EmptySettings& settings);

struct ScaledSettings;
XRefShape CreateShapeScaled(const ScaledSettings& settings);
struct RotatedTranslatedSettings;
XRefShape CreateShapeRotatedTranslated(const RotatedTranslatedSettings& settings);
struct OffsetCenterOfMassSettings;
XRefShape CreateShapeOffsetCenterOfMass(const OffsetCenterOfMassSettings& settings);

struct SubShapeSettings;
struct SubShape;
struct StaticCompoundSettings;
XRefShape CreateShapeStaticCompound(const StaticCompoundSettings& settings);
struct MutableCompoundSettings;
XRefShape CreateShapeMutableCompound(const MutableCompoundSettings& settings);

inline const SubShape* CompoundShapeGetSubShape(XRefShape ref, uint idx) { return (const SubShape*)&AsRefConst<CompoundShape>(ref)->GetSubShape(idx); }

//
// system
//

class DebugRenderable {
public:
	RENDERER_ONLY(virtual void Render(DebugRenderer* render) const {};)
};

class BPLayerInterfaceImpl final: public BroadPhaseLayerInterface {
	virtual uint GetNumBroadPhaseLayers() const override { return 3; }
	virtual BroadPhaseLayer GetBroadPhaseLayer(ObjectLayer bp) const override { return BroadPhaseLayer(RsObjToBpLayer(bp)); }
	PROFILE_ONLY(virtual const char* GetBroadPhaseLayerName(BroadPhaseLayer bp) const override { return RsBpLayerName(uint8(bp)).data(); })
};

class ObjectVsBroadPhaseLayerFilterImpl: public ObjectVsBroadPhaseLayerFilter {
public:
	virtual bool ShouldCollide(ObjectLayer obj, BroadPhaseLayer bp) const override { return RsObjBpLayerFilter(obj, uint8(bp)); }
};

class ObjectLayerPairFilterImpl: public ObjectLayerPairFilter {
public:
	virtual bool ShouldCollide(ObjectLayer obj1, ObjectLayer obj2) const override { return RsObjObjLayerFilter(obj1, obj2); }
};

struct XContactCollector;
class CxxContactCollector: public ContactListener {
private:
	XContactCollector* _collector;
public:
	CxxContactCollector(XContactCollector* collector);
	virtual ValidateResult OnContactValidate(const Body& body1, const Body& body2, RVec3Arg baseOffset, const CollideShapeResult& result) override;
	virtual void OnContactAdded(const Body& body1, const Body& body2, const ContactManifold& manifold, ContactSettings& settings) override;
	virtual void OnContactPersisted(const Body& body1, const Body& body2, const ContactManifold& manifold, ContactSettings& settings) override;
	virtual void OnContactRemoved(const SubShapeIDPair& pair) override;
};

typedef EMotionType MotionType;
typedef EMotionQuality MotionQuality;
typedef EAllowedDOFs AllowedDOFs;
typedef EOverrideMassProperties OverrideMassProperties;
typedef EActivation Activation;

void GlobalInitialize();
void GlobalFinalize();

class XPhysicsSystem: public RefTarget<XPhysicsSystem> {
private:
	TempAllocatorImpl _allocator;
	JobSystemThreadPool _jobSys;
	PhysicsSystem _phySys;

	BPLayerInterfaceImpl _bpLayerItf;
	ObjectVsBroadPhaseLayerFilterImpl _objBpLayerFilter;
	ObjectLayerPairFilterImpl _objObjLayerFilter;

	CxxContactCollector _contacts;
	RENDERER_ONLY(unordered_set<DebugRenderable*> _renderables;)

private:
	XPhysicsSystem(XContactCollector* contacts);

public:
	~XPhysicsSystem() { PRINT_ONLY(printf("~XPhysicsSystem\n")); }
	static Ref<XPhysicsSystem> Create(XContactCollector* contacts);
	PhysicsSystem& PhySys() { return this->_phySys; }
	JobSystemThreadPool& JobSys() { return this->_jobSys; }
	TempAllocatorImpl& Allocator() { return this->_allocator; }
	BodyInterface& BodyItf(bool lock) { return lock ? this->_phySys.GetBodyInterface() : this->_phySys.GetBodyInterfaceNoLock(); }

public:
	void Prepare();
	uint32 Update(float delta);
	BodyID CreateBody(const BodyCreationSettings& settings, bool lock);
	Vec3 GetGravity() const { return this->_phySys.GetGravity(); }
	RENDERER_ONLY(void AddRenderable(DebugRenderable* renderable) { _renderables.insert(renderable); })
	RENDERER_ONLY(void RemoveRenderable(DebugRenderable* renderable) { _renderables.erase(renderable); })
	RENDERER_ONLY(void DebugRender(DebugRenderer* debugRenderer);)
};

XRefPhysicsSystem CreatePhysicSystem(XContactCollector* contacts);
inline void DropRefPhysicsSystem(XRefPhysicsSystem ref) { DropRefT<XPhysicsSystem>(ref); }
inline XRefPhysicsSystem CloneRefPhysicsSystem(XRefPhysicsSystem ref) { return CloneRefT<XPhysicsSystem>(ref); }
inline uint32 CountRefPhysicsSystem(XRefPhysicsSystem ref) { return CountRefT<XPhysicsSystem>(ref); }

class XBodyInterface: public BodyInterface {
public:
	~XBodyInterface() { PRINT_ONLY(printf("~XBodyInterface\n")); }
	BodyID CreateBody(const BodyCreationSettings& settings);
	BodyID CreateAddBody(const BodyCreationSettings& settings, EActivation activation);
	Isometry GetPositionAndRotation(const BodyID& bodyId) const;
};

XBodyInterface* CreateBodyInterface(XPhysicsSystem* system, bool lock);

//
// character
//

typedef Character::EGroundState GroundState;
typedef EBackFaceMode BackFaceMode;
typedef CharacterVirtual::ExtendedUpdateSettings ExtendedUpdateSettings;

class XCharacterCommon: public Character, public DebugRenderable {
private:
	XPhysicsSystem* _system;
public:
	XCharacterCommon(XPhysicsSystem* system, const CharacterSettings* settings, Vec3 position, Quat rotation, uint64 userData);
	~XCharacterCommon() override;
	XRefShape GetShape() const { return CreateRefT<Shape, XRefShape>(const_cast<Shape*>(Character::GetShape())); }
	Isometry GetPositionAndRotation(bool lock) const;
	bool SetShape(XRefShape shape, float maxPenetrationDepth, bool lock);
	// TODO: CheckCollision()
	RENDERER_ONLY(void Render(DebugRenderer* debugRender) const override;)
};

struct XCharacterCommonSettings;
unique_ptr<XCharacterCommon> CreateCharacterCommon(
	XPhysicsSystem* system,
	const XCharacterCommonSettings& settings,
	Vec3 position,
	Quat rotation,
	uint64 userData
);
unique_ptr<XCharacterCommon> CreateAddCharacterCommon(
	XPhysicsSystem* system,
	const XCharacterCommonSettings& settings,
	Vec3 position,
	Quat rotation,
	uint64 userData,
	EActivation activation,
	bool lock
);

class XCharacterVirtual: public CharacterVirtual, public CharacterContactListener, public DebugRenderable {
private:
	XPhysicsSystem* _system;
public:
	XCharacterVirtual(XPhysicsSystem* system, const CharacterVirtualSettings* settings, Vec3 position, Quat rotation);
	~XCharacterVirtual() override;
	XRefShape GetShape() const { return CreateRefT<Shape, XRefShape>(const_cast<Shape*>(CharacterVirtual::GetShape())); }
	void Update(ObjectLayer chara_layer, float deltaTime, Vec3 gravity);
	bool CanWalkStairs(Vec3 velocity) const { return CharacterVirtual::CanWalkStairs(velocity); }
	bool WalkStairs(ObjectLayer chara_layer, float deltaTime, Vec3 stepUp, Vec3 stepForward, Vec3 stepForwardTest, Vec3 stepDownExtra);
	bool StickToFloor(ObjectLayer chara_layer, Vec3 stepDown);
	void ExtendedUpdate(ObjectLayer chara_layer, float deltaTime, Vec3 gravity, const ExtendedUpdateSettings& settings);
	void RefreshContacts(ObjectLayer chara_layer);
	void UpdateGroundVelocity() { CharacterVirtual::UpdateGroundVelocity(); }
	bool SetShape(ObjectLayer chara_layer, XRefShape shape, float maxPenetrationDepth);
	// void CheckCollision(ObjectLayer chara_layer, RsVec3 position, RsQuat rotation, RsVec3 movementDirection, float maxPenetrationDepth, XRefShape shape, RsVec3 baseOffset) const;
	RENDERER_ONLY(void Render(DebugRenderer* debugRender) const override;)
public:
	void OnAdjustBodyVelocity(const CharacterVirtual* chara, const Body& body2, Vec3& linearVelocity, Vec3& angularVelocity) override;
	bool OnContactValidate(const CharacterVirtual* chara, const BodyID& body2, const SubShapeID& shape2) override;
	void OnContactAdded(
		const CharacterVirtual* chara,
		const BodyID& body,
		const SubShapeID& shape2,
		Vec3 contactPosition,
		Vec3 contactNormal,
		CharacterContactSettings& settings
	) override;
	void OnContactSolve(
		const CharacterVirtual* chara,
		const BodyID& body2,
		const SubShapeID& shape2,
		Vec3 contactPosition,
		Vec3 contactNormal,
		Vec3 contactVelocity,
		const PhysicsMaterial* contactMaterial,
		Vec3 charaVelocity,
		Vec3& newCharaVelocity
	) override;
};

struct XCharacterVirtualSettings;
unique_ptr<XCharacterVirtual> CreateCharacterVirtual(
	XPhysicsSystem* system,
	const XCharacterVirtualSettings& settings,
	Vec3 position,
	Quat rotation
);

//
// Debug
//

#if defined(JPH_DEBUG_RENDERER)

struct XDebugApp;
void RunDebugApplication(rust::Box<XDebugApp> rs_app);
#endif
