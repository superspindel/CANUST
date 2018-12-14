
use stm32f0x::{RCC};

pub struct ExternalClock;

impl ExternalClock {
    pub fn init(&self, rcc: &RCC) {
        rcc.cr.write(|w| w.hsion().set_bit());
        rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(0) });

        while rcc.cr.read().hsirdy().bit_is_clear() { }
        rcc.cr.modify(|_, w| w.pllon().clear_bit());
        while rcc.cr.read().pllrdy().bit_is_set() { }
        rcc.cfgr.modify(|_, w| unsafe { w.pllmul().bits(4) });
        rcc.cfgr2.modify(|_, w| unsafe { w.prediv().bits(0) });
        rcc.cfgr.modify(|_, w| unsafe { w.pllsrc().bits(1) });
        rcc.cfgr.modify(|_, w| unsafe { w.ppre().bits(4) });
        while rcc.cr.read().hsirdy().bit_is_clear() { }
        rcc.cr.modify(|_, w| w.pllon().clear_bit());
        while rcc.cr.read().pllrdy().bit_is_set() { }
        rcc.cfgr.modify(|_, w| unsafe { w.pllmul().bits(4) });
        rcc.cfgr2.modify(|_, w| unsafe { w.prediv().bits(0) });

        rcc.cfgr.modify(|_, w| unsafe { w.pllsrc().bits(1) });
        rcc.cfgr.modify(|_, w| unsafe { w.ppre().bits(4) });
        rcc.cr.modify(|_, w| w.pllon().set_bit());

        while rcc.cr.read().pllrdy().bit_is_clear() { }
        rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(2) });
        while rcc.cfgr.read().sws().bits() != 2 { }
    }
}


