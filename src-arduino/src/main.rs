#![no_std]
#![no_main]

use panic_halt as _;

// see https://github.com/Rahix/avr-hal/tree/main/examples
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    let mut led = pins.d2.into_output();

    ufmt::uwriteln!(&mut serial, "Hello world\r").unwrap();


    loop {
        let b = serial.read_byte();
        ufmt::uwriteln!(&mut serial, "{}\n", b);


        match(b) {
            49 => if led.is_set_low() {
                led.set_high()
            },
            48 =>  if led.is_set_high() {
                led.set_low()
            },
            _ => { }
        }

        arduino_hal::delay_ms(1000);
    }
}
