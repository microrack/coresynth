#![cfg_attr(not(unix), no_std)]
// See https://github.com/rust-lang/rfcs/issues/2505
#![feature(core_intrinsics)]
#![feature(concat_idents)]
#![feature(raw)]

mod ctypes;
mod num;

mod os;

#[macro_use]
mod hal;

mod config;

pub mod glue;
mod peripheral;
mod store;

use core::ops::DerefMut;

mod encoder;

use cfg_if::cfg_if;

use glue::{
    TIM_CHANNEL_1, TIM_CHANNEL_2, TIM_CHANNEL_3, TIM_CHANNEL_4
};
use os::{Duration, delay, init_timers, spawn};
use crate::hal::init_statics;
// use crate::hal::gpio_pin::{_PIN};
use crate::hal::traits::{Pin, PinState};
// use crate::hal::spi::_SPI;
use crate::os::{MailSender, mail_queue, MailReceiver};
use crate::store::{GlobalEvent, GlobalState, Store, DebugInfo, DEBUG_INFO_RECEIVER, DEBUG_INFO_SENDER};

pub use store::MAIN_SENDER;

fn main_loop(main_sender: MailSender<GlobalEvent>, main_receiver: MailReceiver<GlobalEvent>) -> ! {

    let state = GlobalState::new();

    let mut store = Store::new(
        state,
        |state| {

        },
    );

    // TODO maybe fuse first call with Store::new?
    store.force_update();

    store.handle_events(main_sender, main_receiver);
}
use core::sync::atomic::{Ordering};

extern "C" fn telemetry_thread_fn() {
    let _ = MAIN_SENDER.send(GlobalEvent::Info);

    loop {
        delay(Duration::from_ms(150)).unwrap();
    }
}

extern "C" fn debug_info_thread_fn() {
    loop {
        let debug_info = DEBUG_INFO_RECEIVER.recv();
        if let Ok(debug_info) = debug_info {
            debug_info.print();
        }
    }
}

// This function should be called only from GPIO ISR
#[no_mangle]
pub extern "C" fn handle_input(enc1: u8, enc2: u8) {
    use encoder::Encoder;
    static mut ENCODER: Encoder = Encoder::new();

    use GlobalEvent::Jog;
    use store::JogDirection::*;
    let input = (enc1 == 1, enc2 == 1);
    let diff = unsafe { ENCODER.scan(input) };
    match diff {
        -1 => {
            match MAIN_SENDER.get() {
                Some(x) => {
                    let _ = x.send(Jog(Right));
                },
                None => {},
            };
        },
        1 => {
            match MAIN_SENDER.get() {
                Some(x) => {
                    let _ = x.send(Jog(Left));
                },
                None => {},
            };
        },
        _ => {},
    };
}

// This function should be called only from GPIO ISR
#[no_mangle]
pub extern "C" fn handle_button(button_state: u8) {
    /*
    use GlobalEvent::PhysicalButton;
    match MAIN_SENDER.get() {
        Some(x) => {
            let _ = x.send(PhysicalButton(button_state != 0));
        },
        None => {},
    };
    */
}

#[no_mangle]
pub extern "C" fn app() {

    init_statics()
        .expect("init_statics failed");

    init_timers();
    
    /*
    let enc1_pin = ENC1_PIN.lock()
        .expect("Lock encoder 1 pin");
    let enc2_pin = ENC2_PIN.lock()
        .expect("Lock encoder 2 pin");
    */

    // unsafe { glue::set_pwm(0.0, &mut htim2, TIM_CHANNEL_4) };

    debug_println!("\n\n === core synth ===\n");

    let (main_sender, main_receiver) = mail_queue::<GlobalEvent>(20).unwrap();
    MAIN_SENDER.init(main_sender.clone());

    let (debug_info_sender, debug_info_receiver) = mail_queue::<DebugInfo>(2).unwrap();
    DEBUG_INFO_SENDER.init(debug_info_sender);
    DEBUG_INFO_RECEIVER.init(debug_info_receiver);

    // TODO check stack size in linux
    spawn("telemetry_thread", 128, telemetry_thread_fn).unwrap();
    // spawn("uvlo_thread", 128, uvlo_thread_fn).unwrap();
    spawn("debug_info_thread", 256, debug_info_thread_fn).unwrap();

    debug_println!("Starting main loop");
    main_loop(main_sender, main_receiver);
}

cfg_if! {
    if #[cfg(all(target_arch="arm", target_os="none"))] {
        use core::panic::PanicInfo;

        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            debug_println!("panic: {:?}", info);

            unsafe {glue::system_reset()};

            loop {}
        }
    }
}

cfg_if! {
    if #[cfg(feature = "stub-hal")] {
        pub use hal::stub_hal::*;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
