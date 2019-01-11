/*
 Example where messages can be received with a standard id of either 2044, 2045, 2046 or 2047.
 The program will check the the interrupts active and turn of the light of the leds depending on which interrupt is active.
 So the program shows of how to use the active interrupts method.
*/

#![no_std]
#![feature(int_to_from_bytes)]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f0x;

mod canust;
// Specific code for the board that we've implemented
mod led_api;
mod button;
mod external_clock;

use cortex_m::*;
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Resource, Threshold};
use led_api::{PowerLed, GameLed, ConnectionLed, StatusLed};
use button::BUTTON;
use external_clock::ExternalClock;
use canust::{Canust, CanInitParameters, FilterU16List, FilterFifo, CanInitInterrupts, CanMessage, FilterU32Mask, FilterU16Mask, CanInterruptsActive};

const POWER_LED_ID: u16 = 2047;
const GAME_LED_ID: u16 = 2046;
const CONNECTION_LED_ID: u16 = 2045;
const STATUS_LED_ID: u16 = 2044;

app! {
    device: stm32f0x,
    resources: {
        static CAN: stm32f0x::CAN;
        static GPIOA: stm32f0x::GPIOA;
        static POWER_LED: PowerLed;
        static GAME_LED: GameLed;
        static CONN_LED: ConnectionLed;
        static STATUS_LED: StatusLed;
        static EXTI: stm32f0x::EXTI;
    },

    tasks: {
        /*
        Global CAN interrupt.
        */
        CEC_CAN:
        {
            path: can_handler,
            priority: 3,
            resources: [GPIOA, CAN, POWER_LED, GAME_LED, CONN_LED, STATUS_LED]
        },
        /*
        General external interrupt, connected to button on test board.
        */
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

    /*
    One of the leds connected to the board, initialized and turned on.
    */
    let pwr_led = PowerLed;
    pwr_led.init(&p.device.GPIOA, &p.device.RCC);
    pwr_led.on(&p.device.GPIOA);

    /*
    One of the leds connected to the board, initialized and turned on.
    */
    let game_led = GameLed;
    game_led.init(&p.device.GPIOA, &p.device.RCC);
    game_led.on(&p.device.GPIOA);

    /*
    One of the leds connected to the board, initialized and turned on.
    */
    let conn_led = ConnectionLed;
    conn_led.init(&p.device.GPIOA, &p.device.RCC);
    conn_led.on(&p.device.GPIOA);

    /*
    One of the leds connected to the board, initialized and turned on.
    */
    let status_led = StatusLed;
    status_led.init(&p.device.GPIOA, &p.device.RCC);
    status_led.on(&p.device.GPIOA);

    /*
    The button interrupt initializer.
    */
    BUTTON.init(&p.device.GPIOA, &p.device.RCC, &p.device.SYSCFG_COMP, &p.device.EXTI);

    {
    /*
    The CAN object, has the internal methods used to transmit receive and initialize the CAN connection.
    Currently developed alongside a Canalyzer from Kvaser, internal settings results in a 500kbit stream with Tseg1 = 12, Tseg2 = 3 and SJW = 3.
    */
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
        gpioa11: true,  // pin output initialize
        gpioa12: true,  // pin output initialize
        gpiob8: false,  // pin output initialize
        gpiob9: false,  // pin output initialize
    };
    can_connector.init(&p.device.GPIOA, &p.device.GPIOB, &p.device.RCC, init_can);
    
    /*
    Filter for processing incomming messages, can be set up in 32 bit or 16 bit sizes, with or without a mask as per documentation.
    */
    let sendidfilter = FilterU16List {
        fifo: FilterFifo::_0,
        active: Some(true),
        id1: Some(GAME_LED_ID),
        id2: Some(STATUS_LED_ID),
        id3: Some(CONNECTION_LED_ID),
        id4: Some(POWER_LED_ID),
    };
    can_connector.add_filter_0(sendidfilter);

    /*
    The chosen interrupts to be active on the CPU.
    */
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
    // Initializing those interrupts
    can_connector.canit(canterups);
    }
    
    init::LateResources {
    POWER_LED: pwr_led,
    GAME_LED: game_led,
    STATUS_LED: status_led,
    CONN_LED: conn_led,
    CAN: p.device.CAN,
    GPIOA: p.device.GPIOA,
    EXTI: p.device.EXTI,
    }
}

fn idle() -> !
{   
    loop {
        rtfm::wfi();
    }
}

fn can_handler(t: &mut Threshold, CEC_CAN::Resources {
    GPIOA: gpioa,
    CAN: mut can,
    POWER_LED: pwr_led,
    GAME_LED: game_led,
    CONN_LED: conn_led,
    STATUS_LED: stat_led,
}: CEC_CAN::Resources
) {
    can.claim_mut(t, |canen, _t| {
        let can_connector = Canust(canen);
        let active_interrupts = can_connector.get_interrupts(); // Fetches a CanInterruptsActive struct containing information about all pending / not pending interrupts.
        handle_lights(active_interrupts, &*pwr_led, &*game_led, &*stat_led, &*conn_led, &gpioa);
    });
}

/*
Standard method that takes the led objects, the interrupt struct as well as GPIOA and checks the interrupts active.
*/
fn handle_lights( interrupts: CanInterruptsActive, pwr_led: &PowerLed, game_led: &GameLed, stat_led: &StatusLed, conn_led: &ConnectionLed, gpioa: &stm32f0x::GPIOA) {
    if interrupts.fifo0_message_pending != 0 { pwr_led.off(&gpioa); }   // Once a message is received
    if interrupts.fifo0_message_pending == 2 { conn_led.off(&gpioa); } // Once 2 messages has been received
    if interrupts.fifo0_full { game_led.off(&gpioa); }  // Once the fifo is full
    if interrupts.fifo0_overrun { stat_led.off(&gpioa); }   // Once it has overrun

}

/*
When pressing the button a standard message will be sent with stid 2046, dlc 1 and data0 = 50. Example of transmission.
*/
fn button_clicked(t: &mut Threshold, EXTI2_3::Resources {
    CAN: mut can,
    EXTI: exti,
}: EXTI2_3::Resources
) {
    BUTTON.reset(&*exti);
    can.claim_mut(t, |canen, _t| {
        let can_connector = Canust(canen);
        let mut message = CanMessage::new();
        message.stid = GAME_LED_ID;
        message.dlc = 1;
        message.dataset_0[0] = 50;
        match can_connector.transmit(message) { // Transmits a message
            Ok(mbx) => {}
            Err(mbx) => {},
        }
    });
}