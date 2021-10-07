#pragma once

#include <SDL.h>
#include <glm/glm.hpp>

enum class MouseButton: uint8_t {
    Left = SDL_BUTTON_LEFT,
    Middle = SDL_BUTTON_MIDDLE,
    Right = SDL_BUTTON_RIGHT,
};

enum class PressedState {
    Pressed,
    Released
};

enum class ActionKey
{
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Run,
    Crouch,
    ExitGame,

    NumKeys
};

class InputProcessor
{
public:
    InputProcessor();
    ~InputProcessor();

    void init();
    void process(const SDL_Event& event);
    bool is_mouse_pressed(MouseButton mouse_button) const;
    bool is_action_key_down(ActionKey action_key) const;
    bool is_any_mousebtn_clicked(uint8_t mouse_btn) const;
    bool is_any_mousebtn_down(uint8_t mouse_btn) const;
    bool is_any_mousebtn_up(uint8_t mouse_btn) const;
    bool is_mouse_moving() const;
    const glm::vec2& get_mouse_pos() const { return m_mouse_pos; }
    const glm::vec2& get_mouse_acc() const { return m_mouse_acc; }

private:
    const uint8_t* m_keyboard_state;
    uint8_t m_mouse_pressed_state[3] = { false, false, false };
    int32_t m_num_keys;
    bool m_mouse_moving;
    glm::vec2 m_mouse_pos;
    glm::vec2 m_mouse_acc;

    static constexpr uint32_t g_num_action_key = (uint32_t)ActionKey::NumKeys;
    static SDL_Scancode g_action_key_tbl[g_num_action_key];
};
