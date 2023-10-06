use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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
    pub fn from(parts: &Vec<&str>) -> Result<Record, Box<dyn Error>> {
        Ok(Record {
            callsign: parts[0].parse().unwrap(),
            serial: parts[1].parse().unwrap(),
            name: parts[2].parse().unwrap(),
            datetime: NaiveDateTime::parse_from_str(parts[3], "%Y/%m/%d %H:%M:%S")?,
            port: parts[4].parse().unwrap(),
            location: parts[6].parse().unwrap(),
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
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('%').collect();
        let record = Record::from(&parts)?;
        log_map.insert((record.callsign.clone(), record.serial.clone()), record);
    }
    Ok(())
}

pub fn write_csv_file(
    file_path: &str,
    log_map: &HashMap<(String, String), Record>,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    for (_, value) in log_map.iter() {
        file.write_all(value.to_string("%").as_bytes())?;
        file.write_all(&[b'\n'])?;
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
