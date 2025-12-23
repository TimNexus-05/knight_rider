#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output, OutputConfig},
    main,
    uart::{self, Uart},
    Config,
};

// Add UART for serial output
use core::fmt::Write;

#[main]
fn main() -> ! {
    let config = Config::default();
    let peripherals = esp_hal::init(config);

    let _io = Io::new(peripherals.IO_MUX);
    let delay = Delay::new();

    // Initialize UART for serial output
    let uart_config = uart::Config::default();
    let mut uart0 = Uart::new(peripherals.UART0, uart_config).unwrap();

    let _ = writeln!(uart0, "=== Knight Rider Scanner Starting ===");
    let _ = writeln!(uart0, "Initializing LED pins...");
    
    // Configure LED pins as outputs
    let mut led0 = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let mut led1 = Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default());
    let mut led2 = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    let mut led3 = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());

    let _ = writeln!(uart0, "LEDs initialized: GPIO 2, 3, 4, 5");
    let _ = writeln!(uart0, "Starting scanner pattern...");
    let _ = writeln!(uart0, "");

    let mut cycle_count: u32 = 0;
    let leds = [&mut led0, &mut led1, &mut led2, &mut led3];

    loop {
        cycle_count += 1;
        
        // Start of cycle
        if cycle_count == 1 {
            let _ = writeln!(uart0, "🎯 Cycle #{}", cycle_count);
            let _ = writeln!(uart0, "   Moving: ▶️  Left to Right");
        } else if cycle_count % 5 == 0 {
            let _ = writeln!(uart0, "🎯 Cycle #{}", cycle_count);
            let _ = writeln!(uart0, "   Moving: ▶️  Left to Right");
        }

        // Knight Rider effect: left to right
        for i in 0..leds.len() {
            let _ = writeln!(uart0, "   LED {} ON", i+1);
            leds[i].set_high();
            if i > 0 {
                let _ = writeln!(uart0, "   LED {} OFF", i);
                leds[i - 1].set_low();
            }
            delay.delay_millis(100u32);
        }

        // Turn off the last one
        leds[leds.len() - 1].set_low();
        let _ = writeln!(uart0, "   LED 4 OFF");
        
        // Brief pause at the right end
        let _ = writeln!(uart0, "   ⏸️  Pause at right end");
        delay.delay_millis(200u32);

        if cycle_count % 5 == 0 {
            let _ = writeln!(uart0, "   Moving: ◀️  Right to Left");
        }

        // Right to left
        for i in (0..leds.len()).rev() {
            let _ = writeln!(uart0, "   LED {} ON", i+1);
            leds[i].set_high();
            if i < leds.len() - 1 {
                let _ = writeln!(uart0, "   LED {} OFF", i+2);
                leds[i + 1].set_low();
            }
            delay.delay_millis(100u32);
        }

        // Turn off the first one
        leds[0].set_low();
        let _ = writeln!(uart0, "   LED 1 OFF");
        
        // Brief pause at the left end
        let _ = writeln!(uart0, "   ⏸️  Pause at left end");
        let _ = writeln!(uart0, "");
        delay.delay_millis(200u32);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}