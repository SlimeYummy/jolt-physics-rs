#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/shape.rs.h"

struct XSphereShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float radius;
};
static_assert(sizeof(XSphereShapeSettings) == 24, "XSphereShapeSettings size");

Shape* CreateSphereShape(const XSphereShapeSettings& st) {
	SphereShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mRadius = st.radius;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XBoxShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfX, halfY, halfZ;
	float convexRadius;
};
static_assert(sizeof(XBoxShapeSettings) == 40, "XBoxShapeSettings size");

Shape* CreateBoxShape(const XBoxShapeSettings& st) {
	BoxShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mHalfExtent = Vec3(st.halfX, st.halfY, st.halfZ);
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(std::move(result.Get()));
}

struct XCapsuleShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float radius;
};
static_assert(sizeof(XCapsuleShapeSettings) == 32, "XCapsuleShapeSettings size");

Shape* CreateCapsuleShape(const XCapsuleShapeSettings& st) {
	CapsuleShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mHalfHeightOfCylinder = st.halfHeight;
	settings.mRadius = st.radius;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XTaperedCapsuleShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float topRadius;
	float bottomRadius;
};
static_assert(sizeof(XTaperedCapsuleShapeSettings) == 32, "XTaperedCapsuleShapeSettings size");

Shape* CreateTaperedCapsuleShape(const XTaperedCapsuleShapeSettings& st) {
	TaperedCapsuleShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mHalfHeightOfTaperedCylinder = st.halfHeight;
	settings.mTopRadius = st.topRadius;
	settings.mBottomRadius = st.bottomRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XCylinderShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float radius;
	float convexRadius;
};
static_assert(sizeof(XCylinderShapeSettings) == 32, "XCylinderShapeSettings size");

Shape* CreateCylinderShape(const XCylinderShapeSettings& st) {
	CylinderShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mHalfHeight = st.halfHeight;
	settings.mRadius = st.radius;
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XTaperedCylinderShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float topRadius;
	float bottomRadius;
	float convexRadius;
};
static_assert(sizeof(XTaperedCylinderShapeSettings) == 40, "XTaperedCylinderShapeSettings size");

Shape* CreateTaperedCylinderShape(const XTaperedCylinderShapeSettings& st) {
	TaperedCylinderShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mHalfHeight = st.halfHeight;
	settings.mTopRadius = st.topRadius;
	settings.mBottomRadius = st.bottomRadius;
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XConvexHullShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	rust::Slice<Vec3> points;
	float maxConvexRadius;
	float maxErrorConvexRadius;
	float hullTolerance;
};
static_assert(sizeof(XConvexHullShapeSettings) == 56, "XConvexHullShapeSettings size");

Shape* CreateConvexHullShape(const XConvexHullShapeSettings& st) {
	ConvexHullShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mPoints.assign(st.points.begin(), st.points.end());
	settings.mMaxConvexRadius = st.maxConvexRadius;
	settings.mMaxErrorConvexRadius = st.maxErrorConvexRadius;
	settings.mHullTolerance = st.hullTolerance;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XTriangleShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float convexRadius;
	Vec3 v1;
	Vec3 v2;
	Vec3 v3;
};
static_assert(sizeof(XTriangleShapeSettings) == 80, "XTriangleShapeSettings size");

Shape* CreateTriangleShape(const XTriangleShapeSettings& st) {
	TriangleShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mDensity = st.density;
	settings.mV1 = st.v1;
	settings.mV2 = st.v2;
	settings.mV3 = st.v3;
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XPlaneShapeSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	Plane plane;
	float halfExtent;
};
static_assert(sizeof(XPlaneShapeSettings) == 48, "XPlaneShapeSettings size");

Shape* CreatePlaneShape(const XPlaneShapeSettings& st) {
	PlaneShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = st.material;
	settings.mPlane = st.plane;
	settings.mHalfExtent = st.halfExtent;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XMeshShapeSettings {
	uint64 userData;
	rust::Slice<Float3> triangleVertices;
	rust::Slice<IndexedTriangle> indexedTriangles;
	rust::Slice<PhysicsMaterial*> materials;
	uint32 maxTrianglesPerLeaf;
	float activeEdgeCosThresholdAngle;
};
static_assert(sizeof(XMeshShapeSettings) == 64, "XMeshShapeSettings size");

Shape* CreateMeshShape(const XMeshShapeSettings& st) {
	MeshShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mTriangleVertices.assign(st.triangleVertices.begin(), st.triangleVertices.end());
	settings.mIndexedTriangles.assign(st.indexedTriangles.begin(), st.indexedTriangles.end());
	settings.mMaterials = PhysicsMaterialList(
		(PhysicsMaterialRefC*)st.materials.data(),
		(PhysicsMaterialRefC*)st.materials.data() + st.materials.size()
	);
	settings.mMaxTrianglesPerLeaf = st.maxTrianglesPerLeaf;
	settings.mActiveEdgeCosThresholdAngle = st.activeEdgeCosThresholdAngle;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XHeightFieldShapeSettings {
	uint64 userData;
	Vec3 offset;
	Vec3 scale;
	uint32 sampleCount;
	float minHeightValue;
	float maxHeightValue;
	uint32 blockSize;
	uint32 bitsPerSample;
	rust::Slice<float> heightSamples;
	rust::Slice<uint8_t> materialIndices;
	rust::Slice<PhysicsMaterial*> materials;
	float activeEdgeCosThresholdAngle;
};
static_assert(sizeof(XHeightFieldShapeSettings) == 128, "XHeightFieldShapeSettings size");

Shape* CreateHeightFieldShape(const XHeightFieldShapeSettings& st) {
	if (st.heightSamples.size() != st.sampleCount * st.sampleCount) {
		return nullptr;
	}
	HeightFieldShapeSettings settings(
		st.heightSamples.data(),
		st.offset,
		st.scale,
		st.sampleCount,
		st.materialIndices.data(),
		PhysicsMaterialList(
			(PhysicsMaterialRefC*)st.materials.data(),
			(PhysicsMaterialRefC*)st.materials.data() + st.materials.size()
		)
	);
	settings.mUserData = st.userData;
	settings.mMinHeightValue = st.minHeightValue;
	settings.mMaxHeightValue = st.maxHeightValue;
	settings.mBlockSize = st.blockSize;
	settings.mBitsPerSample = st.bitsPerSample;
	settings.mActiveEdgeCosThresholdAngle = st.activeEdgeCosThresholdAngle;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XEmptyShapeSettings {
	uint64 userData;
	Vec3 centerOfMass;
};
static_assert(sizeof(XEmptyShapeSettings) == 32, "XEmptyShapeSettings size");

Shape* CreateEmptyShape(const XEmptyShapeSettings& st) {
	EmptyShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mCenterOfMass = st.centerOfMass;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XScaledShapeSettings {
	uint64 userData;
	RefConst<Shape> innerShape;
	Vec3 scale;
};
static_assert(sizeof(XScaledShapeSettings) == 32, "XScaledShapeSettings size");

Shape* CreateScaledShape(const XScaledShapeSettings& st) {
	ScaledShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mInnerShapePtr = st.innerShape;
	settings.mScale = st.scale;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	
	return LeakRefT<Shape>(result.Get());
}

struct XRotatedTranslatedShapeSettings {
	uint64 userData;
	RefConst<Shape> innerShape;
	Vec3 position;
	Quat rotation;
};
static_assert(sizeof(XRotatedTranslatedShapeSettings) == 48, "XRotatedTranslatedShapeSettings size");

Shape* CreateRotatedTranslatedShape(const XRotatedTranslatedShapeSettings& st) {
	RotatedTranslatedShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mInnerShapePtr = st.innerShape;
	settings.mPosition = st.position;
	settings.mRotation = st.rotation;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XOffsetCenterOfMassShapeSettings {
	uint64 userData;
	RefConst<Shape> innerShape;
	Vec3 offset;
};
static_assert(sizeof(XOffsetCenterOfMassShapeSettings) == 32, "XOffsetCenterOfMassShapeSettings size");

Shape* CreateOffsetCenterOfMassShape(const XOffsetCenterOfMassShapeSettings& st) {
	OffsetCenterOfMassShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mInnerShapePtr = st.innerShape;
	settings.mOffset = st.offset;
	auto result = settings.Create();
	if (result.HasError()) {
		return nullptr;
	}
	return LeakRefT<Shape>(result.Get());
}

struct XSubShapeSettings {
	void* _shape;
	RefConst<Shape> shape;
	Vec3 position;
	Quat rotation;
	uint32 userData;
};
static_assert(sizeof(XSubShapeSettings) == 64, "XSubShapeSettings size");

struct JoltArray {
	size_t size;
	size_t capacity;
	XSubShapeSettings* elements;
};
static_assert(sizeof(JoltArray) == sizeof(Array<XSubShapeSettings>));

struct XStaticCompoundShapeSettings {
	uint64 userData;
	rust::Slice<XSubShapeSettings> subShapes;
};
static_assert(sizeof(XStaticCompoundShapeSettings) == 24, "XStaticCompoundShapeSettings size");

StaticCompoundShape* CreateStaticCompoundShape(const XStaticCompoundShapeSettings& st) {
	StaticCompoundShapeSettings settings;
	settings.mUserData = st.userData;
	JoltArray* subShapes = (JoltArray*)&settings.mSubShapes;
	subShapes->size = st.subShapes.size();
	subShapes->capacity = st.subShapes.size();
	subShapes->elements = st.subShapes.data();
	auto result = settings.Create();
	subShapes->size = 0;
	subShapes->capacity = 0;
	subShapes->elements = nullptr;
	if (result.HasError()) {
		return nullptr;
	}
	return (StaticCompoundShape*)LeakRefT<Shape>(result.Get());
}

struct XMutableCompoundShapeSettings {
	uint64 userData;
	rust::Slice<XSubShapeSettings> subShapes;
};
static_assert(sizeof(XMutableCompoundShapeSettings) == 24, "XMutableCompoundShapeSettings size");

MutableCompoundShape* CreateMutableCompoundShape(const XMutableCompoundShapeSettings& st) {
	MutableCompoundShapeSettings settings;
	settings.mUserData = st.userData;
	JoltArray* subShapes = (JoltArray*)&settings.mSubShapes;
	subShapes->size = st.subShapes.size();
	subShapes->capacity = st.subShapes.size();
	subShapes->elements = st.subShapes.data();
	auto result = settings.Create();
	subShapes->size = 0;
	subShapes->capacity = 0;
	subShapes->elements = nullptr;
	if (result.HasError()) {
		return nullptr;
	}
	return (MutableCompoundShape*)LeakRefT<Shape>(result.Get());
}
