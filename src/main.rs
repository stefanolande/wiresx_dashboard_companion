#![windows_subsystem = "windows"]

extern crate user32;
extern crate winapi;

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use chrono::NaiveDateTime;
use tray_item::{IconSource, TrayItem};
use user32::{FindWindowA, MessageBoxA, PostMessageA};
use winapi::um::winuser::{MB_ICONINFORMATION, MB_OK, WM_KEYDOWN};

use wiresx_csv::Record;

use crate::conf::Config;

mod conf;
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

    show_dialog(
        "Wires-X Dashboard Companion",
        "Starting the program in the tray bar - this message will close in 5 seconds",
        Some(5),
    );

    loop {
        read_csv_file(&cfg.wires_x_log, &mut lines)?;
        trim_map_to_last_n(&mut lines, cfg.max_log_size);
        write_csv_file(&cfg.write_log, &lines)?;
        thread::sleep(Duration::from_secs(cfg.refres_interval as u64));

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

fn show_dialog(title: &str, message: &str, close_after: Option<u64>) {
    let lp_text = CString::new(message).unwrap();
    let lp_caption = CString::new(title).unwrap();
    let caption_clone = lp_caption.clone();

    match close_after {
        Some(delay) => {
            thread::spawn(move || unsafe {
                thread::sleep(Duration::from_secs(delay));

                let target_window =
                    FindWindowA(std::ptr::null(), caption_clone.as_ptr() as *const i8);
                PostMessageA(target_window, WM_KEYDOWN, 0x0D, 0);
            });
        }
        None => {}
    }

    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        );
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
