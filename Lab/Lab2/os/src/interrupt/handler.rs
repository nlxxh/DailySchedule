use super::context::Context;
use riscv::register::{stvec,scause::{Scause,Trap,Exception,Interrupt}};
use super::timer;

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
    match scause.cause(){
        Trap::Exception(Exception::Breakpoint)=>breakpoint(context),
        Trap::Interrupt(Interrupt::SupervisorTimer)=>supervisor_timer(context),
        _=>fault(context,scause,stval),
    }
}

fn breakpoint(context:&mut Context){
    println!("Breakpoint at 0x{:x}",context.sepc);
    context.sepc+=2;
}
fn supervisor_timer(context:&mut Context){
    timer::tick();
}
fn fault (context:&mut Context,scause:Scause,stval:usize){
    panic!("Unresolved interrupt:{:?}\n{:x}\nstval:{:x}",scause.cause(),context.sepc,stval);

}
