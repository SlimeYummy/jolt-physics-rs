#include "jolt-physics-rs/src/ffi.h"
#include "jolt-physics-rs/src/body.rs.h"

static_assert(sizeof(BodyCreationSettings) == 256, "BodyCreationSettings size");
static_assert(sizeof(Body) == 128, "Body size");
static_assert(sizeof(CollisionGroup) == 16, "CollisionGroup size");
static_assert(sizeof(MassProperties) == 80, "MassProperties size");
