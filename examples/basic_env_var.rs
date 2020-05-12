fn main() {
    // 该代码的效果类似`go env`命令
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}

/*
Apple_PubSub_Socket_Render: /private/tmp/com.apple.launchd.DQjqUGmxsM/Render
CARGO: /Users/w/.rustup/toolchains/stable-x86_64-apple-darwin/bin/cargo
CARGO_HOME: /Users/w/.cargo
CARGO_MANIFEST_DIR: /Users/w/workspace/rust_projects/rust_learn
CARGO_PKG_AUTHORS: 吴翱翔 <pylint@yandex.com>
CARGO_PKG_DESCRIPTION:
CARGO_PKG_HOMEPAGE:
CARGO_PKG_NAME: rust_learn
CARGO_PKG_REPOSITORY:
CARGO_PKG_VERSION: 0.1.0
CARGO_PKG_VERSION_MAJOR: 0
CARGO_PKG_VERSION_MINOR: 1
CARGO_PKG_VERSION_PATCH: 0
CARGO_PKG_VERSION_PRE:
DYLD_FALLBACK_LIBRARY_PATH: /Users/w/workspace/rust_projects/rust_learn/target/debug/deps:/Users/w/workspace/rust_projects/rust_learn/target/debug:/Users/w/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib:/Users/w/.rustup/toolchains/stable-x86_64-apple-darwin/lib:/Users/w/lib:/usr/local/lib:/usr/lib
GOPATH: /Users/w/go
HOME: /Users/w
JAVA_HOME: /Users/w/.jenv/versions/1.8/
JENV_LOADED: 1
JENV_SHELL: bash
LC_CTYPE: en_US.UTF-8
LOGNAME: w
NVM_BIN: /Users/w/.nvm/versions/node/v10.16.0/bin
NVM_CD_FLAGS:
NVM_DIR: /Users/w/.nvm
NVM_INC: /Users/w/.nvm/versions/node/v10.16.0/include/node
PATH: /Users/w/.cargo/bin:/Users/w/.cargo/bin:/Users/w/.jenv/shims:/Users/w/.jenv/bin:/Users/w/.rbenv/shims:/Users/w/.rbenv/bin:/Users/w/.nvm/versions/node/v10.16.0/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/Applications/Postgres.app/Contents/Versions/latest/bin:/usr/local/mysql/bin
PWD: /Users/w/workspace/rust_projects/rust_learn
RBENV_SHELL: bash
RUSTUP_HOME: /Users/w/.rustup
RUSTUP_TOOLCHAIN: stable-x86_64-apple-darwin
RUST_BACKTRACE: short
RUST_RECURSION_COUNT: 1
SHELL: /bin/bash
SSH_AUTH_SOCK: /private/tmp/com.apple.launchd.sCVlRuCMek/Listeners
TERM: ansi
TMPDIR: /var/folders/1q/95drssmn7c55mrk5wy_dlk8r0000gn/T/
USER: w
VERSIONER_PYTHON_PREFER_32_BIT: no
VERSIONER_PYTHON_VERSION: 2.7
XPC_FLAGS: 0x0
XPC_SERVICE_NAME: com.jetbrains.CLion.16628
__CF_USER_TEXT_ENCODING: 0x1F5:0x0:0x0
*/
