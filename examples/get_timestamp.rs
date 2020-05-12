use chrono::NaiveDateTime;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let system_time_timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let chrono_local: i64 = chrono::Local::now().timestamp();
    let chrono_utc: i64 = chrono::Utc::now().timestamp();
    let chrono_utc_naive_utc: i64 = chrono::Utc::now().naive_utc().timestamp();
    let chrono_local_naive_utc: i64 = chrono::Local::now().naive_utc().timestamp();
    let chrono_utc_naive_local: i64 = chrono::Utc::now().naive_local().timestamp();
    // FIXME chrono::Local和naive_local()连用会出错，时区从+8变+16...
    let chrono_local_naive_local: i64 = chrono::Local::now().naive_local().timestamp();
    assert_eq!(system_time_timestamp, chrono_local as u64);
    assert_eq!(chrono_local, chrono_utc);
    assert_eq!(chrono_utc, chrono_utc_naive_utc);
    assert_eq!(chrono_utc_naive_utc, chrono_local_naive_utc);
    assert_eq!(chrono_local_naive_utc, chrono_utc_naive_local);
    // FIXME chrono::Local和naive_local()连用会出错，时区从+8变+16...
    assert_ne!(chrono_utc_naive_local, chrono_local_naive_local);
    dbg!(NaiveDateTime::from_timestamp(
        system_time_timestamp as i64,
        0,
    ));
    dbg!(NaiveDateTime::from_timestamp(chrono_local, 0));
    dbg!(NaiveDateTime::from_timestamp(chrono_utc, 0));
    dbg!(NaiveDateTime::from_timestamp(chrono_utc_naive_utc, 0));
    dbg!(NaiveDateTime::from_timestamp(chrono_local_naive_utc, 0));
    dbg!(NaiveDateTime::from_timestamp(chrono_utc_naive_local, 0));
    dbg!(NaiveDateTime::from_timestamp(chrono_local_naive_local, 0));

    let chrono_naive_utc = chrono::Local::now().naive_utc();
    let chrono_naive_local = chrono::Local::now().naive_local();
    println!(
        "chrono::Local::now().naive_utc().timestamp()  = {:?}",
        chrono_naive_utc
    );
    println!(
        "chrono::Local::now().naive_local().timestamp()  = {:?}",
        chrono_naive_local
    );
}
