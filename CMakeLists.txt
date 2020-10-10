cmake_minimum_required(VERSION 3.14...3.18)

project(
  𝔜𝔄ℭ𝔈
  VERSION 0.0.0.0
  DESCRIPTION "Yet Another CPU Emulator"
  # Note: we don't really have C sources, but clang-cl fails miserably for pure CXX projects.
  LANGUAGES C CXX)

if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
  if(MSVC)
    # YACE is C++17 project. But we need designated initializers, too. Simplest way is to use C++20.
    set(CMAKE_CXX_STANDARD 20 CACHE STRING "C++ standard to conform to")
    # Make __int128 integral: https://quuxplusone.github.io/blog/2019/02/28/is-int128-integral/
    set(CMAKE_CXX_EXTENSIONS OFF)
    if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
      add_compile_options(/utf-8 /permissive- /wd5105)
    else()
      add_compile_options(/utf-8 /permissive- /Zc:preprocessor /wd5105)
    endif()
  else()
    # YACE is C++17 project. We don't plan to support earlier standards.
    set(CMAKE_CXX_STANDARD 17 CACHE STRING "C++ standard to conform to")
    # Make __int128 integral: https://quuxplusone.github.io/blog/2019/02/28/is-int128-integral/
    set(CMAKE_CXX_EXTENSIONS OFF) 
  endif()
  # Support IDEs
  set_property(GLOBAL PROPERTY USE_FOLDERS ON)
  include(CTest)
  find_package(Doxygen)
  if(Doxygen_FOUND)
    add_subdirectory(docs)
  else()
    message(STATUS "Doxygen not found, not building docs")
  endif()
  find_package(Git QUIET)
  if(GIT_FOUND AND EXISTS "${PROJECT_SOURCE_DIR}/.git")
    option(GIT_SUBMODULE "Check submodules during build" ON)
    if(GIT_SUBMODULE)
      message(STATUS "Submodule update")
      execute_process(COMMAND ${GIT_EXECUTABLE} submodule update --init --recursive
                      WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
                      RESULT_VARIABLE GIT_SUBMOD_RESULT)
      if(NOT GIT_SUBMOD_RESULT EQUAL "0")
        message(FATAL_ERROR
                "git submodule update --init failed with ${GIT_SUBMOD_RESULT}, "
                "please checkout submodules")
      endif()
    endif()
  endif()
  if(BUILD_TESTING)
    if(NOT EXISTS "${PROJECT_SOURCE_DIR}/extern/googletest/CMakeLists.txt")
      message(FATAL_ERROR
              "The submodules were not downloaded! "
              "GIT_SUBMODULE was turned off or failed. "
              "Please update submodules and try again.")
    endif()
    include(GoogleTest)
    add_subdirectory(tests)
  endif()
endif()

add_subdirectory(extern)
add_subdirectory(lib)
#add_subdirectory(tools)
#add_subdirectory(utils)
