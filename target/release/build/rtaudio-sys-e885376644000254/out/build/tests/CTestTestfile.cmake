# CMake generated Testfile for 
# Source directory: C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests
# Build directory: C:/林乐/lingyue/target/release/build/rtaudio-sys-e885376644000254/out/build/tests
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
if(CTEST_CONFIGURATION_TYPE MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
  add_test(apinames "C:/林乐/lingyue/target/release/build/rtaudio-sys-e885376644000254/out/build/tests/Debug/apinames.exe")
  set_tests_properties(apinames PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;32;add_test;C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;0;")
elseif(CTEST_CONFIGURATION_TYPE MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
  add_test(apinames "C:/林乐/lingyue/target/release/build/rtaudio-sys-e885376644000254/out/build/tests/Release/apinames.exe")
  set_tests_properties(apinames PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;32;add_test;C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;0;")
elseif(CTEST_CONFIGURATION_TYPE MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
  add_test(apinames "C:/林乐/lingyue/target/release/build/rtaudio-sys-e885376644000254/out/build/tests/MinSizeRel/apinames.exe")
  set_tests_properties(apinames PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;32;add_test;C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;0;")
elseif(CTEST_CONFIGURATION_TYPE MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
  add_test(apinames "C:/林乐/lingyue/target/release/build/rtaudio-sys-e885376644000254/out/build/tests/RelWithDebInfo/apinames.exe")
  set_tests_properties(apinames PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;32;add_test;C:/Users/User/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rtaudio-sys-0.3.0/rtaudio/tests/CMakeLists.txt;0;")
else()
  add_test(apinames NOT_AVAILABLE)
endif()
