#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptFrame {
    pub rip:    u64,
    pub cs:     u64,
    pub rflags: u64,
    pub rsp:    u64,
    pub ss:     u64,
}

macro_rules! handler {
    ($name:ident, $body:expr) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            core::arch::naked_asm!(
                "push rbp",
                "mov rbp, rsp",
                "push rax",
                "push rbx",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push r8",
                "push r9",
                "push r10",
                "push r11",
                "push r12",
                "push r13",
                "push r14",
                "push r15",
                "mov rdi, rbp",
                "add rdi, 8",
                concat!("call ", stringify!($body)),
                "pop r15",
                "pop r14",
                "pop r13",
                "pop r12",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rdi",
                "pop rsi",
                "pop rdx",
                "pop rcx",
                "pop rbx",
                "pop rax",
                "pop rbp",
                "iretq",
            );
        }
    };
}

macro_rules! handler_error {
    ($name:ident, $body:expr) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            core::arch::naked_asm!(
                "push rbp",
                "mov rbp, rsp",
                "push rax",
                "push rbx",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push r8",
                "push r9",
                "push r10",
                "push r11",
                "push r12",
                "push r13",
                "push r14",
                "push r15",
                "mov rdi, rbp",
                "add rdi, 8",
                concat!("call ", stringify!($body)),
                "pop r15",
                "pop r14",
                "pop r13",
                "pop r12",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rdi",
                "pop rsi",
                "pop rdx",
                "pop rcx",
                "pop rbx",
                "pop rax",
                "pop rbp",
                "add rsp, 8",
                "iretq",
            );
        }
    };
}

handler!(divide_error_handler,          divide_error_inner);
handler!(debug_handler,                 debug_inner);
handler!(nmi_handler,                   nmi_inner);
handler!(breakpoint_handler,            breakpoint_inner);
handler!(overflow_handler,              overflow_inner);
handler!(bound_range_handler,           bound_range_inner);
handler!(invalid_opcode_handler,        invalid_opcode_inner);
handler!(device_not_available_handler,  device_not_available_inner);
handler_error!(double_fault_handler,    double_fault_inner);
handler_error!(invalid_tss_handler,     invalid_tss_inner);
handler_error!(segment_not_present_handler, segment_not_present_inner);
handler_error!(stack_fault_handler,     stack_fault_inner);
handler_error!(general_protection_handler, general_protection_inner);
handler_error!(page_fault_handler,      page_fault_inner);

#[no_mangle]
extern "C" fn divide_error_inner(frame: &InterruptFrame) {
    panic_exception("Division by zero", frame, None);
}

#[no_mangle]
extern "C" fn debug_inner(frame: &InterruptFrame) {
    panic_exception("Debug", frame, None);
}

#[no_mangle]
extern "C" fn nmi_inner(frame: &InterruptFrame) {
    panic_exception("NMI", frame, None);
}

#[no_mangle]
extern "C" fn breakpoint_inner(frame: &InterruptFrame) {
    panic_exception("Breakpoint", frame, None);
}

#[no_mangle]
extern "C" fn overflow_inner(frame: &InterruptFrame) {
    panic_exception("Overflow", frame, None);
}

#[no_mangle]
extern "C" fn bound_range_inner(frame: &InterruptFrame) {
    panic_exception("Bound range exceeded", frame, None);
}

#[no_mangle]
extern "C" fn invalid_opcode_inner(frame: &InterruptFrame) {
    panic_exception("Invalid opcode", frame, None);
}

#[no_mangle]
extern "C" fn device_not_available_inner(frame: &InterruptFrame) {
    panic_exception("Device not available", frame, None);
}

#[no_mangle]
extern "C" fn double_fault_inner(frame: &InterruptFrame) {
    panic_exception("Double fault", frame, None);
}

#[no_mangle]
extern "C" fn invalid_tss_inner(frame: &InterruptFrame) {
    panic_exception("Invalid TSS", frame, None);
}

#[no_mangle]
extern "C" fn segment_not_present_inner(frame: &InterruptFrame) {
    panic_exception("Segment not present", frame, None);
}

#[no_mangle]
extern "C" fn stack_fault_inner(frame: &InterruptFrame) {
    panic_exception("Stack fault", frame, None);
}

#[no_mangle]
extern "C" fn general_protection_inner(frame: &InterruptFrame) {
    panic_exception("General protection fault", frame, None);
}

#[no_mangle]
extern "C" fn page_fault_inner(frame: &InterruptFrame) {
    panic_exception("Page fault", frame, None);
}

fn panic_exception(name: &str, frame: &InterruptFrame, _error: Option<u64>) {
    use crate::drivers::vga::{Color, VgaWriter};
    let mut w = VgaWriter::new();
    w.set_color(Color::Red, Color::Black);
    w.print("\n[EXCEPTION] ");
    w.print(name);
    w.print("\n  RIP : 0x");
    w.print_hex(frame.rip);
    w.print("\n  RSP : 0x");
    w.print_hex(frame.rsp);
    w.print("\n  CS  : 0x");
    w.print_hex(frame.cs);
    w.print("\n");
    unsafe {
        core::arch::asm!("2: jmp 2b", options(nostack, nomem, noreturn));
    }
}