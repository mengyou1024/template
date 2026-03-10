fn main() {
    cxx_build::bridge("src/main.rs")
        .file("cxx/src/blobstore.cpp")
        .std("{{cpp_standard}}")
        .compile("cxx-bridge");

    println!("cargo:rerun-if-changed=cxx/src/blobstore.cpp");
    println!("cargo:rerun-if-changed=cxx/include/blobstore.hpp");
}