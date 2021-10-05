cmake_minimum_required(VERSION 3.14)

include(FetchContent)
include(ExternalProject)
# set the project name
project(Test)

ExternalProject_Add(catch2
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/catch2
    URL https://github.com/catchorg/Catch2/archive/v2.13.4.tar.gz
    INSTALL_DIR ${CMAKE_CURRENT_BINARY_DIR}/catch2
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/${CATCH2_NAME}
               -DCMAKE_C_COMPILER_WORKS=${CMAKE_C_COMPILER_WORKS} 
               -DCMAKE_C_FLAGS=${CATCH2_C_FLAGS} 
               -DCMAKE_XCODE_EFFECTIVE_PLATFORMS=${CMAKE_XCODE_EFFECTIVE_PLATFORMS} 
               -DCMAKE_OSX_SYSROOT=${CMAKE_OSX_SYSROOT}
               -DCMAKE_C_COMPILER=${CMAKE_C_COMPILER} 
               -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
               -DMSGPACK_CXX17=ON
)


ExternalProject_Add(ejdb
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/ejdb
    GIT_REPOSITORY https://github.com/Softmotions/ejdb.git
    INSTALL_DIR ${CMAKE_CURRENT_BINARY_DIR}/ejdb
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/${EJDB_NAME}
               -DCMAKE_C_COMPILER_WORKS=${CMAKE_C_COMPILER_WORKS} 
               -DCMAKE_C_FLAGS=${CATCH2_C_FLAGS} 
               -DCMAKE_XCODE_EFFECTIVE_PLATFORMS=${CMAKE_XCODE_EFFECTIVE_PLATFORMS} 
               -DCMAKE_OSX_SYSROOT=${CMAKE_OSX_SYSROOT}
               -DCMAKE_C_COMPILER=${CMAKE_C_COMPILER} 
               -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
               -DMSGPACK_CXX17=ON
               -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
)

ExternalProject_Add(result
    PREFIX ${CMAKE_CURRENT_BINARY_DIR}/result
    GIT_REPOSITORY https://github.com/bitwizeshift/result.git
    INSTALL_DIR ${CMAKE_CURRENT_BINARY_DIR}/result
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/${RESULT_NAME}
               -DCMAKE_C_COMPILER_WORKS=${CMAKE_C_COMPILER_WORKS} 
               -DCMAKE_C_FLAGS=${CATCH2_C_FLAGS} 
               -DCMAKE_XCODE_EFFECTIVE_PLATFORMS=${CMAKE_XCODE_EFFECTIVE_PLATFORMS} 
               -DCMAKE_OSX_SYSROOT=${CMAKE_OSX_SYSROOT}
               -DCMAKE_C_COMPILER=${CMAKE_C_COMPILER} 
               -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
               -DMSGPACK_CXX17=ON
               -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
)



#FetchContent_Declare(
#  ejdb
#  GIT_REPOSITORY https://github.com/Softmotions/ejdb.git
  #GIT_TAG        703bd9caab50b139428cea1aaff9974ebee5742e # release-1.10.0
#)
#FetchContent_MakeAvailable(ejdb)

# add the executable
add_executable(Test main.c EJDB.cpp EJDBQuery.cpp)
add_dependencies(Test catch2 ejdb result)
target_compile_features(Test PRIVATE cxx_std_17)
target_include_directories(Test PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/include)
target_link_directories(Test PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/lib)
target_link_libraries(Test PRIVATE ejdb2)
