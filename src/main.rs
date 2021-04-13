//! main.rs

//#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(asm, test)]


use cortex_m::asm;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    prelude::*,
    gpio::*,
};
use rtic::cyccnt::{Instant, U32Ext};
use core::hint::black_box;

const PERIOD: u32 = 8_000_000;

#[derive(Clone, Debug)]
enum TestEnum {
    ITest(i32),
    UTest(u32),
    Struct(TestStruct),
    Non,
}

#[derive(Clone, Debug)]
struct TestStruct {
    flag: bool,
    num: u32,
}

#[rtic::app(device = stm32f4xx_hal::stm32, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        led: gpioa::PA5<Output<PushPull>>
    }

    #[init(schedule = [led_on])]
    fn init(mut cx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("init");
        // Initialize LED output
        let gpioa = cx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Initialize cyccnt
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();
        
        // Schedule led to turn on
        cx.schedule.led_on(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources{
            led
        }
    }
    
//    #[no_mangle]
//    #[inline(never)]
    #[task(schedule = [led_off], resources = [led])]
    fn led_on(cx: led_on::Context) {
        let mut my_num = 10;
        let mut test_enum1 = TestEnum::ITest(20);
        let mut test_enum2 = TestEnum::Non;
        let mut test_struct = TestStruct{
            flag: true,
            num: 123,
        };
        let mut  test_enum3 = TestEnum::Struct(test_struct.clone());
        
        my_num = black_box(my_num);
        test_enum1 = black_box(test_enum1);
        test_enum2 = black_box(test_enum2);
        test_enum3 = black_box(test_enum3);
        test_struct = black_box(test_struct);

        black_box(&cx);
        //unsafe{asm!{"bkpt"};}  //asm::bkpt();
        black_box(&cx);

//        black_box(&cx.resources.led);
        cx.schedule.led_off(cx.scheduled + PERIOD.cycles()).unwrap();
        cx.resources.led.set_high().unwrap();
//        black_box(&cx.resources.led);

        //unsafe{asm!{"bkpt"};}  //asm::bkpt();

//        black_box(&cx);
        black_box(my_num);
        black_box(test_enum1);
        black_box(test_enum2);
        black_box(test_enum3);
        black_box(test_struct);
    }

    #[task(schedule = [led_on], resources = [led])]
    fn led_off(cx: led_off::Context) {
        let my_num = 30;
        let test_enum = TestEnum::UTest(40);

        cx.schedule.led_on(cx.scheduled + PERIOD.cycles()).unwrap();
        cx.resources.led.set_low().unwrap();
        
//        unsafe{asm!{"bkpt"};}  //asm::bkpt();

        //black_box(my_num);
        //black_box(test_enum);
    }


    #[idle]
    fn idle(_cx: idle::Context) -> ! {
//        rprintln!("idle");
//        panic!("panic");
        loop {
            continue;
        }
    }


    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }
};
