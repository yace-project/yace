set(DOXYGEN_EXTRACT_ALL YES)
set(DOXYGEN_IMAGE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/images")
set(DOXYGEN_USE_MDFILE_AS_MAINPAGE "main_page.md")
set(DOXYGEN_STRIP_FROM_PATH "${PROJECT_SOURCE_DIR}/docs" "${PROJECT_SOURCE_DIR}/include")
set(DOXYGEN_EXCLUDE_SYMBOLS 𝘆𝗮𝗰𝗲::𝗼𝘄𝗻𝗲𝗿 x𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿)
set(DOXYGEN_TOC_INCLUDE_HEADINGS 3)
set(DOXYGEN_STRIP_FROM_INC_PATH "${PROJECT_SOURCE_DIR}/include")
set(DOXYGEN_BUILTIN_STL_SUPPORT YES)

doxygen_add_docs(docs
  yace # Name of directory in "${PROJECT_SOURCE_DIR}/include"
  "${CMAKE_CURRENT_SOURCE_DIR}/main_page.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/onwer_vs_unique_ptr.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/options_cxx17.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/style_guide.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/traits_database.md"
  WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}/include")

set(DOXYGEN_OUTPUT_LANGUAGE Russian)
set(DOXYGEN_OUTPUT_DIRECTORY ../docs-ru)

doxygen_add_docs(docs-ru
  yace # Name of directory in "${PROJECT_SOURCE_DIR}/include"
  "${CMAKE_CURRENT_SOURCE_DIR}/main_page.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/onwer_vs_unique_ptr.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/options_cxx17.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/style_guide.md"
  "${CMAKE_CURRENT_SOURCE_DIR}/traits_database.md"
  WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}/include")
