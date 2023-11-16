use std::process::Child;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use windows::Win32::Foundation::{HWND, LPARAM, BOOL, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, 
    GetWindowThreadProcessId,
    GetWindowTextW,
    PostMessageW,
    WM_KEYDOWN,
    WM_CHAR
};

lazy_static! {
    static ref TYPING_LOOP_ACTIVE: AtomicBool = AtomicBool::new(false);
    static ref CONAN_SANDBOX_HWND: Arc<Mutex<Option<HWND>>> = Arc::new(Mutex::new(None));
    static ref TYPING_JOIN_HANDLE: Arc<Mutex<Option<JoinHandle<()>>>> = Arc::new(Mutex::new(None));
}

const ENTER_KEY: usize = 0x0D;
const ESC_KEY: usize = 0x1B;
const BACKSPACE_KEY: usize = 0x08;
const A_KEY: usize = 0x41;

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

pub fn hook_into_existing() {

    unsafe {
            
        match EnumWindows(Some(enum_windows_existing_proc), LPARAM(0)) {
            Ok(_) => {},
            Err(_) => {}
        }
        
    }

}

fn post_message(msg_type: u32, wparam: WPARAM, millis: u64) {

    if let Some(hwnd) = CONAN_SANDBOX_HWND.lock().unwrap().as_ref() {

        unsafe {
            let _ = PostMessageW(*hwnd, msg_type, wparam, LPARAM(0));
        }
        thread::sleep(Duration::from_millis(millis));

    }

}

fn typing_loop() {

    post_message(WM_KEYDOWN, WPARAM(ESC_KEY), 250);
    post_message(WM_KEYDOWN, WPARAM(ESC_KEY), 250);
    post_message(WM_KEYDOWN, WPARAM(ENTER_KEY), 100);

    while TYPING_LOOP_ACTIVE.load(Ordering::Relaxed) {

        post_message(WM_CHAR, WPARAM(A_KEY), 500);
        post_message(WM_KEYDOWN, WPARAM(BACKSPACE_KEY), 500);

    }

}

#[tauri::command]
pub fn submit_actual_post(post: String) {

    TYPING_LOOP_ACTIVE.store(false, Ordering::Relaxed);
    TYPING_JOIN_HANDLE.lock().unwrap().take().map(|handle| handle.join());

    let post = post.replace("ChatGPT", "");
    for c in post.chars() {
        post_message(WM_CHAR, WPARAM(c as usize), 5);
    }
    post_message(WM_KEYDOWN, WPARAM(ENTER_KEY), 0);

}

#[tauri::command]
pub fn start_typing_loop() {

    #[cfg(debug_assertions)] {
        println!("Checking if typing loop is active");
    }

    if TYPING_LOOP_ACTIVE.load(Ordering::Relaxed) {
        return;
    }

    #[cfg(debug_assertions)] {
        println!("Starting typing loop");
    }

    TYPING_LOOP_ACTIVE.store(true, Ordering::Relaxed);
    TYPING_JOIN_HANDLE.lock().unwrap().replace(thread::spawn(typing_loop));

}

#[tauri::command]
pub fn force_stop_loop() {
    
    if !TYPING_LOOP_ACTIVE.load(Ordering::Relaxed) {
        return;
    }

    TYPING_LOOP_ACTIVE.store(false, Ordering::Relaxed);
    TYPING_JOIN_HANDLE.lock().unwrap().take().map(|handle| handle.join());

}

#[tauri::command]
pub fn is_hooked_in() -> bool {

    CONAN_SANDBOX_HWND.lock().unwrap().is_some()

}