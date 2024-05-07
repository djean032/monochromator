mod stepper;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    let per = Peripherals::take()?;

    let mut grating_motor = stepper::Stepper::new(
        per.pins.gpio1,
        per.pins.gpio2,
        per.pins.gpio3,
        per.pins.gpio4,
    );

    loop {
        grating_motor.step_forward()?;
        grating_motor.step_backward()?;
    }
}

fn scan(
    current_position: i32,
    start_wavelength: i32,
    end_wavelength: i32,
    mut motor: stepper::Stepper,
) -> anyhow::Result<()> {
    if current_position == start_wavelength {
        let difference = end_wavelength - start_wavelength;
        // TODO
        let mut steps = difference / motor.step_size;

        while number != 0 {
            motor.step_forward()?;
            number += 1;
        }
    } else {
        let mut number = current_position - start_wavelength;
        if number > 0 {
            while number != start_wavelength {
                motor.step_backward()?;
                number -= 1;
            }

        } else {
            while number != start_wavelength {
                motor.step_forward()?;
                number += 1;
            }
        }
    }
    Ok(())
}
