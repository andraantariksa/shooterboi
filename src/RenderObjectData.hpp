#ifndef _SRC_RENDEROBJECTDATA_HPP
#define _SRC_RENDEROBJECTDATA_HPP

#include "Common.hpp"
#include "logic/components/Transform.hpp"

enum class RenderObjectDataType: uint32_t {
    Nothing = 0,
    Enemy = 1
};

// 16 byte alignment
class RenderObjectData {
public:
    RenderObjectDataType m_type;
    float _pad[3];
    union RenderObjectUnion {
        struct {
            Transform transform;
            float _pad;
        } enemies;

        RenderObjectUnion() {
        }
    } m_data;

    RenderObjectData():
        m_type(RenderObjectDataType::Nothing) {
    }
};

#endif
