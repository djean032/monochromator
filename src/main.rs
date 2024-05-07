mod stepper;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let per = Peripherals::take()?;

    let mut grating_motor = stepper::Stepper::new(
        per.pins.gpio22,
        per.pins.gpio23,
        per.pins.gpio32,
        per.pins.gpio33,
        1., //TODO determine this value, experimentally
        0,
        50,
        0,
    );
    loop {
        grating_motor.step_backward()?;
        esp_idf_svc::log::EspLogger::initialize_default();

        log::info!("{}", grating_motor.step_counter);
    }

}

// Functions that I don't know where else to put.
fn scan(
    current_position: &i32,
    start_wavelength: i32,
    end_wavelength: i32,
    mut motor: stepper::Stepper,
) -> anyhow::Result<()> {
    let mut start_dif = step_dif(current_position, &start_wavelength, &motor);
    let mut delta = step_dif(&start_wavelength, &end_wavelength, &motor);
    while start_dif != 0 {
        if start_dif > 0 {
            motor.step_backward()?;
            start_dif -= 1;
        } else {
            motor.step_forward()?;
            start_dif += 1;
        }
    }
    while delta != 0 {
        motor.step_forward()?;
        delta += 1;
    }
    Ok(())
}

fn move_to(
    current_position: &i32,
    requested_wavelength: i32,
    mut motor: stepper::Stepper,
) -> anyhow::Result<()> {
    if *current_position != requested_wavelength {
        let mut dif = step_dif(current_position, &requested_wavelength, &motor);
        if dif > 0 {
            while dif != 0 {
                motor.step_backward()?;
                dif -= 1;
            }
        } else {
            while dif != 0 {
                motor.step_forward()?;
                dif += 1;
            }
        }
    }
    Ok(())
}

fn step_dif(current_position: &i32, wavelength: &i32, motor: &stepper::Stepper) -> i32 {
    if current_position == wavelength {
        0
    } else {
        let difference = wavelength - current_position;
        (difference as f32 / motor.step_size) as i32
    }
}
