pub const DEFAULT_EXE_CMAKE_LISTS: &str = 
r#"cmake_minimum_required(VERSION {{cmake_min_version}})
project({{project_name}} VERSION 1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD {{cpp_standard}})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3")

### Put the names of your source files here ###
set(SRC_FILES 
    main.cpp
)

list(TRANSFORM SRC_FILES PREPEND "src/")

add_executable(${PROJECT_NAME} ${SRC_FILES})

set_target_properties(${PROJECT_NAME} PROPERTIES DEBUG_POSTFIX -d)

### Add link directories here ###
target_link_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/external/lib)

### Add include directories here ###
target_include_directories(${PROJECT_NAME} PRIVATE external)

### Uncomment this line if you want the standard libraries to be linked statically ###
# target_link_options(${PROJECT_NAME} PRIVATE -static-libgcc -static-libstdc++)

### Use this line to link external libraries
# target_link_libraries(${PROJECT_NAME} your_library_name)
"#;

pub const DEFAULT_LIB_CMAKE_LISTS: &str = 
r#"cmake_minimum_required(VERSION {{cmake_min_version}})
project({{project_name}} VERSION 1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD {{cpp_standard}})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3")

### Put the names of your source files here ###
set(SRC_FILES 
    main.cpp
)

list(TRANSFORM SRC_FILES PREPEND "src/")

### Change STATIC to SHARED to build dynamic library instead of the static one ###
add_library(${PROJECT_NAME} STATIC ${SRC_FILES})

set_target_properties(${PROJECT_NAME} PROPERTIES DEBUG_POSTFIX -d)

### Add link directories here ###
target_link_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/external/lib)

### Add include directories here ###
target_include_directories(${PROJECT_NAME} PRIVATE external)

### Uncomment this line if you want the standard libraries to be linked statically ###
# target_link_options(${PROJECT_NAME} PRIVATE -static-libgcc -static-libstdc++)

### Use this line to link external libraries
# target_link_libraries(${PROJECT_NAME} your_library_name)

### System type specifics
if(WIN32)
	target_link_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/external/lib/win)
    target_link_options(${PROJECT_NAME} PRIVATE -Wl,--export-all-symbols)
elseif(UNIX)
	target_link_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/external/lib/linux)
endif()
"#;