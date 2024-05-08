use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;

// TODO Add in current position as struct field
pub struct Slit {
    pub in1: PinDriver<'static, AnyIOPin, Output>,
    pub in2: PinDriver<'static, AnyIOPin, Output>,
    pub in3: PinDriver<'static, AnyIOPin, Output>,
    pub in4: PinDriver<'static, AnyIOPin, Output>,
    pub step_size: f32,
    pub current_position: i32,
    pub delay_ms: u32,
    pub step_counter: i32,
}

#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
impl Slit {
    pub fn new(
        pin1: impl IOPin + 'static,
        pin2: impl IOPin + 'static,
        pin3: impl IOPin + 'static,
        pin4: impl IOPin + 'static,
        step_size: f32,
        current_position: i32,
        delay_ms: u32,
        step_counter: i32,
    ) -> Slit {
        let pin1 = pin1.downgrade();
        let in1 = PinDriver::output(pin1).unwrap();
        let pin2 = pin2.downgrade();
        let in2 = PinDriver::output(pin2).unwrap();
        let pin3 = pin3.downgrade();
        let in3 = PinDriver::output(pin3).unwrap();
        let pin4 = pin4.downgrade();
        let in4 = PinDriver::output(pin4).unwrap();
        Slit {
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

    #[allow(dead_code)]
    pub fn zero_slit(&mut self, zero_sense: bool) -> anyhow::Result<bool> {
        while zero_sense == false {
            self.step_backward()?;
        }
        Ok(true)
    }

    #[allow(dead_code)]
    pub fn set_width(&mut self, width: f32) -> anyhow::Result<()> {
        let position = (width * self.step_size) as i32;
        if position == self.current_position {
        } else {
            let dif = self.current_position - position;
            self.step(-dif)?;
        }
        Ok(())
    }

    pub fn step(&mut self, mut steps: i32) -> anyhow::Result<()> {
        if steps < 0 {
            while steps != 0 {
                self.step_backward()?;
                steps += 1;
            }
        } else {
            while steps != 0 {
                self.step_forward()?;
                steps -= 1;
            }
        }
        Ok(())
    }

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
            0 => self.step_one()?,
            1 => self.step_two()?,
            2 => self.step_three()?,
            3 => self.step_four()?,
            _ => (),
        }
        Ok(())
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
