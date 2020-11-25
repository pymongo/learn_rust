//! ## 解释chrono::NaiveDate源码中的719162
//! 719162 is the number of days between 1/1/1970 and 1/1/0001
//! In the Gregorian calendar, there are 477 leap years between 1 and 1970, so 365 * 1969 + 477 = 719162 days
//! https://unix.stackexchange.com/questions/149858/convert-a-number-of-seconds-elapsed-to-date-from-arbitrary-start-date
//! chrono为了更方便的计算日期以及闰年的影响，要将unix时间戳距离1970年1日1日的日数偏移到距离0001年1月1日的日数
//! 719162+1是因为Rust的chrono将去年的12月31日作为第一天，这样下标1就等于1月1日比较方便
use libc::{c_int, time_t};

/// My gettimeofday syscall warrper
struct LocalTime {
    timezone_offset_in_seconds: time_t,
}

impl LocalTime {
    fn new() -> Self {
        let mut out: libc::tm = unsafe { std::mem::zeroed() };
        // get localtime timezone and daylight_saving_time information with 0 seconds
        unsafe {
            // time_t is i64 in common 64 bit OS, but time_t is i32 in raspberrypi
            let time_zero: time_t = 0;
            if libc::localtime_r(&time_zero as *const time_t, &mut out).is_null() {
                panic!("error in localtime_r system call");
            }
            dbg!(std::ffi::CStr::from_ptr(out.tm_zone));
        }
        dbg!(out.tm_gmtoff, out.tm_isdst, out.tm_hour);

        let tz_dsttime;
        if out.tm_isdst.is_negative() {
            println!("daylight_saving_time information is not available");
            tz_dsttime = 0;
        } else if out.tm_isdst == 0 {
            println!("daylight_saving_time=0 is not in effect");
            tz_dsttime = 0;
        } else {
            println!("daylight_saving_time={} is in effect", out.tm_isdst);
            tz_dsttime = out.tm_isdst;
        }

        Self {
            timezone_offset_in_seconds: out.tm_gmtoff,
        }
    }

    /// [std::time::SystemTime::now() syscall `gettimeofday`](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/time.rs#L186)
    /// 由于目前我只是想显示时分秒，不关心时间戳的微秒部分，所以用time系统调用即可，在libc源码中gettimeofday内的tv_ses部分就是time()函数求出的，所以time()更轻量
    /// 而且gettimeofday的第二个参数传一个timezone结构体似乎并不能让时间戳offset
    fn now_hms(&self) -> (time_t, time_t, time_t) {
        let mut now_sec: time_t = unsafe { std::mem::zeroed() };
        unsafe {
            libc::time(&mut now_sec as *mut time_t);
        }
        now_sec += self.timezone_offset_in_seconds;
        let today_seconds = now_sec % (24 * 3600);
        // 不能先算hour，否则hour准但是second和minute都不准
        // 「秒转时分秒跟加法要从最小位开始逐步进位一样的思想」: 要优先填充小的刻度second，second进位部分成minutes，minutes进位部分成hour
        let minutes = today_seconds / 60;
        let second = today_seconds - minutes * 60;
        let hour = minutes / 60;
        let minute = minutes - hour * 60;
        println!("localtime now is {}:{}:{}", hour, minute, second);
        (hour, minute, second)
    }
}

fn main() {
    let localtime = LocalTime::new();
    localtime.now_hms();
}
