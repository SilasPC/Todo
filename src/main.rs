#![allow(dead_code)]

use std::fs;

mod data;
use data::*;

const FILE: &str = "/etc/rust_todo";

fn main() {
    if let Err(e) = main_r() {
        println!("{}",e)
    }
}

fn main_r() -> Result<(), String> {
    let mut data = read();
    let mut note = String::new();
    let mut pri = 0;
    let mut date: IncOpt = None.into();
    let mut argi = std::env::args()
        .skip(1)
        .filter(|a| !a.is_empty());
    while let Some(arg) = argi.next() {
        if arg.chars().nth(0).unwrap() != '-' {
            note.push_str(&arg);
            break;
        }
        let mut citer = arg.chars().skip(1);
        while let Some(c) = citer.next() {
            match c {
                'r' => {
                    citer
                        .next()
                        .map_or(Ok(()), |_| Err("Expected argument after -r"))?;
                    let idx: usize = argi
                        .next()
                        .and_then(|s| s.parse().ok())
                        .ok_or("Expected an index for -r")?;
                    if idx > data.tasks.len() || idx == 0 {
                        return Err("Index out of range".into());
                    }
                    data.tasks.remove(idx-1);
                    write(data);
                    return Ok(());
                },
                'd' => {
                    citer
                        .next()
                        .map_or(Ok(()), |_| Err("Expected argument after -r"))?;
                    let date_string = argi
                        .next()
                        .ok_or("Expected argument after -d")?;
                    if date.0.is_some() {
                        return Err("More than one date specified".into());
                    }
                    use chrono::NaiveDate;
                    date = NaiveDate::parse_from_str(&date_string,"%d/%m/%y")
                        .map_err(|_| "Date must be in d/m/y format")?.into();
                },
                'p' => {
                    pri += 1;
                }
                _ => {
                    return Err(format!("Unknown option '{}'",c));
                }
            }
        }
    }
    for arg in argi {
        note.push_str(" ");
        note.push_str(&arg);
    }
    if note.is_empty() {
        data.print();
        return Ok(());
    }
    let task = Task {
        pri,
        date,
        note,
    };
    data.insert(task);
    write(data);
    Ok(())
}

fn read() -> Data {
    let s = fs::read(FILE)
        .unwrap_or_else(|_| serde_json::to_string(&Data::default()).unwrap().into());
    let s = String::from_utf8(s).unwrap();
    serde_json::from_str(&s).unwrap()
}
fn write(data: Data) {
    fs::write(FILE, &serde_json::to_string(&data).unwrap()).unwrap();
}