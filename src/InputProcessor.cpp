#include "InputProcessor.hpp"
#include <iostream>

InputProcessor::InputProcessor() :
    m_keyboard_state(nullptr),
    m_num_keys(0),
    m_mouse_moving(false),
    m_mouse_btn_state(0)
{
}

InputProcessor::~InputProcessor()
{
}

void InputProcessor::init()
{
    m_keyboard_state = SDL_GetKeyboardState(&m_num_keys);
}

bool InputProcessor::is_mouse_pressed(uint32_t mouse_button) const {
    return (m_mouse_btn_state & mouse_button) > 0;
}

void InputProcessor::process(const SDL_Event& event)
{
    static constexpr float smoothness = 0.6f;
    int x, y;
    glm::vec2 old_acc = m_mouse_acc;

    m_mouse_btn_state = SDL_GetRelativeMouseState(&x, &y);
    
    // Add low-pass filter for smooth movement
    m_mouse_acc.x = (std::abs((float)x - old_acc.x) > 0.001f) ? ((float)x * smoothness + old_acc.x * smoothness) : 0.0f;
    m_mouse_acc.y = (std::abs((float)y - old_acc.y) > 0.001f) ? ((float)y * smoothness + old_acc.y * smoothness) : 0.0f;
}

bool InputProcessor::is_action_key_down(ActionKey action_key) const
{
    if (action_key >= ActionKey::NumKeys) {
        return false;
    }

    uint32_t action_key_id = (uint32_t)action_key;
    return m_keyboard_state[g_action_key_tbl[action_key_id]];
}

bool InputProcessor::is_mouse_moving() const
{
    return (std::abs(m_mouse_acc.x) > 0) || (std::abs(m_mouse_acc.y) > 0);
}

SDL_Scancode InputProcessor::g_action_key_tbl[InputProcessor::g_num_action_key] = {
    SDL_SCANCODE_W,
    SDL_SCANCODE_S,
    SDL_SCANCODE_A,
    SDL_SCANCODE_D,
    SDL_SCANCODE_LSHIFT,
    SDL_SCANCODE_SPACE,
    SDL_SCANCODE_LCTRL,
    SDL_SCANCODE_ESCAPE
};
