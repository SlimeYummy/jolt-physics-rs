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
#include <Jolt/Physics/Collision/BroadPhase/BroadPhaseLayer.h>
#include <Jolt/Physics/Collision/CollideShape.h>
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

//
// base
//

typedef EShapeType ShapeType;
typedef EShapeSubType ShapeSubType;
typedef EBodyType BodyType;
typedef EMotionType MotionType;
typedef EMotionQuality MotionQuality;
typedef EAllowedDOFs AllowedDOFs;
typedef EOverrideMassProperties OverrideMassProperties;
typedef EActivation Activation;
typedef ECanSleep CanSleep;

static_assert(sizeof(Vec3) == 16, "Vec3 size");
static_assert(sizeof(Vec4) == 16, "Vec4 size");
static_assert(sizeof(Quat) == 16, "Quat size");
static_assert(sizeof(Mat44) == 64, "Mat44 size");

static_assert(sizeof(Float3) == 12, "Float3 size");
struct Int3 { int32_t x, y, z; };
static_assert(sizeof(Int3) == 12, "Int3 size");
static_assert(sizeof(Plane) == 16, "Plane size");
static_assert(sizeof(IndexedTriangle) == 20, "IndexedTriangle size");

static_assert(sizeof(BodyID) == 4, "BodyID size");
static_assert(sizeof(SubShapeID) == 4, "SubShapeID size");

static_assert(sizeof(Ref<int>) == sizeof(size_t), "RsRef size");

static_assert(sizeof(ObjectLayer) == 4, "ObjectLayer size");
static_assert(sizeof(BroadPhaseLayer) == 1, "BroadPhaseLayer size");

template <class T>
T* LeakRefT(RefConst<T> cxx_ref) {
	T* new_ptr = nullptr;
	reinterpret_cast<RefConst<T>*>(&new_ptr)->operator=(std::move(cxx_ref));
	return new_ptr;
}

template <class T>
void DropRef(T* rs_ptr) {
	if (rs_ptr != nullptr) {
		reinterpret_cast<RefConst<T>*>(&rs_ptr)->~RefConst();
	}
}

template <class T> T* CloneRef(T* rs_ptr) {
	T* new_ptr = nullptr;
	*reinterpret_cast<RefConst<T>*>(&new_ptr) = *reinterpret_cast<RefConst<T>*>(&rs_ptr);
	return new_ptr;
}

template <class T>
uint32 RefCountRef(const T* rs_ptr) {
	return rs_ptr == nullptr ? 0 : (*reinterpret_cast<RefConst<T>*>(&rs_ptr))->GetRefCount();
}

class XPhysicsSystem;

#include "jolt-physics-rs/src/base.rs.h"
#include "jolt-physics-rs/src/layer.rs.h"

//
// shape
//

inline void DropPhysicsMaterial(PhysicsMaterial* ptr) { DropRef<PhysicsMaterial>(ptr); }
inline PhysicsMaterial* ClonePhysicsMaterial(PhysicsMaterial* ptr) { return CloneRef<PhysicsMaterial>(ptr); }
inline uint32 CountRefPhysicsMaterial(const PhysicsMaterial* ptr) { return RefCountRef<PhysicsMaterial>(ptr); }

inline void DropShape(Shape* ptr) { DropRef<Shape>(ptr); }
inline Shape* CloneShape(Shape* ptr) { return CloneRef<Shape>(ptr); }
inline uint32 CountRefShape(const Shape* ptr) { return RefCountRef<Shape>(ptr); }

inline void DropStaticCompoundShape(StaticCompoundShape* ptr) { DropRef<StaticCompoundShape>(ptr); }
inline StaticCompoundShape* CloneStaticCompoundShape(StaticCompoundShape* ptr) { return CloneRef<StaticCompoundShape>(ptr); }
inline uint32 CountRefStaticCompoundShape(const StaticCompoundShape* ptr) { return RefCountRef<StaticCompoundShape>(ptr); }

inline void DropMutableCompoundShape(MutableCompoundShape* ptr) { DropRef<MutableCompoundShape>(ptr); }
inline MutableCompoundShape* CloneMutableCompoundShape(MutableCompoundShape* ptr) { return CloneRef<MutableCompoundShape>(ptr); }
inline uint32 CountRefMutableCompoundShape(const MutableCompoundShape* ptr) { return RefCountRef<MutableCompoundShape>(ptr); }

struct XSphereShapeSettings;
Shape* CreateSphereShape(const XSphereShapeSettings& settings);
struct XBoxShapeSettings;
Shape* CreateBoxShape(const XBoxShapeSettings& settings);
struct XCapsuleShapeSettings;
Shape* CreateCapsuleShape(const XCapsuleShapeSettings& settings);
struct XTaperedCapsuleShapeSettings;
Shape* CreateTaperedCapsuleShape(const XTaperedCapsuleShapeSettings& settings);
struct XCylinderShapeSettings;
Shape* CreateCylinderShape(const XCylinderShapeSettings& settings);
struct XTaperedCylinderShapeSettings;
Shape* CreateTaperedCylinderShape(const XTaperedCylinderShapeSettings& settings);
struct XConvexHullShapeSettings;
Shape* CreateConvexHullShape(const XConvexHullShapeSettings& settings);
struct XTriangleShapeSettings;
Shape* CreateTriangleShape(const XTriangleShapeSettings& settings);
struct XPlaneShapeSettings;
Shape* CreatePlaneShape(const XPlaneShapeSettings& settings);
struct XMeshShapeSettings;
Shape* CreateMeshShape(const XMeshShapeSettings& settings);
struct XHeightFieldShapeSettings;
Shape* CreateHeightFieldShape(const XHeightFieldShapeSettings& settings);
struct XEmptyShapeSettings;
Shape* CreateEmptyShape(const XEmptyShapeSettings& settings);

struct XScaledShapeSettings;
Shape* CreateScaledShape(const XScaledShapeSettings& settings);
struct XRotatedTranslatedShapeSettings;
Shape* CreateRotatedTranslatedShape(const XRotatedTranslatedShapeSettings& settings);
struct XOffsetCenterOfMassShapeSettings;
Shape* CreateOffsetCenterOfMassShape(const XOffsetCenterOfMassShapeSettings& settings);

struct XSubShapeSettings;
struct XStaticCompoundShapeSettings;
StaticCompoundShape* CreateStaticCompoundShape(const XStaticCompoundShapeSettings& settings);
struct XMutableCompoundShapeSettings;
MutableCompoundShape* CreateMutableCompoundShape(const XMutableCompoundShapeSettings& settings);
typedef CompoundShape::SubShape XCompoundSubShape;
static_assert(sizeof(XCompoundSubShape) == 40, "XCompoundSubShape size");

//
// system
//

class XDebugRenderable {
public:
	RENDERER_ONLY(virtual void Render(DebugRenderer* render) const {};)
};

void GlobalInitialize();
void GlobalFinalize();

using XBodyStats = BodyManager::BodyStats;
static_assert(sizeof(XBodyStats) == 36, "XBodyStats size");

class XPhysicsSystem: public RefTarget<XPhysicsSystem> {
private:
	TempAllocatorImpl _allocator;
	JobSystemThreadPool _jobSys;
	PhysicsSystem _phySys;
	RENDERER_ONLY(unordered_set<XDebugRenderable*> _renderables;)

public:
	XPhysicsSystem(const BroadPhaseLayerInterface& bpli, const ObjectVsBroadPhaseLayerFilter& obplf, const ObjectLayerPairFilter& olpf);
	~XPhysicsSystem() { PRINT_ONLY(printf("~XPhysicsSystem %d\n", GetRefCount())); }
	PhysicsSystem& PhySys() { return this->_phySys; }
	JobSystemThreadPool& JobSys() { return this->_jobSys; }
	TempAllocatorImpl& Allocator() { return this->_allocator; }
	BodyInterface& BodyItf(bool lock) { return lock ? this->_phySys.GetBodyInterface() : this->_phySys.GetBodyInterfaceNoLock(); }

public:
	PhysicsSystem* GetPhysicsSystem() { return &this->_phySys; }
	uint32 Update(float delta);
	void GetBodies(rust::Vec<BodyID>& bodies) const;
	void GetActiveBodies(EBodyType bodyType, rust::Vec<BodyID>& bodies) const;
	RENDERER_ONLY(void AddRenderable(XDebugRenderable* renderable) { _renderables.insert(renderable); })
	RENDERER_ONLY(void RemoveRenderable(XDebugRenderable* renderable) { _renderables.erase(renderable); })
	RENDERER_ONLY(void DebugRender(DebugRenderer* debugRenderer);)
};

XPhysicsSystem* CreatePhysicSystem(const BroadPhaseLayerInterface& bpli, const ObjectVsBroadPhaseLayerFilter& obplf, const ObjectLayerPairFilter& olpf);
inline void DropXPhysicsSystem(XPhysicsSystem* ptr) { DropRef<XPhysicsSystem>(ptr); }
inline XPhysicsSystem* CloneXPhysicsSystem(XPhysicsSystem* ptr) { return CloneRef<XPhysicsSystem>(ptr); }
inline uint32 CountRefXPhysicsSystem(const XPhysicsSystem* ptr) { return RefCountRef<XPhysicsSystem>(ptr); }

class XBodyInterface: public BodyInterface {
public:
	~XBodyInterface() { PRINT_ONLY(printf("~XBodyInterface\n")); }
	BodyID CreateBody(const BodyCreationSettings& settings);
	BodyID CreateBodyWithID(const BodyID &bodyId, const BodyCreationSettings& settings);
	BodyID CreateAddBody(const BodyCreationSettings& settings, EActivation activation);
};

XBodyInterface* CreateBodyInterface(XPhysicsSystem* system, bool lock);

//
// character
//

typedef Character::EGroundState GroundState;
typedef EBackFaceMode BackFaceMode;
typedef CharacterVirtual::ExtendedUpdateSettings ExtendedUpdateSettings;

class XCharacter: public Character, public XDebugRenderable {
private:
	Ref<XPhysicsSystem> _system;
public:
	XCharacter(Ref<XPhysicsSystem> system, const CharacterSettings* settings, Vec3 position, Quat rotation, uint64 userData);
	~XCharacter() override;
	RENDERER_ONLY(void Render(DebugRenderer* debugRender) const override;)
};

struct XCharacterSettings;
XCharacter* CreateCharacter(
	XPhysicsSystem* system,
	const XCharacterSettings& settings,
	Vec3 position,
	Quat rotation,
	uint64 userData
);
XCharacter* CreateAddCharacter(
	XPhysicsSystem* system,
	const XCharacterSettings& settings,
	Vec3 position,
	Quat rotation,
	uint64 userData,
	EActivation activation,
	bool lock
);
inline void DropXCharacter(XCharacter* ptr) { DropRef<XCharacter>(ptr); }
inline XCharacter* CloneXCharacter(XCharacter* ptr) { return CloneRef<XCharacter>(ptr); }
inline uint32 CountRefXCharacter(const XCharacter* ptr) { return RefCountRef<XCharacter>(ptr); }

class XCharacterVirtual: public CharacterVirtual, public XDebugRenderable {
private:
	Ref<XPhysicsSystem> _system;

public:
	XCharacterVirtual(Ref<XPhysicsSystem> system, const CharacterVirtualSettings* settings, Vec3 position, Quat rotation);
	~XCharacterVirtual() override;
	void Update(ObjectLayer chara_layer, float deltaTime, Vec3 gravity);
	bool CanWalkStairs(Vec3 velocity) const { return CharacterVirtual::CanWalkStairs(velocity); }
	bool WalkStairs(ObjectLayer chara_layer, float deltaTime, Vec3 stepUp, Vec3 stepForward, Vec3 stepForwardTest, Vec3 stepDownExtra);
	bool StickToFloor(ObjectLayer chara_layer, Vec3 stepDown);
	void ExtendedUpdate(ObjectLayer chara_layer, float deltaTime, Vec3 gravity, const ExtendedUpdateSettings& settings);
	void RefreshContacts(ObjectLayer chara_layer);
	void UpdateGroundVelocity() { CharacterVirtual::UpdateGroundVelocity(); }
	bool SetShape(ObjectLayer chara_layer, const Shape* shape, float maxPenetrationDepth);
	// void CheckCollision(ObjectLayer chara_layer, RsVec3 position, RsQuat rotation, RsVec3 movementDirection, float maxPenetrationDepth, Shape* shape, RsVec3 baseOffset) const;
	RENDERER_ONLY(void Render(DebugRenderer* debugRender) const override;)
};

struct XCharacterVirtualSettings;
XCharacterVirtual* CreateCharacterVirtual(
	XPhysicsSystem* system,
	const XCharacterVirtualSettings& settings,
	Vec3 position,
	Quat rotation
);
inline void DropXCharacterVirtual(XCharacterVirtual* ptr) { DropRef<XCharacterVirtual>(ptr); }
inline XCharacterVirtual* CloneXCharacterVirtual(XCharacterVirtual* ptr) { return CloneRef<XCharacterVirtual>(ptr); }
inline uint32 CountRefXCharacterVirtual(const XCharacterVirtual* ptr) { return RefCountRef<XCharacterVirtual>(ptr); }

//
// Unit tests
//

const char* TestBroadPhaseLayerInterface(const BroadPhaseLayerInterface* itf);
const char* TestObjectVsBroadPhaseLayerFilter(const ObjectVsBroadPhaseLayerFilter* filter);
const char* TestObjectLayerPairFilter(const ObjectLayerPairFilter* filter);
const char* TestBodyActivationListener(BodyActivationListener* listener);
const char* TestContactListener(ContactListener* listener, XPhysicsSystem* system);
const char* TestCharacterContactListener(
	CharacterContactListener* listener,
	XPhysicsSystem* system,
	XCharacterVirtual* chara1,
	XCharacterVirtual* chara2
);

//
// Debug
//

#if defined(JPH_DEBUG_RENDERER)
struct RustDebugApp;
void RunDebugApplication(rust::Box<RustDebugApp> rs_app);
#endif
