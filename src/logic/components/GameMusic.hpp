#ifndef _SRC_LOGIC_COMPONENTS_GAME_MUSIC_HPP
#define _SRC_LOGIC_COMPONENTS_GAME_MUSIC_HPP

#include <soloud.h>

#include "../../Engine.hpp"

class GameMusic
{
private:
    AudioResourceID m_background_music;
    AudioResourceID m_battle_music;
public:
    GameMusic(Engine& engine, AudioResourceID background_music, AudioResourceID battle_music) :
        m_background_music(background_music),
        m_battle_music(battle_music) {
        engine.get_soloud().play(*engine.get_audio_resources(m_background_music));
    }
};

#endif
