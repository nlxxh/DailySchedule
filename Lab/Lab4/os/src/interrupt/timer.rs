use crate::sbi::set_timer;
use riscv::register::{time,sie,sstatus};
static INTERVAL:usize=100000;
pub static mut TICKS:usize=0;

pub fn init(){
    unsafe{
        sie::set_stimer();
        //sstatus::set_sie();  
    }
    set_next_timeout();
}
fn set_next_timeout(){
    set_timer(time::read()+INTERVAL);
}
pub fn tick(){
    set_next_timeout();
    unsafe{
        TICKS+=1;
    }
}
