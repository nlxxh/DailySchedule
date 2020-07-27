use super::context::Context;
use riscv::register::{stvec,scause::{Scause,Trap,Exception,Interrupt}};
use super::timer;
use crate::process::PROCESSOR;

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
pub fn handle_interrupt(context:&mut Context,scause:Scause,stval:usize)-> *mut Context{
    {
        let mut processor = PROCESSOR.lock();
        let current_thread = processor.current_thread();
        if current_thread.as_ref().inner().dead {
            println!("thread {} exit", current_thread.id);
            processor.kill_current_thread();
            return processor.prepare_next_thread();
        }
    }
    match scause.cause(){
        Trap::Exception(Exception::Breakpoint)=>breakpoint(context),
        Trap::Interrupt(Interrupt::SupervisorTimer)=>supervisor_timer(context),
        _=>fault(context,scause,stval),
    }
}

fn breakpoint(context:&mut Context) -> *mut Context{
    println!("Breakpoint at 0x{:x}",context.sepc);
    context.sepc+=2;
    context
}
fn supervisor_timer(context:&mut Context)-> *mut Context{
    timer::tick();
    PROCESSOR.lock().park_current_thread(context);
    PROCESSOR.lock().prepare_next_thread()
}
fn fault (context:&mut Context,scause:Scause,stval:usize)-> *mut Context{
    panic!("Unresolved interrupt:{:?}\n{:x}\nstval:{:x}",scause.cause(),context.sepc,stval);

}
