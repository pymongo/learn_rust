/*!
omit some env_var

## compile-time env_var(build.rs)

read build.rs stdout from `cat target/debug/build/print_env-4ddb605b8c366546/output`

```text
CARGO: /home/pi/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/cargo
CARGO_CFG_PANIC: unwind
CARGO_CFG_TARGET_ARCH: arm
CARGO_CFG_TARGET_ENDIAN: little
CARGO_CFG_TARGET_ENV: gnu
CARGO_CFG_TARGET_FAMILY: unix
CARGO_CFG_TARGET_FEATURE: aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2
CARGO_CFG_TARGET_HAS_ATOMIC: 16,32,64,8,ptr
CARGO_CFG_TARGET_HAS_ATOMIC_EQUAL_ALIGNMENT: 16,32,64,8,ptr
CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE: 16,32,64,8,ptr
CARGO_CFG_TARGET_OS: linux
CARGO_CFG_TARGET_POINTER_WIDTH: 32
CARGO_CFG_TARGET_THREAD_LOCAL:
CARGO_CFG_TARGET_VENDOR: unknown
CARGO_CFG_UNIX:
CARGO_HOME: /home/pi/.cargo
CARGO_MAKEFLAGS: --jobserver-fds=5,6 -j --jobserver-auth=5,6 -j
CARGO_MANIFEST_DIR: /home/pi/workspace/temp_projects/print_env
CARGO_PKG_AUTHORS: wuaoxiang <pylint@yandex.com>
CARGO_PKG_DESCRIPTION:
CARGO_PKG_HOMEPAGE:
CARGO_PKG_LICENSE:
CARGO_PKG_LICENSE_FILE:
CARGO_PKG_NAME: print_env
CARGO_PKG_REPOSITORY:
CARGO_PKG_VERSION: 0.1.0
CARGO_PKG_VERSION_MAJOR: 0
CARGO_PKG_VERSION_MINOR: 1
CARGO_PKG_VERSION_PATCH: 0
CARGO_PKG_VERSION_PRE:
DBUS_SESSION_BUS_ADDRESS: unix:path=/run/user/1000/bus
DEBUG: true
HOME: /home/pi
HOST: armv7-unknown-linux-gnueabihf
LANG: en_US.UTF-8
LD_LIBRARY_PATH: /home/pi/workspace/temp_projects/print_env/target/debug/deps:/home/pi/workspace/temp_projects/print_env/target/debug:/home/pi/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/lib:/home/pi/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib
LOGNAME: pi
MAIL: /var/mail/pi
NO_AT_BRIDGE: 1
NUM_JOBS: 4
OLDPWD: /home/pi/workspace/github_clone/rust-analyzer
OPT_LEVEL: 0
OUT_DIR: /home/pi/workspace/temp_projects/print_env/target/debug/build/print_env-4ddb605b8c366546/out
PATH: /home/pi/.cargo/bin:/home/pi/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
PROFILE: debug
PWD: /home/pi/workspace/temp_projects/print_env
RUSTC: rustc
RUSTDOC: rustdoc
RUSTUP_HOME: /home/pi/.rustup
RUSTUP_TOOLCHAIN: nightly-armv7-unknown-linux-gnueabihf
RUST_RECURSION_COUNT: 1
SHELL: /bin/bash
SHLVL: 1
SSH_TTY: /dev/pts/0
SSL_CERT_DIR: /usr/lib/ssl/certs
SSL_CERT_FILE: /usr/lib/ssl/certs/ca-certificates.crt
TARGET: armv7-unknown-linux-gnueabihf
TERM: xterm-256color
TEXTDOMAIN: Linux-PAM
USER: pi
XDG_RUNTIME_DIR: /run/user/1000
XDG_SESSION_TYPE: tty
_: /home/pi/.cargo/bin/cargo
```

---

## runtime env_var
```text
CARGO: /home/pi/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/cargo
CARGO_MANIFEST_DIR: /home/pi/workspace/temp_projects/print_env
CARGO_PKG_NAME: print_env
CARGO_PKG_VERSION_PRE:
DBUS_SESSION_BUS_ADDRESS: unix:path=/run/user/1000/bus
NO_AT_BRIDGE: 1
OLDPWD: /
RUSTUP_HOME: /home/pi/.rustup
RUSTUP_TOOLCHAIN: nightly-armv7-unknown-linux-gnueabihf
RUST_RECURSION_COUNT: 1
SHLVL: 1
_: /home/pi/.cargo/bin/cargo
```
*/
use std::os::unix::io::RawFd;

/// 该代码的效果类似`go env`命令
fn main() {
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
