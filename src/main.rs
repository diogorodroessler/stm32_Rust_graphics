#![no_std]
#![no_main]

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    pixelcolor::{Rgb565}, prelude::*, primitives::{
        Circle, PrimitiveStyle,
    },
};
use ili9341::{Ili9341, Orientation};
use panic_halt as _; // panic handler padrão
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, spi::{Mode, Spi}, time::Hertz};
use stm32f1xx_hal::prelude::*;

/// Simple test
#[allow(unused)]
fn blink() {

    // perifericos da arquitetura
    let cp = cortex_m::Peripherals::take().unwrap();
    // perifericos da placa
    let dp = pac::Peripherals::take().unwrap();

    // incia os perifericos
    let mut _afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // define o pino pb9 como saida
    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);

    // cria um delay bloqueando com o clock da placa
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        // pisca o led a cada 1seg
        led.toggle();
        delay.delay_ms(1000u16);
    }
}

#[entry]
fn main() -> ! {

    // perifericos da arquitetura
    let cp = cortex_m::Peripherals::take().unwrap();
    // perifericos da placa
    let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();

    // incia os perifericos
    let mut _afio = dp.AFIO.constrain();
    _afio.mapr.modify_mapr(|_, w| w.spi1_remap().clear_bit()); // <- remap ativado
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // gpioa | gpiob
    let mut gpiob = dp.GPIOB.split();
    let mut gpioa = dp.GPIOA.split();
    
    // Spi
    
    let clk  = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl); // SCK
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl); // MOSI
    let miso = gpioa.pa6;
    let dc = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut rst = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    // cria um delay bloqueando com o clock da placa
    let mut delay = cp.SYST.delay(&clocks);
    rst.set_low();
    delay.delay_ms(10_u16);
    rst.set_high();
    delay.delay_ms(120_u16);
    let cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let spi = Spi::spi1(
        dp.SPI1,
        (clk, miso, mosi),
        &mut _afio.mapr,
        Mode {
            polarity: stm32f1xx_hal::spi::Polarity::IdleLow,
            phase: stm32f1xx_hal::spi::Phase::CaptureOnFirstTransition,
        },
        Hertz::MHz(8),
        clocks
    );
    let _iface = SPIInterface::new(spi, dc, cs);


    // ILI9341
    let mut ili = Ili9341::new(
        _iface, rst, &mut delay, Orientation::Landscape, ili9341::DisplaySize240x320
    )
    .unwrap()
    ;

    /* Drawing Area ------------------------- */

    Circle::new(Point::new(0, 0), 20)
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 2))
    .draw(&mut ili)
    .unwrap()
    ;

    loop {

    }
}
