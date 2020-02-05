#![no_main]
#![no_std]

use aux::{entry, prelude::*};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds,gpioa,rcc) = aux::init();

    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());

    gpioa.moder.modify(|_,w| w.moder0().input());

    loop{
        if  gpioa.idr.read().idr0().bit_is_clear(){
            let ms = 50_u8;
                for curr in 0..8 {
                    
                    let next = (curr + 1) % 8;
        
                    leds[next].on();
                    delay.delay_ms(ms);
                    leds[curr].off();
                    delay.delay_ms(ms);
                }
             }
    }
}