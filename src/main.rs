mod stepper;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

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

        log::info!("{}", grating_motor.step_counter);
    }
}
