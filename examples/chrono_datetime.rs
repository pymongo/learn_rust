use chrono::{Date, Datelike, Local, NaiveTime, TimeZone, Weekday, Timelike};
use std::ops::Add;

fn main() {}

// 获取昨天、明天的API
#[test]
fn get_yesterday_and_tomorrow_api() {
    let today: Date<Local> = Local::today();
    let yesterday: Date<Local> = today.pred();
    let tomorrow: Date<Local> = today.succ();
    dbg!(yesterday, today, tomorrow);
}


// 两个日期相减
#[test]
fn timedelta_days_subtract() {
    let now: Date<Local> = Local::today();
    let begin_of_year: Date<Local> = chrono::Local.ymd(now.year(), 1, 1);
    let date_diff = now.signed_duration_since(begin_of_year);
    dbg!(date_diff);
    let date1: Date<Local> = chrono::Local.ymd(now.year(), 1, 1);
    let date2: Date<Local> = chrono::Local.ymd(now.year(), 1, 2);
    assert_eq!(date2.signed_duration_since(date1).num_days(), 1);
}

// 判断当前时间是否在工作时间内
#[test]
fn is_work_time() {
    let time: NaiveTime = chrono::Local::now().time();
    // 去掉获取到的当前时间秒的小数部分
    let now = NaiveTime::from_hms(time.hour(), time.minute(), time.second());
    let work_start_time: NaiveTime = NaiveTime::from_hms(9, 0, 0);
    let work_end_time: NaiveTime = NaiveTime::from_hms(17, 0, 0);
    dbg!(now >= work_start_time && now <= work_end_time);
}

#[test]
fn weekend_of_a_year() {
    let first_day: Date<Local> = chrono::Local.ymd(2020, 1, 1);
    let mut workdays = ['1'; 366];
    for i in 0..366 {
        let date = first_day.add(chrono::Duration::days(i as i64));
        if date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun {
            workdays[i] = '0';
        }
    }
}

#[test]
fn get_local_timezone() {
    // https://twitter.com/andelf/status/1276470803387740160
    // 如果获取 unix timestamp=0 的话，以下写法会受到「夏令时影响」
    // let timezone = chrono::Local.timestamp(0, 0).offset().to_string();
    // 所以需要明确「当前时差」和「历史某一时刻时差」的影响，用的是offset_from_utc_date API传Local::now()做参数
    // 简单来说用UTC时区就行了，像BJT/CST这种没有夏时制影响的时区实在太惯着程序员的
    let timezone = chrono::Local::now().offset().to_string();
    dbg!(timezone);

    /* 以下笨方法不要学
    let local_now = chrono::Local::now().to_string();
    let temp: Vec<&str> = local_now.split_ascii_whitespace().collect();
    temp[2].to_string();
    */
}

