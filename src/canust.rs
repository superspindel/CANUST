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

macro_rules! receive_fifo {
    ($FUNCNAME:ident: ($can_rfXr:ident, $fmpX:ident, $can_riXr:ident, $can_rdtXr:ident, $can_rdlXr:ident, $can_rdhXr:ident, $rfomX:ident)) => {
        impl<'a, U>Canust<'a, U>
        where
            U: Any + cantrait
        {
            pub fn $FUNCNAME (&self) -> Result<CAN_MESSAGE, &str> {
                let mut message = CAN_MESSAGE::new();
                let can_reg = self.0;
                if(can_reg.$can_rfXr.read().$fmpX().bits() != 0) {
                    message.stid = can_reg.$can_riXr.read().stid().bits();
                    if can_reg.$can_riXr.read().ide().bit_is_set() { message.exid = Some(can_reg.$can_riXr.read().exid().bits()); }
                    message.rtr = Some(can_reg.$can_riXr.read().rtr().bit_is_set());
                    message.time = Some(can_reg.$can_rdtXr.read().time().bits());
                    message.fmi = Some(can_reg.$can_rdtXr.read().fmi().bits());
                    let dlc = can_reg.$can_rdtXr.read().dlc().bits();
                    message.dlc = Some(dlc);
                    message.data0 = can_reg.$can_rdlXr.read().data0().bits();
                    if (dlc > 1) { message.data1 = Some(can_reg.$can_rdlXr.read().data1().bits()); }
                    if (dlc > 2) { message.data2 = Some(can_reg.$can_rdlXr.read().data2().bits()); }
                    if (dlc > 3) { message.data3 = Some(can_reg.$can_rdlXr.read().data3().bits()); }
                    if (dlc > 4) { message.data4 = Some(can_reg.$can_rdhXr.read().data4().bits()); }
                    if (dlc > 5) { message.data5 = Some(can_reg.$can_rdhXr.read().data5().bits()); }
                    if (dlc > 6) { message.data6 = Some(can_reg.$can_rdhXr.read().data6().bits()); }
                    if (dlc > 7) { message.data7 = Some(can_reg.$can_rdhXr.read().data7().bits()); }
                    can_reg.$can_rfXr.modify(|_, w| w.$rfomX().set_bit());
                    Ok(message)
                } else {
                    Err("No message available")
                }
            }
        }
    }
}

receive_fifo! { receive_fifo0: (can_rf0r, fmp0, can_ri0r, can_rdt0r, can_rdl0r, can_rdh0r, rfom0) }
receive_fifo! { receive_fifo1: (can_rf1r, fmp1, can_ri1r, can_rdt1r, can_rdl1r, can_rdh1r, rfom1) }

impl<'a, U>Canust<'a, U>
where
    U: Any + cantrait
{
    pub fn receive(&self) -> Result<CAN_MESSAGE, &str> {
        let can_reg = self.0;
        
    }
}



/*
        receive
        "PUBLIC"
        Receives a message from the input mailbox. If no message available, returns a Err result with a error message inside.
        @fifo       -> The FILTER_FIFO that the message should be stored in.
    
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

}
*/


macro_rules! transmit_mailbox {
    ($FUNCNAME:ident: ($tdtXr:ident, $tiXr:ident, $tdlXr:ident, $tdhXr:ident)) => {
        impl<'a, U>Canust<'a, U>
        where
            U: Any + cantrait
        {
            fn $FUNCNAME(&self, message: CAN_MESSAGE) {
                let can_reg = self.0;
                can_reg.$tdtXr.modify(|_, w| unsafe { w.dlc().bits(message.dlc.unwrap())});
                can_reg.$tiXr.write(|w| unsafe { w.stid().bits(message.stid) });
                if message.rtr.is_some() { can_reg.$tiXr.modify(|_, w| w.rtr().bit(message.rtr.unwrap())); }
                if message.exid.is_some() { can_reg.$tiXr.modify(|_, w| unsafe { w.exid().bits(message.exid.unwrap()) }); }
                can_reg.$tdlXr.modify(|_, w| unsafe { w.data0().bits(message.data0) });
                if message.data1.is_some() { can_reg.$tdlXr.modify(|_, w| unsafe { w.data1().bits(message.data1.unwrap()) }); }
                if message.data2.is_some() { can_reg.$tdlXr.modify(|_, w| unsafe { w.data2().bits(message.data2.unwrap()) }); }
                if message.data3.is_some() { can_reg.$tdlXr.modify(|_, w| unsafe { w.data3().bits(message.data3.unwrap()) }); }
                if message.data4.is_some() { can_reg.$tdhXr.modify(|_, w| unsafe { w.data4().bits(message.data4.unwrap()) }); }
                if message.data5.is_some() { can_reg.$tdhXr.modify(|_, w| unsafe { w.data5().bits(message.data5.unwrap()) }); }
                if message.data6.is_some() { can_reg.$tdhXr.modify(|_, w| unsafe { w.data6().bits(message.data6.unwrap()) }); }
                if message.data7.is_some() { can_reg.$tdhXr.modify(|_, w| unsafe { w.data7().bits(message.data7.unwrap()) }); }
                can_reg.$tiXr.modify(|_, w| w.txrq().set_bit());
                while(can_reg.$tiXr.read().txrq().bit_is_set()) { }
            }
        }
    }
}

transmit_mailbox! { transmit_mailbox_0: (can_tdt0r, can_ti0r, can_tdl0r, can_tdh0r) }
transmit_mailbox! { transmit_mailbox_1: (can_tdt1r, can_ti1r, can_tdl1r, can_tdh1r) }
transmit_mailbox! { transmit_mailbox_2: (can_tdt2r, can_ti2r, can_tdl2r, can_tdh2r) }

macro_rules! transmit {
    ($FUNCNAME:ident: ($mbx_trans_0:ident, $mbx_trans_1:ident, $mbx_trans_2:ident )) => {
        impl<'a, U>Canust<'a, U>
        where
            U: Any + cantrait
        {
            fn $FUNCNAME(&self, message: CAN_MESSAGE) -> Result<u8, &str> {
                let can_reg = self.0;
                let mut free_mailbox: Option<u8> = None;
                if (can_reg.can_tsr.read().tme0().bit_is_set()) { self.$mbx_trans_0(message); Ok(0) }         // Check if mailbox 0 is empty, if not check 1 and then 2.
                else if (can_reg.can_tsr.read().tme1().bit_is_set()) { self.$mbx_trans_1(message); Ok(1) }
                else if (can_reg.can_tsr.read().tme2().bit_is_set()) { self.$mbx_trans_2(message); Ok(2) }
                else { Err("No mailbox empty") }
            }
        }
    }
}

transmit! { transmit: (transmit_mailbox_0, transmit_mailbox_1, transmit_mailbox_2) }


macro_rules! add_specific_filter {
    ($FUNCNAME:ident: ($fbmX:ident, $fscX:ident, $ffaX:ident, $fXr1:ident, $fXr2:ident, $factX:ident)) => {
        impl<'a, U> Canust<'a, U>
        where
            U: Any + cantrait
        {
            fn $FUNCNAME<T>(&self, filter_settings: T)
            where
                T: can_filter_trait
            {
            let can_reg = self.0;
            
            let mode = filter_settings.get_mode();
            let scale = filter_settings.get_scale();
            let active = filter_settings.get_active();
            let fifo = filter_settings.get_fifo();
            let reg1 = filter_settings.get_reg1();
            let reg2 = filter_settings.get_reg2();

            can_reg.can_fmr.modify(|_, w| w.finit().set_bit());

            if mode.is_some() {   can_reg.can_fm1r.modify(|_, w| w.$fbmX().bit(mode.unwrap())) };
            if scale.is_some() {  can_reg.can_fs1r.modify(|_, w| w.$fscX().bit(scale.unwrap())) };
            can_reg.can_ffa1r.modify(|_, w| w.$ffaX().bit(fifo));
            if reg1.is_some() {   can_reg.$fXr1.write(|w| unsafe { w.bits(reg1.unwrap()) }) };
            if reg2.is_some() {   can_reg.$fXr2.write(|w| unsafe { w.bits(reg2.unwrap()) }) };
            if active.is_some() { can_reg.can_fa1r.modify(|_, w| w.$factX().bit(active.unwrap())) };
            
            can_reg.can_fmr.modify(|_, w| w.finit().clear_bit());
            }
        }
    }
}


add_specific_filter! { add_filter_0: (fbm0, fsc0, ffa0, f0r1, f0r2, fact0) }
add_specific_filter! { add_filter_1: (fbm1, fsc1, ffa1, f1r1, f1r2, fact1) }
add_specific_filter! { add_filter_2: (fbm2, fsc2, ffa2, f2r1, f2r2, fact2) }

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

impl CAN_MESSAGE {
    pub fn new() -> Self {
        Self {
            data0: 0,
            data1: None,
            data2: None,
            data3: None,
            data4: None,
            data5: None,
            data6: None,
            data7: None,
            rtr: None,
            stid: 0,
            exid: None,
            fmi: None,
            time: None,
            dlc: None,
        }
    }
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