set(IMGUI_DIR extern/imgui)
	
set(IMGUI_HEADER
	${IMGUI_DIR}/backends/imgui_impl_sdl.h
	${IMGUI_DIR}/backends/imgui_impl_opengl2.h
	${IMGUI_DIR}/backends/imgui_impl_opengl3.h
	${IMGUI_DIR}/imgui_internal.h
	${IMGUI_DIR}/imgstb_rectpack.h
	${IMGUI_DIR}/imstb_textedit.h
	${IMGUI_DIR}/imstb_truetype.h)

add_library(IMGUI_LIBRARY
	${IMGUI_DIR}/imgui.cpp
	${IMGUI_DIR}/imgui_demo.cpp
	${IMGUI_DIR}/imgui_draw.cpp
	${IMGUI_DIR}/imgui_tables.cpp
	${IMGUI_DIR}/imgui_widgets.cpp
	${IMGUI_DIR}/backends/imgui_impl_sdl.cpp
	${IMGUI_DIR}/backends/imgui_impl_opengl2.cpp
	${IMGUI_DIR}/backends/imgui_impl_opengl3.cpp)
	
target_include_directories(IMGUI_LIBRARY PUBLIC
	${IMGUI_DIR})

target_link_libraries(IMGUI_LIBRARY PUBLIC
	SDL2::SDL2-static)
