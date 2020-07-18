#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#[macro_use]

//mod console is ahead of panic and sbi
mod console;
mod panic;
mod sbi;
mod interrupt;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main()  {
    interrupt::init();
    unsafe{
        llvm_asm!("ebreak"::::"volatile");
    };
}
