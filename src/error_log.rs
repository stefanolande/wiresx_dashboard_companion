use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Error as IOError;
use std::io::Write;

use chrono::Local;

pub fn write_error(error: &Box<dyn Error>) -> Result<(), Box<dyn Error>> {
    let dir = env::current_exe().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let path = dir
        .parent()
        .ok_or("Failed to get parent directory")?
        .to_str()
        .ok_or("Failed to convert to str")?;

    let now = Local::now();
    let formatted_timestamp = now.format("%Y-%m-%d-%H%M%S").to_string();

    let file_path = format!("{}\\wxdc-error-report-{}.txt", path, formatted_timestamp);
    let error_kind = if let Some(io_error) = error.downcast_ref::<IOError>() {
        io_error.kind().to_string()
    } else {
        "".to_string()
    };
    let error_str = format!("{} - {}", error_kind, error.as_ref().to_string());

    let mut file = File::create(file_path).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    file.write_all(error_str.as_bytes())?;

    Ok(())
}
