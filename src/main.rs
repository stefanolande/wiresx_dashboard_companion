#![windows_subsystem = "windows"]

extern crate user32;
extern crate winapi;

use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use chrono::NaiveDateTime;
use tray_item::{IconSource, TrayItem};

use wiresx_csv::Record;

use crate::conf::Config;
use crate::windows::show_dialog;
use crate::wiresx_csv::{read_csv_file, trim_map_to_last_n, write_csv_file};

mod conf;
mod windows;
mod wiresx_csv;

enum Message {
    Quit,
}

fn main() {
    let res = main_logic();

    match res {
        Ok(_) => (),
        Err(err) => show_dialog(
            "Wires-X Dashboard Companion Error",
            err.to_string().as_str(),
            None,
        ),
    }
}

fn main_logic() -> Result<(), Box<dyn Error>> {
    let cfg = Config::load()?;
    let mut lines: BTreeMap<NaiveDateTime, Record> = BTreeMap::new();

    let mut tray = TrayItem::new(
        "Wires-X Dashboard Companion",
        IconSource::Resource("aa-exe-icon"),
    )?;

    tray.add_label("Wires-X Dashboard Companion")?;
    tray.inner_mut().add_separator()?;

    let (tx, rx) = mpsc::sync_channel(1);

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })?;

    println!("Wires-X Dashboard Companion started");

    if cfg.show_startup_message {
        show_dialog(
            "Wires-X Dashboard Companion",
            "Starting the program in the tray bar.\nThis message will close in 5 seconds.",
            Some(5),
        );
    }

    if Path::new(&cfg.write_log).exists() {
        read_csv_file(&cfg.write_log, &mut lines)?;
    }

    loop {
        read_csv_file(&cfg.wires_x_log, &mut lines)?;
        trim_map_to_last_n(&mut lines, cfg.max_log_size);
        write_csv_file(&cfg.write_log, &lines)?;
        thread::sleep(Duration::from_secs(cfg.refresh_interval as u64));

        match rx.recv_timeout(Duration::from_millis(1)) {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            _ => {}
        }
    }
    return Ok(());
}
