#ifndef _SRC_LOGIC_COMPONENTS_AUDIOSOURCE_HPP
#define _SRC_LOGIC_COMPONENTS_AUDIOSOURCE_HPP

#include <soloud.h>
#include <soloud_wav.h>
#include <memory>

class AudioSource {
protected:
    std::unique_ptr<SoLoud::AudioSource> m_source;
public:
    AudioSource(SoLoud::Soloud& soloud, std::unique_ptr<SoLoud::AudioSource>& source, bool play_on_start = false) :
        m_source(std::move(source)) {
        if (play_on_start) {
            soloud.play(*m_source);
        }
    }

    inline void play(SoLoud::Soloud& soloud) { soloud.play(*m_source); }
};

#endif
