#![no_std]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f0x;

mod canust;
mod led_api;
mod button;

use cortex_m::*;
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Resource, Threshold};
use led_api::{PowerLed, GameLed, ConnectionLed, StatusLed};
use button::BUTTON;

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
    let pwr_led = PowerLed;
    pwr_led.init(&p.device.GPIOA, &p.device.RCC);

    let game_led = GameLed;
    game_led.init(&p.device.GPIOA, &p.device.RCC);

    let conn_led = ConnectionLed;
    conn_led.init(&p.device.GPIOA, &p.device.RCC);

    let status_led = StatusLed;
    status_led.init(&p.device.GPIOA, &p.device.RCC);

    BUTTON.init(&p.device.GPIOA, &p.device.RCC, &p.device.SYSCFG_COMP, &p.device.EXTI);

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

fn can_handler(t: &mut Threshold, r: CEC_CAN::Resources)
{

}

fn button_clicked(t: &mut Threshold, r: EXTI2_3::Resources)
{
    BUTTON.reset(&*r.EXTI);
}