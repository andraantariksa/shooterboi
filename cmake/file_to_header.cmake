function(file_to_header file include_dir)
    get_filename_component(name ${file} NAME_WE)
    get_filename_component(directory ${file} DIRECTORY)
    get_filename_component(directory ${directory} NAME)
    string(TOUPPER ${name} NAME)
    string(TOUPPER ${directory} DIRECTORY)

    set(new_file ${include_dir}/${name}.hpp)

    if (${file} IS_NEWER_THAN  ${new_file})
        file(READ ${file} content)

        string(REGEX REPLACE "\"" "\\\\\"" content "${content}")
        string(REGEX REPLACE "[\r\n]" "\\\\n\"\\\\\n\"" content "${content}")
        set(content "\"${content}\"")
        set(content "#ifndef ${DIRECTORY}_${NAME}\n#define ${DIRECTORY}_${NAME}\nconst char* ${NAME} = ${content}; \n#endif")

        file(WRITE ${new_file} "${content}")
    endif()
endfunction()

function(convert_file_to_header_directory layout_dir include_dir)
    file(GLOB layouts ${layout_dir}/*.glsl)
    foreach(filename ${layouts})
        file_to_header(${filename} ${include_dir})
    endforeach()
endfunction()