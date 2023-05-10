#![allow(dead_code, unused_variables)]

use std::net::Ipv4Addr;
use std::str::FromStr;

use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Asia::Shanghai;

use crate::study_enum::Gender;
use crate::study_struct::{Count, User};

mod study_enum;
mod study_struct;

fn main() {
    let user1 = &User {
        name: "张某某".to_string(),
        age: 20,
        gender: Gender::Male,
    };
    println!("{:?}", user1.gender.index());
    println!("{:?}", user1.summarize());
    println!("{}", user1)
}

fn study_datetime() {
    println!("{}", Utc::now());
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    let local1 = local.with_hour(5).unwrap();
    println!("{}", local1);
    let tz = FixedOffset::east_opt(8 * 3600).unwrap();
    println!("{}", utc.with_timezone(&tz).format("%Y-%m-%d %H:%M:%S"));
    println!(
        "{}",
        utc.with_timezone(&Shanghai).format("%Y-%m-%d %H:%M:%S")
    );
    println!("{}", Utc::now().timestamp());
    println!("{}", Shanghai.timestamp_opt(1683275206, 0).unwrap());
    let dt1 = Utc.with_ymd_and_hms(2013, 11, 14, 8, 9, 10).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2014, 1, 14, 10, 9, 8).unwrap();
    let mut dt = dt1;
    while dt < dt2 {
        println!("{}", dt);
        dt += Duration::days(1);
    }
}

fn study_str2num() {
    let i1 = 8999999_i64.to_string();
    let s1 = String::from("456");
    let i2: i64 = s1.parse().unwrap();
    let i3 = i64::from_str("456").unwrap();
    println!("{i1} {i2} {i3}")
}

fn study_ipnetwork() {
    let ip: Ipv4Addr = "1.2.3.4".parse().unwrap();
    let ip1 = "1.2.3.4".parse::<Ipv4Addr>().unwrap();
    let ip2 = Ipv4Addr::from_str("1.2.3.4").unwrap();
    println!("{} {} {}", ip, ip1, ip2);
}

fn study_concat() {
    let mut a = String::from("aaaa");
    let b = String::from("bbbb");
    println!("{}", a.clone() + "333");
    a += &b;
    let somestr = format!("{a}{b}");
    println!("{}", somestr);
}
