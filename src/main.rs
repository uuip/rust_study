#![allow(dead_code, unused_variables)]

use std::env;
use std::fs::{read_dir, read_to_string, File};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;

use calamine::Reader;
use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Asia::Shanghai;
use chrono_tz::UTC;
use glob::glob;

use crate::study_enum::Gender;
use crate::study_struct::{Count, User};

mod study_enum;
mod study_struct;

fn main() -> anyhow::Result<()> {
    // read_file();
    // write_file()?;
    // read_file_line()?;
    // write_file_line();
    // read_yaml()?;
    // write_yaml()?;
    // read_xlsx()?;
    // write_xlsx()?;
    path_operation()?;
    study_datetime();
    study_str2num().expect("TODO: panic message");
    let user1 = &User {
        name: "张某某".to_string(),
        age: 20,
        gender: Gender::Male,
    };
    let data: serde_yaml::Value = serde_yaml::to_value(user1)?;
    let data1: serde_json::Value = serde_json::to_value(user1)?;
    println!("{}", serde_yaml::to_string(&data)?);
    println!("{}", serde_json::to_string(&data1)?);
    println!("{:?}", user1.gender.index());
    println!("{:?}", user1.summarize());
    println!("{}", user1);
    Ok(())
}

fn read_file() {
    let file = read_to_string(r"D:\OneDrive\python\tool.py").unwrap();
    println!("{}", file)
}

fn read_file_line() -> anyhow::Result<()> {
    let file = File::open(r"D:\OneDrive\python\tool.py").unwrap();
    for l in BufReader::new(file).lines() {
        let line = l?;
        println!("{line}")
    }
    Ok(())
}

fn write_file() -> anyhow::Result<()> {
    let mut file = File::create("data.txt")?;
    file.write_all("999".as_bytes())?;
    Ok(())
}

fn write_file_line() {
    let file = File::create("data.txt").unwrap();
    let mut writer = LineWriter::new(file);
    for i in 1..10 {
        writer
            .write_all("ppp\n".as_bytes())
            .expect("error when write");
    }
}

fn read_yaml() -> anyhow::Result<()> {
    let file = File::open(r"C:\Users\sharp\AppData\Local\Programs\clash_win\config.yaml")?;
    let data: serde_yaml::Value = serde_yaml::from_reader(file)?;

    println!("{:?}", data["dns"]["nameserver"].as_sequence().unwrap());
    Ok(())
}

fn write_yaml() -> anyhow::Result<()> {
    let rdr = File::open(r"C:\Users\sharp\AppData\Local\Programs\clash_win\config.yaml")?;
    let data: serde_yaml::Value = serde_yaml::from_reader(rdr)?;
    let writer = File::create("data.txt")?;
    serde_yaml::to_writer(writer, &data)?;
    Ok(())
}

fn read_xlsx() -> anyhow::Result<()> {
    let mut workbook: calamine::Xlsx<_> =
        calamine::open_workbook(r"C:\Users\sharp\Desktop\data\2023-04-21-plan2-all-f11.xlsx")?;
    let sheet = workbook.worksheet_range("全国").unwrap()?;
    for row in sheet.rows() {
        println!("{:?}", row)
    }
    Ok(())
}

fn write_xlsx() -> anyhow::Result<()> {
    let mut workbook = rust_xlsxwriter::Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("都放到")?;
    sheet.write(0, 0, "some文本")?;
    workbook.save("data.xlsx")?;
    Ok(())
}

fn path_operation() -> anyhow::Result<()> {
    println!("{:?}", env::current_dir()?);
    println!("{}", PathBuf::from("aaa").exists());

    for f in read_dir(".")? {
        println!("{:?}", f?.file_name())
    }
    for f in glob(r"D:\project\rust\rs-df/**/*.rs")? {
        let p = f?;
        println!("{:?} {}", p.file_name().unwrap(), p.is_dir())
    }
    Ok(())
}

fn study_datetime() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("Utc now {}", utc);
    println!("timestamp {}", Utc::now().timestamp());
    println!(
        "datetime from timestamp {}",
        Shanghai.timestamp_opt(1683275206, 0).unwrap()
    );
    //转换时区
    let tz = FixedOffset::east_opt(8 * 3600).unwrap();
    println!(
        "utc-> FixedOffset{}",
        utc.with_timezone(&tz).format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "utc-> Shanghai{}",
        utc.with_timezone(&Shanghai).format("%Y-%m-%d %H:%M:%S")
    );
    // 替换时区
    println!(
        "replace tz {}",
        local.naive_local().and_local_timezone(UTC).unwrap()
    );
    //修改日期--指定时间
    let local1 = local.with_hour(5).unwrap();
    println!("replace hour {}", local1);
    let dt1 = Utc.with_ymd_and_hms(2013, 11, 14, 8, 9, 10).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2014, 1, 14, 10, 9, 8).unwrap();
    //修改日期--增量
    println!(
        "add Duration {}",
        dt1.checked_add_signed(Duration::days(1)).unwrap()
    );
    println!("add Duration {}", dt1 + Duration::days(1));
    // 遍历某段时间
    let mut dt = dt1;
    while dt < dt2 {
        println!("{}", dt);
        dt += Duration::days(1);
    }
}

fn study_concat() {
    let mut a = String::from("aaaa");
    let b = String::from("bbbb");
    println!("{}", a.clone() + "333");
    a += &b;
    let somestr = format!("{a}{b}");
    println!("{}", somestr);
}

fn study_str2num() -> anyhow::Result<()> {
    let i1 = 8999999_i64.to_string();
    let s1 = String::from("456");
    let i2: i64 = s1.parse()?;
    let i3 = i64::from_str("456")?;
    println!("{i1} {i2} {i3}");

    let f1 = 100.20.to_string();
    let s1 = String::from("456.360");
    let f2: f32 = s1.parse()?;
    let f3 = f32::from_str(&s1)?;
    let f4 = i3 as f32;
    println!("{f1} {f2} {} {}", f3.type_name(), f4.type_name());
    Ok(())
}

fn study_ipnetwork() -> anyhow::Result<()> {
    let ip: Ipv4Addr = "1.2.3.4".parse()?;
    let ip1 = "1.2.3.4".parse::<Ipv4Addr>()?;
    let ip2 = Ipv4Addr::from_str("1.2.3.4")?;
    println!("{} {} {}", ip, ip1, ip2);
    Ok(())
}

pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
