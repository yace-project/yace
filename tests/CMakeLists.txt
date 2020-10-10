function(yace_add_test TESTNAME)
  add_executable(${TESTNAME} ${ARGN})
  target_link_libraries(${TESTNAME} gtest gtest_main)
  gtest_discover_tests(${TESTNAME}
    WORKING_DIRECTORY ${PROJECT_DIR}
    PROPERTIES VS_DEBUGGER_WORKING_DIRECTORY "${PROJECT_DIR}")
  set_target_properties(${TESTNAME} PROPERTIES FOLDER tests)
endfunction()

add_subdirectory(assembler)
