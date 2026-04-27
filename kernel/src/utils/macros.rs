#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use crate::drivers::vga::color::Color;
        $crate::WRITER.lock(|w| {
            w.set_color(Color::White, Color::Black);
            let _ = core::fmt::write(w, format_args!($($arg)*));
        });
    });
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! klog {
    (ok, $($arg:tt)*) => ({
        use crate::drivers::vga::color::Color;
        $crate::WRITER.lock(|w| w.set_color(Color::Green, Color::Black));
        $crate::kprint!("[OK] ");
        $crate::WRITER.lock(|w| w.set_color(Color::White, Color::Black));
        $crate::kprintln!($($arg)*);
    });
    (err, $($arg:tt)*) => ({
        use crate::drivers::vga::color::Color;
        $crate::WRITER.lock(|w| w.set_color(Color::Red, Color::Black));
        $crate::kprint!("[ERR] ");
        $crate::WRITER.lock(|w| w.set_color(Color::White, Color::Black));
        $crate::kprintln!($($arg)*);
    });
    (info, $($arg:tt)*) => ({
        use crate::drivers::vga::color::Color;
        $crate::WRITER.lock(|w| w.set_color(Color::Yellow, Color::Black));
        $crate::kprint!("[..] ");
        $crate::WRITER.lock(|w| w.set_color(Color::White, Color::Black));
        $crate::kprintln!($($arg)*);
    });
}