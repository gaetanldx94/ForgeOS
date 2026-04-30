pub mod handlers;
pub mod idt;
pub mod pic;
pub mod paging;

use idt::Idt;
use handlers::*;

static mut IDT: Idt = Idt::new();

pub unsafe fn init_idt() {
    IDT.set_handler(0,  divide_error_handler           as u64);
    IDT.set_handler(1,  debug_handler                  as u64);
    IDT.set_handler(2,  nmi_handler                    as u64);
    IDT.set_trap    (3,  breakpoint_handler             as u64);
    IDT.set_handler(4,  overflow_handler               as u64);
    IDT.set_handler(5,  bound_range_handler            as u64);
    IDT.set_handler(6,  invalid_opcode_handler         as u64);
    IDT.set_handler(7,  device_not_available_handler   as u64);
    IDT.set_handler(8,  double_fault_handler           as u64);
    IDT.set_handler(10, invalid_tss_handler            as u64);
    IDT.set_handler(11, segment_not_present_handler    as u64);
    IDT.set_handler(12, stack_fault_handler            as u64);
    IDT.set_handler(13, general_protection_handler     as u64);
    IDT.set_handler(14, page_fault_handler             as u64);
    IDT.set_handler(32, timer_handler                  as u64);
    IDT.load();
}

pub unsafe fn init() {
    pic::remap();
    pic::disable();
    init_idt();
    pic::init_pit(11932);   // ~100 Hz
    pic::enable_timer();
}