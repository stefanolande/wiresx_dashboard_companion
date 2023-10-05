use std::collections::BTreeMap;
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
    lines: &mut BTreeMap<NaiveDateTime, Record>,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path).expect("Failed to open log file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('%').collect();
        let record = Record::from(&parts)?;
        lines.insert(record.datetime, record);
    }
    Ok(())
}

pub fn write_csv_file(
    file_path: &str,
    lines: &BTreeMap<NaiveDateTime, Record>,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    for (_, value) in lines.iter() {
        file.write_all(value.to_string("%").as_bytes())?;
        file.write_all(&[b'\n'])?;
    }
    Ok(())
}

pub fn trim_map_to_last_n(map: &mut BTreeMap<NaiveDateTime, Record>, n: usize) {
    let excess_items = map.len().saturating_sub(n);
    let keys_to_remove: Vec<_> = map.keys().take(excess_items).cloned().collect();
    for key in keys_to_remove {
        map.remove(&key);
    }
}
