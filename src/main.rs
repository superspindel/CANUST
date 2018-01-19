// #![deny(unsafe_code)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f0x;

use cortex_m::*;
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Resource, Threshold};

app! {
    device: stm32f0x,
    resources: {
        static ON: bool = true;
    },
    tasks: {
        USART2:
        {
            path: loopback,
            resources: [USART2, GPIOB, ON]
        },
        TIM2:
        {
            path: test_timer,
            resources: [GPIOB, ON, TIM2, USART2]
        },
        CEC_CAN:
        {
            path: can_handler,
            resources: [USART2, GPIOB, CAN]
        }
    },
}

fn init(p: init::Peripherals, _r: init::Resources)
{
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8000000);
    p.SYST.enable_counter();

    p.RCC.ahbenr.modify(|_, w| w.iopben().set_bit());
    p.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());
    p.RCC.apb1enr.modify(|_, w| w.tim2en().set_bit());
    p.RCC.apb1enr.write(|w| w.usart2en().set_bit());

    p.GPIOB.moder.modify(|_, w| unsafe { w.moder3().bits(1 as u8) });
    p.GPIOB.odr.modify(|_, w| w.odr3().bit(true));

    let cnt_value_tim2: u32 = 250000;
    unsafe {
    p.TIM2.arr.write(|w| w.bits(cnt_value_tim2));
    }
    let psc_value_tim2: u16 = 32;
    unsafe {
    p.TIM2.psc.write(|w| w.psc().bits(psc_value_tim2));
    }

    p.TIM2.dier.write(|w| w.uie().set_bit());
    p.TIM2.egr.write(|w| w.ug().set_bit());
    p.TIM2.cr1.write(|w| w.cen().bit(true));
    p.TIM2.cr1.write(|w| w.arpe().set_bit());

    p.GPIOA.moder.modify(|_, w| unsafe { w.moder2().bits(2).moder15().bits(2) });
    p.GPIOA.afrl.write(|w| unsafe { w.afrl2().bits(1) });
    p.GPIOA.afrh.write(|w| unsafe { w.afrh15().bits(1) });
    p.USART2.cr1.write(|w| w.ue().set_bit().te().set_bit().re().set_bit().rxneie().set_bit());
    let baud = 8000000 / 115200;
    p.USART2.brr.write(|w| unsafe { w.bits(baud) });


    /* Initialize CAN:
    - set INRQ bit in MCR
    - wait for INAK bit in MSR to be set

    To initialize CAN controller bit timing ( can_btr ) and CAN option ( can_mcr ) but be set up through software.
    To initialize CAN filter banks (mode, scale, FIFO assignment, activation and filter values), software has to set the FINIT bit ( can_fmr ),
    this can be done outsode of initialization. 

    ( FINIT=1 ) > CAN reception is deactivated.

    - clear INRQ bit
    - Wait for INAK bit to clear


    Once intialization is done, software must request the hardware to enter Normal mode to be able to synchronize on the CAN bus and start reception and
    transmission. The request to enter Normal mode is issued by clearing the INRQ bit in the can_mcr.
    - Wait for 11 consecutive recessive bits ( bus idle state ).

    Sleep Mode is entered by setting the SLEEP bit in the can_mcr register. bxCAN can exit sleep mode either by software, clearing SLEEP bit or by
    detection of activity on the CAN bus.
    - Hardware will clear SLEEP bit if AWUM bit in the can_mcr register is set.

    If the wakeup interrupt is enabled ( WKUIE bit in can_ier ) a wakeup interrupt will be generated on detection of CAN bus activity, even if the bxCAN
    automatically performs the wakeup sequence.

    After SLEEP bit is cleared, sleep mode is exited once bxCAN has synchronized with the CAN bus. When the SLAK bit has been cleared by hardware.
    */

    p.CAN.can_mcr.write(|w| w.inrq().set_bit());
    while !(p.CAN.can_msr.read().inak().bit_is_set())
    {
        // wait for bxCAN to enter intialization state
    }
    p.CAN.can_mcr.write(|w| w.sleep().clear_bit());
    p.CAN.can_btr.write(|w| unsafe { w.ts2().bits(2 as u8) });
    p.CAN.can_btr.write(|w| unsafe { w.ts1().bits(3 as u8) });
    p.CAN.can_btr.write(|w| unsafe { w.brp().bits(5 as u16) });

    p.CAN.can_mcr.write(|w| w.inrq().clear_bit());
    while !(p.CAN.can_msr.read().inak().bit_is_clear())
    {
        // wait for bxCAN to enter intialization state
    }
    p.CAN.can_fmr.write(|w| w.finit().set_bit());
    p.CAN.can_fa1r.write(|w| w.fact0().set_bit());
    p.CAN.f0r1.write(|w| unsafe { w.bits(0xFF700010 as u32) });
    p.CAN.can_fmr.write(|w| w.finit().clear_bit());
    p.CAN.can_ier.write(|w| w.fmpie0().set_bit());

}

fn idle() -> !
{   
    loop {
        rtfm::wfi();
    }
}

fn loopback(t: &mut Threshold, mut r: USART2::Resources)
{
    let received_character = r.USART2.rdr.read().bits() as u8;
    if **r.ON
    {
        r.GPIOB.odr.modify(|_, w| w.odr3().bit(true));
        print_usart(t, r.USART2, "ON");
    }
    if !**r.ON
    {
        r.GPIOB.odr.modify(|_, w| w.odr3().bit(false));
        print_usart(t, r.USART2, "OFF");
    }
    **r.ON = !**r.ON;
    unsafe 
    {
        r.USART2.tdr.write(|w| w.tdr().bits(received_character as u16));
    }
    while r.USART2.isr.read().tc().bit_is_set() 
    {
        // Transmitting data, bit_is_set will be true when transmission completes
    }
}

fn print_usart<A>(t: &mut Threshold, usart2: &mut A,  message: &str)
where
    A: Resource<Data = stm32f0x::USART2>,
{
    // Go through the characters in the message and send them through USART connection.
    for character in message.chars() {
        usart2.claim_mut(t, |usart2, _t| {
            unsafe 
            {
                usart2.tdr.write(|w| w.tdr().bits(character as u16));
            }
            while usart2.isr.read().tc().bit_is_set() 
            {
                
            }
        });
    }
}

fn test_timer(t: &mut Threshold, r: TIM2::Resources)
{
    r.TIM2.cr1.write(|w| w.cen().bit(true));
    r.TIM2.sr.write(|w| w.uif().bit(false));
    if **r.ON
    {
        r.GPIOB.odr.modify(|_, w| w.odr3().bit(true));
        print_usart(t, r.USART2, "ON");
    }
    if !**r.ON
    {
        r.GPIOB.odr.modify(|_, w| w.odr3().bit(false));
        print_usart(t, r.USART2, "OFF");
    }
    **r.ON = !**r.ON;
}

fn transmit(t: &mut Threshold, r: CEC_CAN::Resources)
{
    if(r.CAN.can_tsr.read().tme0().bit_is_set())
    {
        r.CAN.can_tdt0r.write(|w| w.dlc().bits(1 as u8));
        r.CAN.can_tdl0r.write(|w| w.data0().bits(14 as u8));
        r.CAN.can_ti0r.write(|w| w.stid().bits(1 as u16));
        r.CAN.can_ti0r.write(|w| w.txrq().set_bit());
    }
}

fn receive(t: &mut Threshold, r: CEC_CAN::Resources)
{
    if(r.CAN.can_rf0r.read().fmp0().bit_is_set())
    {
        let can_received_message = r.CAN.can_rdl0r.read().data0().bits();
        r.CAN.can_rf0r.write(|w| w.rfom0().set_bit());
        if (can_received_message as u16 == 1)
        {
            /* message received correctly */
        }
    }
}

fn can_handler(t: &mut Threshold, r: CEC_CAN::Resources)
{
    /*
    To transmit:
    - Select one empty transmit mailbox
    - Set up identifier
    - Set up data length code (DLC)
    - Set up Data
    - Request the transmission by setting the corresponding TXRQ bit in the CAN_TIxR register.
    - Pending state is entered, where the mailbox waits to become the highest priority mailbox. (See transmit priority)
    - Scheduled for transmission
    - Can bus becomes idle
    - Message sent
    - Mailbox becomes empty
    - RQCP and TXOK bits in the CAN_TSR register is set upon successful transmission.

    - Transmission fails, indicated by the ALST bit in the CAN_TSR register in case of abitration lost, TERR bit if transmission error detected.
    Once the mailbox has left empty state, the software no longer has write access to the mailbox registers.

    Transmit priority ( Identifier )
    Set by the identifier in the message, if same then lower mailbox number = higher priority.

    ( Transmit request order ) set TXFP bit in can_mcr.
    FIFO queue.
    */


    /*
    To receive:
    - Starting from empty state
    - First valid message received is stored in the FIFO which becomes pending_1
    - Hardware signals the event by setting the FMP[1:0] bits in the CAN_RFR register to the value 01b.
    - Message is available in the FIFO output mailbox. 
    - Software reads out the mailbox content and releases it by setting the RFOM bit in the CAN_RFR register.
    - FIFO becomes empty again. 
    - If another message has been received during this time, the FIFO stays in pending_1 state and the new message
    is available in the output mailbox. 

    If the application does not release the mailbox, the next messsage is stored in the FIFO which enters pending_2 state. (FMP[1:0] = 10b)
    This is repeated for pending_3 (11b).
    At this point the software must release the putput mailbox by setting the RFOM bit, so that a mailbox is free to store the next valid message.
    Otherwhise the next message will cause a loss of message.

    In case of overrun, a new message received while all mailboxes are full, the hardware signals the condition by setting the FOVR bit in the CAN_RFR
    register. 
    Which message is lost depends on the configuration, FIFO lock function disabled( RFLM bit in the can_mcr register is cleared) the last message stored
    in the FIFO will be overwritten by the new incoming message. 
    If enabled (RFLM bit in the can_mcr) the most recent message will be discarded and the software will have the three oldest messages in the FIFO 
    available.

    Once a message has been stored in the FIFO, the FMP[1:0] bits are updated and an interupt request is generated if the FMPIE bit in the can_ier register
    is set.
    If FIFO becomes full, interrupt is generated if FFIE bit in the can_ier register is set and the FULL bit in the CAN_RFR is set.

    On overrun condition, the FOVR bit is set and an interrupt is generated if the FOVIE bit in hte can_ier register is set.



    Filter bank:

    - Each filter bank x consists of two 32-bit registers, CAN_FxR0 and CAN_FxR1.
    - Configured through the corresponding can_fmr register.
    - To configure the FACT bit in the CAN_FAR bit must be deactivated.
    - Filter scale > FSCx bits in the CAN_FS1r register
    - Identidiger list or identifier mask > FBMx bits in the can_fmr register.
    Big book page 824. For more information.



    Error handling:
    - Transmit error counter (TEC value, in CAN_ESR register)
    - Receive error counter (REC value, CAN_ESR register)
    - Bus-off state if TEC > 255. Indicated by the BOFF bit in the CAN_ESR register.
    - Depending on the ABOM bit in the can_mcr register the bxCAN will recover from Bus-Off either automatically or on software request.
    - Has to wait for the recovery sequence specified in the CAN standard, (128 occurences of 11 consecutive recessive bits monitored on CANRX).
    - If ABOM is set, bxCAN will start recovery sequence automatically after it has entered Bus-Off state.
    - If ABOM is cleared, request to enter and to leave initialization mode will initiate recovering sequence.



    can_ier( can interrupt enable register )
    can_btr only accessible in intitialization mode.
    */
}