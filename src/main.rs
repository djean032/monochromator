mod stepper;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;




fn main() ->  anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    
    let per = Peripherals::take()?;

    let mut grating_motor = stepper::Stepper::new(per.pins.gpio1, per.pins.gpio2, per.pins.gpio3, per.pins.gpio4);

    loop {
        grating_motor.in2.set_high()?;
        FreeRtos::delay_ms(1000);
        grating_motor.in2.set_low()?;
        FreeRtos::delay_ms(1000);
    }

}
