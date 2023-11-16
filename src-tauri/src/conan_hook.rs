use std::ffi::CString;
use std::process::Child;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::{HWND, LPARAM, BOOL, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, 
    GetWindowThreadProcessId,
    GetWindowTextW, SendMessageA, PostMessageA, WM_KEYDOWN
};

lazy_static! {
    static ref TYPING_LOOP_ACTIVE: AtomicBool = AtomicBool::new(false);
    static ref CONAN_SANDBOX_HWND: Arc<Mutex<Option<HWND>>> = Arc::new(Mutex::new(None));
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, param1: LPARAM) -> BOOL {

    let mut process_id: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id as *mut u32));

    if process_id == param1.0 as u32 {
        CONAN_SANDBOX_HWND.lock().unwrap().replace(hwnd);
        return BOOL(0);
    }

    return BOOL(1);

}

unsafe extern "system" fn enum_windows_existing_proc(hwnd: HWND, _param1: LPARAM) -> BOOL {

    let mut text: [u16; 256] = [0; 256];
    GetWindowTextW(hwnd, &mut text);

    let window_text = String::from_utf16(&text)
        .unwrap()
        .replace("\0", "");

    if window_text == "ConanSandbox " {
        CONAN_SANDBOX_HWND.lock().unwrap().replace(hwnd);
        return BOOL(0);
    }

    return BOOL(1);

}

pub fn hook_into_process(child: Child) {

    unsafe {

        match EnumWindows(Some(enum_windows_proc), LPARAM(child.id() as isize)) {
            Ok(_) => {},
            Err(_) => {}
        }

    }

}

pub fn hook_into_existing() -> bool {

    unsafe {
            
        match EnumWindows(Some(enum_windows_existing_proc), LPARAM(0)) {
            Ok(_) => {
                true
            },
            Err(_) => {
                false
            }
        }
    
    }

}

fn post_message(msg_type: u32, wparam: WPARAM) {

    if let Some(hwnd) = CONAN_SANDBOX_HWND.lock().unwrap().as_ref() {

        unsafe {
            let _ = PostMessageA(*hwnd, msg_type, wparam, LPARAM(0));
        }

    }

}

fn typing_loop() {

    while TYPING_LOOP_ACTIVE.load(Ordering::Relaxed) {

        post_message(WM_KEYDOWN, WPARAM('a' as usize));
        thread::sleep(Duration::from_millis(500));
        post_message(WM_KEYDOWN, WPARAM(0x08));
        thread::sleep(Duration::from_millis(500));

    }

}


#[tauri::command]
pub fn submit_actual_post(post: String) {

    let post = post.replace("ChatGPT", "");
    for c in post.chars() {
        post_message(WM_KEYDOWN, WPARAM(c as usize));
    }

}

#[tauri::command]
pub fn start_typing_loop() {

    if TYPING_LOOP_ACTIVE.load(Ordering::Relaxed) {
        return;
    }

    TYPING_LOOP_ACTIVE.store(true, Ordering::Relaxed);
    thread::spawn(typing_loop);

}