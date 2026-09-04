#![no_std]
#![feature(panic_info_message)]

///////////////////////////////////////
///         RUST MACROS             ///
///////////////////////////////////////
#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {{}};
}

#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });

    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });

    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

///////////////////////////////////////////////
///     LANGUAGE STRUCTURES / FUNCTIONS     ///
//////////////////////////////////////////////
#[unsafe(no_mangle)]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line,
            p.file,
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    abort();
}

#[unsafe(no_mangle)]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn kmain() {
    // main should initialize all sub-systems and get
    // ready to start scheduling. The last thing this
    // should do is start the timer
}
