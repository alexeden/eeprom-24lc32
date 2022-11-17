#![no_std]
#![no_main]
use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt,
    i2c::I2c,
    pac,
    prelude::*,
    rcc::{RccExt, SYSCLK_MAX},
};

const DEFAULT_ADDR: u8 = 0x50;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.RCC.constrain().cfgr.sysclk(SYSCLK_MAX.Hz()).freeze();
    let _delay = cp.SYST.delay(&clocks);
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
    let mut buffer = [0u8];

    i2c.write_read(DEFAULT_ADDR, &0u8.to_be_bytes(), &mut buffer)
        .expect("Failed to read");
    rprintln!("Read {:?}", buffer);

    // let mut write = [0u8; 3];
    // write[0..2].copy_from_slice(&1u16.to_be_bytes());
    // write[2] = 0x0A;
    // rprintln!("Writing this {:#?}", write);
    // i2c.write(DEFAULT_ADDR, &write).expect("Failed to write");
    // asm::delay(10_000_000);
    // rprintln!("Written");
    i2c.write_read(DEFAULT_ADDR, &1u16.to_be_bytes(), &mut buffer)
        .expect("Failed to read");
    rprintln!("Read {:?}", buffer);

    i2c.write_read(DEFAULT_ADDR, &2u16.to_be_bytes(), &mut buffer)
        .expect("Failed to read");
    rprintln!("Read {:?}", buffer);
    i2c.write_read(DEFAULT_ADDR, &3u16.to_be_bytes(), &mut buffer)
        .expect("Failed to read");
    rprintln!("Read {:?}", buffer);
    loop {}
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC! {}", info);
    rprintln!("Location {:?}", info.location());
    if let Some(pl) = info.payload().downcast_ref::<&str>() {
        rprintln!("Payload {:?}", pl);
    }
    loop {}
}
