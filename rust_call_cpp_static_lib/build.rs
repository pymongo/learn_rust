fn main() {
    // 当然这里也能用 Command::new("g++")，不过不如cc库那么好的可读性和方便
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-std=c++14")
        .flag("-Wall")
        .flag("-c")
        .file("cpp_lib_2.cpp")
        .compile("cpp_lib");
}