#pragma once

#ifdef WIN32
#include <Windows.h>
#endif
#include <stdarg.h>
#include <thread>
#include <iostream>

#include <Jolt/Jolt.h>
#include <Jolt/RegisterTypes.h>
#include <Jolt/Core/Factory.h>
#include <Jolt/Core/TempAllocator.h>
#include <Jolt/Core/JobSystemThreadPool.h>
#include <Jolt/Core/FPException.h>
#include <Jolt/Physics/PhysicsSettings.h>
#include <Jolt/Physics/PhysicsSystem.h>
#include <Jolt/Physics/Collision/Shape/BoxShape.h>
#include <Jolt/Physics/Collision/Shape/SphereShape.h>
#include <Jolt/Physics/Collision/Shape/CapsuleShape.h>
#include <Jolt/Physics/Collision/Shape/TaperedCapsuleShape.h>
#include <Jolt/Physics/Collision/Shape/CylinderShape.h>
#include <Jolt/Physics/Collision/Shape/ConvexHullShape.h>
#include <Jolt/Physics/Collision/Shape/MeshShape.h>
#include <Jolt/Physics/Collision/Shape/HeightFieldShape.h>
#include <Jolt/Physics/Collision/Shape/ScaledShape.h>
#include <Jolt/Physics/Collision/Shape/RotatedTranslatedShape.h>
#include <Jolt/Physics/Body/BodyCreationSettings.h>
#include <Jolt/Physics/Body/BodyActivationListener.h>
#include <Jolt/Physics/Character/Character.h>
#include <Jolt/Physics/Character/CharacterVirtual.h>

#ifdef JPH_DEBUG_RENDERER
#include <TestFramework/TestFramework.h>
#include <Application/EntryPoint.h>
#include <Application/Application.h>
#endif

#include "rust/cxx.h"
#include "jolt-physics-rs/src/size.rs.h"

// Disable common warnings triggered by Jolt, you can use JPH_SUPPRESS_WARNING_PUSH / JPH_SUPPRESS_WARNING_POP to store and restore the warning state
JPH_SUPPRESS_WARNINGS

#if defined(JPH_EXTERNAL_PROFILE) || defined(JPH_PROFILE_ENABLED)
#define PROFILE_ONLY(code) code
#elif
#define PROFILE_ONLY(code)
#endif

#if defined(JPH_DEBUG_RENDERER)
#define RENDERER_ONLY(code) code
#else
#define RENDERER_ONLY(code)
#endif

using namespace JPH;
using namespace std;

constexpr float MARGIN_FACTOR = 0.08f;

//
// base
//

static_assert(sizeof(Vec3) == size_t(SizeOf::Vec3), "Vec3 size");
static_assert(sizeof(Vec4) == size_t(SizeOf::Vec4), "Vec4 size");
static_assert(sizeof(Quat) == size_t(SizeOf::Quat), "Quat size");
static_assert(sizeof(Mat44) == size_t(SizeOf::Mat4), "Mat44 size");

struct Isometry {
	Vec3 position;
	Quat rotation;
};
static_assert(sizeof(Isometry) == size_t(SizeOf::Isometry), "Isometry size");

struct Transform {
	Vec3 position;
	Quat rotation;
	Vec3 scale;
};
static_assert(sizeof(Transform) == size_t(SizeOf::Transform), "Transform size");

static_assert(sizeof(Float3) == size_t(SizeOf::Float3), "Float3 size");
struct Int3 { int32_t x, y, z; };
static_assert(sizeof(Int3) == size_t(SizeOf::Int3), "Int3 size");
static_assert(sizeof(Plane) == size_t(SizeOf::Plane), "Plane size");

static_assert(sizeof(BodyID) == 4, "BodyID size");

static_assert(sizeof(Ref<int>) == sizeof(size_t), "RsRef size");
template <class T, class R> R CreateRefT(Ref<T> cxx_ref) {
	R rs_ref = R{};
	reinterpret_cast<Ref<T>*>(&rs_ref)->operator=(cxx_ref);
	return rs_ref;
}
template <class T, class R> void DropRefT(R rs_ref) { reinterpret_cast<Ref<T>*>(&rs_ref)->~Ref(); }
template <class T, class R> uint32 CountRefT(R rs_ref) { return (*reinterpret_cast<Ref<T>*>(&rs_ref))->GetRefCount(); }
template <class T, class R> R CloneRefT(R rs_ref) {
	R rs_ref_new = R{};
	*reinterpret_cast<Ref<T>*>(&rs_ref_new) = *reinterpret_cast<Ref<T>*>(&rs_ref);
	return rs_ref_new;
}
template <class T, class R> Ref<T> AsRefMut(R rs_ref) { return *reinterpret_cast<Ref<T>*>(&rs_ref); }
template <class T, class R> RefConst<T> AsRefConst(R rs_ref) { return *reinterpret_cast<RefConst<T>*>(&rs_ref); }

static_assert(sizeof(Array<float>) == 24, "Array<T> size");

using ListFloat = Array<float>;
inline void ArrayFloatNew(ListFloat* arr) { new(arr) ListFloat(); }
inline void ArrayFloatReserve(ListFloat* arr, size_t capacity) { arr->reserve(capacity); }
inline void ArrayFloatClone(const ListFloat* arr1, ListFloat* arr2) { *arr2 = *arr1; }
inline void ArrayFloatDrop(ListFloat* arr) { *arr = ListFloat(); }

using ListVec3 = Array<Vec3>;
inline void ArrayVec3New(ListVec3* arr) { new(arr) ListVec3(); }
inline void ArrayVec3Reserve(ListVec3* arr, size_t capacity) { arr->reserve(capacity); }
inline void ArrayVec3Clone(const ListVec3* arr1, ListVec3* arr2) { *arr2 = *arr1; }
inline void ArrayVec3Drop(ListVec3* arr) { *arr = ListVec3(); }

using ListFloat3 = Array<Float3>;
inline void ArrayFloat3New(ListFloat3* arr) { new(arr) ListFloat3(); }
inline void ArrayFloat3Reserve(ListFloat3* arr, size_t capacity) { arr->reserve(capacity); }
inline void ArrayFloat3Clone(const ListFloat3* arr1, ListFloat3* arr2) { *arr2 = *arr1; }
inline void ArrayFloat3Drop(ListFloat3* arr) { *arr = ListFloat3(); }

using ListIndexedTriangle = Array<IndexedTriangle>;
inline void ArrayIndexedTriangleNew(ListIndexedTriangle* arr) { new(arr) ListIndexedTriangle(); }
inline void ArrayIndexedTriangleReserve(ListIndexedTriangle* arr, size_t capacity) { arr->reserve(capacity); }
inline void ArrayIndexedTriangleClone(const ListIndexedTriangle* arr1, ListIndexedTriangle* arr2) { *arr2 = *arr1; }
inline void ArrayIndexedTriangleDrop(ListIndexedTriangle* arr) { *arr = ListIndexedTriangle(); }

// #include "jolt-physics-rs/src/base.rs.h"
