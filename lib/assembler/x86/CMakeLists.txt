add_library(yace-assembler-x86
  dummy.cc)
target_include_directories(yace-assembler-x86
  PUBLIC "${PROJECT_SOURCE_DIR}/include")
target_compile_features(yace-assembler-x86
  PUBLIC cxx_std_17)
