#ifndef _SRC_LOGIC_COMPONENTS_ENEMY_HPP
#define _SRC_LOGIC_COMPONENTS_ENEMY_HPP

#include "Transform.hpp"

enum class EnemyState
{
    Patrol,
    Chase
};

class Enemy
{
public:
    static constexpr float speed = 0.5f;
    float m_health_point = 100.0f;
    EnemyState m_state = EnemyState::Patrol;

    Enemy(float health_point = 100.0f) :
        m_health_point(health_point),
        m_state(EnemyState::Patrol) {
    }
};

#endif
