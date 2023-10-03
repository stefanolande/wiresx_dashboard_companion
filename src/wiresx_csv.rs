use std::error::Error;

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
