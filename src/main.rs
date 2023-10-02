use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::time::Duration;

use chrono::NaiveDateTime;
use csv::{ReaderBuilder, WriterBuilder};
use tokio::time::sleep;

use crate::conf::Config;
use wiresx_csv::Record;

mod conf;
mod wiresx_csv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::load()?;

    let mut lines: BTreeMap<NaiveDateTime, Record> = BTreeMap::new();

    println!("Wires-X Dashboard Companion started");

    loop {
        // Read the CSV file every 1 second
        read_csv_file(&cfg.wires_x_log, &mut lines).await?;

        trim_map_to_last_n(&mut lines, cfg.max_log_size);

        write_csv_file(&cfg.write_log, &lines).await?;

        // Sleep for 1 second
        sleep(Duration::from_secs(cfg.refres_interval as u64)).await;
    }
}

async fn read_csv_file<'a>(
    file_path: &str,
    lines: &'a mut BTreeMap<NaiveDateTime, Record>,
) -> Result<&'a mut BTreeMap<NaiveDateTime, Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'%')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(file);

    // Process CSV records
    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => {
                lines.insert(record.datetime, record);
            }
            Err(e) => eprintln!("Error reading CSV record: {:?}", e),
        }
    }

    Ok(lines)
}

async fn write_csv_file(
    file_path: &str,
    lines: &BTreeMap<NaiveDateTime, Record>,
) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new()
        .delimiter(b'%')
        .has_headers(false)
        .from_path(file_path)?;
    for (_, value) in lines.iter() {
        writer.serialize(value)?;
    }

    Ok(())
}

fn trim_map_to_last_n(map: &mut BTreeMap<NaiveDateTime, Record>, n: usize) {
    // Calculate the number of items to remove
    let excess_items = map.len().saturating_sub(n);

    // Collect the keys of the items to remove
    let keys_to_remove: Vec<_> = map.keys().take(excess_items).cloned().collect();

    // Remove the excess items from the map
    for key in keys_to_remove {
        map.remove(&key);
    }
}
