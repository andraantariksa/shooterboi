#ifndef SHADERS_MAINVS
#define SHADERS_MAINVS
const char* MAINVS = "#version 330\n"\
"\n"\
"layout (location = 0) in vec2 pos;\n"\
"\n"\
"void main() {\n"\
"   gl_Position = vec4(pos, 0.0f, 1);\n"\
"}\n"\
""; 
#endif