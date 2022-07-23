cmake_minimum_required(VERSION 3.20)
project(screen_capture_lite VERSION 17.1.457)
set(namespace "ScreenCaptureLite")

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
option(BUILD_EXAMPLE "Build example" OFF)
# set(BUILD_SHARED_LIBS OFF CACHE INTERNAL "Build shared libraries")
set (CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

if (NOT EXISTS ${CMAKE_BINARY_DIR}/CMakeCache.txt)
  if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Release" CACHE STRING "" FORCE)
  endif()
endif()

if(MSVC)
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W4 -D_SILENCE_CXX20_OLD_SHARED_PTR_ATOMIC_SUPPORT_DEPRECATION_WARNING")
else()
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -pedantic")
endif()

add_subdirectory(src_cpp)

set(IOS_HEADERS
  include/ios/CGFrameProcessor.h
  include/ios/NSFrameProcessor.h
  include/ios/NSFrameProcessorm.h
  include/ios/NSMouseCapture.h
  include/ios/NSMouseProcessor.h
)

set(LINUX_HEADERS
  include/linux/X11FrameProcessor.h
  include/linux/X11MouseProcessor.h
)

set(WINDOWS_HEADERS
  include/windows/DXFrameProcessor.h
  include/windows/GDIFrameProcessor.h
  include/windows/GDIHelpers.h
  include/windows/GDIMouseProcessor.h
)

set(PUBLIC_HEADERS
  include/ScreenCapture_C_API.h
  include/ScreenCapture.h
)

set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER "${PUBLIC_HEADERS}")

target_include_directories(${PROJECT_NAME}
  PRIVATE
    include
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>/include
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

get_target_property(target_type ${PROJECT_NAME} TYPE)

set(LIB_SUFFIX ${CMAKE_STATIC_LIBRARY_SUFFIX})

message(STATUS "${target_type}")
message(STATUS "Suffix: ${LIB_SUFFIX}")

set(INCLUDE_INSTALL_DIR include/${PROJECT_NAME})
set(LIB_INSTALL_DIR lib)

install (TARGETS ${PROJECT_NAME}
  EXPORT "${PROJECT_NAME}Targets"
	RUNTIME DESTINATION bin
	ARCHIVE DESTINATION ${LIB_INSTALL_DIR}
	LIBRARY DESTINATION ${LIB_INSTALL_DIR}
  PUBLIC_HEADER DESTINATION ${INCLUDE_INSTALL_DIR}
  INCLUDES DESTINATION ${INCLUDE_INSTALL_DIR}
)

install(EXPORT "${PROJECT_NAME}Targets"
  FILE "${PROJECT_NAME}Targets.cmake"
  NAMESPACE ${namespace}::
  DESTINATION lib/cmake/${PROJECT_NAME}
)

include(CMakePackageConfigHelpers)

# generate the version file for the config file
write_basic_config_version_file(
  "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
  VERSION "${VERSION}"
  COMPATIBILITY AnyNewerVersion
)

# create config file
configure_package_config_file(
  ${CMAKE_CURRENT_SOURCE_DIR}/Config.cmake.in
  "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
  INSTALL_DESTINATION lib/cmake/${PROJECT_NAME}
  PATH_VARS INCLUDE_INSTALL_DIR LIB_INSTALL_DIR LIB_SUFFIX
)

# install config file
install(FILES
  "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
  "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
  DESTINATION ${LIB_INSTALL_DIR}/cmake/${PROJECT_NAME}
)

install(FILES
  ${WINDOWS_HEADERS}
  DESTINATION ${INCLUDE_INSTALL_DIR}/windows
  COMPONENT Headers
)

install(FILES
  ${LINUX_HEADERS}
  DESTINATION ${INCLUDE_INSTALL_DIR}/linux
  COMPONENT Headers
)

install(FILES
  ${IOS_HEADERS}
  DESTINATION ${INCLUDE_INSTALL_DIR}/ios
  COMPONENT Headers
)

if (NOT TARGET uninstall)
  configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
    IMMEDIATE @ONLY
  )

  add_custom_target(uninstall
    COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake
  )
endif()

enable_testing()
