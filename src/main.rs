//! main.rs

//#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(asm, test)]


//use cortex_m::asm;
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
        let mut test_false: bool = false;
        let mut test_true: bool = true;

        let mut test_u8: u8 = 5; // Fix
        let mut test_u16: u16 = 500; // Fix
        let mut test_u32: u32 = 5000;
        let mut test_u64: u64 = 50000000000;

        let mut test_i8: i8 = -2; // Fix
        let mut test_i16: i16 = -200; // Fix
        let mut test_i32: i32 = -2000;
        let mut test_i64: i64 = -20000000000;

        let mut test_f32: f32 = 10.2;
        let mut test_f64: f64 = 11.22;

        let mut test_enum1 = TestEnum::ITest(20);
        let mut test_enum2 = TestEnum::Non;
        let mut test_struct = TestStruct{
            flag: true,
            num: 123,
        };
        let mut  test_enum3 = TestEnum::Struct(test_struct.clone());

        let const_ = 100;

        let mut test_str = "hello str";
//        let mut test_string: String = "hello string";
//

        test_false = black_box(test_false);
        test_true = black_box(test_true);

        test_u8 = black_box(test_u8);
        test_u16 = black_box(test_u16);
        test_u32 = black_box(test_u32);
        test_u64 = black_box(test_u64);

        test_i8 = black_box(test_i8);
        test_i16 = black_box(test_i16);
        test_i32 = black_box(test_i32);
        test_i64 = black_box(test_i64);
 
        test_f32 = black_box(test_f32);
        test_f64 = black_box(test_f64);


        test_enum1 = black_box(test_enum1);
        test_enum2 = black_box(test_enum2);
        test_enum3 = black_box(test_enum3);
        test_struct = black_box(test_struct);
        black_box(const_);
        test_str = black_box(test_str);
//        test_string = black_box(test_string);

        black_box(&cx);
        test_i16 = my_test(test_i16);
        //unsafe{asm!{"bkpt"};}  //asm::bkpt();
        black_box(&cx);

//        black_box(&cx.resources.led);
        cx.schedule.led_off(cx.scheduled + PERIOD.cycles()).unwrap();
        cx.resources.led.set_high().unwrap();
//        black_box(&cx.resources.led);

        //unsafe{asm!{"bkpt"};}  //asm::bkpt();

//        black_box(&cx);
        
        test_false = black_box(test_false);
        test_true = black_box(test_true);

        test_u8 = black_box(test_u8);
        test_u16 = black_box(test_u16);
        test_u32 = black_box(test_u32);
        test_u64 = black_box(test_u64);

        test_i8 = black_box(test_i8);
        test_i16 = black_box(test_i16);
        test_i32 = black_box(test_i32);
        test_i64 = black_box(test_i64);
 
        test_f32 = black_box(test_f32);
        test_f64 = black_box(test_f64);

        black_box(test_enum1);
        black_box(test_enum2);
        black_box(test_enum3);
        black_box(test_struct);
        black_box(const_);
        test_str = black_box(test_str);
//        test_string = black_box(test_string);
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
        loop {
            continue;
        }
    }


    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }
};



 
fn my_test(val: i16) -> i16 {
    let mut test_val: i16 = val;
    test_val = black_box(test_val);
    unsafe{asm!{"bkpt"};}  //asm::bkpt();
    test_val
}

