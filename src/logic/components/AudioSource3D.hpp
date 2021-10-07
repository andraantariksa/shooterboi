#ifndef _SRC_LOGIC_COMPONENTS_AUDIOSOURCE3D_HPP
#define _SRC_LOGIC_COMPONENTS_AUDIOSOURCE3D_HPP

#include <soloud.h>
#include <soloud_wav.h>
#include <memory>

class AudioSource3D {
protected:
    std::unique_ptr<SoLoud::AudioSource> m_source;
public:
    AudioSource3D(SoLoud::Soloud& soloud, std::unique_ptr<SoLoud::AudioSource>& source, bool play_on_start = false, glm::vec3& pos = glm::vec3(0.0f)) :
        m_source(std::move(source)) {
        if (play_on_start) {
            soloud.play3d(*m_source, pos.x, pos.y, pos.z);
        }
    }

    inline void play(SoLoud::Soloud& soloud, glm::vec3& pos = glm::vec3(0.0f)) { soloud.play3d(*m_source, pos.x, pos.y, pos.z); }
};

#endif
