mod handler;
mod context;
mod timer;

pub use context::Context;
pub fn init(){
    handler::init();
    timer::init();
    println!("mod interrupt initialized");

}
