#![allow(dead_code)]
use stm32f0x::{GPIOA, RCC};

/*
PowerLed: A7;
GameLed: A15;
ConnectionLed: PA8;
StatusLed: PA6;
*/

macro_rules! LED {
    ($STRUCTNAME:ident: ($moderX:ident, $odrX:ident)) => {
        pub struct $STRUCTNAME;
        impl $STRUCTNAME {
            pub fn init(&self, gpioa: &GPIOA, rcc: &RCC) {
                rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
                gpioa.moder.modify(|_, w| unsafe { w.$moderX().bits(1) });
            }

            pub fn on(&self, gpioa: &GPIOA) { gpioa.odr.modify(|_, w| w.$odrX().set_bit()); }

            pub fn off(&self, gpioa: &GPIOA) { gpioa.odr.modify(|_, w| w.$odrX().clear_bit()); }
        }
    }
}
LED! { PowerLed: (moder7, odr7) }
LED! { StatusLed: (moder6, odr6) }
LED! { ConnectionLed: (moder8, odr8) }
LED! { GameLed: (moder15, odr15) }