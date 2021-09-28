#include "Game.h"
#include <glad/glad.h>

Game::Game() :
    m_window(nullptr)
{
    if (SDL_Init(SDL_INIT_EVERYTHING) < 0)
    {
        assert(false && "Cannot initialize SDL");
    }

    m_window = SDL_CreateWindow(
        "FPS",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        window_width,
        window_height,
        SDL_WINDOW_OPENGL);

    if (m_window == nullptr)
    {
        assert(false && "Cannot create window");
    }

    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GL_RED_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_ALPHA_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);

    m_ogl_context = SDL_GL_CreateContext(m_window);

    if (m_ogl_context == nullptr)
    {
        assert(false && "Cannot create OpenGL context");
    }

    SDL_GL_MakeCurrent(m_window, m_ogl_context);

    if (!gladLoadGL())
    {
        assert(false && "Cannot create load OpenGL");
    }

    SDL_GL_SetSwapInterval(1);
}

Game::~Game()
{
    SDL_GL_DeleteContext(m_window);
    SDL_DestroyWindow(m_window);
    SDL_Quit();
}

int Game::run()
{
    GLfloat vertices[] = {
        -1.f, -1.f, 0.f,
        -1.f, 1.f, 0.f,
        1.f, -1.f, 0.f,
        1.f, 1.f, 0.f,
    };

    const char* fragment_shader_source = R"""(
        #version 440 core
        // Super simple raymarching example. Created by Reinder Nijhoff 2017
        // Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License.
        // @reindernijhoff
        // 
        // https://www.shadertoy.com/view/4dSBz3
        //
        // This is the shader used as example in my ray march tutorial: https://www.shadertoy.com/view/4dSfRc
        //
        // Created for the Shadertoy Competition 2017 
        //

        //
        // Distance field function for the scene. It combines
        // the seperate distance field functions of three spheres
        // and a plane using the min-operator.
        //
        float map(vec3 p) {
            float d = distance(p, vec3(-1, 0, -5)) - 1.;     // sphere at (-1,0,5) with radius 1
            d = min(d, distance(p, vec3(2, 0, -3)) - 1.);    // second sphere
            d = min(d, distance(p, vec3(-2, 0, -2)) - 1.);   // and another
            d = min(d, p.y + 1.);                            // horizontal plane at y = -1
            return d;
        }

        //
        // Calculate the normal by taking the central differences on the distance field.
        //
        vec3 calcNormal(in vec3 p) {
            vec2 e = vec2(1.0, -1.0) * 0.0005;
            return normalize(
                e.xyy * map(p + e.xyy) +
                e.yyx * map(p + e.yyx) +
                e.yxy * map(p + e.yxy) +
                e.xxx * map(p + e.xxx));
        }

        void main() {
            vec3 ro = vec3(0, 0, 1);                           // ray origin

            vec2 q = (gl_FragCoord.xy - .5 * vec2(1280, 720)) / 720;
            vec3 rd = normalize(vec3(q, 0.) - ro);             // ray direction for fragCoord.xy

            // March the distance field until a surface is hit.
            float h, t = 1.;
            for (int i = 0; i < 256; i++) {
                h = map(ro + rd * t);
                t += h;
                if (h < 0.01) break;
            }

            if (h < 0.01) {
                vec3 p = ro + rd * t;
                vec3 normal = calcNormal(p);
                vec3 light = vec3(0, 2, 0);

                // Calculate diffuse lighting by taking the dot product of 
                // the light direction (light-p) and the normal.
                float dif = clamp(dot(normal, normalize(light - p)), 0., 1.);

                // Multiply by light intensity (5) and divide by the square
                // of the distance to the light.
                dif *= 5. / dot(light - p, light - p);


                gl_FragColor = vec4(vec3(pow(dif, 0.4545)), 1);     // Gamma correction
            }
            else {
                gl_FragColor = vec4(0, 0, 0, 1);
            }
        }
        )""";

    GLuint fragment_shader_id = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragment_shader_id, 1, &fragment_shader_source, nullptr);
    glCompileShader(fragment_shader_id);

    GLuint shader_program_id = glCreateProgram();
    glAttachShader(shader_program_id, fragment_shader_id);
    glLinkProgram(shader_program_id);

    GLuint vertex_array_object;
    glGenVertexArrays(1, &vertex_array_object);
    glBindVertexArray(vertex_array_object);

    GLuint vertex_buffer_object;
    glGenBuffers(1, &vertex_buffer_object);
    glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer_object);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
    
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(GLfloat), nullptr);
    glEnableVertexAttribArray(0);

    glBindVertexArray(vertex_array_object);

    bool running = true;
    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
        }

        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        glUseProgram(shader_program_id);
        glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

        SDL_GL_SwapWindow(m_window);
    }

    glDeleteProgram(shader_program_id);
    glDeleteShader(fragment_shader_id);

    return 0;
}
