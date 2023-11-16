import time
import win32con
import win32gui
import win32api

def find_window(title):
    try:
        return win32gui.FindWindow(None, title)
    except win32gui.error:
        return None

def post_key(hwnd, vk_code):
    win32api.PostMessage(hwnd, win32con.WM_KEYDOWN, vk_code, 0)
    #win32api.PostMessage(hwnd, win32con.WM_KEYUP, vk_code, 0)

def main():
    hwnd = find_window("ConanSandbox ")
    if hwnd:
        print(f"Window found: {hwnd}")
        # Sending two characters, e.g., 'AB'
        post_key(hwnd, ord('A'))
        post_key(hwnd, ord('B'))
        time.sleep(0.5)  # Half-second pause
        # Sending two backspaces
        post_key(hwnd, win32con.VK_BACK)
        post_key(hwnd, win32con.VK_BACK)
    else:
        print("Window not found.")

if __name__ == "__main__":
    main()
