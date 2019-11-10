#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;

use uefi::prelude::*;

#[no_mangle]
pub extern "win64" fn uefi_start(_image_handle: uefi::Handle, system_table: &'static SystemTable<Boot>) -> ! {
    loop {}
}

