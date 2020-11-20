fn main() {
    // 当然这里也能用 Command::new("g++")，不过不如cc库那么好的可读性和方便
    cc::Build::new()
        .warnings(true)
        .flag("-v") // verbose
        .file("c_lib.c")
        .compile("c_lib");
    //cc库那可读性和易用性更好，但是rustc更新到某个版本后就`linking with `cc` failed: exit code: 1`
    // std::process::Command::new("g++")
    //     .arg("-std=c++14")
    //     .arg("-Wall")
    //     .spawn();
}
