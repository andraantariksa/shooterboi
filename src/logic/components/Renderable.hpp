#pragma once

#include <glm/glm.hpp>

enum class RenderObjectType
{
    Player,
    Ground,
    Enemy,
    Object
};

enum class ShapeType
{
    None,
    Sphere,
    Box,
    Gun,
    CapsuleLine
};

struct Renderable
{
    RenderObjectType type;
    ShapeType shape_type;
    glm::vec3 color;

    union
    {
        struct
        {
            float radius;
        } shape_sphere;

        struct
        {
            float radius;
            glm::vec3 from;
            glm::vec3 to;
        } shape_capsule_line;

        struct
        {
            glm::vec3 size;
        } shape_box;
    };

    Renderable(
        RenderObjectType type,
        ShapeType shape_type,
        const glm::vec3& color) :
        type(type),
        shape_type(shape_type),
        color(color)
    {
    }
};
