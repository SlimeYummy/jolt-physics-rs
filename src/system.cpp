#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/shape.rs.h"
#include "jolt-physics-rs/src/system.rs.h"

static_assert(sizeof(BodyCreationSettings) == 256, "BodyCreationSettings size");
static_assert(sizeof(CollisionGroup) == 16, "CollisionGroup size");
static_assert(sizeof(MassProperties) == 80, "MassProperties size");

// Callback for traces, connect this to your own trace function if you have one
static void TraceImpl(const char* inFMT, ...)
{
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
static bool AssertFailedImpl(const char* inExpression, const char* inMessage, const char* inFile, uint inLine)
{
	// Print to the TTY
	cout << inFile << ":" << inLine << ": (" << inExpression << ") " << (inMessage != nullptr ? inMessage : "") << endl;

	// Breakpoint
	return true;
};

#endif // JPH_ENABLE_ASSERTS

CxxContactCollector::CxxContactCollector(XContactCollector* collector) {
	this->_collector = collector;
}

ValidateResult CxxContactCollector::OnContactValidate(const Body& body1, const Body& body2, RVec3Arg baseOffset, const CollideShapeResult& result) {
	// cout << "Contact validate callback" << endl;
	// Allows you to ignore a contact before it is created (using layers to not make objects collide is cheaper!)
	return ValidateResult::AcceptAllContactsForThisBodyPair;
}

void CxxContactCollector::OnContactAdded(const Body& body1, const Body& body2, const ContactManifold& manifold, ContactSettings& settings) {
	this->_collector->start_hit_event(body1.GetID(), body2.GetID());
}

void CxxContactCollector::OnContactPersisted(const Body& body1, const Body& body2, const ContactManifold& manifold, ContactSettings& settings) {
	// cout << "A contact was persisted" << endl;
}

void CxxContactCollector::OnContactRemoved(const SubShapeIDPair& pair) {
	this->_collector->stop_hit_event(pair.GetBody1ID(), pair.GetBody2ID());
}

// void MyBodyActivationListener::OnBodyActivated(const BodyID& inBodyID, uint64 inBodyUserData) {
// 	cout << "A body got activated" << endl;
// }

// void MyBodyActivationListener::OnBodyDeactivated(const BodyID& inBodyID, uint64 inBodyUserData) {
// 	cout << "A body went to sleep" << endl;
// }

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

XPhysicsSystem::XPhysicsSystem(XContactCollector* contacts):
	_allocator(TempAllocatorImpl(10 * 1024 * 1024)),
	_jobSys(JobSystemThreadPool(cMaxPhysicsJobs, cMaxPhysicsBarriers, 2)),
	_phySys(PhysicsSystem()),
	_contacts(contacts)
{}

Ref<XPhysicsSystem> XPhysicsSystem::Create(XContactCollector* contacts) {
	Ref<XPhysicsSystem> system = Ref(new XPhysicsSystem(contacts));

	const uint cMaxBodies = 20480;
	const uint cNumBodyMutexes = 0;
	const uint cMaxBodyPairs = 20480;
	const uint cMaxContactConstraints = 5120;

	system->_phySys.Init(
		cMaxBodies,
		cNumBodyMutexes,
		cMaxBodyPairs,
		cMaxContactConstraints,
		system->_bpLayerItf,
		system->_objBpLayerFilter,
		system->_objObjLayerFilter
	);

	// system->_phySys.SetBodyActivationListener(&system->body_activation_listener);
	system->_phySys.SetContactListener(&system->_contacts);

	return system;
}

void XPhysicsSystem::Prepare() {
	this->_phySys.OptimizeBroadPhase();
}

uint32 XPhysicsSystem::Update(float delta) {
	return (uint32)this->_phySys.Update(delta, 1, &this->Allocator(), &this->JobSys());
}

#if defined(JPH_DEBUG_RENDERER)
void XPhysicsSystem::DebugRender(DebugRenderer* debugRenderer) {
	for (auto renderable : this->_renderables) {
		renderable->Render(debugRenderer);
	}
}
#endif

XPhysicsSystem* CreatePhysicSystem(XContactCollector* contacts) {
	auto system = XPhysicsSystem::Create(contacts);
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

XBodyInterface* CreateBodyInterface(XPhysicsSystem* system, bool lock) {
	BodyInterface* bodyItf = &system->BodyItf(lock);
	return reinterpret_cast<XBodyInterface*>(bodyItf);
}
