#----------------------------------------------------------------
# Generated CMake target import file for configuration "MinSizeRel".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "RtAudio::rtaudio" for configuration "MinSizeRel"
set_property(TARGET RtAudio::rtaudio APPEND PROPERTY IMPORTED_CONFIGURATIONS MINSIZEREL)
set_target_properties(RtAudio::rtaudio PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_MINSIZEREL "CXX"
  IMPORTED_LOCATION_MINSIZEREL "${_IMPORT_PREFIX}/lib/rtaudio.lib"
  )

list(APPEND _cmake_import_check_targets RtAudio::rtaudio )
list(APPEND _cmake_import_check_files_for_RtAudio::rtaudio "${_IMPORT_PREFIX}/lib/rtaudio.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
