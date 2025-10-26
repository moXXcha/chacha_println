#![no_std]
#![no_main]

use uefi::prelude::*;
use chacha_println::libs::infra::output::UEFI;
use uefi::proto::console::text::Output;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    // Initialize STDOUT with SystemTable
    unsafe {
        let stdout_ptr = system_table.stdout() as *const _ as *mut Output;
        UEFI::init_stdout(stdout_ptr);
    }

    // Use the chacha_println macro
    chacha_println::chacha_println!("hello {}{}", "chacha", "println!");

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
