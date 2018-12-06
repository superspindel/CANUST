use stm32f0x::{GPIOA, RCC, CAN, gpioa, can};
use core::ops::Deref;
use core::any::{Any, TypeId};
/*
    Enumerator filter
    28 filters can be setup for the STM32F0X processor series. These are individually defined using their filter number.
*/
pub enum filter {   
    _0, _1, _2, _3, _4,
    _5, _6, _7, _8, _9,
    _10, _11, _12, _13, _14,
    _15, _16, _17, _18, _19,
    _20, _21, _22, _23, _24,
    _25, _26, _27,
}
/*
    Implementation for the Canust API
*/

/*
        _transmit
            "PRIVATE"
        inputs:
        @ number                            -> The u8 number of the available output mailbox
        @ d0, d1, d2, d3, d4, d5, d6, d7    -> 8xu8 values that represent the data to be sent over the canbus
        @ rtr                               -> Retransmit request, set this bit if the message is a request for data, d0 is needed, d1-d7 is optional
        @ stid                              -> The standard ID of the message
        @ exid                              -> The extended ID of the message, is Optional
        @ time                              -> Optional time value when the message was constructed
        @ dlc                               -> The length of the message, 1-8 to decide how many of d0-d7 is sent

    fn _transmit(   &self, number: u8, d0: u8, d1: Option<u8>, d2: Option<u8>, d3: Option<u8>, d4: Option<u8>, d5: Option<u8>, d6: Option<u8>, d7: Option<u8>, 
                    rtr: Option<bool>, stid: u16,  exid: Option<u32>, time: Option<u16>, dlc: Option<u8>) {
        match number {
            0 => {
                let can_reg = self.0;
                can_reg.can_tdt0r.modify(|_, w| unsafe { w.dlc().bits(dlc.unwrap()) });
                can_reg.can_ti0r.write(|w| unsafe { w.stid().bits(stid) });
                if rtr.is_some() { can_reg.can_ti0r.modify(|_, w| w.rtr().bit(rtr.unwrap())); }
                if exid.is_some() { can_reg.can_ti0r.modify(|_, w| unsafe { w.exid().bits(exid.unwrap()) }); }
                can_reg.can_tdl0r.modify(|_, w| unsafe { w.data0().bits(d0) });
                if d1.is_some() { can_reg.can_tdl0r.modify(|_, w| unsafe { w.data1().bits(d1.unwrap()) }); }
                if d2.is_some() { can_reg.can_tdl0r.modify(|_, w| unsafe { w.data2().bits(d2.unwrap()) }); }
                if d3.is_some() { can_reg.can_tdl0r.modify(|_, w| unsafe { w.data3().bits(d3.unwrap()) }); }
                if d4.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data4().bits(d4.unwrap()) }); }
                if d5.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data5().bits(d5.unwrap()) }); }
                if d6.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data6().bits(d6.unwrap()) }); }
                if d7.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data7().bits(d7.unwrap()) }); }
                can_reg.can_ti0r.modify(|_, w| w.txrq().set_bit());
                while
                (can_reg.can_ti0r.read().txrq().bit_is_set()) { }
            },
            1 => {
                let can_reg = self.0;
                can_reg.can_tdt1r.modify(|_, w| unsafe { w.dlc().bits(dlc.unwrap()) });
                can_reg.can_ti1r.write(|w| unsafe { w.stid().bits(stid) });
                if rtr.is_some() { can_reg.can_ti1r.modify(|_, w| w.rtr().bit(rtr.unwrap())); }
                if exid.is_some() { can_reg.can_ti1r.modify(|_, w| unsafe { w.exid().bits(exid.unwrap()) }); }
                can_reg.can_tdl1r.modify(|_, w| unsafe { w.data0().bits(d0) });
                if d1.is_some() { can_reg.can_tdl1r.modify(|_, w| unsafe { w.data1().bits(d1.unwrap()) }); }
                if d2.is_some() { can_reg.can_tdl1r.modify(|_, w| unsafe { w.data2().bits(d2.unwrap()) }); }
                if d3.is_some() { can_reg.can_tdl1r.modify(|_, w| unsafe { w.data3().bits(d3.unwrap()) }); }
                if d4.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data4().bits(d4.unwrap()) }); }
                if d5.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data5().bits(d5.unwrap()) }); }
                if d6.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data6().bits(d6.unwrap()) }); }
                if d7.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data7().bits(d7.unwrap()) }); }
                can_reg.can_ti1r.modify(|_, w| w.txrq().set_bit());
                while(can_reg.can_ti1r.read().txrq().bit_is_set()) { }
            },
            2 => {
                let can_reg = self.0;
                can_reg.can_tdt2r.modify(|_, w| unsafe { w.dlc().bits(dlc.unwrap()) });
                can_reg.can_ti2r.write(|w| unsafe { w.stid().bits(stid) });
                if rtr.is_some() { can_reg.can_ti2r.modify(|_, w| w.rtr().bit(rtr.unwrap())); }
                if exid.is_some() { can_reg.can_ti2r.modify(|_, w| unsafe { w.exid().bits(exid.unwrap()) }); }
                can_reg.can_tdl2r.modify(|_, w| unsafe { w.data0().bits(d0) });
                if d1.is_some() { can_reg.can_tdl2r.modify(|_, w| unsafe { w.data1().bits(d1.unwrap()) }); }
                if d2.is_some() { can_reg.can_tdl2r.modify(|_, w| unsafe { w.data2().bits(d2.unwrap()) }); }
                if d3.is_some() { can_reg.can_tdl2r.modify(|_, w| unsafe { w.data3().bits(d3.unwrap()) }); }
                if d4.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data4().bits(d4.unwrap()) }); }
                if d5.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data5().bits(d5.unwrap()) }); }
                if d6.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data6().bits(d6.unwrap()) }); }
                if d7.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data7().bits(d7.unwrap()) }); }
                can_reg.can_ti2r.modify(|_, w| w.txrq().set_bit());
                while(can_reg.can_ti2r.read().txrq().bit_is_set()) { }
            },
            _ => unreachable!(),
        }
    }
}
        transmit
        "PUBLIC"
        Sends a message on the bus if transmit mailbox is available, otherwhise returns a Err Result with a error message, if the message was sent
        returns a OK(x) where x is the number of the mailbox used.
        @message    -> The CAN_MESSAGE struct that contains the data to be transmitted.
    

    pub fn transmit(&self, message: CAN_MESSAGE) -> Result<u8, &str> {
        let can_reg = self.0;
        let mut free_mailbox: Option<u8> = None;
        if (can_reg.can_tsr.read().tme0().bit_is_set()) { free_mailbox = Some(0); }         // Check if mailbox 0 is empty, if not check 1 and then 2.
        else if (can_reg.can_tsr.read().tme1().bit_is_set()) { free_mailbox = Some(1); }
        else if (can_reg.can_tsr.read().tme2().bit_is_set()) { free_mailbox = Some(2); }
        if free_mailbox.is_some() {
            match free_mailbox.unwrap() {
                0 => {
                    self._transmit(0, message.data0, message.data1, message.data2, message.data3, message.data4, message.data5, message.data6, message.data7, message.rtr, message.stid, message.exid, message.time, message.dlc);
                    Ok(0)
                },
                1 => {
                    self._transmit(1, message.data0, message.data1, message.data2, message.data3, message.data4, message.data5, message.data6, message.data7, message.rtr, message.stid, message.exid, message.time, message.dlc);
                    Ok(1)
                },
                2 => {
                    self._transmit(2, message.data0, message.data1, message.data2, message.data3, message.data4, message.data5, message.data6, message.data7, message.rtr, message.stid, message.exid, message.time, message.dlc);
                    Ok(2)
                },
                _ => unreachable!(),
            }
        } else {
            Err("No mailbox empty")
        }
    }
*/

macro_rules! transmit {
    ($FUNCNAME:ident: ($mbx_trans_0:ident, $mbx_trans_1:ident, $mbx_trans_2:ident )) => {
        impl<'a, U>Canust<'a, U>
        where
            U: Any + cantrait
        {
            fn $FUNCNAME(&self, message: CAN_MESSAGE) -> Result<u8, &str> {
                let can_reg = self.0;
                let mut free_mailbox: Option<u8> = None;
                if (can_reg.can_tsr.read().tme0().bit_is_set()) { self.$mbx_trans_0(); Ok(0) }         // Check if mailbox 0 is empty, if not check 1 and then 2.
                else if (can_reg.can_tsr.read().tme1().bit_is_set()) { self.$mbx_trans_1(); Ok(1) }
                else if (can_reg.can_tsr.read().tme2().bit_is_set()) { self.$mbx_trans_2(); Ok(2) }
            }
        }
    }
}



     /*
        _add_filter function
            "PRIVATE"
        inputs:
        @ filter    -> The filter enumerator that defines the identifier of the filter.
        @ active    -> Optional bool that sets if the filter should be active or not active.
        @ mode      -> Optional bool that sets if the filter should be in mask or list mode.
        @ scale     -> Optional bool that sets if the filter should be in u32 or u16 mode.
        @ fifo      -> Bool value, decides which mailbox fifo the filter should filter its messages to.
        @ reg1      -> The value of filter register 1
        @ reg2      -> The value of filter register 2
    */
macro_rules! filter {
    ($FUNCNAME:ident: ($fbmX:ident, $fscX:ident, $ffaX:ident, $fXr1:ident, $fXr2:ident, $factX:ident)) => {
        impl<'a, U> Canust<'a, U>
        where
            U: Any + cantrait
        {
            fn $FUNCNAME(&self, number: filter, active: Option<bool>, mode: Option<bool>, scale: Option<bool>, fifo: bool, reg1: Option<u32>, reg2: Option<u32>) {
            let can_reg = self.0;
            if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.$fbmX().bit(mode.unwrap())) };
            if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.$fscX().bit(scale.unwrap())) };
            can_reg.can_ffa1r.modify(|_, w| w.$ffaX().bit(fifo));
            if reg1.is_some() {   can_reg.$fXr1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
            if reg2.is_some() {   can_reg.$fXr2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.$factX().bit(active.unwrap())) };
            }
        }
    }
}


filter! { add_filter_0: (fbm0, fsc0, ffa0, f0r1, f0r2, fact0) }
filter! { add_filter_1: (fbm1, fsc1, ffa1, f1r1, f1r2, fact1) }
filter! { add_filter_2: (fbm2, fsc2, ffa2, f2r1, f2r2, fact2) }

impl<'a, U> Canust<'a, U>
where
    U: Any + cantrait,
{  

    /*
    fn _add_filter(&self, number: filter, active: Option<bool>, mode: Option<bool>, scale: Option<bool>, fifo: bool, reg1: Option<u32>, reg2: Option<u32>) {
        let can_reg = self.0;
        match number {
            filter::_0 => { 
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm0().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc0().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa0().bit(fifo));
                if reg1.is_some() {   can_reg.f0r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f0r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact0().bit(active.unwrap())) };
                
            },
            filter::_1 => {
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm1().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc1().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa1().bit(fifo));
                if reg1.is_some() {   can_reg.f1r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f1r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact1().bit(active.unwrap())) };
                
            },
            filter::_2 => {
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm2().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc2().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa2().bit(fifo));
                if reg1.is_some() {   can_reg.f2r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f2r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact2().bit(active.unwrap())) };
                
            },
            filter::_3 => {
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm3().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc3().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa3().bit(fifo));
                if reg1.is_some() {   can_reg.f3r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f3r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact3().bit(active.unwrap())) };
            },
            filter::_4 => {
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm4().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc4().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa4().bit(fifo));
                if reg1.is_some() {   can_reg.f4r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f4r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact4().bit(active.unwrap())) };
            },
            filter::_5 => {
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm5().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc5().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa5().bit(fifo));
                if reg1.is_some() {   can_reg.f5r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f5r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact5().bit(active.unwrap())) };
            },
            filter::_6 => {
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm6().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc6().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa6().bit(fifo));
                if reg1.is_some() {   can_reg.f6r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f6r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact6().bit(active.unwrap())) };
            },
            filter::_7 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact7().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm7().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc7().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa7().bit(fifo));
                if reg1.is_some() {   can_reg.f7r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f7r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_8 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact8().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm8().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc8().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa8().bit(fifo));
                if reg1.is_some() {   can_reg.f8r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f8r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_9 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact9().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm9().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc9().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa9().bit(fifo));
                if reg1.is_some() {   can_reg.f9r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f9r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_10 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact10().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm10().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc10().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa10().bit(fifo));
                if reg1.is_some() {   can_reg.f10r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f10r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_11 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact11().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm11().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc11().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa11().bit(fifo));
                if reg1.is_some() {   can_reg.f11r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f11r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_12 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact12().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm12().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc12().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa12().bit(fifo));
                if reg1.is_some() {   can_reg.f12r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f12r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_13 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact13().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm13().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc13().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa13().bit(fifo));
                if reg1.is_some() {   can_reg.f13r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f13r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_14 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact14().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm14().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc14().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa14().bit(fifo));
                if reg1.is_some() {   can_reg.f14r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f14r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_15 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact15().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm15().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc15().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa15().bit(fifo));
                if reg1.is_some() {   can_reg.f15r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f15r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_16 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact16().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm16().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc16().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa16().bit(fifo));
                if reg1.is_some() {   can_reg.f16r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f16r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_17 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact17().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm17().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc17().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa17().bit(fifo));
                if reg1.is_some() {   can_reg.f17r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f17r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_18 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact18().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm18().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc18().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa18().bit(fifo));
                if reg1.is_some() {   can_reg.f18r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f18r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_19 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact19().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm19().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc19().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa19().bit(fifo));
                if reg1.is_some() {   can_reg.f19r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f19r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_20 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact20().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm20().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc20().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa20().bit(fifo));
                if reg1.is_some() {   can_reg.f20r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f20r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_21 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact21().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm21().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc21().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa21().bit(fifo));
                if reg1.is_some() {   can_reg.f21r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f21r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_22 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact22().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm22().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc22().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa22().bit(fifo));
                if reg1.is_some() {   can_reg.f22r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f22r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_23 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact23().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm23().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc23().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa23().bit(fifo));
                if reg1.is_some() {   can_reg.f23r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f23r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_24 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact24().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm24().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc24().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa24().bit(fifo));
                if reg1.is_some() {   can_reg.f24r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f24r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_25 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact25().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm25().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc25().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa25().bit(fifo));
                if reg1.is_some() {   can_reg.f25r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f25r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_26 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact26().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm26().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc26().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa26().bit(fifo));
                if reg1.is_some() {   can_reg.f26r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f26r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            },
            filter::_27 => {
                if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.fact27().bit(active.unwrap())) };
                if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.fbm27().bit(mode.unwrap())) };
                if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.fsc27().bit(scale.unwrap())) };
                can_reg.can_ffa1r.modify(|_, w| w.ffa27().bit(fifo));
                if reg1.is_some() {   can_reg.f27r1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
                if reg2.is_some() {   can_reg.f27r2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            }
        }
    }
    */
    /*
        _transmit
            "PRIVATE"
        inputs:
        @ number                            -> The u8 number of the available output mailbox
        @ d0, d1, d2, d3, d4, d5, d6, d7    -> 8xu8 values that represent the data to be sent over the canbus
        @ rtr                               -> Retransmit request, set this bit if the message is a request for data, d0 is needed, d1-d7 is optional
        @ stid                              -> The standard ID of the message
        @ exid                              -> The extended ID of the message, is Optional
        @ time                              -> Optional time value when the message was constructed
        @ dlc                               -> The length of the message, 1-8 to decide how many of d0-d7 is sent
    */
    fn _transmit(   &self, number: u8, d0: u8, d1: Option<u8>, d2: Option<u8>, d3: Option<u8>, d4: Option<u8>, d5: Option<u8>, d6: Option<u8>, d7: Option<u8>, 
                    rtr: Option<bool>, stid: u16,  exid: Option<u32>, time: Option<u16>, dlc: Option<u8>) {
        match number {
            0 => {
                let can_reg = self.0;
                can_reg.can_tdt0r.modify(|_, w| unsafe { w.dlc().bits(dlc.unwrap()) });
                can_reg.can_ti0r.write(|w| unsafe { w.stid().bits(stid) });
                if rtr.is_some() { can_reg.can_ti0r.modify(|_, w| w.rtr().bit(rtr.unwrap())); }
                if exid.is_some() { can_reg.can_ti0r.modify(|_, w| unsafe { w.exid().bits(exid.unwrap()) }); }
                can_reg.can_tdl0r.modify(|_, w| unsafe { w.data0().bits(d0) });
                if d1.is_some() { can_reg.can_tdl0r.modify(|_, w| unsafe { w.data1().bits(d1.unwrap()) }); }
                if d2.is_some() { can_reg.can_tdl0r.modify(|_, w| unsafe { w.data2().bits(d2.unwrap()) }); }
                if d3.is_some() { can_reg.can_tdl0r.modify(|_, w| unsafe { w.data3().bits(d3.unwrap()) }); }
                if d4.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data4().bits(d4.unwrap()) }); }
                if d5.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data5().bits(d5.unwrap()) }); }
                if d6.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data6().bits(d6.unwrap()) }); }
                if d7.is_some() { can_reg.can_tdh0r.modify(|_, w| unsafe { w.data7().bits(d7.unwrap()) }); }
                can_reg.can_ti0r.modify(|_, w| w.txrq().set_bit());
                while
                (can_reg.can_ti0r.read().txrq().bit_is_set()) { }
            },
            1 => {
                let can_reg = self.0;
                can_reg.can_tdt1r.modify(|_, w| unsafe { w.dlc().bits(dlc.unwrap()) });
                can_reg.can_ti1r.write(|w| unsafe { w.stid().bits(stid) });
                if rtr.is_some() { can_reg.can_ti1r.modify(|_, w| w.rtr().bit(rtr.unwrap())); }
                if exid.is_some() { can_reg.can_ti1r.modify(|_, w| unsafe { w.exid().bits(exid.unwrap()) }); }
                can_reg.can_tdl1r.modify(|_, w| unsafe { w.data0().bits(d0) });
                if d1.is_some() { can_reg.can_tdl1r.modify(|_, w| unsafe { w.data1().bits(d1.unwrap()) }); }
                if d2.is_some() { can_reg.can_tdl1r.modify(|_, w| unsafe { w.data2().bits(d2.unwrap()) }); }
                if d3.is_some() { can_reg.can_tdl1r.modify(|_, w| unsafe { w.data3().bits(d3.unwrap()) }); }
                if d4.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data4().bits(d4.unwrap()) }); }
                if d5.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data5().bits(d5.unwrap()) }); }
                if d6.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data6().bits(d6.unwrap()) }); }
                if d7.is_some() { can_reg.can_tdh1r.modify(|_, w| unsafe { w.data7().bits(d7.unwrap()) }); }
                can_reg.can_ti1r.modify(|_, w| w.txrq().set_bit());
                while(can_reg.can_ti1r.read().txrq().bit_is_set()) { }
            },
            2 => {
                let can_reg = self.0;
                can_reg.can_tdt2r.modify(|_, w| unsafe { w.dlc().bits(dlc.unwrap()) });
                can_reg.can_ti2r.write(|w| unsafe { w.stid().bits(stid) });
                if rtr.is_some() { can_reg.can_ti2r.modify(|_, w| w.rtr().bit(rtr.unwrap())); }
                if exid.is_some() { can_reg.can_ti2r.modify(|_, w| unsafe { w.exid().bits(exid.unwrap()) }); }
                can_reg.can_tdl2r.modify(|_, w| unsafe { w.data0().bits(d0) });
                if d1.is_some() { can_reg.can_tdl2r.modify(|_, w| unsafe { w.data1().bits(d1.unwrap()) }); }
                if d2.is_some() { can_reg.can_tdl2r.modify(|_, w| unsafe { w.data2().bits(d2.unwrap()) }); }
                if d3.is_some() { can_reg.can_tdl2r.modify(|_, w| unsafe { w.data3().bits(d3.unwrap()) }); }
                if d4.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data4().bits(d4.unwrap()) }); }
                if d5.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data5().bits(d5.unwrap()) }); }
                if d6.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data6().bits(d6.unwrap()) }); }
                if d7.is_some() { can_reg.can_tdh2r.modify(|_, w| unsafe { w.data7().bits(d7.unwrap()) }); }
                can_reg.can_ti2r.modify(|_, w| w.txrq().set_bit());
                while(can_reg.can_ti2r.read().txrq().bit_is_set()) { }
            },
            _ => unreachable!(),
        }
    }
}
    /*
        struct: CAN_MESSAGE
            "PUBLIC"
        Consists of data fields, retransmit bit, standard id, extended id bits, fmi which is the filter match index that the passed through when receiving
        time bits and dlc bits. This is used for both receiving and transmitting messages. To transmit fill in fields:
        data0, stid and dlc at least. Option to fill in data 1 - data 7, rtr, time and exid. dlc should match the number of data bytes to send. fmi is only for receiving messages.
    */
pub struct CAN_MESSAGE {
    pub data0: u8,
    pub data1: Option<u8>,
    pub data2: Option<u8>,
    pub data3: Option<u8>,
    pub data4: Option<u8>,
    pub data5: Option<u8>,
    pub data6: Option<u8>,
    pub data7: Option<u8>,
    pub rtr: Option<bool>,
    pub stid: u16,
    pub exid: Option<u32>,
    pub fmi: Option<u8>,
    pub time: Option<u16>,
    pub dlc: Option<u8>
}
/*
    trait: cantrait
    Implemented on the CAN registerblock of the stm32f0x set of processors. Also specifies the GPIO ports to be used for the CAN.
*/
pub unsafe trait cantrait: Deref<Target = can::RegisterBlock> {
    type GPIO: Deref<Target = gpioa::RegisterBlock>;
}
/*
    Implementation of cantrait on the CAN registerblock, also specifies the GPIO ports to be used.
*/
unsafe impl cantrait for CAN {
    type GPIO = GPIOA;
}
/*
    struct: Canust
    Consists of a object that implements the cantrait trait.
*/
pub struct Canust<'a, U>(pub &'a U)
where
    U: Any + cantrait;

/*
    Clone for Canust
    Implementation of clone for the Canust struct
*/
impl<'a, U> Clone for Canust<'a, U>
where
    U: Any + cantrait,
{
    fn clone(&self) -> Self {
        *self
    }
}


impl<'a, U> Copy for Canust<'a, U>
where
    U: Any + cantrait,
{
}

impl<'a, U> Canust<'a, U>
where
    U: Any + cantrait,
{
    /*
        Init
        "PUBLIC"
        Initializes the CAN on the bus, sets bit timing bits and enters normal state. Needs to read 11 consecutive recessive bits on the bus to enter Normal state.
        @gpio           -> GPIO bort to be used for the CAN messages, specified as GPIOA in the cantrait implementation.
        @rcc            -> Reset and clock control register, for enabling clocks on GPIOA and CAN.
        @parameters     -> Struct of can parameters to be utilized during initialization, specifies bit timing and settings for CAN transmission.
    */    
    pub fn init(&self, gpio: &U::GPIO, rcc: &RCC, parameters: CAN_INIT_PARAMETERS) {
        let can_reg = self.0;
        rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
        rcc.apb1enr.modify(|_, w| w.canen().set_bit());
        gpio.moder.modify(|_, w| unsafe { w.moder11().bits(2).moder12().bits(2) });
        gpio.otyper.modify(|_, w| w.ot11().clear_bit().ot12().clear_bit());
        gpio.pupdr.modify(|_, w| unsafe { w.pupdr11().bits(1).pupdr12().bits(1) });
        gpio.ospeedr.modify(|_, w| unsafe { w.ospeedr11().bits(0).ospeedr12().bits(0) });
        gpio.afrh.modify(|_, w| unsafe { w.afrh11().bits(4).afrh12().bits(4) });
        can_reg.can_mcr.modify(|_, w| w.inrq().set_bit());
        while (can_reg.can_msr.read().inak().bit_is_clear()) { /* wait for bxCAN to enter intialization state */ }
        can_reg.can_mcr.modify(|_, w| w.sleep().clear_bit());
        while can_reg.can_msr.read().slaki().bit_is_set() { /* wait for bxCAN to exit sleep state */ }
        can_reg.can_mcr.modify(|_, w| unsafe { w.dbf().bit(parameters.dbf).ttcm().bit(parameters.ttcm).abom().bit(parameters.abom).awum().bit(parameters.awum).nart().bit(parameters.nart).rflm().bit(parameters.rflm).txfp().bit(parameters.txfp) });
        can_reg.can_btr.modify(|_, w| unsafe { w.brp().bits(parameters.brp).ts1().bits(parameters.tseg1).ts2().bits(parameters.tseg2).lbkm().bit(parameters.lbkm).sjw().bits(parameters.sjw) });
        can_reg.can_mcr.modify(|_, w| w.inrq().clear_bit());
        while (can_reg.can_msr.read().inak().bit_is_set()) { /* wait for bxCAN to exit intialization state and enter normal state */ }
    }
    /*
        canit
        "PUBLIC"
        Sets up the interrupts that the CAN should handle, further described in the CAN_INTERRUPTS struct.
        @interrupts     -> The struct containing which interrupts should be active and not active.
    */
    pub fn canit(&self, interrupts: CAN_INTERRUPTS) {
        let can_reg = self.0;
        if (interrupts.transmit_mailbox_empty.is_some()) { can_reg.can_ier.modify(|_, w| w.tmeie().bit(interrupts.transmit_mailbox_empty.unwrap())) }
        if (interrupts.fifo0_message_pending.is_some()) { can_reg.can_ier.modify(|_, w| w.fmpie0().bit(interrupts.fifo0_message_pending.unwrap())) }
        if (interrupts.fifo0_full.is_some()) { can_reg.can_ier.modify(|_, w| w.ffie0().bit(interrupts.fifo0_full.unwrap())) }
        if (interrupts.fifo0_overrun.is_some()) { can_reg.can_ier.modify(|_, w| w.fovie0().bit(interrupts.fifo0_overrun.unwrap())) }
        if (interrupts.fifo1_message_pending.is_some()) { can_reg.can_ier.modify(|_, w| w.fmpie1().bit(interrupts.fifo1_message_pending.unwrap())) }
        if (interrupts.fifo1_full.is_some()) { can_reg.can_ier.modify(|_, w| w.ffie1().bit(interrupts.fifo1_full.unwrap())) }
        if (interrupts.fifo1_overrun.is_some()) { can_reg.can_ier.modify(|_, w| w.fovie1().bit(interrupts.fifo1_overrun.unwrap())) }
        if (interrupts.error_warning.is_some()) { can_reg.can_ier.modify(|_, w| w.ewgie().bit(interrupts.error_warning.unwrap())) }
        if (interrupts.error_passive.is_some()) { can_reg.can_ier.modify(|_, w| w.epvie().bit(interrupts.error_passive.unwrap())) }
        if (interrupts.bus_off.is_some()) { can_reg.can_ier.modify(|_, w| w.bofie().bit(interrupts.bus_off.unwrap())) }
        if (interrupts.last_error_code.is_some()) { can_reg.can_ier.modify(|_, w| w.lecie().bit(interrupts.last_error_code.unwrap())) }
        if (interrupts.error.is_some()) { can_reg.can_ier.modify(|_, w| w.errie().bit(interrupts.error.unwrap())) }
        if (interrupts.wakeup.is_some()) { can_reg.can_ier.modify(|_, w| w.wkuie().bit(interrupts.wakeup.unwrap())) }
        if (interrupts.sleep.is_some()) { can_reg.can_ier.modify(|_, w| w.slkie().bit(interrupts.sleep.unwrap())) }
    }
    /*
        transmit
        "PUBLIC"
        Sends a message on the bus if transmit mailbox is available, otherwhise returns a Err Result with a error message, if the message was sent
        returns a OK(x) where x is the number of the mailbox used.
        @message    -> The CAN_MESSAGE struct that contains the data to be transmitted.
    */
    pub fn transmit(&self, message: CAN_MESSAGE) -> Result<u8, &str> {
        let can_reg = self.0;
        let mut free_mailbox: Option<u8> = None;
        if (can_reg.can_tsr.read().tme0().bit_is_set()) { free_mailbox = Some(0); }         // Check if mailbox 0 is empty, if not check 1 and then 2.
        else if (can_reg.can_tsr.read().tme1().bit_is_set()) { free_mailbox = Some(1); }
        else if (can_reg.can_tsr.read().tme2().bit_is_set()) { free_mailbox = Some(2); }
        if free_mailbox.is_some() {
            match free_mailbox.unwrap() {
                0 => {
                    self._transmit(0, message.data0, message.data1, message.data2, message.data3, message.data4, message.data5, message.data6, message.data7, message.rtr, message.stid, message.exid, message.time, message.dlc);
                    Ok(0)
                },
                1 => {
                    self._transmit(1, message.data0, message.data1, message.data2, message.data3, message.data4, message.data5, message.data6, message.data7, message.rtr, message.stid, message.exid, message.time, message.dlc);
                    Ok(1)
                },
                2 => {
                    self._transmit(2, message.data0, message.data1, message.data2, message.data3, message.data4, message.data5, message.data6, message.data7, message.rtr, message.stid, message.exid, message.time, message.dlc);
                    Ok(2)
                },
                _ => unreachable!(),
            }
        } else {
            Err("No mailbox empty")
        }
    }
    /*
        receive
        "PUBLIC"
        Receives a message from the input mailbox. If no message available, returns a Err result with a error message inside.
        @fifo       -> The FILTER_FIFO that the message should be stored in.
    */
    pub fn receive(&self, fifo: FILTER_FIFO) -> Result<CAN_MESSAGE, &str> {
        match fifo {
            FILTER_FIFO::_0 => self._receive_fifo0(),
            FILTER_FIFO::_1 => self._receive_fifo1(),
        }
    }
    /*
        _receive_fifo0
        "PRIVATE"
        Returns the message if available from the input mailbox corresponding to FIFO0 otherwhise a Err result with a suitable error message.
    */
    fn _receive_fifo0(&self) -> Result<CAN_MESSAGE, &str> {
        let can_reg = self.0;
        let mut exid = None;
        let mut data1 = None;
        let mut data2 = None;
        let mut data3 = None;
        let mut data4 = None;
        let mut data5 = None;
        let mut data6 = None;
        let mut data7 = None;
        if(can_reg.can_rf0r.read().fmp0().bits() != 0) {
            let stid = can_reg.can_ri0r.read().stid().bits();
            if can_reg.can_ri0r.read().ide().bit_is_set() { let exid = Some(can_reg.can_ri0r.read().exid().bits()); }
            let rtr = Some(can_reg.can_ri0r.read().rtr().bit_is_set());
            let time = Some(can_reg.can_rdt0r.read().time().bits());
            let fmi = Some(can_reg.can_rdt0r.read().fmi().bits());
            let dlc = can_reg.can_rdt0r.read().dlc().bits();
            let data0 = can_reg.can_rdl0r.read().data0().bits();
            if (dlc > 1) { data1 = Some(can_reg.can_rdl0r.read().data1().bits()); }
            if (dlc > 2) { data2 = Some(can_reg.can_rdl0r.read().data2().bits()); }
            if (dlc > 3) { data3 = Some(can_reg.can_rdl0r.read().data3().bits()); }
            if (dlc > 4) { data4 = Some(can_reg.can_rdh0r.read().data4().bits()); }
            if (dlc > 5) { data5 = Some(can_reg.can_rdh0r.read().data5().bits()); }
            if (dlc > 6) { data6 = Some(can_reg.can_rdh0r.read().data6().bits()); }
            if (dlc > 7) { data7 = Some(can_reg.can_rdh0r.read().data7().bits()); }

            let message = CAN_MESSAGE {
                data0: data0,
                data1: data1,
                data2: data2,
                data3: data3,
                data4: data4,
                data5: data5,
                data6: data6,
                data7: data7,
                rtr: rtr,
                stid: stid,
                exid: exid,
                fmi: fmi,
                time: time,
                dlc: Some(dlc),
            };
            can_reg.can_rf0r.modify(|_, w| w.rfom0().set_bit());
            Ok(message)
        } else {
            Err("No message available")
        }
    }
    /*
        _receive_fifo1
        "PRIVATE"
        Returns the message if available from the input mailbox corresponding to FIFO1 otherwhise a Err result with a suitable error message.
    */
    fn _receive_fifo1(&self) -> Result<CAN_MESSAGE, &str> {
        let can_reg = self.0;
        let mut exid = None;
        let mut data1 = None;
        let mut data2 = None;
        let mut data3 = None;
        let mut data4 = None;
        let mut data5 = None;
        let mut data6 = None;
        let mut data7 = None;
        if(can_reg.can_rf1r.read().fmp1().bits() != 0) {
            let stid = can_reg.can_ri1r.read().stid().bits();
            if can_reg.can_ri1r.read().ide().bit_is_set() { exid = Some(can_reg.can_ri1r.read().exid().bits()); }
            let rtr = Some(can_reg.can_ri1r.read().rtr().bit_is_set());
            let time = Some(can_reg.can_rdt1r.read().time().bits());
            let fmi = Some(can_reg.can_rdt1r.read().fmi().bits());
            let dlc = can_reg.can_rdt1r.read().dlc().bits();
            let data0 = can_reg.can_rdl1r.read().data0().bits();
            if (dlc > 1) { data1 = Some(can_reg.can_rdl1r.read().data1().bits()); }
            if (dlc > 2) { data2 = Some(can_reg.can_rdl1r.read().data2().bits()); }
            if (dlc > 3) { data3 = Some(can_reg.can_rdl1r.read().data3().bits()); }
            if (dlc > 4) { data4 = Some(can_reg.can_rdh1r.read().data4().bits()); }
            if (dlc > 5) { data5 = Some(can_reg.can_rdh1r.read().data5().bits()); }
            if (dlc > 6) { data6 = Some(can_reg.can_rdh1r.read().data6().bits()); }
            if (dlc > 7) { data7 = Some(can_reg.can_rdh1r.read().data7().bits()); }

            let message = CAN_MESSAGE {
                data0: data0,
                data1: data1,
                data2: data2,
                data3: data3,
                data4: data4,
                data5: data5,
                data6: data6,
                data7: data7,
                rtr: rtr,
                stid: stid,
                exid: exid,
                fmi: fmi,
                time: time,
                dlc: Some(dlc),
            };
            can_reg.can_rf1r.modify(|_, w| w.rfom1().set_bit());
            Ok(message)
        } else {
            Err("No message available")
        }
    }
    /*
    TODO: Add to filter macro
        add_filter
        "PUBLIC"
        Adds a filter to the CAN so as to let messages through that passes the filter into the mailbox
        @filter_settings    -> The struct containing the filter settings to be used
        @filter_number      -> Which filter number out of the 28 available to be used
    
    pub fn add_filter<T>(&self, filter_settings: T, filter_number: filter)
    where
        T: can_filter_trait 
    {
        let can_reg = self.0;
        can_reg.can_fmr.modify(|_, w| w.finit().set_bit());
        self._add_filter(   filter_number, filter_settings.get_active(), filter_settings.get_mode(), filter_settings.get_scale(),
                            filter_settings.get_fifo(), filter_settings.get_reg1(), filter_settings.get_reg2());
        can_reg.can_fmr.modify(|_, w| w.finit().clear_bit());
    }
    */
}
/*
    struct: CAN_INIT_PARAMETERS
    Used to setup the CAN, containts bittiming settings such as tseg1, tseg2, sjw and brp.
    Also lbkm and silent for loopback mode and silent mode for debugging. Further specification can be found in the
    reference manual as per the usage and definition of the can settings.
*/
pub struct CAN_INIT_PARAMETERS {
    pub tseg1: u8,
    pub tseg2: u8,
    pub sjw: u8,
    pub lbkm: bool,
    pub silent: bool,
    pub brp: u16,
    pub dbf: bool,
    pub ttcm: bool,
    pub abom: bool,
    pub awum: bool,
    pub nart: bool,
    pub rflm: bool,
    pub txfp: bool,
}
/*
    struct: CAN_INTERRUPTS
    This is used to define which interrupts should be active on the CAN. They are described in full in the reference manual
    but important ones are, fifo0_message_pending and fifo1_message_pending that alerts the interrupt handler when a new message is available,
    fifo1_full and fifo0_full which alerts is either of the FIFOs for the incomming messages are full.
*/
pub struct CAN_INTERRUPTS {
    pub sleep: Option<bool>,
    pub wakeup: Option<bool>,
    pub error: Option<bool>,
    pub last_error_code: Option<bool>,
    pub bus_off: Option<bool>,
    pub error_passive: Option<bool>,
    pub error_warning: Option<bool>,
    pub fifo1_overrun: Option<bool>,
    pub fifo1_full: Option<bool>,
    pub fifo1_message_pending: Option<bool>,
    pub fifo0_overrun: Option<bool>,
    pub fifo0_full: Option<bool>,
    pub fifo0_message_pending: Option<bool>,
    pub transmit_mailbox_empty: Option<bool>,
}
/*
    struct: filter_u16_list
    Used to setup a filter in listmode with 4x16 bit list filters.
*/
pub struct filter_u16_list {
    pub fifo: FILTER_FIFO,
    pub active: Option<bool>,
    pub id1: Option<u16>,
    pub id2: Option<u16>,
    pub id3: Option<u16>,
    pub id4: Option<u16>,
}

/*
    Implementation of can_filter_trait for filter_u16_list. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl can_filter_trait for filter_u16_list {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FILTER_FIFO::_0 => false,
            FILTER_FIFO::_1 => true,
        }
    }
    fn get_reg1(&self) -> Option<u32> { 
        if(self.id1.is_some()) {
            if (self.id2.is_some()) {
                Some((self.id2.unwrap() as u32) << 16 | self.id1.unwrap() as u32)
            } else {
                Some(self.id1.unwrap() as u32)
            }
        }  else if(self.id2.is_some()) {
            Some((self.id2.unwrap() as u32) << 16)
        } else {
            None
        }
    }
    fn get_reg2(&self) -> Option<u32> {
        if(self.id3.is_some()) {
            if (self.id4.is_some()) {
                Some((self.id4.unwrap() as u32) << 16 | self.id3.unwrap() as u32)
            } else {
                Some(self.id3.unwrap() as u32)
            }
        } else if(self.id4.is_some()) {
            Some((self.id4.unwrap() as u32) << 16)
        } else {
            None
        }
    }
    fn get_scale(&self) -> Option<bool> {
        Some(false)
    }
    fn get_mode(&self) -> Option<bool> {
        Some(true)
    }
}
/*
    struct: filter_u16_mask
    Used to setup a filter in maskmode with 2x16 bit filters with 2x16 bit masks.
*/
pub struct filter_u16_mask {
    pub fifo: FILTER_FIFO,
    pub active: Option<bool>,
    pub id1: Option<u16>,
    pub mask1: Option<u16>,
    pub id2: Option<u16>,
    pub mask2: Option<u16>,
}
/*
    Implementation of can_filter_trait for filter_u16_mask. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl can_filter_trait for filter_u16_mask {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FILTER_FIFO::_0 => false,
            FILTER_FIFO::_1 => true,
        }
    }
    fn get_reg1(&self) -> Option<u32> {
        if(self.id1.is_some()) {
            if (self.mask1.is_some()) {
                Some((self.mask1.unwrap() as u32) << 16 | self.id1.unwrap() as u32)
            } else {
                Some(self.id1.unwrap() as u32)
            }
        }  else if(self.mask1.is_some()) {
            Some((self.mask1.unwrap() as u32) << 16)
        } else {
            None
        }
    }
    fn get_reg2(&self) -> Option<u32> {
        if(self.id2.is_some()) {
            if (self.mask2.is_some()) {
                Some((self.mask2.unwrap() as u32) << 16 | self.id2.unwrap() as u32)
            } else {
                Some(self.id2.unwrap() as u32)
            }
        } else if(self.mask2.is_some()) {
            Some((self.mask2.unwrap() as u32) << 16)
        } else {
            None
        }
    }
    fn get_scale(&self) -> Option<bool> {
        Some(false)
    }
    fn get_mode(&self) -> Option<bool> {
        Some(false)
    }
}
/*
    struct: filter_u32_list
    Used to setup a filter in listmode with 2x32 bit list filters.
*/
pub struct filter_u32_list {
    pub fifo: FILTER_FIFO,
    pub active: Option<bool>,
    pub id1: Option<u32>,
    pub id2: Option<u32>,
}
/*
    Implementation of can_filter_trait for filter_u32_list. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl can_filter_trait for filter_u32_list {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FILTER_FIFO::_0 => false,
            FILTER_FIFO::_1 => true,
        }
    }
    fn get_reg1(&self) -> Option<u32> {
        self.id1
    }
    fn get_reg2(&self) -> Option<u32> {
        self.id2
    }
    fn get_scale(&self) -> Option<bool> {
        Some(true)
    }
    fn get_mode(&self) -> Option<bool> {
        Some(true)
    }
}
/*
    struct: filter_u32_list
    Used to setup a filter in listmode with 1x32 bit filter with 1x32 mask.
*/
pub struct filter_u32_mask {
    pub fifo: FILTER_FIFO,
    pub active: Option<bool>,
    pub id: Option<u32>,
    pub mask: Option<u32>,
}
/*
    Implementation of can_filter_trait for filter_u32_mask. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl can_filter_trait for filter_u32_mask {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FILTER_FIFO::_0 => false,
            FILTER_FIFO::_1 => true,
        }
    }
    fn get_reg1(&self) -> Option<u32> {
        self.id
    }
    fn get_reg2(&self) -> Option<u32> {
        self.mask
    }
    fn get_scale(&self) -> Option<bool> {
        Some(true)
    }
    fn get_mode(&self) -> Option<bool> {
        Some(false)
    }
}
/*
    trait: can_filter_trait
    Used to generalize the filter structs for implementation in the Canust struct.
*/
pub trait can_filter_trait {
    fn get_active(&self) -> Option<bool>;   // Returns if the struct should be active
    fn get_fifo(&self) -> bool;             // Returns the decided FIFO
    fn get_reg1(&self) -> Option<u32>;      // Returns values to set in register 1
    fn get_reg2(&self) -> Option<u32>;      // Returns values to set in register 2
    fn get_scale(&self) -> Option<bool>;    // Returns value to set in scale
    fn get_mode(&self) -> Option<bool>;     // Returns value to set in mode
}
/*
    enum: FILTER_FIFO
    Simple enum for the two mailbox FIFOs.
*/
pub enum FILTER_FIFO {
    _0,
    _1,
}