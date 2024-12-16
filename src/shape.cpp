#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/shape.rs.h"

struct SphereSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float radius;
};
static_assert(sizeof(SphereSettings) == 24, "SphereSettings size");

XRefShape CreateShapeSphere(const SphereSettings& st) {
	SphereShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mRadius = st.radius;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct BoxSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfX, halfY, halfZ;
	float convexRadius;
};
static_assert(sizeof(BoxSettings) == 40, "BoxSettings size");

XRefShape CreateShapeBox(const BoxSettings& st) {
	BoxShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mHalfExtent = Vec3(st.halfX, st.halfY, st.halfZ);
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct CapsuleSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float radius;
};
static_assert(sizeof(CapsuleSettings) == 32, "CapsuleSettings size");

XRefShape CreateShapeCapsule(const CapsuleSettings& st) {
	CapsuleShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mHalfHeightOfCylinder = st.halfHeight;
	settings.mRadius = st.radius;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct TaperedCapsuleSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float topRadius;
	float bottomRadius;
};
static_assert(sizeof(TaperedCapsuleSettings) == 32, "TaperedCapsuleSettings size");

XRefShape CreateShapeTaperedCapsule(const TaperedCapsuleSettings& st) {
	TaperedCapsuleShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mHalfHeightOfTaperedCylinder = st.halfHeight;
	settings.mTopRadius = st.topRadius;
	settings.mBottomRadius = st.bottomRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct CylinderSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float radius;
	float convexRadius;
};
static_assert(sizeof(CylinderSettings) == 32, "CylinderSettings size");

XRefShape CreateShapeCylinder(const CylinderSettings& st) {
	CylinderShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mHalfHeight = st.halfHeight;
	settings.mRadius = st.radius;
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct TaperedCylinderSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float halfHeight;
	float topRadius;
	float bottomRadius;
	float convexRadius;
};
static_assert(sizeof(TaperedCylinderSettings) == 40, "TaperedCylinderSettings size");

XRefShape CreateShapeTaperedCylinder(const TaperedCylinderSettings& st) {
	TaperedCylinderShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mHalfHeight = st.halfHeight;
	settings.mTopRadius = st.topRadius;
	settings.mBottomRadius = st.bottomRadius;
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct ConvexHullSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	rust::Slice<Vec3> points;
	float maxConvexRadius;
	float maxErrorConvexRadius;
	float hullTolerance;
};
static_assert(sizeof(ConvexHullSettings) == 56, "ConvexHullSettings size");

XRefShape CreateShapeConvexHull(const ConvexHullSettings& st) {
	ConvexHullShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mPoints.assign(st.points.begin(), st.points.end());
	settings.mMaxConvexRadius = st.maxConvexRadius;
	settings.mMaxErrorConvexRadius = st.maxErrorConvexRadius;
	settings.mHullTolerance = st.hullTolerance;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct TriangleSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	float density;
	float convexRadius;
	Vec3 v1;
	Vec3 v2;
	Vec3 v3;
};
static_assert(sizeof(TriangleSettings) == 80, "TriangleSettings size");

XRefShape CreateShapeTriangle(const TriangleSettings& st) {
	TriangleShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mDensity = st.density;
	settings.mV1 = st.v1;
	settings.mV2 = st.v2;
	settings.mV3 = st.v3;
	settings.mConvexRadius = st.convexRadius;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct PlaneSettings {
	uint64 userData;
	RefConst<PhysicsMaterial> material;
	Plane plane;
	float halfExtent;
};
static_assert(sizeof(PlaneSettings) == 48, "PlaneSettings size");

XRefShape CreateShapePlane(const PlaneSettings& st) {
	PlaneShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mMaterial = AsRefConst<PhysicsMaterial>(st.material);
	settings.mPlane = st.plane;
	settings.mHalfExtent = st.halfExtent;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct MeshSettings {
	uint64 userData;
	rust::Slice<Float3> triangleVertices;
	rust::Slice<IndexedTriangle> indexedTriangles;
	rust::Slice<XRefPhysicsMaterial> materials;
	uint32 maxTrianglesPerLeaf;
	float activeEdgeCosThresholdAngle;
};
static_assert(sizeof(MeshSettings) == 64, "MeshSettings size");

XRefShape CreateShapeMesh(const MeshSettings& st) {
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
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct HeightFieldSettings {
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
	rust::Slice<XRefPhysicsMaterial> materials;
	float activeEdgeCosThresholdAngle;
};
static_assert(sizeof(HeightFieldSettings) == 128, "HeightFieldSettings size");

XRefShape CreateShapeHeightField(const HeightFieldSettings& st) {
	if (st.heightSamples.size() != st.sampleCount * st.sampleCount) {
		return XRefShape{};
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
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct EmptySettings {
	uint64 userData;
	Vec3 centerOfMass;
};
static_assert(sizeof(EmptySettings) == 32, "EmptySettings size");

XRefShape CreateShapeEmpty(const EmptySettings& st) {
	EmptyShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mCenterOfMass = st.centerOfMass;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct ScaledSettings {
	uint64 userData;
	RefConst<Shape> innerShape;
	Vec3 scale;
};
static_assert(sizeof(ScaledSettings) == 32, "ScaledSettings size");

XRefShape CreateShapeScaled(const ScaledSettings& st) {
	ScaledShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mInnerShapePtr = AsRefConst<Shape>(st.innerShape);
	settings.mScale = st.scale;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct RotatedTranslatedSettings {
	uint64 userData;
	RefConst<Shape> innerShape;
	Vec3 position;
	Quat rotation;
};
static_assert(sizeof(RotatedTranslatedSettings) == 48, "RotatedTranslatedSettings size");

XRefShape CreateShapeRotatedTranslated(const RotatedTranslatedSettings& st) {
	RotatedTranslatedShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mInnerShapePtr = AsRefConst<Shape>(st.innerShape);
	settings.mPosition = st.position;
	settings.mRotation = st.rotation;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct OffsetCenterOfMassSettings {
	uint64 userData;
	RefConst<Shape> innerShape;
	Vec3 offset;
};
static_assert(sizeof(OffsetCenterOfMassSettings) == 32, "OffsetCenterOfMassSettings size");

XRefShape CreateShapeOffsetCenterOfMass(const OffsetCenterOfMassSettings& st) {
	OffsetCenterOfMassShapeSettings settings;
	settings.mUserData = st.userData;
	settings.mInnerShapePtr = AsRefConst<Shape>(st.innerShape);
	settings.mOffset = st.offset;
	auto result = settings.Create();
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct SubShapeSettings {
	void* _shape;
	RefConst<Shape> shape;
	Vec3 position;
	Quat rotation;
	uint32 userData;
};
static_assert(sizeof(SubShapeSettings) == 64, "SubShapeSettings size");

struct SubShape {
	RefConst<Shape> shape;
	Float3 positionCOM;
	Float3 rotation;
	uint32 userData;
	bool isRotationIdentity;
};
static_assert(sizeof(SubShape) == 40, "SubShape size");
static_assert(sizeof(CompoundShape::SubShape) == 40, "CompoundShape::SubShape size");

struct JoltArray {
	size_t size;
	size_t capacity;
	SubShapeSettings* elements;
};

struct StaticCompoundSettings {
	uint64 userData;
	rust::Slice<SubShapeSettings> subShape;
};
static_assert(sizeof(StaticCompoundSettings) == 24, "StaticCompoundSettings size");

XRefShape CreateShapeStaticCompound(const StaticCompoundSettings& st) {
	StaticCompoundShapeSettings settings;
	settings.mUserData = st.userData;
	JoltArray* subShapes = (JoltArray*)&settings.mSubShapes;
	subShapes->size = st.subShape.size();
	subShapes->capacity = st.subShape.size();
	subShapes->elements = st.subShape.data();
	auto result = settings.Create();
	subShapes->size = 0;
	subShapes->capacity = 0;
	subShapes->elements = nullptr;
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

struct MutableCompoundSettings {
	uint64 userData;
	rust::Slice<SubShapeSettings> subShape;
};
static_assert(sizeof(MutableCompoundSettings) == 24, "MutableCompoundSettings size");

XRefShape CreateShapeStaticCompound(const MutableCompoundSettings& st) {
	MutableCompoundShapeSettings settings;
	settings.mUserData = st.userData;
	JoltArray* subShapes = (JoltArray*)&settings.mSubShapes;
	subShapes->size = st.subShape.size();
	subShapes->capacity = st.subShape.size();
	subShapes->elements = st.subShape.data();
	auto result = settings.Create();
	subShapes->size = 0;
	subShapes->capacity = 0;
	subShapes->elements = nullptr;
	if (result.HasError()) {
		return XRefShape{};
	}
	return CreateRefT<Shape, XRefShape>(result.Get());
}

// const SubShape* XStaticCompoundShape::GetSubShape(uint inIdx) const {
// 	return (SubShape*)&AsRefConst<CompoundShape>(this)->GetSubShape(inIdx);
// }
