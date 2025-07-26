#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/debug.h"
#include "jolt-physics-rs/src/system.rs.h"
#include "jolt-physics-rs/src/debug.rs.h"

#include <TestFramework.h>
#include <UI/UIManager.h>
#include <Application/DebugUI.h>
#include <Utils/Log.h>
#include <Utils/CustomMemoryHook.h>
#include <Jolt/Core/Factory.h>
#include <Jolt/RegisterTypes.h>
#include <Renderer/DebugRendererImp.h>
#ifdef JPH_PLATFORM_WINDOWS
    #include <crtdbg.h>
    #include <Input/Win/KeyboardWin.h>
    #include <Input/Win/MouseWin.h>
    #include <Window/ApplicationWindowWin.h>
#elif defined(JPH_PLATFORM_LINUX)
    #include <Input/Linux/KeyboardLinux.h>
    #include <Input/Linux/MouseLinux.h>
    #include <Window/ApplicationWindowLinux.h>
#elif defined(JPH_PLATFORM_MACOS)
    #include <Input/MacOS/KeyboardMacOS.h>
    #include <Input/MacOS/MouseMacOS.h>
    #include <Window/ApplicationWindowMacOS.h>
#endif

const float WORLD_SCALE = 1.0f;

static_assert(sizeof(CameraState) == 64, "CameraState size");

//
// JoltPhysics Application (modified version)
//

DebugApp::DebugApp(rust::Box<RustDebugApp> rsApp) :
    mDebugRenderer(nullptr),
    mRenderer(nullptr),
    mKeyboard(nullptr),
    mMouse(nullptr),
    mUI(nullptr),
    mDebugUI(nullptr),
    mRsApp(move(rsApp))
{
#if defined(JPH_PLATFORM_WINDOWS) && defined(_DEBUG)
    // Enable leak detection
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif

    // Register trace implementation
    Trace = TraceImpl;

#ifdef JPH_ENABLE_ASSERTS
    // Register assert failed handler
    AssertFailed = [](const char *inExpression, const char *inMessage, const char *inFile, uint inLine) {
        Trace("%s (%d): Assert Failed: %s", inFile, inLine, inMessage != nullptr? inMessage : inExpression);
        return true;
    };
#endif // JPH_ENABLE_ASSERTS

    {
        // Disable allocation checking
        DisableCustomMemoryHook dcmh;

        // Create window
#ifdef JPH_PLATFORM_WINDOWS
        mWindow = new ApplicationWindowWin;
#elif defined(JPH_PLATFORM_LINUX)
        mWindow = new ApplicationWindowLinux;
#elif defined(JPH_PLATFORM_MACOS)
        mWindow = new ApplicationWindowMacOS;
#else
        #error No window defined
#endif
        mWindow->Initialize("Jolt Physics Debugger");

        // Create renderer
        mRenderer = Renderer::sCreate();
        mRenderer->Initialize(mWindow);

        // Create font
        Font *font = new Font(mRenderer);
        font->Create("Roboto-Regular", 24);
        mFont = font;

        // Init debug renderer
        mDebugRenderer = new DebugRendererImp(mRenderer, mFont);

        // Init keyboard
#ifdef JPH_PLATFORM_WINDOWS
        mKeyboard = new KeyboardWin;
#elif defined(JPH_PLATFORM_LINUX)
        mKeyboard = new KeyboardLinux;
#elif defined(JPH_PLATFORM_MACOS)
        mKeyboard = new KeyboardMacOS;
#else
        #error No keyboard defined
#endif
        mKeyboard->Initialize(mWindow);

        // Init mouse
#ifdef JPH_PLATFORM_WINDOWS
        mMouse = new MouseWin;
#elif defined(JPH_PLATFORM_LINUX)
        mMouse = new MouseLinux;
#elif defined(JPH_PLATFORM_MACOS)
        mMouse = new MouseMacOS;
#else
        #error No mouse defined
#endif
        mMouse->Initialize(mWindow);

        // Init UI
        mUI = new UIManager(mRenderer);
        mUI->SetVisible(false);

        // Init debug UI
        mDebugUI = new DebugUI(mUI, mFont);
    }

    // Get initial time
    mLastUpdateTime = chrono::high_resolution_clock::now();
    
    // Rust section
    {
        auto phySys = this->mRsApp->GetPhysicsSystem();
        PRINT_ONLY(printf("DebugApplication system %d\n", phySys->GetRefCount()));
        this->mPhySys = Ref(phySys);
        PRINT_ONLY(printf("DebugApplication system %d\n", this->mPhySys->GetRefCount()));
    }
}

DebugApp::~DebugApp() {
    // Rust section
    {
        PRINT_ONLY(printf("~DebugApplication system %d\n", this->mPhySys->GetRefCount() - 1));
    }

    {
        // Disable allocation checking
        DisableCustomMemoryHook dcmh;

        delete mDebugUI;
        delete mUI;
        delete mMouse;
        delete mKeyboard;
        delete mDebugRenderer;
        mFont = nullptr;
        delete mRenderer;
        delete mWindow;
    }
}

// Clear debug lines / triangles / texts that have been accumulated
void DebugApp::ClearDebugRenderer() {
    JPH_PROFILE_FUNCTION();

    static_cast<DebugRendererImp *>(mDebugRenderer)->Clear();

    mDebugRendererCleared = true;
}

void DebugApp::Run()
{
    // Set initial camera position
    ResetCamera();

    // Enter the main loop
    mWindow->MainLoop([this]() { return RenderFrame(); });
}

// Main loop
bool DebugApp::RenderFrame()
{
    // Get new input
    mKeyboard->Poll();
    mMouse->Poll();

    // Handle keyboard input
    for (EKey key = mKeyboard->GetFirstKey(); key != EKey::Invalid; key = mKeyboard->GetNextKey()) {
        if (key == EKey::P) {
            mIsPaused = !mIsPaused;
        } else if (key == EKey::O) {
            mSingleStep = true;
        } else if (key == EKey::I) {
            // Dump timing info to file
            JPH_PROFILE_DUMP();
        } else if (key == EKey::U) {
            mCursorVisible ? mMouse->HideCursor() : mMouse->ShowCursor();
            mCursorVisible = !mCursorVisible;
        } else if (key == EKey::Escape) {
            mDebugUI->ToggleVisibility();
        }
    }

    // Calculate delta time
    chrono::high_resolution_clock::time_point time = chrono::high_resolution_clock::now();
    chrono::microseconds delta = chrono::duration_cast<chrono::microseconds>(time - mLastUpdateTime);
    mLastUpdateTime = time;
    float clock_delta_time = 1.0e-6f * delta.count();
    float world_delta_time = 0.0f;
    if (mRequestedDeltaTime <= 0.0f) {
        // If no fixed frequency update is requested, update with variable time step
        world_delta_time = !mIsPaused || mSingleStep? clock_delta_time : 0.0f;
        mResidualDeltaTime = 0.0f;
    } else {
        // Else use fixed time steps
        if (mSingleStep) {
            // Single step
            world_delta_time = mRequestedDeltaTime;
        }
        else if (!mIsPaused) {
            // Calculate how much time has passed since the last render
            world_delta_time = clock_delta_time + mResidualDeltaTime;
            if (world_delta_time < mRequestedDeltaTime) {
                // Too soon, set the residual time and don't update
                mResidualDeltaTime = world_delta_time;
                world_delta_time = 0.0f;
            } else {
                // Update and clamp the residual time to a full update to avoid spiral of death
                mResidualDeltaTime = min(mRequestedDeltaTime, world_delta_time - mRequestedDeltaTime);
                world_delta_time = mRequestedDeltaTime;
            }
        }
    }
    mSingleStep = false;

    // Clear debug lines if we're going to step
    if (world_delta_time > 0.0f) {
        ClearDebugRenderer();
    }

    {
        JPH_PROFILE("UpdateFrame");
        if (!UpdateFrame(world_delta_time)) return false;
    }

    // Draw coordinate axis
    if (mDebugRendererCleared) {
        mDebugRenderer->DrawCoordinateSystem(RMat44::sIdentity());
    }

    // For next frame: mark that we haven't cleared debug stuff
    mDebugRendererCleared = false;

    // Update the camera position
    if (!mUI->IsVisible()) {
        UpdateCamera(clock_delta_time);
    }

    // Start rendering
    mRenderer->BeginFrame(mWorldCamera, WORLD_SCALE);

    // Draw from light
    static_cast<DebugRendererImp *>(mDebugRenderer)->DrawShadowPass();

    // Start drawing normally
    mRenderer->EndShadowPass();

    // Draw debug information
    static_cast<DebugRendererImp *>(mDebugRenderer)->Draw();

    // Draw the frame rate counter
    DrawFPS(clock_delta_time);

    if (mUI->IsVisible()) {
        // Send mouse input to UI
        bool left_pressed = mMouse->IsLeftPressed();
        if (left_pressed && !mLeftMousePressed)
            mUI->MouseDown(mMouse->GetX(), mMouse->GetY());
        else if (!left_pressed && mLeftMousePressed)
            mUI->MouseUp(mMouse->GetX(), mMouse->GetY());
        mLeftMousePressed = left_pressed;
        mUI->MouseMove(mMouse->GetX(), mMouse->GetY());

        {
            // Disable allocation checking
            DisableCustomMemoryHook dcmh;

            // Update and draw the menu
            mUI->Update(clock_delta_time);
            mUI->Draw();
        }
    } else {
        // Menu not visible, cancel any mouse operations
        mUI->MouseCancel();
    }

    // Show the frame
    mRenderer->EndFrame();

    // Notify of next frame
    JPH_PROFILE_NEXTFRAME();

    return true;
}

void DebugApp::GetCameraLocalHeadingAndPitch(float &outHeading, float &outPitch) {
    outHeading = ATan2(mLocalCamera.mForward.GetZ(), mLocalCamera.mForward.GetX());
    outPitch = ATan2(mLocalCamera.mForward.GetY(), Vec3(mLocalCamera.mForward.GetX(), 0, mLocalCamera.mForward.GetZ()).Length());
}

void DebugApp::ConvertCameraLocalToWorld(float inCameraHeading, float inCameraPitch) {
    // Convert local to world space using the camera pivot
    RMat44 pivot = GetCameraPivot(inCameraHeading, inCameraPitch);
    mWorldCamera = mLocalCamera;
    mWorldCamera.mPos = pivot * mLocalCamera.mPos;
    mWorldCamera.mForward = pivot.Multiply3x3(mLocalCamera.mForward);
    mWorldCamera.mUp = pivot.Multiply3x3(mLocalCamera.mUp);
}

void DebugApp::ResetCamera() {
    // Get local space camera state
    mLocalCamera = CameraState();
    GetInitialCamera(mLocalCamera);

    // Convert to world space
    float heading, pitch;
    GetCameraLocalHeadingAndPitch(heading, pitch);
    ConvertCameraLocalToWorld(heading, pitch);
}

// Update camera position
void DebugApp::UpdateCamera(float inDeltaTime) {
    JPH_PROFILE_FUNCTION();

    // Determine speed
    float speed = 10.0f * inDeltaTime;
	bool shift = mKeyboard->IsKeyPressed(EKey::LShift) || mKeyboard->IsKeyPressed(EKey::RShift);
	bool control = mKeyboard->IsKeyPressed(EKey::LControl) || mKeyboard->IsKeyPressed(EKey::RControl);
	bool alt = mKeyboard->IsKeyPressed(EKey::LAlt) || mKeyboard->IsKeyPressed(EKey::RAlt);
    if (shift) {
        speed *= 10.0f;
    } else if (control) {
        speed /= 25.0f;
    } else if (alt) {
        speed = 0.0f;
    }

    // Position
    Vec3 right = mLocalCamera.mForward.Cross(mLocalCamera.mUp);
    if (mKeyboard->IsKeyPressed(EKey::Left)) mLocalCamera.mPos -= speed * right;
    if (mKeyboard->IsKeyPressed(EKey::Right)) mLocalCamera.mPos += speed * right;
    if (mKeyboard->IsKeyPressed(EKey::Up)) mLocalCamera.mPos += speed * mLocalCamera.mForward;
    if (mKeyboard->IsKeyPressed(EKey::Down)) mLocalCamera.mPos -= speed * mLocalCamera.mForward;

    // Forward
    float heading, pitch;
    GetCameraLocalHeadingAndPitch(heading, pitch);
    heading += DegreesToRadians(mMouse->GetDX() * 0.5f);
    pitch = Clamp(pitch - DegreesToRadians(mMouse->GetDY() * 0.5f), -0.49f * JPH_PI, 0.49f * JPH_PI);
    mLocalCamera.mForward = Vec3(Cos(pitch) * Cos(heading), Sin(pitch), Cos(pitch) * Sin(heading));

    // Convert to world space
    ConvertCameraLocalToWorld(heading, pitch);
}

void DebugApp::DrawFPS(float inDeltaTime) {
    JPH_PROFILE_FUNCTION();

    // Don't divide by zero
    if (inDeltaTime <= 0.0f) return;

    // Switch tho ortho mode
    mRenderer->SetOrthoMode();

    // Update stats
    mTotalDeltaTime += inDeltaTime;
    mNumFrames++;
    if (mNumFrames > 10) {
        mFPS = mNumFrames / mTotalDeltaTime;
        mNumFrames = 0;
        mTotalDeltaTime = 0.0f;
    }

    // Create string
    String fps = StringFormat("%.1f", (double)mFPS);

    // Get size of text on screen
    Float2 text_size = mFont->MeasureText(fps);
    int text_w = int(text_size.x * mFont->GetCharHeight());
    int text_h = int(text_size.y * mFont->GetCharHeight());

    // Draw FPS counter
    int x = (mWindow->GetWindowWidth() - text_w) / 2 - 20;
    int y = 10;
    mUI->DrawQuad(x - 5, y - 3, text_w + 10, text_h + 6, UITexturedQuad(), Color(0, 0, 0, 128));
    mUI->DrawText(x, y, fps, mFont);

    // Draw status string
    if (!mStatusString.empty()) {
        mUI->DrawText(5, 5, mStatusString, mFont);
    }

    // Draw paused string if the app is paused
    if (mIsPaused) {
        string_view paused_str = "P: Unpause, ESC: Menu";
        Float2 pause_size = mFont->MeasureText(paused_str);
        mUI->DrawText(mWindow->GetWindowWidth() - 5 - int(pause_size.x * mFont->GetCharHeight()), 5, paused_str, mFont);
    }

    // Restore state
    mRenderer->SetProjectionMode();
}

//
// Rust bridge functions
//
bool DebugApp::UpdateFrame(float delta) {
    auto ret = this->mRsApp->UpdateFrame(delta, this->GetCamera(), this->mMouse, this->mKeyboard);
    
    BodyManager::DrawSettings settings;
    settings.mDrawShape = true;
    //settings.mDrawShapeWireframe = true;
    settings.mDrawBoundingBox = true;
    settings.mDrawVelocity = true;
    settings.mDrawSleepStats = true;
    this->mPhySys->PhySys().DrawBodies(settings, this->mDebugRenderer);

    this->mPhySys->DebugRender(this->mDebugRenderer);

    return ret;
}

void DebugApp::GetInitialCamera(CameraState& state) {
    this->mRsApp->GetInitialCamera(&state);
}

RMat44 DebugApp::GetCameraPivot(float cameraHeading, float cameraPitch) {
    auto position = this->mRsApp->GetCameraPivot(cameraHeading, cameraPitch);
    return Mat44::sTranslation(position);
}

void RunDebugApplication(rust::Box<RustDebugApp> rsApp) {
    RegisterDefaultAllocator();
    JPH_PROFILE_START("Main");
    FPExceptionsEnable enableExceptions;
    JPH_UNUSED(enableExceptions);
    {
        DebugApp app(move(rsApp));
        app.Run();
    }
    JPH_PROFILE_END();
}
