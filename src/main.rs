// #![no_std]
// #![no_main]

// #[macro_use]

// #[allow(unused_imports)]
// extern crate panic_semihosting ;
// extern crate cortex_m_rt;
// extern crate cortex_m;
// extern crate cortex_m_semihosting;
// //extern crate stm32f3;

// use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
// use cortex_m_semihosting::hprint;
// use stm32f3::stm32f303;

// // use `main` as the entry point of this application
// // `main` is not allowed to return

// #[entry]
// fn main() ->! {
//     //initializing peripherals
//     let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
//     //calling rcc from library
//     let rcc = &peripherals.RCC;
//     let gpioa = &peripherals.GPIOA;

// //******** 1- RCC Peripheral: Enable GPIOA ********/  
//     //enabling input/output port
//     rcc.ahbenr.write(|w|
//         w 
//             .iopaen().set_bit()
//     );
//     //enabling apb1 adc2 clock
//     rcc.apb1enr.write(|w|
//         w
//             .dac2en().set_bit()

//     );

// //******** 2- GPIOA: Configure PA5 pin as an analoge mode********/    
   
//     //mode selection
//     gpioa.moder.write(|w|
//     w
//         .moder5().bits(0b11) //0b11 = analoge
//     );


//     //pullup/pulldown selection
//     gpioa.pupdr.write(|w| unsafe { 
//     w
//         .pupdr5().bits(0b00) // no pull pull up/pull down
//     });

// //****3 -DAC configuing *********/
// let dac = peripherals.DAC;
// dac.cr.write(|w| unsafe{
// w
//     .en2().set_bit()  //enablind dac channel 2
//     .boff2().set_bit() //enabling chnnel 2 output buffer
//     .ten2().set_bit() //DAC channel1 trigger enable
//     .tsel2().bits(0b111) //DAC channel2 trigger selection as software triger
//     .wave2().bits(0b10) //DAC channel2 triangle wave generation enable
//     .mamp2().bits(0b1011) //DAC channel2 mask/amplitude selector (1011=4095)
// });

//     dac.swtrigr.write(|w|
//         w
//         .swtrig2().set_bit() //DAC channel2 software trigger enabling
//     );

//     let dac_value = 3;

//     dac.dhr12r2.write(|w|unsafe{
//         w
//             .dacc2dhr().bits(dac_value)
//         });
//     let mut data1 = dac.dhr12r2.read().dacc2dhr().bits();
//     hprint!("voltage {}",data1);

//     let mut data = dac.dor2.read().bits();
//     //let voltage = 3 * (data/4095);
//     hprintln!("voltage {}",data);
    
//     loop{
// //******* 4-Starts convertion ********* */
//     dac.swtrigr.write(|w|
//         w
//         .swtrig2().set_bit() //DAC channel2 software trigger enabling
//     );

//     let mut data = dac.dor2.read().bits();
//     //let voltage = 3 * (data/4095);
//     hprint!("voltage {}",data1);
//     hprintln!("voltage {}",data);
    

    
//     // let mut counter = 0;
//     // while counter==10{
//     //     counter += 1;
//     // }

    



//     }
// }
//**************traits example aplication in embedded *********/
use cast::u32;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::hal::blocking::delay::{DelayMs, DelayUs};
use crate::rcc::Clocks;

/// System timer (SysTick) as a delay provider
pub struct Delay {
    clocks: Clocks,
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST, clocks: Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay { syst, clocks }
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32(ms));
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32(ms));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        let rvr = us * (self.clocks.sysclk().0 / 1_000_000);

        assert!(rvr < (1 << 24));

        self.syst.set_reload(rvr);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32(us))
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32(us))
    }
}