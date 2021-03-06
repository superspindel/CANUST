#![no_std]
#![feature(int_to_from_bytes)]
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

const POWER_LED_ID: u16 = 2047;
const GAME_LED_ID: u16 = 2046;
const CONNECTION_LED_ID: u16 = 2045;
const STATUS_LED_ID: u16 = 2044;

app! {
    device: stm32f0x,
    resources: {
        static CAN: stm32f0x::CAN;
        static EXTI: stm32f0x::EXTI;
    },

    tasks: {
         EXTI2_3:
        {
            path: button_clicked,
            priority: 1,
            resources: [CAN, EXTI]
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources
{
    p.device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    /*
    Standby output to run the tranceiver, should be set to low.
    */
    p.device.RCC.ahbenr.modify(|_, w| w.iopben().set_bit());
    p.device.GPIOB.moder.modify(|_, w| unsafe { w.moder2().bits(1) });
    p.device.GPIOB.odr.modify(|_, w| unsafe { w.odr2().bit(false) });
    
    /*
    External clock set to 48MHz, prescaled down to 24MHz.
    */
    let ext_clock = ExternalClock;
    ext_clock.init(&p.device.RCC);

    BUTTON.init(&p.device.GPIOA, &p.device.RCC, &p.device.SYSCFG_COMP, &p.device.EXTI);

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
        id1: Some(GAME_LED_ID),
        id2: Some(STATUS_LED_ID),
        id3: Some(CONNECTION_LED_ID),
        id4: Some(POWER_LED_ID),
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
        fifo1_message_pending: false,
        fifo0_overrun: false,
        fifo0_full: false,
        fifo0_message_pending: false,
        transmit_mailbox_empty: false,
    };
    can_connector.canit(canterups);
    }
    
    init::LateResources {
    CAN: p.device.CAN,
    EXTI: p.device.EXTI,
    }
}

fn idle() -> !
{   
    loop {
        rtfm::wfi();
    }
}

fn button_clicked(t: &mut Threshold, EXTI2_3::Resources {
    CAN: mut can,
    EXTI: exti,
}: EXTI2_3::Resources
) {
    BUTTON.reset(&*exti);
    can.claim_mut(t, |canen, _t| { // Claim can in case of usage in other methods
        let can_connector = Canust(canen); // The Canust object with the transmit method
        let mut message = CanMessage::new(); // Create new message
        message.stid = GAME_LED_ID; // Setting standard id to 2046
        message.dlc = 1; // Setting DLC to 1.
        message.dataset_0[0] = 50;
        match can_connector.transmit(message) { // match the transmit to check if a transmit mailbox was available
            Ok(mbx) => {} // Transmit OK
            Err(mbx) => {}, // Transmit mailboxes are full
        }
    });
}