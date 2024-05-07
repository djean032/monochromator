use core::f32;
use core::i32;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;

pub struct Stepper {
    pub in1: PinDriver<'static, AnyIOPin, Output>,
    pub in2: PinDriver<'static, AnyIOPin, Output>,
    pub in3: PinDriver<'static, AnyIOPin, Output>,
    pub in4: PinDriver<'static, AnyIOPin, Output>,
    pub step_size: f32,
}

impl Stepper {
    pub fn new(
        pin1: impl IOPin + 'static,
        pin2: impl IOPin + 'static,
        pin3: impl IOPin + 'static,
        pin4: impl IOPin + 'static,
        step_size: f32,
    ) -> Stepper {
        let pin1 = pin1.downgrade();
        let in1 = PinDriver::output(pin1).unwrap();
        let pin2 = pin2.downgrade();
        let in2 = PinDriver::output(pin2).unwrap();
        let pin3 = pin3.downgrade();
        let in3 = PinDriver::output(pin3).unwrap();
        let pin4 = pin4.downgrade();
        let in4 = PinDriver::output(pin4).unwrap();
        Stepper {
            in1,
            in2,
            in3,
            in4,
            step_size,
        }
    }
    pub fn man_step(&self, wavelength_requested: f32, current_position: f32) {
        let distance = current_position - wavelength_requested;
        let mut direction = true;
        if distance < 0. {
            direction = false
        } else {
        }
        let steps = (distance / self.step_size) as i32;
        if steps > 0 {
            for i in [1..steps + 1] {
                self.in1.set_high();
                FreeRtos::delay_ms(300);
                self.in1.set_low();
            }
        } else {
            let abs_steps = steps.abs();
            for i in [1..steps + 1] {
                self.in3.set_high();
                FreeRtos::delay_ms(300);
                self.in3.set_low();
            }
        }
    }
    /*
        fn get_pin1(self) -> PinDriver<'static, AnyIOPin, Output> {
            self.in1
        }
        fn get_pin2(self) -> PinDriver<'static, AnyIOPin, Output> {
            self.in2
        }
        fn get_pin3(self) -> PinDriver<'static, AnyIOPin, Output> {
            self.in3
        }
        fn get_pin4(self) -> PinDriver<'static, AnyIOPin, Output> {
            self.in4
        }
    */
}
