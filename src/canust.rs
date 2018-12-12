#![allow(dead_code)]
use stm32f0x::{GPIOA, RCC, CAN, gpioa, can, GPIOB, gpiof};
use core::ops::Deref; 
use core::any::{Any};
/*
    Implementation for the Canust API
*/

macro_rules! receive_fifo {
    ($FUNCNAME:ident: ($can_rfXr:ident, $fmpX:ident, $can_riXr:ident, $can_rdtXr:ident, $can_rdlXr:ident, $can_rdhXr:ident, $rfomX:ident)) => {
        impl<'a, U>Canust<'a, U>
        where
            U: Any + CanTrait
        {
            pub fn $FUNCNAME (&self) -> Result<CanMessage, &str> {
                let mut message = CanMessage::new();
                let can_reg = self.0;
                if can_reg.$can_rfXr.read().$fmpX().bits() != 0 {
                    message.stid = can_reg.$can_riXr.read().stid().bits();
                    if can_reg.$can_riXr.read().ide().bit_is_set() { message.exid = Some(can_reg.$can_riXr.read().exid().bits()); }
                    message.rtr = Some(can_reg.$can_riXr.read().rtr().bit_is_set());
                    message.time = Some(can_reg.$can_rdtXr.read().time().bits());
                    message.fmi = Some(can_reg.$can_rdtXr.read().fmi().bits());
                    let dlc = can_reg.$can_rdtXr.read().dlc().bits();
                    message.dlc = dlc;
                    message.data0 = can_reg.$can_rdlXr.read().data0().bits();
                    if dlc > 1 { message.data1 = Some(can_reg.$can_rdlXr.read().data1().bits()); }
                    if dlc > 2 { message.data2 = Some(can_reg.$can_rdlXr.read().data2().bits()); }
                    if dlc > 3 { message.data3 = Some(can_reg.$can_rdlXr.read().data3().bits()); }
                    if dlc > 4 { message.data4 = Some(can_reg.$can_rdhXr.read().data4().bits()); }
                    if dlc > 5 { message.data5 = Some(can_reg.$can_rdhXr.read().data5().bits()); }
                    if dlc > 6 { message.data6 = Some(can_reg.$can_rdhXr.read().data6().bits()); }
                    if dlc > 7 { message.data7 = Some(can_reg.$can_rdhXr.read().data7().bits()); }
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
    U: Any + CanTrait
{
    pub fn receive(&self) -> Result<CanMessage, &str> {
        let can_reg = self.0;
        if can_reg.can_rf0r.read().full0().bit_is_set() { self.receive_fifo0() }
        else if can_reg.can_rf1r.read().full1().bit_is_set() { self.receive_fifo1() }
        else if can_reg.can_rf0r.read().fmp0().bits() != 0 { self.receive_fifo0() }
        else { self.receive_fifo1() }
    }

    pub fn get_interrupts(&self) -> CanInterruptsActive {
        let can_reg = self.0;
        let mut interrupts = CanInterruptsActive::new();
        interrupts.sleep = can_reg.can_msr.read().slaki().bit_is_set();
        interrupts.wakeup = can_reg.can_msr.read().wkui().bit_is_set();
        interrupts.error = can_reg.can_msr.read().erri().bit_is_set();
        interrupts.transmit_mailbox_0_empty = can_reg.can_tsr.read().tme0().bit_is_set();
        interrupts.transmit_mailbox_1_empty = can_reg.can_tsr.read().tme1().bit_is_set();
        interrupts.transmit_mailbox_2_empty = can_reg.can_tsr.read().tme2().bit_is_set();
        interrupts.transmit_error_0 = can_reg.can_tsr.read().terr0().bit_is_set();
        interrupts.transmit_error_1 = can_reg.can_tsr.read().terr1().bit_is_set();
        interrupts.transmit_error_2 = can_reg.can_tsr.read().terr2().bit_is_set();
        interrupts.albitration_error_0 = can_reg.can_tsr.read().alst0().bit_is_set();
        interrupts.albitration_error_1 = can_reg.can_tsr.read().alst1().bit_is_set();
        interrupts.albitration_error_2 = can_reg.can_tsr.read().alst2().bit_is_set();
        interrupts.fifo0_overrun = can_reg.can_rf0r.read().fovr0().bit_is_set();
        interrupts.fifo0_full = can_reg.can_rf0r.read().full0().bit_is_set();
        interrupts.fifo0_message_pending = can_reg.can_rf0r.read().fmp0().bits();
        interrupts.fifo1_overrun = can_reg.can_rf1r.read().fovr1().bit_is_set();
        interrupts.fifo1_full = can_reg.can_rf1r.read().full1().bit_is_set();
        interrupts.fifo1_message_pending = can_reg.can_rf1r.read().fmp1().bits();
        interrupts.last_error_code = can_reg.can_esr.read().lec().bits();
        interrupts.bus_off = can_reg.can_esr.read().boff().bit_is_set();
        interrupts.error_passive = can_reg.can_esr.read().epvf().bit_is_set();
        interrupts.error_warning = can_reg.can_esr.read().ewgf().bit_is_set();
        interrupts
    }
}

macro_rules! transmit_mailbox {
    ($FUNCNAME:ident: ($tdtXr:ident, $tiXr:ident, $tdlXr:ident, $tdhXr:ident)) => {
        impl<'a, U>Canust<'a, U>
        where
            U: Any + CanTrait
        {
            fn $FUNCNAME(&self, message: CanMessage) {
                let can_reg = self.0;
                can_reg.$tdtXr.modify(|_, w| unsafe { w.dlc().bits(message.dlc) });
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
                while can_reg.$tiXr.read().txrq().bit_is_set() { }
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
            U: Any + CanTrait
        {
            pub fn $FUNCNAME(&self, message: CanMessage) -> Result<u8, &str> {
                let can_reg = self.0;
                if can_reg.can_tsr.read().tme0().bit_is_set() { self.$mbx_trans_0(message); Ok(0) }         // Check if mailbox 0 is empty, if not check 1 and then 2.
                else if can_reg.can_tsr.read().tme1().bit_is_set() { self.$mbx_trans_1(message); Ok(1) }
                else if can_reg.can_tsr.read().tme2().bit_is_set() { self.$mbx_trans_2(message); Ok(2) }
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
            U: Any + CanTrait
        {
            pub fn $FUNCNAME<T>(&self, filter_settings: T)
            where
                T: CanFilterTrait
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
//add_specific_filter! { add_filter_1: (fbm1, fsc1, ffa1, f1r1, f1r2, fact1) }
//add_specific_filter! { add_filter_2: (fbm2, fsc2, ffa2, f2r1, f2r2, fact2) }
//add_specific_filter! { add_filter_3: (fbm3, fsc3, ffa3, f3r1, f3r2, fact3) }
//add_specific_filter! { add_filter_4: (fbm4, fsc4, ffa4, f4r1, f4r2, fact4) }

pub struct CanMessage {
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
    pub dlc: u8,
}

impl CanMessage {
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
            dlc: 0,
        }
    }
}
/*
    trait: CanTrait
    Implemented on the CAN registerblock of the stm32f0x set of processors. Also specifies the GPIO ports to be used for the CAN.
*/
pub unsafe trait CanTrait: Deref<Target = can::RegisterBlock> {
    type gpioa: Deref<Target = gpioa::RegisterBlock>;
    type gpiob: Deref<Target = gpiof::RegisterBlock>;
}
/*
    Implementation of CanTrait on the CAN registerblock, also specifies the GPIO ports to be used.
*/
unsafe impl CanTrait for CAN {
    type gpioa = GPIOA;
    type gpiob = GPIOB;
}
/*
    struct: Canust
    Consists of a object that implements the CanTrait trait.
*/
pub struct Canust<'a, U>(pub &'a U)
where
    U: Any + CanTrait;

/*
    Clone for Canust
    Implementation of clone for the Canust struct
*/
impl<'a, U> Clone for Canust<'a, U>
where
    U: Any + CanTrait,
{
    fn clone(&self) -> Self {
        *self
    }
}


impl<'a, U> Copy for Canust<'a, U>
where
    U: Any + CanTrait,
{
}

impl<'a, U> Canust<'a, U>
where
    U: Any + CanTrait,
{
    /*
        Init
        "PUBLIC"
        Initializes the CAN on the bus, sets bit timing bits and enters normal state. Needs to read 11 consecutive recessive bits on the bus to enter Normal state.
        @gpio           -> GPIO bort to be used for the CAN messages, specified as GPIOA in the CanTrait implementation.
        @rcc            -> Reset and clock control register, for enabling clocks on GPIOA and CAN.
        @parameters     -> Struct of can parameters to be utilized during initialization, specifies bit timing and settings for CAN transmission.
    */    
    pub fn init(&self, gpioa: &U::gpioa, gpiob: &U::gpiob, rcc: &RCC, parameters: CanInitParameters) {
        let can_reg = self.0;
        rcc.apb1enr.modify(|_, w| w.canen().set_bit());

        if parameters.gpioa11 || parameters.gpioa12 { rcc.ahbenr.modify(|_, w| w.iopaen().set_bit()); }
        /*
        Initialize gpioa pin 11 (RX)
        */
        if parameters.gpioa11 {
            gpioa.moder.modify(|_, w| unsafe { w.moder11().bits(2) });
            gpioa.otyper.modify(|_, w| w.ot11().clear_bit());
            gpioa.pupdr.modify(|_, w| unsafe { w.pupdr11().bits(1) });
            gpioa.ospeedr.modify(|_, w| unsafe { w.ospeedr11().bits(0) });
            gpioa.afrh.modify(|_, w| unsafe { w.afrh11().bits(4) });
        }
        /*
        Initialize gpioa pin 12 (TX)
        */
        if parameters.gpioa12 {
            gpioa.moder.modify(|_, w| unsafe { w.moder12().bits(2) });
            gpioa.otyper.modify(|_, w| w.ot12().clear_bit());
            gpioa.pupdr.modify(|_, w| unsafe { w.pupdr12().bits(1) });
            gpioa.ospeedr.modify(|_, w| unsafe { w.ospeedr12().bits(0) });
            gpioa.afrh.modify(|_, w| unsafe { w.afrh12().bits(4) });
        }

        if parameters.gpiob8 || parameters.gpiob9 { rcc.ahbenr.modify(|_, w| w.iopben().set_bit()); }
        /*
        Initialize gpiob pin 8 (RX)
        */
        if parameters.gpiob8 {
            gpiob.moder.modify(|_, w| unsafe { w.moder8().bits(2) });
            gpiob.otyper.modify(|_, w| w.ot8().clear_bit());
            gpiob.pupdr.modify(|_, w| unsafe { w.pupdr8().bits(1) });
            gpiob.ospeedr.modify(|_, w| unsafe { w.ospeedr8().bits(0) });
            gpiob.afrh.modify(|_, w| unsafe { w.afrh8().bits(4) });
        }
        /*
        Initialize gpiob pin 9 (TX)
        */
        if parameters.gpiob9 {
            gpiob.moder.modify(|_, w| unsafe { w.moder9().bits(2) });
            gpiob.otyper.modify(|_, w| w.ot9().clear_bit());
            gpiob.pupdr.modify(|_, w| unsafe { w.pupdr9().bits(1) });
            gpiob.ospeedr.modify(|_, w| unsafe { w.ospeedr9().bits(0) });
            gpiob.afrh.modify(|_, w| unsafe { w.afrh9().bits(4) });
        }

        can_reg.can_mcr.modify(|_, w| w.inrq().set_bit());
        while can_reg.can_msr.read().inak().bit_is_clear() { /* wait for bxCAN to enter intialization state */ }
        can_reg.can_mcr.modify(|_, w| w.sleep().clear_bit());
        while can_reg.can_msr.read().slaki().bit_is_set() { /* wait for bxCAN to exit sleep state */ }
        can_reg.can_mcr.modify(|_, w| w.dbf().bit(parameters.dbf).ttcm().bit(parameters.ttcm).abom().bit(parameters.abom).awum().bit(parameters.awum).nart().bit(parameters.nart).rflm().bit(parameters.rflm).txfp().bit(parameters.txfp));
        can_reg.can_btr.modify(|_, w| unsafe { w.brp().bits(parameters.brp).ts1().bits(parameters.tseg1).ts2().bits(parameters.tseg2).lbkm().bit(parameters.lbkm).sjw().bits(parameters.sjw) });
        can_reg.can_mcr.modify(|_, w| w.inrq().clear_bit());
        while can_reg.can_msr.read().inak().bit_is_set() { /* wait for bxCAN to exit intialization state and enter normal state */ }
    }
    /*
        canit
        "PUBLIC"
        Sets up the interrupts that the CAN should handle, further described in the CAN_INTERRUPTS struct.
        @interrupts     -> The struct containing which interrupts should be active and not active.
    */
    pub fn canit(&self, interrupts: CanInitInterrupts) {
        let can_reg = self.0;
        can_reg.can_ier.modify(|_, w| w.tmeie().bit(interrupts.transmit_mailbox_empty));
        can_reg.can_ier.modify(|_, w| w.fmpie0().bit(interrupts.fifo0_message_pending));
        can_reg.can_ier.modify(|_, w| w.ffie0().bit(interrupts.fifo0_full));
        can_reg.can_ier.modify(|_, w| w.fovie0().bit(interrupts.fifo0_overrun));
        can_reg.can_ier.modify(|_, w| w.fmpie1().bit(interrupts.fifo1_message_pending));
        can_reg.can_ier.modify(|_, w| w.ffie1().bit(interrupts.fifo1_full));
        can_reg.can_ier.modify(|_, w| w.fovie1().bit(interrupts.fifo1_overrun));
        can_reg.can_ier.modify(|_, w| w.ewgie().bit(interrupts.error_warning));
        can_reg.can_ier.modify(|_, w| w.epvie().bit(interrupts.error_passive));
        can_reg.can_ier.modify(|_, w| w.bofie().bit(interrupts.bus_off));
        can_reg.can_ier.modify(|_, w| w.lecie().bit(interrupts.last_error_code));
        can_reg.can_ier.modify(|_, w| w.errie().bit(interrupts.error));
        can_reg.can_ier.modify(|_, w| w.wkuie().bit(interrupts.wakeup));
        can_reg.can_ier.modify(|_, w| w.slkie().bit(interrupts.sleep));
    }
}
/*
    struct: CanInitParameters
    Used to setup the CAN, containts bittiming settings such as tseg1, tseg2, sjw and brp.
    Also lbkm and silent for loopback mode and silent mode for debugging. Further specification can be found in the
    reference manual as per the usage and definition of the can settings.
*/
pub struct CanInitParameters {
    pub gpioa11: bool,
    pub gpioa12: bool,
    pub gpiob8: bool,
    pub gpiob9: bool,
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
pub struct CanInterruptsActive {
    pub sleep: bool,
    pub wakeup: bool,
    pub error: bool,
    pub last_error_code: u8,
    pub bus_off: bool,
    pub error_passive: bool,
    pub error_warning: bool,
    pub fifo1_overrun: bool,
    pub fifo1_full: bool,
    pub fifo1_message_pending: u8,
    pub fifo0_overrun: bool,
    pub fifo0_full: bool,
    pub fifo0_message_pending: u8,
    pub transmit_mailbox_0_empty: bool,
    pub transmit_mailbox_1_empty: bool,
    pub transmit_mailbox_2_empty: bool,
    pub transmit_error_0: bool,
    pub transmit_error_1: bool,
    pub transmit_error_2: bool,
    pub albitration_error_0: bool,
    pub albitration_error_1: bool,
    pub albitration_error_2: bool,
}

pub struct CanInitInterrupts {
    pub sleep: bool,
    pub wakeup: bool,
    pub error: bool,
    pub last_error_code: bool,
    pub bus_off: bool,
    pub error_passive: bool,
    pub error_warning: bool,
    pub fifo1_overrun: bool,
    pub fifo1_full: bool,
    pub fifo1_message_pending: bool,
    pub fifo0_overrun: bool,
    pub fifo0_full: bool,
    pub fifo0_message_pending: bool,
    pub transmit_mailbox_empty: bool,
}

impl CanInterruptsActive {
    pub fn new() -> Self {
        Self {
            sleep: false,
            wakeup: false,
            error: false,
            last_error_code: 0,
            bus_off: false,
            error_passive: false,
            error_warning: false,
            fifo1_overrun: false,
            fifo1_full: false,
            fifo1_message_pending: 0,
            fifo0_overrun: false,
            fifo0_full: false,
            fifo0_message_pending: 0,
            transmit_mailbox_0_empty: false,
            transmit_mailbox_1_empty: false,
            transmit_mailbox_2_empty: false,
            transmit_error_0: false,
            transmit_error_1: false,
            transmit_error_2: false,
            albitration_error_0: false,
            albitration_error_1: false,
            albitration_error_2: false,
        }
    }
}
/*
    struct: FilterU16List
    Used to setup a filter in listmode with 4x16 bit list filters.
*/
pub struct FilterU16List {
    pub fifo: FilterFifo,
    pub active: Option<bool>,
    pub id1: Option<u16>,
    pub id2: Option<u16>,
    pub id3: Option<u16>,
    pub id4: Option<u16>,
}

/*
    Implementation of CanFilterTrait for FilterU16List. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl CanFilterTrait for FilterU16List {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FilterFifo::_0 => false,
            FilterFifo::_1 => true,
        }
    }
    fn get_reg1(&self) -> Option<u32> { 
        if self.id1.is_some() {
            if self.id2.is_some() {
                Some((self.id2.unwrap() as u32) << 16 | self.id1.unwrap() as u32)
            } else {
                Some(self.id1.unwrap() as u32)
            }
        }  else if self.id2.is_some() {
            Some((self.id2.unwrap() as u32) << 16)
        } else {
            None
        }
    }
    fn get_reg2(&self) -> Option<u32> {
        if self.id3.is_some() {
            if self.id4.is_some() {
                Some((self.id4.unwrap() as u32) << 16 | self.id3.unwrap() as u32)
            } else {
                Some(self.id3.unwrap() as u32)
            }
        } else if self.id4.is_some() {
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
    struct: FilterU16Mask
    Used to setup a filter in maskmode with 2x16 bit filters with 2x16 bit masks.
*/
pub struct FilterU16Mask {
    pub fifo: FilterFifo,
    pub active: Option<bool>,
    pub id1: Option<u16>,
    pub mask1: Option<u16>,
    pub id2: Option<u16>,
    pub mask2: Option<u16>,
}
/*
    Implementation of CanFilterTrait for FilterU16Mask. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl CanFilterTrait for FilterU16Mask {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FilterFifo::_0 => false,
            FilterFifo::_1 => true,
        }
    }
    fn get_reg1(&self) -> Option<u32> {
        if self.id1.is_some() {
            if self.mask1.is_some() {
                Some((self.mask1.unwrap() as u32) << 16 | self.id1.unwrap() as u32)
            } else {
                Some(self.id1.unwrap() as u32)
            }
        }  else if self.mask1.is_some() {
            Some((self.mask1.unwrap() as u32) << 16)
        } else {
            None
        }
    }
    fn get_reg2(&self) -> Option<u32> {
        if self.id2.is_some() {
            if self.mask2.is_some() {
                Some((self.mask2.unwrap() as u32) << 16 | self.id2.unwrap() as u32)
            } else {
                Some(self.id2.unwrap() as u32)
            }
        } else if self.mask2.is_some() {
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
    struct: FilterU32List
    Used to setup a filter in listmode with 2x32 bit list filters.
*/
pub struct FilterU32List {
    pub fifo: FilterFifo,
    pub active: Option<bool>,
    pub id1: Option<u32>,
    pub id2: Option<u32>,
}
/*
    Implementation of CanFilterTrait for FilterU32List. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl CanFilterTrait for FilterU32List {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FilterFifo::_0 => false,
            FilterFifo::_1 => true,
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
    struct: FilterU32List
    Used to setup a filter in listmode with 1x32 bit filter with 1x32 mask.
*/
pub struct FilterU32Mask {
    pub fifo: FilterFifo,
    pub active: Option<bool>,
    pub id: Option<u32>,
    pub mask: Option<u32>,
}
/*
    Implementation of CanFilterTrait for FilterU32Mask. Needed to make the type in the add_filter method generic of the Canust struct.
*/
impl CanFilterTrait for FilterU32Mask {
    fn get_active(&self) -> Option<bool> {
        self.active
    }
    fn get_fifo(&self) -> bool {
        match self.fifo {
            FilterFifo::_0 => false,
            FilterFifo::_1 => true,
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
    trait: CanFilterTrait
    Used to generalize the filter structs for implementation in the Canust struct.
*/
pub trait CanFilterTrait {
    fn get_active(&self) -> Option<bool>;   // Returns if the struct should be active
    fn get_fifo(&self) -> bool;             // Returns the decided FIFO
    fn get_reg1(&self) -> Option<u32>;      // Returns values to set in register 1
    fn get_reg2(&self) -> Option<u32>;      // Returns values to set in register 2
    fn get_scale(&self) -> Option<bool>;    // Returns value to set in scale
    fn get_mode(&self) -> Option<bool>;     // Returns value to set in mode
}
/*
    enum: FilterFifo
    Simple enum for the two mailbox FIFOs.
*/
pub enum FilterFifo {
    _0,
    _1,
}