use crate::hal::gpio_pin::{LED_0_PIN, LED_1_PIN, LED_2_PIN, LED_3_PIN};
use crate::hal::traits::{Pin, PinState, GpioMode};

const LED_SIZE: usize = 9;

const LED_LAYOUT: [(usize, usize); LED_SIZE] = [
    (0, 1),
    (1, 0),
    (1, 2),
    (2, 1),
    (2, 3),
    (3, 2),
    (1, 3),
    (3, 1),
    (0, 3)
];

struct CharlieLedManager<'a, TPin: Pin> {
    led_state: [bool; LED_SIZE],
    current_led: Option<core::iter::Cycle<(usize, usize)>>,
    leds: [&'a mut TPin; LED_SIZE],
}

impl<TPin: Pin> CharlieLedManager<'_, TPin> {
    pub fn set_led(&mut self, led_id: usize, state: bool) {
        self.led_state[led_id] = state;
    }

    pub fn next(&self) {
        for led in &self.leds {
            led.mode(GpioMode::GPIO_MODE_INPUT);
        }

        let layout = match self.current_led {
            Some(x) => x.next(),
            None => {
                self.current_led = Some(LED_LAYOUT.iter().cycle());
                self.current_led.next()
            }
        };

        let led_0 = self.leds[layout.0];
        let led_1 = self.leds[layout.1];

        led_0.mode(GpioMode::GPIO_MODE_OUTPUT_PP);
        led_0.write(PinState::Reset);
        led_1.mode(GpioMode::GPIO_MODE_OUTPUT_PP);
        led_1.write(PinState::Set);
    }
}
