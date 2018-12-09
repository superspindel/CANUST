use stm32f0x::{GPIOA, RCC, SYSCFG_COMP, EXTI};

pub struct BUTTON;

impl BUTTON {
    pub fn init(&self, gpioa: &GPIOA, rcc: &RCC, syscfg: &SYSCFG_COMP, exti: &EXTI) {
        gpioa.moder.modify(|_, w| unsafe { w.moder2().bits(0) });
        gpioa.pupdr.modify(|_, w| unsafe { w.pupdr2().bits(0b10) });
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        syscfg.syscfg_exticr1.modify(|_, w| unsafe { w.exti2().bits(0b000) });
        exti.imr.modify(|_, w| w.mr2().set_bit());
        exti.ftsr.modify(|_, w| w.tr2().set_bit());
    }

    pub fn reset(&self, exti: &EXTI) { exti.pr.modify(|_, w| w.pif2().set_bit()); }
}
