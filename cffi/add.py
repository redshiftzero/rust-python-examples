import cffi

# The extension may differ on your system (it can be: *.so, *.dylib, *.dll)
location = "adder/target/release/libadder.dylib"

ffi = cffi.FFI()
ffi.cdef("""
    int add(int, int);
""")
adder = ffi.dlopen(location)

assert adder.add(2, 2) == 4
