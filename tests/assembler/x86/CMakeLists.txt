yace_add_test(test-assembler-x86 dummy.cc modrm_address_static.cc)
target_link_libraries(test-assembler-x86 yace-assembler-x86)
