// Jolt Physics Library (https://github.com/jrouwe/JoltPhysics)
// SPDX-FileCopyrightText: 2021 Jorrit Rouwe
// SPDX-License-Identifier: MIT

#include <Renderer/Renderer.h>
#include <Renderer/Font.h>
#include <Input/Keyboard.h>
#include <Input/Mouse.h>
#include <Jolt/Core/Reference.h>
#include <chrono>

class DebugApp {
private:
    CameraState mLocalCamera;
    CameraState mWorldCamera;

protected:
    DebugRenderer* mDebugRenderer;
    Renderer* mRenderer;
    RefConst<Font> mFont;
    Keyboard* mKeyboard;
    Mouse* mMouse;
    UIManager* mUI;
    DebugUI* mDebugUI;
    String mStatusString; // A string that is shown on screen to indicate the status of the application
    
    rust::Box<XDebugApp> mRsApp;
    Ref<XPhysicsSystem> mPhySys;
    bool mCursorVisible = true;

public:
    DebugApp(rust::Box<XDebugApp> rsApp);
    virtual ~DebugApp();
    void Run();

protected:
    // Update the application
    virtual bool UpdateFrame(float inDeltaTime);

    // Pause / unpause the simulation
    void Pause(bool inPaused) { mIsPaused = inPaused; }

    // Programmatically single step the simulation
    void SingleStep() { mIsPaused = true; mSingleStep = true; }

    // Set the frequency at which we want to render frames
    void SetRenderFrequency(float inFrequency) { mRequestedDeltaTime = 1.0f / inFrequency; }

    // Will restore camera position to that returned by GetInitialCamera
    void ResetCamera();

    // Override to specify the initial camera state (local to GetCameraPivot)
    virtual void GetInitialCamera(CameraState& ioState);

    // Override to specify a camera pivot point and orientation (world space)
    virtual RMat44 GetCameraPivot(float inCameraHeading, float inCameraPitch);

    // Get current state of the camera (world space)
    const CameraState& GetCamera() const { return mWorldCamera; }

    // Clear debug lines / triangles / texts that have been accumulated
    void ClearDebugRenderer();

private:
    // Extract heading and pitch from the local space (relative to the camera pivot) camera forward
    void GetCameraLocalHeadingAndPitch(float& outHeading, float& outPitch);

    // Convert local space camera to world space camera
    void ConvertCameraLocalToWorld(float inCameraHeading, float inCameraPitch);

    // Update the local and world space camera transform
    void UpdateCamera(float inDeltaTime);

    // Draw the frame rate counter
    void DrawFPS(float inDeltaTime);

    chrono::high_resolution_clock::time_point mLastUpdateTime;
    bool mIsPaused = false;
    bool mSingleStep = false;
    bool mDebugRendererCleared = true;
    bool mLeftMousePressed = false;
    float mFPS = 0.0f;
    float mRequestedDeltaTime = 0.0f;
    float mResidualDeltaTime = 0.0f;
    float mTotalDeltaTime = 0.0f;
    int mNumFrames = 0;
};
