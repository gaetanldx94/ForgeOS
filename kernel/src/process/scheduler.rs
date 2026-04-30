use alloc::vec::Vec;
use super::context::{Context, switch_context};
use super::process::{Pid, Process, ProcessState};

pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current:   usize,
    next_pid:      Pid,
    kernel_context: Context,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            processes:      Vec::new(),
            current:        0,
            next_pid:       1,
            kernel_context: Context::new(),
        }
    }

    pub fn spawn(&mut self, name: &'static str, entry: u64) -> Pid {
        let pid = self.next_pid;
        self.next_pid += 1;
        self.processes.push(Process::new(pid, name, entry));
        pid
    }

    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    pub unsafe fn schedule(&mut self) {
        if self.processes.is_empty() { return; }

        let count = self.processes.len();
        let old   = self.current;

        let mut next = (old + 1) % count;
        let mut found = false;
        for _ in 0..count {
            if self.processes[next].state != ProcessState::Dead {
                found = true;
                break;
            }
            next = (next + 1) % count;
        }
        if !found { return; }
        if old == next { return; }

        self.processes[old].state = ProcessState::Ready;
        self.processes[next].state = ProcessState::Running;
        self.current = next;

        let old_ctx = &mut self.processes[old].context as *mut Context;
        let new_ctx = &self.processes[next].context as *const Context;

        switch_context(old_ctx, new_ctx);
    }
}