#ifndef _SRC_LOGIC_COMPONENTS_AUDIOSOURCE_HPP
#define _SRC_LOGIC_COMPONENTS_AUDIOSOURCE_HPP

#include <soloud.h>
#include <soloud_wav.h>
#include <memory>

#include "../../Engine.hpp"

class AudioSource {
private:
    AudioResourceID m_resource_id;
public:
    AudioSource(Engine& engine, AudioResourceID resource_id, bool play_on_start = false) :
        m_resource_id(resource_id) {
        if (play_on_start) {
            engine.get_soloud().play(*engine.get_audio_resources(m_resource_id));
        }
    }

    inline void play(Engine& engine)
    {
        engine.get_soloud().play(*engine.get_audio_resources(m_resource_id));
    }
};

#endif
