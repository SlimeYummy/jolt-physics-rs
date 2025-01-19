set CC=clang-cl
set CXX=clang-cl
set CFLAGS=/clang:-fms-compatibility /clang:-fms-extensions
set CXXFLAGS=/clang:-fms-compatibility /clang:-fms-extensions /D RUST_CXX_NO_EXCEPTIONS
set AR=llvm-ar

cargo test %*
