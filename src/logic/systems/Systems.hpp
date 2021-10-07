#ifndef _SRC_LOGIC_SYSTEMS
#define _SRC_LOGIC_SYSTEMS

#include <entt/entt.hpp>

#include "../../InputProcessor.h"
#include "../../Camera.hpp"

void init(entt::registry& registry, SoLoud::Soloud& soloud);
void update(entt::registry& registry, float delta_time, InputProcessor& inputManager, Camera& camera, SoLoud::Soloud& soloud);

#endif
