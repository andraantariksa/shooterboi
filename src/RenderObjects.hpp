#ifndef _SRC_RENDEROBJECTS_HPP
#define _SRC_RENDEROBJECTS_HPP

#include <array>

#include "RenderObjectData.hpp"
#include "logic/components/RenderObject.hpp"
#include "logic/components/Transform.hpp"

template <uint32_t S>
class RenderObjects {
private:
    std::array<RenderObjectData, S> m_render_objects;
    uint32_t m_unoccupied_index = 0;
public:
    uint32_t findUnoccupiedIndex() {
        for (uint32_t i = m_unoccupied_index; i < S; i++) {
            if (m_render_objects[i].m_type == RenderObjectDataType::Nothing) {
                return i;
            }
        }
        return S;
    }

    constexpr uint32_t size() const { return S; }

    const RenderObjectData* data() const { return m_render_objects.data(); }

    RenderObjectData& get(uint32_t index) { return m_render_objects.data()[index]; }
    
    void create(entt::registry& registry, entt::entity& entity, RenderObjectDataType type) {
        uint32_t index = findUnoccupiedIndex();
        m_render_objects[index].m_type = type;
        registry.emplace<RenderObject>(entity, index);
    }
};

#endif
