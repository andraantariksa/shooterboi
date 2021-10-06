#include "InputProcessor.h"

InputProcessor::InputProcessor() :
    m_keyboard_state(nullptr),
    m_num_keys(0),
    m_mouse_moving(false)
{
}

InputProcessor::~InputProcessor()
{
}

void InputProcessor::init()
{
    m_keyboard_state = SDL_GetKeyboardState(&m_num_keys);
}

void InputProcessor::process(const SDL_Event& event)
{
    m_mouse_acc = glm::vec2(0.0f);

    if (event.type == SDL_MOUSEMOTION) {
        glm::vec2 old(m_mouse_pos);
        m_mouse_pos.x = (float)event.motion.x;
        m_mouse_pos.y = (float)event.motion.y;
        m_mouse_acc = m_mouse_pos - old;
    }
}

bool InputProcessor::is_action_key_down(ActionKey action_key) const
{
    if (action_key >= ActionKey::NumKeys) {
        return false;
    }

    uint32_t action_key_id = (uint32_t)action_key;
    return m_keyboard_state[g_action_key_tbl[action_key_id]];
}

bool InputProcessor::is_any_mousebtn_clicked(uint8_t mouse_btn) const
{
    return false;
}

bool InputProcessor::is_any_mousebtn_down(uint8_t mouse_btn) const
{
    return false;
}

bool InputProcessor::is_any_mousebtn_up(uint8_t mouse_btn) const
{
    return false;
}

bool InputProcessor::is_mouse_moving() const
{
    return false;
}

SDL_Scancode InputProcessor::g_action_key_tbl[InputProcessor::g_num_action_key] = {
    SDL_SCANCODE_W,
    SDL_SCANCODE_S,
    SDL_SCANCODE_A,
    SDL_SCANCODE_D,
    SDL_SCANCODE_LSHIFT,
    SDL_SCANCODE_LCTRL,
    SDL_SCANCODE_ESCAPE
};
