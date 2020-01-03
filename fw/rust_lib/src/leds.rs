use crate::hal::gpio_pin::{LED_0_PIN, LED_1_PIN, LED_2_PIN, LED_3_PIN};
use crate::hal::traits::{Pin, PinState, GpioMode};

pub const LED_SIZE: usize = 9;
pub const LED_IO_SIZE: usize = 4;

const LED_LAYOUT: [(usize, usize); LED_SIZE] = [
    (0, 2),
    (2, 0),
    (0, 1),
    (1, 0),
    (2, 3),
    (3, 2),
    (2, 1),
    (1, 2),
    (0, 3)
];

use crate::os::Mutex;
use crate::peripheral::Static;

pub struct CharlieLedManager<TPin: 'static + Pin> {
    led_state: [bool; LED_SIZE],
    current_led: Option<core::iter::Cycle<core::ops::Range<usize>>>,
    leds: [&'static Static<Mutex<TPin>>; LED_IO_SIZE],
}

impl<TPin: Pin> CharlieLedManager<TPin> {
    pub fn new(leds: [&'static Static<Mutex<TPin>>; LED_IO_SIZE]) -> Self {
        CharlieLedManager {
            led_state: [false; LED_SIZE],
            current_led: None,
            leds
        }
    }

    pub fn set_led(&mut self, led_id: usize, state: bool) {
        self.led_state[led_id] = state;
    }

    pub fn set_leds(&mut self, led_state: [bool; LED_SIZE]) {
        self.led_state = led_state;
    }

    pub fn next(&mut self) {
        for led in &self.leds {
            led.lock().expect("Lock pin").mode(GpioMode::GPIO_MODE_INPUT);
        }

        match &self.current_led {
            Some(x) => {},
            None => self.current_led = Some((0..LED_SIZE).cycle())
        };

        // TODO === begin syntax bullshit ===

        let led_id = match &mut self.current_led {
            Some(x) => x,
            None => panic!("Unexpected missing current_led iterator")
        };

        let id_iter = led_id.next();

        let led_id = match &id_iter {
            Some(x) => x,
            None => panic!("Unexpected end of current_led iterator")
        };

        // TODO === end syntax bullshit ===

        let layout = LED_LAYOUT[*led_id];

        // debug_println!("id {}: {:?}", led_id, layout);

        if self.led_state[*led_id] {
            {
                let led_0 = &mut self.leds[layout.0];
                led_0.lock().expect("Lock pin").mode(GpioMode::GPIO_MODE_OUTPUT_PP);
                led_0.lock().expect("Lock pin").write(PinState::Reset);
            }

            {
                let led_1 = &mut self.leds[layout.1];
                led_1.lock().expect("Lock pin").mode(GpioMode::GPIO_MODE_OUTPUT_PP);
                led_1.lock().expect("Lck pin").write(PinState::Set);
            }
        }
    }
}
