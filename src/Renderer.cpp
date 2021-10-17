#include <imgui.h>

#include "Renderer.hpp"
#include "shaders/generated/MainPS.hpp"
#include "shaders/generated/PlayerViewPS.hpp"


static const char* vs_shader_source =
R"(
#version 450 core

void main()
{
    float x = -1.0 + float((gl_VertexID & 1) << 2);
    float y = -1.0 + float((gl_VertexID & 2) << 1);
    gl_Position = vec4(x, y, 0, 1);
}
)";

// test fragment shader
static const char* fs_shader_source =
R"(
#version 450 core

layout(location = 0) out vec4 o_color;

layout(std140, binding = 0) uniform RenderingInfo {
    vec3 reso_time;
    vec3 cam_pos;
    vec3 cam_dir;
    uint queue_count;
};

void main()
{
    vec2 pos = gl_FragCoord.xy/reso_time.xy;
    o_color = vec4(pos.x, pos.y, 1.0f, 1.0f);
}
)";

Renderer::Renderer()
{
}

Renderer::~Renderer()
{
}

void Renderer::init()
{
    // Setup shaders
    GLint status = 0;
    GLuint full_screen_vs = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(full_screen_vs, 1, &vs_shader_source, nullptr);
    glCompileShader(full_screen_vs);
    glGetShaderiv(full_screen_vs, GL_COMPILE_STATUS, &status);

    // check errors
    if (status == GL_FALSE) {
        GLint info_length;
        GLint ret_length;
        glGetShaderiv(full_screen_vs, GL_INFO_LOG_LENGTH, &info_length);
        
        char* err = new char[info_length];
        assert(err != nullptr);

        glGetShaderInfoLog(full_screen_vs, info_length, &ret_length, err);
        std::cout << err << std::endl;
        delete[] err;
        assert(false);
    }

    GLuint full_screen_fs = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(full_screen_fs, 1, &MAINPS, nullptr);
    glCompileShader(full_screen_fs);
    glGetShaderiv(full_screen_fs, GL_COMPILE_STATUS, &status);

    // check errors
    if (status == GL_FALSE) {
        GLint info_length;
        GLint ret_length;
        glGetShaderiv(full_screen_fs, GL_INFO_LOG_LENGTH, &info_length);

        char* err = new char[info_length];
        assert(err != nullptr);

        glGetShaderInfoLog(full_screen_fs, info_length, &ret_length, err);
        std::cout << err << std::endl;
        delete[] err;
        assert(false);
    }

    GLuint full_screen_fs_2 = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(full_screen_fs_2, 1, &PLAYERVIEWPS, nullptr);
    glCompileShader(full_screen_fs_2);
    glGetShaderiv(full_screen_fs_2, GL_COMPILE_STATUS, &status);

    // check errors
    if (status == GL_FALSE) {
        GLint info_length;
        GLint ret_length;
        glGetShaderiv(full_screen_fs_2, GL_INFO_LOG_LENGTH, &info_length);

        char* err = new char[info_length];
        assert(err != nullptr);

        glGetShaderInfoLog(full_screen_fs_2, info_length, &ret_length, err);
        std::cout << err << std::endl;
        delete[] err;
        assert(false);
    }

    // create program
    m_main_program = glCreateProgram();
    glAttachShader(m_main_program, full_screen_vs);
    glAttachShader(m_main_program, full_screen_fs);
    glLinkProgram(m_main_program);
    glGetProgramiv(m_main_program, GL_LINK_STATUS, &status);

    if (status == GL_FALSE) {
        GLint info_length;
        GLint ret_length;

        glGetProgramiv(full_screen_fs, GL_INFO_LOG_LENGTH, &info_length);
        if (info_length > 0) {
            char* err = new char[info_length];
            assert(err != nullptr);

            glGetProgramInfoLog(full_screen_fs, info_length, &ret_length, err);
            std::cout << err << std::endl;
            delete err;
            assert(false);
        }
    }

    m_player_view_program = glCreateProgram();
    glAttachShader(m_player_view_program, full_screen_vs);
    glAttachShader(m_player_view_program, full_screen_fs_2);
    glLinkProgram(m_player_view_program);
    glGetProgramiv(m_player_view_program, GL_LINK_STATUS, &status);

    if (status == GL_FALSE) {
        GLint info_length;
        GLint ret_length;

        glGetProgramiv(full_screen_fs_2, GL_INFO_LOG_LENGTH, &info_length);
        if (info_length > 0) {
            char* err = new char[info_length];
            assert(err != nullptr);

            glGetProgramInfoLog(full_screen_fs_2, info_length, &ret_length, err);
            std::cout << err << std::endl;
            delete err;
            assert(false);
        }
    }

    // we don't need these anymore
    glDeleteShader(full_screen_vs);
    glDeleteShader(full_screen_fs);
    glDeleteShader(full_screen_fs_2);

    // initialize render queue SSBO
    glGenBuffers(1, &m_render_queue_ssbo);
    glBindBuffer(GL_SHADER_STORAGE_BUFFER, m_render_queue_ssbo);
    glBufferData(
        GL_SHADER_STORAGE_BUFFER,
        sizeof(RenderQueueData) * MAX_RENDER_QUEUE,
        nullptr,
        GL_DYNAMIC_DRAW);

    // initialize rendering info uniform buffer
    glGenBuffers(1, &m_rendering_info_ubo);
    glBindBuffer(GL_UNIFORM_BUFFER, m_rendering_info_ubo);
    glBufferData(
        GL_UNIFORM_BUFFER,
        sizeof(RenderingInfo),
        nullptr,
        GL_DYNAMIC_DRAW);

    glBindBufferBase(GL_UNIFORM_BUFFER, g_rendering_info_binding, m_rendering_info_ubo);
    glBindBufferBase(GL_SHADER_STORAGE_BUFFER, g_render_queue_binding, m_render_queue_ssbo);
}

void Renderer::begin()
{
    glBindBuffer(GL_SHADER_STORAGE_BUFFER, m_render_queue_ssbo);
    m_mapped_render_queue = (RenderQueueData*)glMapBuffer(GL_SHADER_STORAGE_BUFFER, GL_WRITE_ONLY);
    m_render_queue_pos = 0;
}

void Renderer::submit(const Transform& transform, const Renderable& renderable)
{
    RenderQueueData* current_data = &m_mapped_render_queue[m_render_queue_pos];
    current_data->position = transform.m_position;
    current_data->type = renderable.type;
    current_data->shape_type = renderable.shape_type;
    current_data->color = renderable.color;
    
    switch (renderable.shape_type) {
        case ShapeType::Sphere:
            current_data->shape_data.x = renderable.shape_sphere.radius;
            break;
        case ShapeType::CapsuleLine:
            current_data->shape_data.x = renderable.shape_capsule_line.from.x;
            current_data->shape_data.y = renderable.shape_capsule_line.from.y;
            current_data->shape_data.z = renderable.shape_capsule_line.from.z;
            current_data->shape_data.w = renderable.shape_capsule_line.radius;

            current_data->shape_data2.x = renderable.shape_capsule_line.to.x;
            current_data->shape_data2.y = renderable.shape_capsule_line.to.y;
            current_data->shape_data2.z = renderable.shape_capsule_line.to.z;
            break;
        default:
            assert(false);
    }

    m_render_queue_pos++;
}

void Renderer::end()
{
    glUnmapBuffer(GL_SHADER_STORAGE_BUFFER);
}

void Renderer::render(
    float time,
    const glm::vec2& resolution,
    const glm::vec3& cam_pos,
    const glm::vec3& cam_dir,
    bool render_game)
{
    if (render_game) {
        glBindBuffer(GL_UNIFORM_BUFFER, m_rendering_info_ubo);
        RenderingInfo* rendering_info = (RenderingInfo*)glMapBuffer(GL_UNIFORM_BUFFER, GL_WRITE_ONLY);
        rendering_info->reso_time.x = resolution.x;
        rendering_info->reso_time.y = resolution.y;
        rendering_info->reso_time.z = time;
        rendering_info->cam_pos.x = cam_pos.x;
        rendering_info->cam_pos.y = cam_pos.y;
        rendering_info->cam_pos.z = cam_pos.z;
        rendering_info->cam_dir.x = cam_dir.x;
        rendering_info->cam_dir.y = cam_dir.y;
        rendering_info->cam_dir.z = cam_dir.z;
        rendering_info->queue_count = m_render_queue_pos;
        glUnmapBuffer(GL_UNIFORM_BUFFER);
    }

    ImGui::Render();
    glViewport(0, 0, (GLint)resolution.x, (GLint)resolution.y);
    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);
    if (render_game) {
        glUseProgram(m_main_program);
        glDrawArrays(GL_TRIANGLES, 0, 3);
        glUseProgram(m_player_view_program);
        glDrawArrays(GL_TRIANGLES, 0, 3);
    }
}

void Renderer::shutdown()
{
    glDeleteBuffers(1, &m_render_queue_ssbo);
    glDeleteBuffers(1, &m_rendering_info_ubo);
    glDeleteProgram(m_main_program);
}
