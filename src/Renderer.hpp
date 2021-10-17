#pragma once

#include <array>
#include <glad/glad.h>
#include "logic/components/Transform.hpp"
#include "logic/components/Renderable.hpp"

#define MAX_RENDER_QUEUE 100

// must 16-byte aligned
struct alignas(16) RenderQueueData
{
    alignas(16) glm::vec3 position;
    alignas(16) glm::vec3 scale;
    alignas(16) glm::vec3 rotation;
    alignas(16) glm::vec3 color;
    alignas(16) glm::vec4 shape_data;
    RenderObjectType type;
    ShapeType shape_type;
    ShapeOperator shape_op;
};

struct alignas(16) RenderingInfo
{
    alignas(16) glm::vec3 reso_time;
    alignas(16) glm::vec3 cam_pos;
    alignas(16) glm::vec3 cam_dir;
    uint32_t queue_count;
};

static_assert((sizeof(RenderQueueData) % 16) == 0, "Not 16-byte aligned"); // always check alignment for validity

class Renderer
{
public:
    Renderer();
    ~Renderer();

    void init();

    void begin();

    void submit(
        const Transform& transform,
        const Renderable& renderable);

    void end();

    void set_map_data(char* map_data, uint32_t width, uint32_t height);

    void render(
        float time,
        const glm::vec2& resolution,
        const glm::vec3& cam_pos,
        const glm::vec3& cam_dir);

    void shutdown();

private:
    GLuint m_program                            = 0;
    GLuint m_render_queue_ssbo                  = 0;
    GLuint m_rendering_info_ubo                 = 0;
    GLuint m_map_texture                        = GL_INVALID_INDEX;

    RenderQueueData* m_mapped_render_queue      = nullptr;
    uint32_t m_render_queue_pos                 = 0;

    // the binding is hardcoded in glsl code
    static constexpr GLuint g_rendering_info_binding             = 0;
    static constexpr GLuint g_render_queue_binding               = 1;
    static constexpr GLuint g_terrain_texture_binding            = 2;
};
