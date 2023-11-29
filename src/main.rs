#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;
use rtt_target::rprintln;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let raw_channels = rtt_target::rtt_init! {
        up: {
            0: {
                size: 4096
                mode: BlockIfFull
                name: "Terminal"
            }
        }
    };
    rtt_target::set_print_channel(raw_channels.up.0);

    let _p = embassy_stm32::init(Default::default());
    rprintln!("Hello World!");

    for i in 0..usize::MAX {
        rprintln!("Tick {}", i);
        Timer::after_millis(1000).await;
    }
}
