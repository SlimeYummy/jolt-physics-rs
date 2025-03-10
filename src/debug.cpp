#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/debug.h"
#include "jolt-physics-rs/src/system.rs.h"
#include "jolt-physics-rs/src/debug.rs.h"

#include <TestFramework.h>
#include <UI/UIManager.h>
#include <Application/DebugUI.h>
#include <Utils/Log.h>
#include <Utils/CustomMemoryHook.h>
#include <Jolt/Core/FPException.h>
#include <Jolt/Core/Factory.h>
#include <Jolt/RegisterTypes.h>
#include <Renderer/DebugRendererImp.h>
#include <crtdbg.h>

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
#if defined(_DEBUG)
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

    // Create factory
    Factory::sInstance = new Factory;

    // Register physics types with the factory
    RegisterTypes();

    {
        // Disable allocation checking
        DisableCustomMemoryHook dcmh;

        // Create renderer
        mRenderer = new Renderer;
        mRenderer->Initialize();

        // Create font
        Font *font = new Font(mRenderer);
        font->Create("Arial", 24);
        mFont = font;

        // Init debug renderer
        mDebugRenderer = new DebugRendererImp(mRenderer, mFont);

        // Init keyboard
        mKeyboard = new Keyboard;
        mKeyboard->Initialize(mRenderer);

        // Init mouse
        mMouse = new Mouse;
        mMouse->Initialize(mRenderer);

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
    }

    // Unregisters all types with the factory and cleans up the default material
    UnregisterTypes();

    delete Factory::sInstance;
    Factory::sInstance = nullptr;
}

// Clear debug lines / triangles / texts that have been accumulated
void DebugApp::ClearDebugRenderer() {
    JPH_PROFILE_FUNCTION();

    static_cast<DebugRendererImp *>(mDebugRenderer)->Clear();

    mDebugRendererCleared = true;
}

// Main loop
void DebugApp::Run() {
    // Set initial camera position
    ResetCamera();

    // Main message loop
    MSG msg;
    memset(&msg, 0, sizeof(msg));
    while (WM_QUIT != msg.message) {
        if (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
            JPH_PROFILE("DispatchMessage");

            TranslateMessage(&msg);
            bool ignore = (msg.message == WM_SYSKEYDOWN || msg.message == WM_SYSKEYUP) && msg.wParam == VK_MENU; // ignore alt
            if (!ignore) {
                DispatchMessage(&msg);
            }
        } else {
            // Get new input
            mKeyboard->Poll();
            mMouse->Poll();

            // Handle keyboard input
            for (int key = mKeyboard->GetFirstKey(); key != 0; key = mKeyboard->GetNextKey()) {
                if (key == DIK_F1) {
                    mIsPaused = !mIsPaused;
                } else if (key == DIK_F2) {
                    mSingleStep = true;
                } else if (key == DIK_F3) {
                    // Dump timing info to file
                    JPH_PROFILE_DUMP();
                } else if (key == DIK_F4) {
                    mCursorVisible ? mMouse->HideCursor() : mMouse->ShowCursor();
                    mCursorVisible = !mCursorVisible;
                } else if (key == DIK_ESCAPE) {
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
                if (!UpdateFrame(world_delta_time)) break;
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
        }
    }
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
    bool shift = mKeyboard->IsKeyPressed(DIK_LSHIFT) || mKeyboard->IsKeyPressed(DIK_RSHIFT);
    bool control = mKeyboard->IsKeyPressed(DIK_LCONTROL) || mKeyboard->IsKeyPressed(DIK_RCONTROL);
    bool alt = mKeyboard->IsKeyPressed(DIK_LALT) || mKeyboard->IsKeyPressed(DIK_RALT);
    if (shift) {
        speed *= 10.0f;
    } else if (control) {
        speed /= 25.0f;
    } else if (alt) {
        speed = 0.0f;
    }

    // Position
    Vec3 right = mLocalCamera.mForward.Cross(mLocalCamera.mUp);
    if (mKeyboard->IsKeyPressed(DIK_LEFT)) mLocalCamera.mPos -= speed * right;
    if (mKeyboard->IsKeyPressed(DIK_RIGHT)) mLocalCamera.mPos += speed * right;
    if (mKeyboard->IsKeyPressed(DIK_UP)) mLocalCamera.mPos += speed * mLocalCamera.mForward;
    if (mKeyboard->IsKeyPressed(DIK_DOWN)) mLocalCamera.mPos -= speed * mLocalCamera.mForward;

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
    int x = (mRenderer->GetWindowWidth() - text_w) / 2 - 20;
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
        mUI->DrawText(mRenderer->GetWindowWidth() - 5 - int(pause_size.x * mFont->GetCharHeight()), 5, paused_str, mFont);
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
