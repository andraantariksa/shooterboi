#pragma once

#include <SDL.h>
#include <glm/glm.hpp>

struct MouseButton
{
    enum
    {
        Left = SDL_BUTTON_LMASK,
        Middle = SDL_BUTTON_MMASK,
        Right = SDL_BUTTON_RMASK,
    };
};


enum class PressedState
{
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
    Jump,
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
    bool is_mouse_pressed(uint32_t mouse_button) const;
    bool is_action_key_down(ActionKey action_key) const;
    bool is_mouse_moving() const;
    const glm::vec2& get_mouse_pos() const { return m_mouse_pos; }
    const glm::vec2& get_mouse_acc() const { return m_mouse_acc; }

private:
    const uint8_t* m_keyboard_state;
    int32_t m_num_keys;
    bool m_mouse_moving;
    uint32_t m_mouse_btn_state;
    glm::vec2 m_mouse_pos{};
    glm::vec2 m_mouse_acc{};

    static constexpr uint32_t g_num_action_key = (uint32_t)ActionKey::NumKeys;
    static SDL_Scancode g_action_key_tbl[g_num_action_key];
};
