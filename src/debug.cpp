#if defined(JPH_DEBUG_RENDERER)

#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/system.rs.h"
#include "jolt-physics-rs/src/debug.rs.h"

static_assert(sizeof(CameraState) == 64, "CameraState size");

class DebugApplication: public Application {
private:
	bool _inited = false;
	int _steps = 1;
	rust::Box<XDebugApplication> _rs_app;
	Ref<XPhysicsSystem> _system;

public:
	DebugApplication(rust::Box<XDebugApplication> rs_app): _rs_app(move(rs_app)) {
		auto system = this->_rs_app->Initialize();
		PRINT_ONLY(printf("DebugApplication %d\n", system->GetRefCount()));
		this->_system = Ref(system);
		PRINT_ONLY(printf("DebugApplication %d\n", this->_system->GetRefCount()));
	}

	~DebugApplication() {
		PRINT_ONLY(printf("~DebugApplication %d\n", this->_system->GetRefCount()));
	}

	virtual void GetInitialCamera(CameraState& state) const override {
		// This will become the local space offset, look down the x axis and slightly down
		state.mPos = RVec3::sZero();
		state.mForward = Vec3(10.0f, -2.0f, 0).Normalized();
	}

	virtual bool RenderFrame(float delta) override {
		auto ret = this->_rs_app->RenderFrame(delta, this->GetCamera(), this->mKeyboard);
		
        BodyManager::DrawSettings settings;
        settings.mDrawShape = true;
        //settings.mDrawShapeWireframe = true;
        settings.mDrawBoundingBox = true;
        settings.mDrawVelocity = true;
        settings.mDrawSleepStats = true;
        this->_system->PhySys().DrawBodies(settings, this->mDebugRenderer);

		this->_system->DebugRender(this->mDebugRenderer);

        return ret;
	}

	virtual Mat44 GetCameraPivot(float inCameraHeading, float inCameraPitch) const override {
		auto position = this->_rs_app->GetCameraPivot(inCameraHeading, inCameraPitch);
		return Mat44::sTranslation(position);
	}
};

void RunDebugApplication(rust::Box<XDebugApplication> rs_app) {
	RegisterDefaultAllocator();
	JPH_PROFILE_START("Main");
	FPExceptionsEnable enable_exceptions;
	JPH_UNUSED(enable_exceptions);
	{
		DebugApplication app(move(rs_app));
		app.Run();
	}
	JPH_PROFILE_END();
}

#endif
