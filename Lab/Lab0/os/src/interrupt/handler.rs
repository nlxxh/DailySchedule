use super::context::Context;
use riscv::register::{stvec,scause::Scause};

global_asm!(include_str!("interrupt.asm"));

pub fn init(){
    unsafe{
        extern "C"{
            fn _interrupt();
        }
        stvec::write(_interrupt as usize,stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn handle_interrupt(context:&mut Context,scause:Scause,stval:usize){
    panic!("Interrupted:{:?}",scause.cause());
}
