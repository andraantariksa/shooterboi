#ifndef _SRC_LOGIC_COMPONENTS_AUDIOSOURCE3D_HPP
#define _SRC_LOGIC_COMPONENTS_AUDIOSOURCE3D_HPP

#include <soloud.h>
#include <soloud_wav.h>
#include <memory>

#include "../../Engine.hpp"

class AudioSource3D {
private:
    AudioResourceID m_resource_id;
public:
    AudioSource3D(Engine& engine, AudioResourceID resource_id, bool play_on_start = false, glm::vec3& pos = glm::vec3(0.0f)) :
        m_resource_id(resource_id) {
        if (play_on_start) {
            engine.get_soloud().play3d(*engine.get_audio_resources(m_resource_id), pos.x, pos.y, pos.z);
        }
    }

    inline void play(Engine& engine, glm::vec3& pos = glm::vec3(0.0f))
    {
        engine.get_soloud().play3d(*engine.get_audio_resources(m_resource_id), pos.x, pos.y, pos.z);
    }
};

#endif
