#![no_std]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f0x;

mod canust;
mod led_api;
mod button;
mod external_clock;

use cortex_m::*;
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Resource, Threshold};
use led_api::{PowerLed, GameLed, ConnectionLed, StatusLed};
use button::BUTTON;
use external_clock::ExternalClock;
use canust::{Canust, CanInitParameters, FilterU16List, FilterFifo, CanInitInterrupts, CanMessage, FilterU32Mask, FilterU16Mask};

app! {
    device: stm32f0x,
    resources: {
        static CAN: stm32f0x::CAN;
    },

    tasks: {
        CEC_CAN:
        {
            path: can_handler,
            priority: 3,
            resources: [CAN]
        },
    },
}

fn init(p: init::Peripherals) -> init::LateResources
{
    p.device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    p.device.RCC.ahbenr.modify(|_, w| w.iopben().set_bit());
    p.device.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

    /*
    Standby output to run the tranceiver, should be set to low.
    */
    p.device.GPIOB.moder.modify(|_, w| unsafe { w.moder2().bits(1) });
    p.device.GPIOB.odr.modify(|_, w| unsafe { w.odr2().bit(false) });
    
    /*
    External clock set to 48MHz, prescaled down to 24MHz.
    */
    let ext_clock = ExternalClock;
    ext_clock.init(&p.device.RCC);
    {
    let can_connector = Canust(&p.device.CAN);
    // 500kbit/s 12 3 + SJW 3
    // 11 2 2 2 settings
    let init_can = CanInitParameters {
        tseg1: 11,
        tseg2: 2,
        sjw: 2,
        lbkm: false,
        silent: false,
        brp: 2,
        dbf: false,
        ttcm: false,
        abom: false,
        awum: false,
        nart: false,
        rflm: false,
        txfp: false,
        gpioa11: true,
        gpioa12: true,
        gpiob8: false,
        gpiob9: false,
    };
    can_connector.init(&p.device.GPIOA, &p.device.GPIOB, &p.device.RCC, init_can);
    
    let sendidfilter = FilterU16List {
        fifo: FilterFifo::_0,
        active: Some(true),
        id1: Some(1),
        id2: Some(2),
        id3: Some(3),
        id4: Some(4),
    };
    can_connector.add_filter_0(sendidfilter);

    let canterups = CanInitInterrupts {
        sleep: false,
        wakeup: false,
        error: false,
        last_error_code: false,
        bus_off: false,
        error_passive: false,
        error_warning: false,
        fifo1_overrun: false,
        fifo1_full: false,
        fifo1_message_pending: true,
        fifo0_overrun: false,
        fifo0_full: false,
        fifo0_message_pending: true,
        transmit_mailbox_empty: false,
    };
    can_connector.canit(canterups);
    }
    
    init::LateResources {
    CAN: p.device.CAN,
    }
}

fn idle() -> !
{   
    loop {
        rtfm::wfi();
    }
}

fn can_handler(t: &mut Threshold, CEC_CAN::Resources {
    CAN: mut can,
}: CEC_CAN::Resources
) {
    let mut message_received: CanMessage = CanMessage::new();
    can.claim_mut(t, |canen, _t| {
        let can_connector = Canust(canen);
        match can_connector.receive() {
            Ok(message) => message_received = message,
            Err(_) => unimplemented!(),
        }
    });
}
