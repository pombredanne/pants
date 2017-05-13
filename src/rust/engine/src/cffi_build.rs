extern crate gcc;

/*

N.B. This build script is invoked by `cargo` by way of this configuration
in our Cargo.toml:

    [project]
    ...
    build = "src/cffi_build.rs"

Within, we use the `gcc` crate to compile the CFFI C sources (`native_engine.c`)
generated by `bootstrap.sh` into a (private) static lib (`libnative_engine_ffi.a`),
which then gets linked into the final `cargo build` product (the native engine binary).
This process mixes the Python module initialization function and other symbols into the
native engine binary, allowing us to address it both as an importable python module
(`from _native_engine import X`) as well as a C library (`ffi.dlopen(native_engine.so)`).

*/

fn main() {
  gcc::Config::new()
    // N.B. The filename of this source code - at generation time - must line up 1:1 with the
    // python import name, as python keys the initialization function name off of the import name.
    .file("src/cffi/native_engine.c")
    .compile("libnative_engine_ffi.a");
}
