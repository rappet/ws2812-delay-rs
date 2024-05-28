//! # Bitbang ws2812 leds with delay
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! If it's too slow (e.g.  e.g. all/some leds are white or display the wrong color)
//! you may want to try the `slow` feature.

#![no_std]

use embedded_hal as hal;

use crate::hal::digital::OutputPin;
use crate::hal::delay::DelayNs;
use smart_leds_trait::{SmartLedsWrite, RGB8};

pub struct Ws2812<DELAY, PIN> {
    delay: DELAY,
    pin: PIN,
}

impl<DELAY, PIN> Ws2812<DELAY, PIN>
where
    DELAY: DelayNs,
    PIN: OutputPin,
{
    pub fn new(delay: DELAY, mut pin: PIN) -> Ws2812<DELAY, PIN> {
        pin.set_low().ok();
        Self { delay, pin }
    }

    /// Write a single color for ws2812 devices
    #[cfg(feature = "slow")]
    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
                self.pin.set_high().ok();
                self.delay.delay_ns(600);
                self.pin.set_low().ok();
                self.delay.delay_ns(600);
            } else {
                self.pin.set_high().ok();
                self.pin.set_low().ok();
                self.delay.delay_ns(800);
            }
            data <<= 1;
        }
    }

    /// Write a single color for ws2812 devices
    #[cfg(not(feature = "slow"))]
    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            // We use timings on the shorter side here,
            // as they are a bit different between SK6812 and WS2812
            // and most microcontroller will actually be a bit slower
            if (data & 0x80) != 0 {
                self.pin.set_high().ok();
                self.delay.delay_ns(600);
                self.pin.set_low().ok();
                self.delay.delay_ns(600);
            } else {
                self.pin.set_high().ok();
                self.delay.delay_ns(300);
                self.pin.set_low().ok();
                self.delay.delay_ns(800);
            }
            data <<= 1;
        }
    }
}

impl<DELAY, PIN> SmartLedsWrite for Ws2812<DELAY, PIN>
where
    DELAY: DelayNs,
    PIN: OutputPin,
{
    type Error = ();
    type Color = RGB8;
    /// Write all the items of an iterator to a ws2812 strip
    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: IntoIterator<Item = I>,
        I: Into<Self::Color>,
    {
        for item in iterator {
            let item = item.into();
            self.write_byte(item.g);
            self.write_byte(item.r);
            self.write_byte(item.b);
        }
        self.delay.delay_us(300);
        Ok(())
    }
}
