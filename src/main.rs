mod grating;
mod slit;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let per = Peripherals::take()?;

    let mut grating_motor = grating::Grating::new(
        per.pins.gpio22,
        per.pins.gpio23,
        per.pins.gpio32,
        per.pins.gpio33,
        1., //TODO determine this value, experimentally
        0,
        50,
        0,
        400.,
    );
//    let mut grating_motor2 = grating::Grating::new(pin1, pin2, pin3, pin4, step_size, current_position, delay_ms, step_counter, offset)

    let mut slit_motor = slit::Slit::new(pin1, pin2, pin3, pin4, step_size, 0, 50, 0);


    loop {
        grating_motor.step_backward()?;

        log::info!("{}", grating_motor.step_counter);
    }
}
