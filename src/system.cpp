#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/shape.rs.h"
#include "jolt-physics-rs/src/system.rs.h"

static_assert(sizeof(PhysicsSettings) == 84, "PhysicsSettings size");
static_assert(sizeof(CollideShapeResult) == 1120, "CollideShapeResult size");
static_assert(sizeof(ContactManifold) == 2128, "ContactManifold size");

// Callback for traces, connect this to your own trace function if you have one
static void TraceImpl(const char* inFMT, ...) {
	// Format the message
	va_list list;
	va_start(list, inFMT);
	char buffer[1024];
	vsnprintf(buffer, sizeof(buffer), inFMT, list);
	va_end(list);

	// Print to the TTY
	cout << buffer << endl;
}

#ifdef JPH_ENABLE_ASSERTS
// Callback for asserts, connect this to your own assert handler if you have one
static bool AssertFailedImpl(const char* inExpression, const char* inMessage, const char* inFile, uint inLine) {
	// Print to the TTY
	cout << inFile << ":" << inLine << ": (" << inExpression << ") " << (inMessage != nullptr ? inMessage : "") << endl;

	// Breakpoint
	return true;
};
#endif // JPH_ENABLE_ASSERTS

void GlobalInitialize() {
	RegisterDefaultAllocator();
	Trace = TraceImpl;
	JPH_IF_ENABLE_ASSERTS(AssertFailed = AssertFailedImpl;)
	Factory::sInstance = new Factory();
	RegisterTypes();
}

void GlobalFinalize() {
	UnregisterTypes();
	if (Factory::sInstance != nullptr) {
		delete Factory::sInstance;
		Factory::sInstance = nullptr;
	}
}

//
// PhysicsSystem
//

XPhysicsSystem::XPhysicsSystem(
	rust::Fn<void (XPhysicsSystem&)> rustCleanUp,
	const BroadPhaseLayerInterface* bpli,
	const ObjectVsBroadPhaseLayerFilter* obplf,
	const ObjectLayerPairFilter* olpf
):
	_allocator(TempAllocatorImpl(10 * 1024 * 1024)),
	_jobSys(JobSystemThreadPool(cMaxPhysicsJobs, cMaxPhysicsBarriers, 2)),
	_phySys(PhysicsSystem()),
	_rustCleanUp(rustCleanUp),
	_bpli(bpli),
	_obplf(obplf),
	_olpf(olpf)
{
	const uint cMaxBodies = 20480;
	const uint cNumBodyMutexes = 0;
	const uint cMaxBodyPairs = 20480;
	const uint cMaxContactConstraints = 5120;
	_phySys.Init(cMaxBodies, cNumBodyMutexes, cMaxBodyPairs, cMaxContactConstraints, *bpli, *obplf, *olpf);
}

XPhysicsSystem::~XPhysicsSystem() {
	PRINT_ONLY(printf("~XPhysicsSystem %d\n", GetRefCount()));
	this->_rustCleanUp(*this);
}

XBodyInterface* XPhysicsSystem::GetBodyInterface(bool lock) {
	BodyInterface* bodyItf = &this->BodyItf(lock);
	return reinterpret_cast<XBodyInterface*>(bodyItf);
}

uint32 XPhysicsSystem::Update(float delta) {
	return (uint32)this->_phySys.Update(delta, 1, &this->Allocator(), &this->JobSys());
}

void XPhysicsSystem::GetBodies(rust::Vec<BodyID>& bodies) const {
	BodyIDVector tmp;
	this->_phySys.GetBodies(tmp);
	bodies.clear();
	bodies.reserve(tmp.size());
	for (BodyID body_id : tmp) {
		bodies.push_back(body_id);
	}
}

void XPhysicsSystem::GetActiveBodies(EBodyType bodyType, rust::Vec<BodyID>& bodies) const {
	BodyIDVector tmp;
	this->_phySys.GetActiveBodies(bodyType, tmp);
	bodies.clear();
	bodies.reserve(tmp.size());
	for (BodyID body_id : tmp) {
		bodies.push_back(body_id);
	}
}

#if defined(JPH_DEBUG_RENDERER)
void XPhysicsSystem::DebugRender(DebugRenderer* debugRenderer) {
	for (auto renderable : this->_renderables) {
		renderable->Render(debugRenderer);
	}
}
#endif

XPhysicsSystem* CreatePhysicSystem(
	rust::Fn<void (XPhysicsSystem&)> rustCleanUp,
	const BroadPhaseLayerInterface* bpli,
	const ObjectVsBroadPhaseLayerFilter* obplf,
	const ObjectLayerPairFilter* olpf
) {
	Ref<XPhysicsSystem> system = Ref(new XPhysicsSystem(rustCleanUp, bpli, obplf, olpf));
	return LeakRefT<XPhysicsSystem>(system);
}

//
// BodyInterface
//

BodyID XBodyInterface::CreateBody(const BodyCreationSettings& settings) {
	auto body = BodyInterface::CreateBody(settings);
	if (body == nullptr) {
		return BodyID();
	}
	return body->GetID();
}

BodyID XBodyInterface::CreateBodyWithID(const BodyID &bodyId, const BodyCreationSettings& settings) {
	auto body = BodyInterface::CreateBodyWithID(bodyId, settings);
	if (body == nullptr) {
		return BodyID();
	}
	return body->GetID();
}

BodyID XBodyInterface::CreateAddBody(const BodyCreationSettings& settings, EActivation activation) {
	return BodyInterface::CreateAndAddBody(settings, activation);
}
