use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Record {
    pub callsign: String,
    pub serial: String,
    pub name: String,
    pub datetime: NaiveDateTime,
    pub port: String,
    pub location: String,
}

const DATETIME_FORMAT: &'static str = "%Y/%m/%d %H:%M:%S";

impl Record {
    pub fn from(parts: &[&str]) -> Option<Record> {
        Some(Record {
            callsign: parts.get(0)?.parse().ok()?,
            serial: parts.get(1)?.parse().ok()?,
            name: parts.get(2)?.parse().ok()?,
            datetime: NaiveDateTime::parse_from_str(parts.get(3)?, "%Y/%m/%d %H:%M:%S").ok()?,
            port: parts.get(4)?.parse().ok()?,
            location: parts.get(6)?.parse().ok()?,
        })
    }

    pub fn to_string(&self, sep: &str) -> String {
        [
            &self.callsign,
            &self.serial,
            &self.name,
            &self.datetime.format(DATETIME_FORMAT).to_string(),
            &self.port,
            "",
            &self.location,
            "",
        ]
        .join(sep)
    }
}

pub fn read_csv_file(
    file_path: &str,
    log_map: &mut HashMap<(String, String), Record>,
    retries: usize,
) -> Result<(), Box<dyn Error>> {
    for _ in 0..retries - 1 {
        let result = read_csv_file_internal(file_path, log_map);
        if result.is_ok() {
            return result;
        }
    }

    read_csv_file_internal(file_path, log_map)
}
fn read_csv_file_internal(
    file_path: &str,
    log_map: &mut HashMap<(String, String), Record>,
) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read(file_path)?;
    let string_file = String::from_utf8_lossy(&file_content);
    for line in string_file.lines() {
        let parts: Vec<&str> = line.split('%').collect();
        match Record::from(&parts) {
            None => (),
            Some(record) => {
                log_map.insert((record.callsign.clone(), record.serial.clone()), record);
            }
        }
    }
    Ok(())
}

pub fn write_csv_file(
    file_path: &str,
    log_map: &HashMap<(String, String), Record>,
    retries: usize,
) -> Result<(), Box<dyn Error>> {
    for _ in 0..retries - 1 {
        let result = write_csv_file_internal(file_path, log_map);
        if result.is_ok() {
            return result;
        }
    }

    read_csv_file_internal(file_path, log_map)
}
fn write_csv_file_internal(
    file_path: &str,
    log_map: &HashMap<(String, String), Record>,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    for (_, value) in log_map.iter() {
        file.write_all(value.to_string("%").as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

pub fn trim_map_to_last_n(log_map: &mut HashMap<(String, String), Record>, n: usize) {
    let mut records_vec: Vec<(&(String, String), &Record)> = log_map.iter().collect();
    records_vec.sort_by(|(_, a), (_, b)| b.datetime.cmp(&a.datetime));

    let keys_to_remove: Vec<(String, String)> = records_vec
        .iter()
        .skip(n)
        .map(|&(key, _)| key.clone())
        .collect();

    for key in keys_to_remove {
        log_map.remove(&key);
    }
}
