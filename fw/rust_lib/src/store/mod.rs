mod action;
mod generic;

pub use self::action::{GlobalEvent, JogDirection};
pub use self::generic::Store;
use self::generic::StoreState;

use crate::os::{Duration, TimerId, timeout, MailSender, cancel_timeout, Instant, MailReceiver};
use crate::peripheral::Static;

#[derive(Debug)]
pub struct ButtonState {
    actual_state: bool,
    logic_state: bool,
    debounce_timer_id: Option<TimerId>,
    long_press_timer_id: Option<TimerId>,
    long_press_occurred: bool,
}

use crate::config::{
    DEBOUNCE_TIME, LONG_PRESS_TIME, INFO_INTERVAL
};

const SEQUENCER_JOG_DIVIDER: i32 = 3;

#[derive(Debug)]
pub struct GlobalState {
    raw_encoder: i32,

    pub s_led: [bool; 8],
    pub run_led: bool,

    sequencer_encoder_counter: i32,
    sequencer_counter: i8,

    info_timer_id: Option<TimerId>,
}

const SEND_RETRY: u8 = 10;

use crate::os;

fn safe_send(sender: &MailSender<GlobalEvent>, event: GlobalEvent) {
    let mut ret_err: Option<os::Error> = None;

    for _ in 0..SEND_RETRY {
        match sender.send(event) {
            Ok(_) => {
                ret_err = None;
                break;
            },
            Err(err) => {ret_err = Some(err)},
        };

        os::delay(Duration::from_ms(1)).unwrap();
    };

    match ret_err {
        Some(error) => panic!("send error: {:?}", error),
        None => {},
    };
}

pub struct DebugInfo {
    jog_absolute: i32,
    sequencer_counter: i8,
}

impl DebugInfo {
    pub fn print(&self) {
        // It needs to be separate debug_println to unlock UART mutex often to allow other threads to make progress
        // On baud rate 115200 71ms is required to transmit 1 kB, it's too much for main thread
        debug_println!("\n# DEBUG START");

        debug_println!("# JOG_ABS:{}, JOG_SEQ:{}",
            self.jog_absolute,
            self.sequencer_counter,
        );

        debug_println!("# DEBUG END\n");
    }
}

impl GlobalState {
    pub fn new() -> GlobalState {

        let mut res = GlobalState {
            raw_encoder: 0,

            s_led: [false; 8],
            run_led: false,

            sequencer_encoder_counter: 0,
            sequencer_counter: 0,

            info_timer_id: None,
        };

        res
    }

    fn update_button(&mut self, _sender: &MailSender<GlobalEvent>) {
        /*
        if self.button_state.actual_state != self.button_state.logic_state {
            self.button_state.logic_state = self.button_state.actual_state;

            debug_println!("# BTN_PRESSED:{}", self.button_state.logic_state);

            if self.button_state.logic_state {
                self.on_button_down(sender);
            } else {
                self.on_button_up(sender);
            }
        }
        */
    }

    fn on_physical_button(&mut self, state:bool, _sender: &MailSender<GlobalEvent>) {
        /*
        self.button_state.actual_state = state;
        match self.button_state.debounce_timer_id {
            None => {
                self.update_button(sender);

                let sender = sender.clone();
                self.button_state.debounce_timer_id = Some(timeout(
                    DEBOUNCE_TIME,
                    move || { safe_send(&sender, GlobalEvent::ButtonDebounce); }
                ));
            },
            Some(_) => {
                // do nothing
            }
        }
        */
    }

    fn on_button_down(&mut self, _sender: &MailSender<GlobalEvent>) {
        /*
        self.button_state.long_press_timer_id = Some(timeout(
            LONG_PRESS_TIME,
            move || { safe_send(&sender, GlobalEvent::LongPress); }
        ));
        */
    }

    fn on_button_up(&mut self, _sender: &MailSender<GlobalEvent>) {
        /*
        if let Some(id) = self.button_state.long_press_timer_id {
            cancel_timeout(id);
        }
        self.button_state.long_press_timer_id = None;

        if !self.button_state.long_press_occurred {
            self.on_click(sender);
        }
        self.button_state.long_press_occurred = false;
        */
    }

    fn on_long_press(&mut self, _sender: &MailSender<GlobalEvent>) {
        /*
        self.button_state.long_press_timer_id = None;
        self.button_state.long_press_occurred = true;
        */
    }

    fn on_button_debounce(&mut self, sender: &MailSender<GlobalEvent>) {
        /*
        self.button_state.debounce_timer_id = None;
        self.update_button(sender);
        */
    }

    fn on_click(&mut self, _sender: &MailSender<GlobalEvent>) {

    }

    fn turn_on(&mut self, _sender: &MailSender<GlobalEvent>) {

    }



    fn on_jog(&mut self, dir: JogDirection, _sender: &MailSender<GlobalEvent>) {
        use JogDirection::*;

        match dir {
            Left => self.raw_encoder -= 1,
            Right => self.raw_encoder += 1,
        }

        match dir {
            Left => self.sequencer_encoder_counter -= 1,
            Right => self.sequencer_encoder_counter += 1,
        }

        if self.sequencer_encoder_counter == SEQUENCER_JOG_DIVIDER || self.sequencer_encoder_counter == -SEQUENCER_JOG_DIVIDER {
            if self.sequencer_encoder_counter == -SEQUENCER_JOG_DIVIDER && true {
                if self.sequencer_counter > 0 {
                    // debug_println!("prev");
                    self.sequencer_counter -= 1;
                }
            }

            if self.sequencer_encoder_counter == SEQUENCER_JOG_DIVIDER && true {
                if self.sequencer_counter + 1 < 9 {
                    // debug_println!("next");
                    self.sequencer_counter += 1;
                }
            }

            for led in &mut self.s_led {
                *led = false;
            }

            if self.sequencer_counter >= 0 && self.sequencer_counter < 8 {
                self.s_led[self.sequencer_counter as usize] = true;
            }

            self.run_led = self.sequencer_counter == 8;

            self.sequencer_encoder_counter = 0;
        }
    }

    fn make_debug_info(&mut self) -> DebugInfo {
        use core::sync::atomic::{Ordering};

        let result = DebugInfo {
            jog_absolute: self.raw_encoder,
            sequencer_counter: self.sequencer_counter,
        };

        result
    }

    fn show_info(&mut self, sender: &MailSender<GlobalEvent>) {
        if let Err(err) = DEBUG_INFO_SENDER.send(self.make_debug_info()) {
            debug_println!("Debug info sending error");
        }

        if let Some(id) = self.info_timer_id {
            crate::os::cancel_timeout(id);
        }
        let sender = sender.clone();
        self.info_timer_id = Some(timeout(
            INFO_INTERVAL,
            move || { safe_send(&sender, GlobalEvent::Info); }
        ));
    }
}

impl StoreState<GlobalEvent> for GlobalState {
    fn handle_event(&mut self, event: GlobalEvent, sender: &MailSender<GlobalEvent>) {
        use self::GlobalEvent::*;

        match event {
            Jog(_) => {},
            _ => debug_println!("event: {:?}", event),
        }

        match event {
            Jog(dir) => self.on_jog(dir, sender),

            // PhysicalButton(state) => self.on_physical_button(state, sender),
            // ButtonDebounce => self.on_button_debounce(sender),
            // LongPress => self.on_long_press(sender),

            Wakeup => self.turn_on(sender),
            Info => self.show_info(sender),
        }
    }
}

pub static MAIN_SENDER: Static<MailSender<GlobalEvent>> = Static::new();
pub static DEBUG_INFO_SENDER: Static<MailSender<DebugInfo>> = Static::new();
pub static DEBUG_INFO_RECEIVER: Static<MailReceiver<DebugInfo>> = Static::new();
