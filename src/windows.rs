use std::ffi::CString;
use std::thread;
use std::time::Duration;
use user32::{FindWindowA, MessageBoxA, PostMessageA};
use winapi::um::winuser::{MB_ICONINFORMATION, MB_OK, WM_KEYDOWN};

pub fn show_dialog(title: &str, message: &str, close_after: Option<u64>) {
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
