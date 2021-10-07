#ifndef _SRC_RENDEROBJECTDATA_HPP
#define _SRC_RENDEROBJECTDATA_HPP

#include "Common.h"
#include "logic/components/Transform.hpp"

enum class RenderObjectDataType: uint32_t {
    Nothing = 0,
    Enemy = 1
};

class RenderObjectData {
public:
    RenderObjectDataType m_type;
    union RenderObjectUnion {
        Transform enemies;

        RenderObjectUnion() {
        }
    } m_data;

    RenderObjectData():
        m_type(RenderObjectDataType::Nothing) {
    }
};

#endif
