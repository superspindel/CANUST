#![no_std]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f0x;

mod canust;
mod led_api;

use cortex_m::*;
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Resource, Threshold};
use led_api::{PowerLed, GameLed, ConnectionLed, StatusLed};

app! {
    device: stm32f0x,
    resources: {
        static CAN: stm32f0x::CAN;
        static USART2: stm32f0x::USART2;
        static GPIOA: stm32f0x::GPIOA;
        static POWER_LED: PowerLed;
    },

    tasks: {
        TIM2:
        {
            path: test_timer,
            resources: [CAN, USART2, GPIOA, POWER_LED]
        },
        CEC_CAN:
        {
            path: can_handler,
            resources: [USART2, GPIOA, CAN]
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources
{



    init::LateResources {
    POWER_LED: PowerLed::new(&p.device.GPIOA, &p.device.RCC),
    CAN: p.device.CAN,
    USART2: p.device.USART2,
    GPIOA: p.device.GPIOA,
    }
    


}

fn idle() -> !
{   
    loop {
        rtfm::wfi();
    }
}


fn test_timer(t: &mut Threshold, mut r: TIM2::Resources)
{

}

fn transmit(t: &mut Threshold, r: TIM2::Resources)
{

}

fn receive(t: &mut Threshold, r: CEC_CAN::Resources)
{

}

fn can_handler(t: &mut Threshold, r: CEC_CAN::Resources)
{

}