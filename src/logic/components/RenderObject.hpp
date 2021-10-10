#ifndef _SRC_RENDEROBJECT_HPP
#define _SRC_RENDEROBJECT_HPP

class RenderObject {
private:
    uint32_t m_render_objects_index;
public:
    RenderObject(uint32_t render_objects_index) :
        m_render_objects_index(render_objects_index) {
    }

    inline uint32_t get_render_objects_index() { return m_render_objects_index; }
};

#endif
