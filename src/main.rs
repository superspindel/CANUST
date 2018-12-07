// #![deny(unsafe_code)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f0x;

mod canust;
mod can_hal;

use cortex_m::*;
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Resource, Threshold};

app! {
    device: stm32f0x,
    resources: {
        static ON: bool = true;
        static CAN: stm32f0x::CAN;
        static TIM: stm32f0x::TIM2;
        static USART2: stm32f0x::USART2;
        static GPIOA: stm32f0x::GPIOA;
    },
    tasks: {
        TIM2:
        {
            path: test_timer,
            resources: [CAN, ON, TIM, USART2, GPIOA]
        },
        CEC_CAN:
        {
            path: can_handler,
            resources: [USART2, GPIOA, CAN]
        }
    },
}

fn init(p: init::Peripherals, _r: init::Resources) -> init::LateResources
{



    init::LateResources {
    CAN: p.device.CAN,
    TIM: p.device.TIM2,
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