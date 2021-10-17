#ifndef _SRC_LOGIC_COMPONENTS_SELF_DESTRUCT_HPP
#define _SRC_LOGIC_COMPONENTS_SELF_DESTRUCT_HPP

#include <chrono>

class SelfDestruct
{
private:
    std::chrono::steady_clock::time_point m_start_time;
    float m_destroy_time;
public:
    SelfDestruct(float destroy_time) :
        m_start_time(std::chrono::high_resolution_clock::now()),
        m_destroy_time(destroy_time) {
    }

    bool update_and_is_ready_to_delete()
    {
        auto new_time = std::chrono::high_resolution_clock::now();
        float time_diff = std::chrono::duration_cast<std::chrono::duration<float>>(new_time - m_start_time).count();
        if (time_diff > m_destroy_time) {
            return true;
        }
        return false;
    }
};

#endif
