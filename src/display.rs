use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use mipidsi::Display;
use mipidsi::options::{Orientation, Rotation};
use mipidsi::{Builder, interface::SpiInterface, models::ST7789, options::ColorInversion};
use rppal::gpio::OutputPin;
use rppal::hal::Delay;
use rppal::{
    gpio::Gpio,
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use static_cell::StaticCell;
#[cfg(feature = "tracing")]
use tracing::debug;

// Pins
const BL: u8 = 24;
const DC: u8 = 25;
const RST: u8 = 27;

// Display
const WIDTH: u16 = 240;
const HEIGHT: u16 = 240;

static BUFFER: StaticCell<[u8; 512]> = StaticCell::new();

pub type WaveshareDisplay = Display<
    SpiInterface<'static, ExclusiveDevice<Spi, NoCs, NoDelay>, OutputPin>,
    ST7789,
    OutputPin,
>;

pub fn setup() -> (WaveshareDisplay, OutputPin) {
    #[cfg(feature = "tracing")]
    debug!("Setting up gpio pins for display");
    let mut delay = Delay::new();
    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(DC).unwrap().into_output();
    let rst = {
        let mut rst = gpio.get(RST).unwrap().into_output();
        rst.set_low();
        delay.delay_ms(20);
        rst.set_high();
        delay.delay_ms(120);
        rst
    };
    let backlight = gpio.get(BL).unwrap().into_output();

    // SPI Display
    #[cfg(feature = "tracing")]
    debug!("Setting up SPI");
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 60_000_000_u32, Mode::Mode0).unwrap();
    #[cfg(feature = "tracing")]
    debug!("Setting up SPI device");
    let spi_device = ExclusiveDevice::new_no_delay(spi, NoCs).unwrap();
    let buffer: &'static mut [u8; 512] = BUFFER.init([0u8; 512]);
    let di = SpiInterface::new(spi_device, dc, buffer);
    #[cfg(feature = "tracing")]
    debug!("Building internal display");
    let mut display = Builder::new(ST7789, di)
        .display_size(WIDTH, HEIGHT)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();
    display
        .set_orientation(Orientation::default().rotate(Rotation::Deg90))
        .unwrap();
    #[cfg(feature = "tracing")]
    debug!("Finished setting up display");
    (display, backlight)
}

/// Noop `OutputPin` implementation.
///
/// This is passed to `ExclusiveDevice`, because the CS pin is handle in
/// hardware.
pub struct NoCs;

impl embedded_hal::digital::OutputPin for NoCs {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl embedded_hal::digital::ErrorType for NoCs {
    type Error = core::convert::Infallible;
}
