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
    Gun
};

enum class ShapeOperator
{
    Union,
    Intersect,
    Subtract
};

struct Renderable
{
    RenderObjectType type;
    ShapeType shape_type;
    ShapeOperator shape_op;
    glm::vec3 color;

    union
    {
        struct
        {
            float radius;
        } sh_sphere;

        struct
        {
            glm::vec3 size;
        } sh_box;
    };

    Renderable(
        RenderObjectType type,
        ShapeType shape_type,
        ShapeOperator shape_op,
        const glm::vec3& color) :
        type(type),
        shape_type(shape_type),
        shape_op(shape_op),
        color(color)
    {
    }
};
