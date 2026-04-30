pub mod context;
pub mod process;
pub mod scheduler;

use scheduler::Scheduler;

pub static mut SCHEDULER: Option<Scheduler> = None;

pub fn init() {
    unsafe {
        SCHEDULER = Some(Scheduler::new());
    }
}

pub fn spawn(name: &'static str, entry: u64) -> process::Pid {
    unsafe {
        SCHEDULER.as_mut().unwrap().spawn(name, entry)
    }
}

pub unsafe fn schedule() {
    if let Some(s) = SCHEDULER.as_mut() {
        s.schedule();
    }
}