#![no_std]
#![no_main]

use drv0 as _;
use drv1 as _;

use drv_common::CallEntry;
// use std::mem;

extern "C" {
    fn initcalls_start() -> u64;
    fn initcalls_end() -> u64;
}

// 主要被卡住的地方就是链接期间找不到 __stack_chk_guard 和 __stack_chk_fail 这两个符号
// 刚开始把重心都放到了initcalls.c中， 一直都以为c程序出错了(也许c程序也是错的)

// 下面手动添加这些符号
#[no_mangle]
pub extern "C" fn __stack_chk_fail() {}

#[no_mangle]
pub static mut __stack_chk_guard: u64 = 0;


fn get_initcalls_start() -> u64 {
    let start: u64 = unsafe { initcalls_start() };
    start
}

fn get_initcalls_end() -> u64 {
    let end: u64 = unsafe { initcalls_end() };
    end
}

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: B0\n");
    verify();
}

/* Todo: Implement it */
fn traverse_drivers() {
    // libos::println!("\n!!! Fix it !!!\n");
    // Parse range of init_calls by calling C function.

    let range_start: u64 = get_initcalls_start();
    let range_end: u64 = get_initcalls_end();

    display_initcalls_range(range_start as usize, range_end as usize);

    // For each driver, display name & compatible
    let mut offset = 0;
    while range_start + offset < range_end {
        let call_entry = (range_start + offset) as *const CallEntry;
        let drv;
        unsafe {
            drv = ((*call_entry).init_fn)();
        }
        display_drv_info(drv.name, drv.compatible);
        offset += 8;  // size of call entry
    }
}

fn display_initcalls_range(start: usize, end: usize) {
    libos::println!("init calls range: 0x{:X} ~ 0x{:X}\n", start, end);
}

fn display_drv_info(name: &str, compatible: &str) {
    libos::println!("Found driver '{}': compatible '{}'", name, compatible);
}

fn verify() {
    traverse_drivers();

    libos::println!("\nResult: Okay!");
}
