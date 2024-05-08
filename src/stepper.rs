use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;

// TODO Add in current position as struct field
pub struct Stepper {
    pub in1: PinDriver<'static, AnyIOPin, Output>,
    pub in2: PinDriver<'static, AnyIOPin, Output>,
    pub in3: PinDriver<'static, AnyIOPin, Output>,
    pub in4: PinDriver<'static, AnyIOPin, Output>,
    pub step_size: f32,
    pub current_position: i32,
    pub delay_ms: u32,
    pub step_counter: i32,
}

impl Stepper {
    pub fn new(
        pin1: impl IOPin + 'static,
        pin2: impl IOPin + 'static,
        pin3: impl IOPin + 'static,
        pin4: impl IOPin + 'static,
        step_size: f32,
        current_position: i32,
        delay_ms: u32,
        step_counter: i32,
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
            current_position,
            delay_ms,
            step_counter,
        }
    }
    // TODO Implement current position adjustment in our step funcs
    pub fn step_forward(&mut self) -> anyhow::Result<()> {
        self.step_counter += 1;
        self.current_position += 1;
        self.step_num(self.step_counter)?;
        FreeRtos::delay_ms(self.delay_ms);
        Ok(())
    }

    pub fn step_backward(&mut self) -> anyhow::Result<()> {
        self.step_counter -= 1;
        self.current_position -= 1;
        self.step_num(self.step_counter)?;
        FreeRtos::delay_ms(self.delay_ms);
        Ok(())
    }
    fn step_num(&mut self, step_counter: i32) -> anyhow::Result<()> {
        let step_num = step_counter.rem_euclid(4);
        match step_num {
            0 => self.step_one(),
            1 => self.step_two(),
            2 => self.step_three(),
            3 => self.step_four(),
            _ => Ok(()),
        }
    }

    fn step_one(&mut self) -> anyhow::Result<()> {
        self.in1.set_high()?;
        self.in2.set_low()?;
        self.in3.set_high()?;
        self.in4.set_low()?;
        Ok(())
    }
    fn step_two(&mut self) -> anyhow::Result<()> {
        self.in1.set_low()?;
        self.in2.set_high()?;
        self.in3.set_high()?;
        self.in4.set_low()?;
        Ok(())
    }
    fn step_three(&mut self) -> anyhow::Result<()> {
        self.in1.set_low()?;
        self.in2.set_high()?;
        self.in3.set_low()?;
        self.in4.set_high()?;
        Ok(())
    }
    fn step_four(&mut self) -> anyhow::Result<()> {
        self.in1.set_high()?;
        self.in2.set_low()?;
        self.in3.set_low()?;
        self.in4.set_high()?;
        Ok(())
    }
}
