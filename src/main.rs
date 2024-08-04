#![no_std]
#![no_main]

mod uefi;

#[no_mangle]
pub fn efi_main(
    _image_handle: uefi::EfiHandle,
    system_table: &uefi::EfiSystemTable,
) -> uefi::EfiStatus {
    system_table.con_out().reset(false);
    system_table.con_out().output_string(
        [
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0,
        ]
        .as_ptr(),
    );

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
