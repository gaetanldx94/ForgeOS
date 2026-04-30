use alloc::vec::Vec;
use super::context::Context;

pub type Pid = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Dead,
}

pub const STACK_SIZE: usize = 64 * 1024;

pub struct Process {
    pub pid:     Pid,
    pub state:   ProcessState,
    pub context: Context,
    pub stack:   Vec<u8>,
    pub name:    &'static str,
}

impl Process {
    pub fn new(pid: Pid, name: &'static str, entry: u64) -> Self {
        let mut stack = Vec::with_capacity(STACK_SIZE);
        stack.resize(STACK_SIZE, 0u8);

        let stack_top = (stack.as_ptr() as u64 + STACK_SIZE as u64) & !0xF;

        let mut context = Context::new();
        context.set_entry(entry, stack_top);

        Process { pid, state: ProcessState::Ready, context, stack, name }
    }
}