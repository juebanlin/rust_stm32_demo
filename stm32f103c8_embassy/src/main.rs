#![no_std]
#![no_main]

use core::panic::PanicInfo;
use byteorder::{ByteOrder, LittleEndian};
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{Config, Error, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_stm32::gpio::low_level::Pin;
use embassy_stm32::gpio::Speed;
use embassy_time::{Delay, Timer};
use rtt_target::{rprintln, rtt_init_print};
use stm32f103c8embassy::bq40z50r2::{Address, BQ40Z50, Cmd};
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[panic_handler]
fn panic_(_info:&PanicInfo) -> !{
    rprintln!("PanicInfo:{}",_info);
    loop {

    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_init_print!();
    rprintln!("rtt init success!");
    let p = embassy_stm32::init(Default::default());
    let mut cfg=embassy_stm32::i2c::Config::default();
    cfg.scl_pullup=true;
    cfg.sda_pullup=true;
    cfg.timeout=embassy_time::Duration::from_millis(100);
    p.PB8.set_high();
    p.PB9.set_high();
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        cfg,
    );
    // loop {
    //     rprintln!("do");
    //     Timer::after_millis(1000).await;
    //     let mut data = [0u8; 2];
    //     match i2c.blocking_write_read(0x16, &[Cmd::CellVoltage1Reg as u8], &mut data) {
    //         Ok(()) => rprintln!("ok:{}", LittleEndian::read_u16(&data[0..2])),
    //         Err(Error::Timeout) => rprintln!("timeout"),
    //         Err(e) => rprintln!("error:{:?}", e),
    //     }
    // }
    let mut bms = BQ40Z50::new(i2c);
    loop {
        Timer::after_millis(1000).await;
        rprintln!("start");
        let temp = bms.get_cell_voltage_1().unwrap();
        rprintln!("b");
        let volt = bms.get_cell_voltage_2().unwrap();
        rprintln!("v");
        let curr = bms.get_cell_voltage_3().unwrap();
        // info!("Temperature: {:.2}\n Voltage: {:.2}\n Current: {:.2}",temp, volt, curr);
    }
}