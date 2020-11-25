//! https://stackoverflow.com/questions/27665396/how-can-i-read-from-a-specific-raw-file-descriptor-in-rust
//! TODO 该链接的代码在Linux/mac系统上都跑不了，在playground上能跑，但如果fd指定了除012以外的index会报错OSError: Bad file descriptor
//! file descriptors index into a per-process file descriptor table maintained by the kernel
//! 文件描述符在数据结构上是非负整数，每个进程都有各自的文件描述符表，其中0-2分别是stdin,stdout,stderr
//! 文件描述符索引表类似虚拟内存，每个进程的fd索引表最终会映射到操作系统的fd表，类似每个进程的虚拟内存会最终映射到物理内存上，以此实现多个进程复用同一个块物理内存或fd
//! 每个进程的fd表会存在`/proc/${PID}/fd`，例如PID=1是systemd，/proc/1/fd# file 11 => 11: symbolic link to /proc/1/mountinfo

#[cfg(target_os = "unix")]
fn main() {
    use std::os::unix::io::FromFawFd;
    let mut f = unsafe { std::fs::File::from_raw_fd(3) };
}

#[cfg(target_os = "macos")]
fn main() {
    println!("std::os::unix::io::FromFawFd is only support in Linux");
}
