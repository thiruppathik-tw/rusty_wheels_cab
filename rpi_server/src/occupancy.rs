use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use rppal::gpio::Error as OccupancyError;
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering.
// BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

// BCM GPIO 16 is tied to physical pin 36.
const GPIO_PIR: u8 = 16;

static pir_status: AtomicBool = AtomicBool::new(true);

pub fn device_info() -> String {
    let d = DeviceInfo::new();
    if d.is_ok() {
        let device = d.unwrap().model().to_string();
        println!("Device : {}", device);
        return device;
    } else {
        println!("Error: {}", d.unwrap_err());
        return "UnknownDevice".to_string();
    }
}

pub fn occupancy_status() -> Result<bool, OccupancyError> {
    todo!(); //read and return the occupancy status
}

pub fn update_led(status: bool) {
    pir_status.store(status, Ordering::SeqCst);
}

pub(crate) fn init_led() {
    thread::spawn(|| {
        // Set pin 23 as output pin
        let mut pin_led = Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();
        loop {
            update_pir_status();
            if pir_status.load(Ordering::SeqCst) {
                pin_led.set_high();
            } else {
                pin_led.set_low()
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}

fn update_pir_status() {
    todo!(); //read and update the PIR status
}
