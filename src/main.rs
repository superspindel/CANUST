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
use external_clock::CLOCK;
use canust::{Canust, CanInitParameters, FilterU16List, FilterFifo, CanInitInterrupts, CanMessage};

const POWER_LED_ID: u16 = 3000;
const GAME_LED_ID: u16 = 4000;
const CONNECTION_LED_ID: u16 = 5000;
const STATUS_LED_ID: u16 = 6000;

app! {
    device: stm32f0x,
    resources: {
        static CAN: stm32f0x::CAN;
        static USART2: stm32f0x::USART2;
        static GPIOA: stm32f0x::GPIOA;
        static POWER_LED: PowerLed;
        static GAME_LED: GameLed;
        static CONN_LED: ConnectionLed;
        static STATUS_LED: StatusLed;
        static EXTI: stm32f0x::EXTI;
    },

    tasks: {
        CEC_CAN:
        {
            path: can_handler,
            resources: [USART2, GPIOA, CAN, POWER_LED, GAME_LED, CONN_LED, STATUS_LED]
        },
         EXTI2_3:
        {
            path: button_clicked,
            resources: [CAN, EXTI]
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources
{
    CLOCK.init(&p.device.RCC);

    let pwr_led = PowerLed;
    pwr_led.init(&p.device.GPIOA, &p.device.RCC);

    let game_led = GameLed;
    game_led.init(&p.device.GPIOA, &p.device.RCC);

    let conn_led = ConnectionLed;
    conn_led.init(&p.device.GPIOA, &p.device.RCC);

    let status_led = StatusLed;
    status_led.init(&p.device.GPIOA, &p.device.RCC);

    BUTTON.init(&p.device.GPIOA, &p.device.RCC, &p.device.SYSCFG_COMP, &p.device.EXTI);

    {
    let can_connector = Canust(&p.device.CAN);
    let init_can = CanInitParameters {
        tseg1: 11,
        tseg2: 2,
        sjw: 3,
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
        fifo1_message_pending: true,
        fifo0_overrun: false,
        fifo0_full: false,
        fifo0_message_pending: true,
        transmit_mailbox_empty: false,
    };
    can_connector.canit(canterups);
    }
    

    init::LateResources {
    POWER_LED: pwr_led,
    GAME_LED: game_led,
    STATUS_LED: status_led,
    CONN_LED: conn_led,
    CAN: p.device.CAN,
    USART2: p.device.USART2,
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
    USART2: usart2,
    GPIOA: gpioa,
    CAN: mut can,
    POWER_LED: pwr_led,
    GAME_LED: game_led,
    CONN_LED: conn_led,
    STATUS_LED: stat_led,
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
    handle_lights(message_received, &*pwr_led, &*game_led, &*conn_led, &*stat_led, &gpioa);
}

fn handle_lights(message: CanMessage, pwr_led: &PowerLed, game_led: &GameLed, conn_led: &ConnectionLed, stat_led: &StatusLed, gpioa: &stm32f0x::GPIOA) {
    match message.stid {
        POWER_LED_ID => { pwr_led.toggle(&gpioa) },
        GAME_LED_ID => { game_led.toggle(&gpioa) },
        CONNECTION_LED_ID => { conn_led.toggle(&gpioa) },
        STATUS_LED_ID => { stat_led.toggle(&gpioa) },
        _ => {},
    }
}

fn button_clicked(t: &mut Threshold, EXTI2_3::Resources {
    CAN: mut can,
    EXTI: exti,
}: EXTI2_3::Resources
) {
    can.claim_mut(t, |canen, _t| {
        let can_connector = Canust(canen);
        let mut message = CanMessage::new();
        message.stid = POWER_LED_ID;
        can_connector.transmit(message).unwrap();
    });
    BUTTON.reset(&*exti);
}