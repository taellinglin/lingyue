#----------------------------------------------------------------
# Generated CMake target import file for configuration "RelWithDebInfo".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "RtAudio::rtaudio" for configuration "RelWithDebInfo"
set_property(TARGET RtAudio::rtaudio APPEND PROPERTY IMPORTED_CONFIGURATIONS RELWITHDEBINFO)
set_target_properties(RtAudio::rtaudio PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELWITHDEBINFO "CXX"
  IMPORTED_LOCATION_RELWITHDEBINFO "${_IMPORT_PREFIX}/lib/rtaudio.lib"
  )

list(APPEND _cmake_import_check_targets RtAudio::rtaudio )
list(APPEND _cmake_import_check_files_for_RtAudio::rtaudio "${_IMPORT_PREFIX}/lib/rtaudio.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
