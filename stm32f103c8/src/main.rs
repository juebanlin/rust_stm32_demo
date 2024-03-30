#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;
use alloc::format;
use rtt_target::{rprintln, rtt_init_print};
use core::alloc::Layout;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr;
use micromath::F32Ext;
use alloc_cortex_m::CortexMHeap;
use byteorder::LittleEndian;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};
use stm32f1xx_hal::gpio::PinState;
use stm32f1xx_hal::i2c::Error;
use stm32f1xx_hal::stm32::Peripherals;
use stm32f103c8::bq40z50r2::{Address, BQ40Z50, Cmd};


// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 512;

//内存分配错误处理
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic_(_info:&PanicInfo) -> !{
    rprintln!("PanicInfo:{}",_info);
    loop {

    }
}

#[entry]
fn main() -> ! {
    // 初始化内存分配器
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }
    rtt_init_print!();
    rprintln!("rtt init success!");
    smbus_demo();
}

fn smbus_demo()->!{
    //https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/i2c-bme280/src/main.rs
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let use1=false;
    let clocks;
    if use1{
        // clocks=rcc.cfgr.freeze(&mut flash.acr);
        clocks=rcc.cfgr.sysclk(16.MHz()).pclk1(6.MHz()).pclk2(6.MHz()).freeze(&mut flash.acr);
    }else{
        clocks=rcc.cfgr.sysclk(8.MHz()).pclk1(6.MHz()).freeze(&mut flash.acr);
        //外部时钟
        // clocks=rcc.cfgr.pclk1(8.MHz()).freeze(&mut flash.acr);
    }
    let mut delay = cp.SYST.delay(&clocks);
    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split();
    dp.I2C1.cr1.modify(|_,w|w.smbus().smbus());
    dp.I2C1.cr1.modify(|_,w|w.smbtype().host());
    // let scl = gpiob.pb6.into_open_drain_output_with_state(&mut gpiob.crl,PinState::High);
    // let sda = gpiob.pb7.into_open_drain_output_with_state(&mut gpiob.crl,PinState::High);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    // dp.I2C1.cr1.modify(|_,w|w.pos().clear_bit());
    // dp.I2C1.cr1.modify(|_,w|w.pec().enabled());
    // dp.I2C1.cr1.modify(|_,w|w.alert().release());
    // dp.I2C1.cr1.modify(|_,w|w.enpec().enabled());
    // dp.I2C1.cr1.modify(|_,w|w.pe().enabled());
    // dp.I2C1.cr1.modify(|_,w|w.nostretch().enabled());
    dp.I2C1.cr1.modify(|_,w|w.ack().ack());
    {
        // dp.I2C1.cr1.modify(|_, w| w.start().set_bit());/// Generate START condition
        // let addr=0x16u8;
        // let read=false;
        // //Sends the (7-Bit) address on the I2C bus. The 8th bit on the bus is set
        // dp.I2C1.sr1.read();
        // dp.I2C1.dr.write(|w| w.dr().bits(addr << 1 | (u8::from(read))));
        // // dp.I2C1.cr1.modify(|_, w| w.stop().set_bit());/// Generate STOP condition
    }

    let pins=(scl,sda);
    delay.delay_ms(100u32);
    let mut i2c = BlockingI2c::i2c1(
        dp.I2C1, pins, &mut afio.mapr,
        Mode::Standard {
            frequency: 100.kHz(),
            // duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        100,
        10,
        100,
        100,
    );
    rprintln!("start1");


    let mut buffer4 = [0u8; 2];
    // i2c.write_read(0x16u8, &[0x00,0x09],&mut buffer4).unwrap();
    let x= i2c.write( 0x16, &[0x00,0x09]).unwrap();

    // // i2c.write_read(0x16u8,&[0x09u8],&mut buffer).unwrap();
    // delay.delay_us(90u32);
    let mut buffer = [0u8; 2];
    let y= i2c.read(0x16,&mut buffer);
    // delay.delay_ms(100u32);
    rprintln!("rsp:{},{}",buffer[0],buffer[1]);
    let mut bms = BQ40Z50::new(i2c);
    loop {
        rprintln!("start");
        let temp = bms.get_cell_voltage_1().unwrap();
        rprintln!("b");
        let volt = bms.get_cell_voltage_2().unwrap();
        rprintln!("v");
        let curr = bms.get_cell_voltage_3().unwrap();
        rprintln!(
            "Temperature: {:.2}\n Voltage: {:.2}\n Current: {:.2}",
            temp, volt, curr
        );
    }
}
