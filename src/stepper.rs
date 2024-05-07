use esp_idf_hal::gpio::*;
use esp_idf_hal::delay::FreeRtos;
//TODO implement step_size
pub struct Stepper {
    pub in1: PinDriver<'static, AnyIOPin, Output>,
    pub in2: PinDriver<'static, AnyIOPin, Output>,
    pub in3: PinDriver<'static, AnyIOPin, Output>,
    pub in4: PinDriver<'static, AnyIOPin, Output>,
}

impl Stepper {
    pub fn new(
        pin1: impl IOPin + 'static,
        pin2: impl IOPin + 'static,
        pin3: impl IOPin + 'static,
        pin4: impl IOPin + 'static,
    ) -> Stepper {
        let pin1 = pin1.downgrade();
        let in1 = PinDriver::output(pin1).unwrap();
        let pin2 = pin2.downgrade();
        let in2 = PinDriver::output(pin2).unwrap();
        let pin3 = pin3.downgrade();
        let in3 = PinDriver::output(pin3).unwrap();
        let pin4 = pin4.downgrade();
        let in4 = PinDriver::output(pin4).unwrap();
        Stepper { in1, in2, in3, in4 }
    }

    pub fn step_forward (&mut self) -> anyhow::Result<()> {
        self.in1.set_high()?;
        FreeRtos::delay_ms(300);
        self.in1.set_low()?;
        FreeRtos::delay_ms(300);
        Ok(())
    }
    
    pub fn step_backward (&mut self) -> anyhow::Result<()> {
        self.in3.set_high()?;
        FreeRtos::delay_ms(300);
        self.in3.set_low()?;
        FreeRtos::delay_ms(300);
        Ok(())
    }
}
