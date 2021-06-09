fn main() {}

/// 获取昨天、明天的API
#[test]
fn get_yesterday_and_tomorrow_api() {
    use chrono::{Date, Local};
    let today: Date<Local> = Local::today();
    let yesterday: Date<Local> = today.pred();
    let tomorrow: Date<Local> = today.succ();
    dbg!(yesterday, today, tomorrow);
}

/// 获取两个日期之间相差几天
#[test]
fn timedelta_days_subtract() {
    use chrono::{Date, Datelike, Local, TimeZone};
    let now: Date<Local> = Local::today();
    let begin_of_year: Date<Local> = Local.ymd(now.year(), 1, 1);
    let date_diff = now.signed_duration_since(begin_of_year);
    dbg!(date_diff);
    let date1: Date<Local> = Local.ymd(now.year(), 1, 1);
    let date2: Date<Local> = Local.ymd(now.year(), 1, 2);
    assert_eq!(date2.signed_duration_since(date1).num_days(), 1);
}

/// 判断某年的每一天是不是周末
#[test]
fn weekend_of_a_year() {
    use chrono::{Date, Datelike, Local, TimeZone, Weekday};
    let first_day: Date<Local> = Local.ymd(2020, 1, 1);
    let mut workdays = ['1'; 366];
    for i in 0..366 {
        let date = first_day + chrono::Duration::days(i as i64);
        if date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun {
            workdays[i] = '0';
        }
    }
}

/** 获取时区
https://twitter.com/andelf/status/1276470803387740160
更准确获取时区的方法是调用C语言的localtime_r或gmtime API
*/
#[test]
fn get_local_timezone() {
    let timezone = chrono::Local::now().offset().to_string();
    dbg!(timezone);
}

/// 获取当前时间戳的三种方法
#[test]
fn get_timestamp() {
    // 方法一: 通过C语言time()函数
    extern "C" {
        fn time(time_t_mut: *mut i64) -> i64;
    }
    let c_now_timestamp = unsafe { time(std::ptr::null_mut() as *mut i64) };

    // 方法二: Rust标准库获取当前时间戳的API
    let std_now_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // 方法三: chrono的API获取当前时间戳
    // warn chrono::Local和naive_local()连用会出错，当地时区便宜会叠加两次，时区从+8变+16...
    // let chrono_local_naive_local: i64 = chrono::Local::now().naive_local().timestamp();
    let chrono_now_timestamp = chrono::Utc::now().timestamp();

    assert_eq!(c_now_timestamp, std_now_timestamp);
    assert_eq!(std_now_timestamp, chrono_now_timestamp);

    assert_eq!(
        chrono::Utc::now().timestamp(),
        chrono::Local::now().timestamp()
    );
}

/** 实现chrono::Local::now()
## 解释chrono::NaiveDate源码中的719162
719162 is the number of days between 1/1/1970 and 1/1/0001
In the Gregorian calendar, there are 477 leap years between 1 and 1970, so 365 * 1969 + 477 = 719162 days
https://unix.stackexchange.com/questions/149858/convert-a-number-of-seconds-elapsed-to-date-from-arbitrary-start-date
chrono为了更方便的计算日期以及闰年的影响，要将unix时间戳距离1970年1日1日的日数偏移到距离0001年1月1日的日数
719162+1是因为Rust的chrono将去年的12月31日作为第一天，这样下标1就等于1月1日比较方便
*/
#[test]
fn impl_chrono_local_now() {
    use std::os::raw::{c_char, c_int, c_long};

    #[allow(non_camel_case_types)]
    type time_t = i64;

    #[allow(non_snake_case)]
    #[derive(Debug)]
    #[repr(C)]
    struct tm {
        tm_sec: c_int,
        tm_min: c_int,
        tm_hour: c_int,
        tm_mday: c_int,
        tm_mon: c_int,
        tm_year: c_int,
        tm_wday: c_int,
        tm_yday: c_int,
        tm_isdst: c_int,
        tm_gmtoff: c_long,
        // mac:   tm_zone: *mut c_char
        // linux: tm_zone: *const c_char
        tm_zone: *mut c_char,
    }

    extern "C" {
        fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm;
        fn time(time: *mut time_t) -> time_t;
    }

    struct LocalTime {
        timezone_offset_in_seconds: time_t,
    }

    impl LocalTime {
        fn new() -> Self {
            let mut tm_struct: tm = unsafe { std::mem::zeroed() };
            // get localtime timezone and daylight_saving_time information with 0 seconds
            unsafe {
                // time_t is i64 in common 64 bit OS, but time_t is i32 in raspberrypi
                let time_zero: time_t = 0;
                if localtime_r(&time_zero as *const time_t, &mut tm_struct).is_null() {
                    panic!("error in localtime_r system call");
                }
                dbg!(std::ffi::CStr::from_ptr(tm_struct.tm_zone));
            }
            dbg!(&tm_struct);

            /*
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
            */
            Self {
                timezone_offset_in_seconds: tm_struct.tm_gmtoff as time_t,
            }
        }

        /// [std::time::SystemTime::now() syscall `gettimeofday`](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/time.rs#L186)
        /// 由于目前我只是想显示时分秒，不关心时间戳的微秒部分，所以用time系统调用即可，在libc源码中gettimeofday内的tv_ses部分就是time()函数求出的，所以time()更轻量
        /// 而且gettimeofday的第二个参数传一个timezone结构体似乎并不能让时间戳offset
        fn now_hms(&self) -> (time_t, time_t, time_t) {
            let mut now_sec: time_t = unsafe { std::mem::zeroed() };
            unsafe {
                time(&mut now_sec as *mut _);
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

    // test
    let localtime = LocalTime::new();
    localtime.now_hms();
}
