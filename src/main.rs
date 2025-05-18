#![no_std] // We don't want to link the standard library    
#![no_main] // We don't want to link the C runtime

// This function is called on panic
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // We want to tell the compiler call _start instead of main
pub extern "C" fn _start() -> ! {
    loop {
        // This is where the main code will go
}
}