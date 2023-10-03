#![windows_subsystem = "windows"]

use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc;
use std::{thread, time};

use chrono::NaiveDateTime;
use tray_item::{IconSource, TrayItem};

use wiresx_csv::Record;

use crate::conf::Config;

mod conf;
mod wiresx_csv;

enum Message {
    Quit,
}

fn main() {
    let cfg = Config::load().unwrap();
    let mut lines: BTreeMap<NaiveDateTime, Record> = BTreeMap::new();

    let mut tray = TrayItem::new(
        "Wires-X Dashboard Companion",
        IconSource::Resource("aa-exe-icon"),
    )
    .unwrap();

    tray.add_label("Wires-X Dashboard Companion").unwrap();
    tray.inner_mut().add_separator().unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    println!("Wires-X Dashboard Companion started");

    loop {
        read_csv_file(&cfg.wires_x_log, &mut lines).unwrap();
        trim_map_to_last_n(&mut lines, cfg.max_log_size);
        write_csv_file(&cfg.write_log, &lines).unwrap();
        thread::sleep(time::Duration::from_secs(cfg.refres_interval as u64));

        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            _ => {}
        }
    }
}

fn read_csv_file(
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

fn write_csv_file(
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

fn trim_map_to_last_n(map: &mut BTreeMap<NaiveDateTime, Record>, n: usize) {
    let excess_items = map.len().saturating_sub(n);
    let keys_to_remove: Vec<_> = map.keys().take(excess_items).cloned().collect();
    for key in keys_to_remove {
        map.remove(&key);
    }
}
