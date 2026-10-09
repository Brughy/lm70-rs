use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
use lm70::{Error, Lm70};

fn read_expectation(bytes: [u8; 2]) -> [SpiTransaction<u8>; 3] {
    [
        SpiTransaction::transaction_start(),
        SpiTransaction::read_vec(bytes.to_vec()),
        SpiTransaction::transaction_end(),
    ]
}

fn configuration_expectation(bytes: [u8; 2]) -> [SpiTransaction<u8>; 4] {
    [
        SpiTransaction::transaction_start(),
        SpiTransaction::read_vec(vec![0x00, 0x00]),
        SpiTransaction::write_vec(bytes.to_vec()),
        SpiTransaction::transaction_end(),
    ]
}

fn finish(sensor: Lm70<SpiMock<u8>>) {
    sensor.destroy().done();
}

#[test]
fn creates_and_destroys_driver() {
    finish(Lm70::new(SpiMock::new(&[])));
}

#[test]
fn reads_positive_temperature() {
    let mut sensor = Lm70::new(SpiMock::new(&read_expectation([0x0c, 0x9c])));
    assert_eq!(sensor.read_temperature().unwrap(), 25.0);
    finish(sensor);
}

#[test]
fn reads_negative_temperature() {
    let mut sensor = Lm70::new(SpiMock::new(&read_expectation([0xff, 0xfc])));
    assert_eq!(sensor.read_temperature().unwrap(), -0.25);
    finish(sensor);
}

#[test]
fn rejects_non_temperature_status_bits() {
    let mut sensor = Lm70::new(SpiMock::new(&read_expectation([0x80, 0x00])));
    assert_eq!(sensor.read_temperature(), Err(Error::InvalidData(0x8000)));
    finish(sensor);
}

#[test]
fn enters_shutdown_mode() {
    let mut sensor = Lm70::new(SpiMock::new(&configuration_expectation([0x00, 0xff])));
    sensor.shutdown().unwrap();
    finish(sensor);
}

#[test]
fn resumes_continuous_conversion() {
    let mut sensor = Lm70::new(SpiMock::new(&configuration_expectation([0x00, 0x00])));
    sensor.continuous().unwrap();
    finish(sensor);
}

#[test]
fn reads_manufacturer_id_in_shutdown_mode() {
    let mut sensor = Lm70::new(SpiMock::new(&read_expectation([0x80, 0x03])));
    assert_eq!(sensor.read_manufacturer_id().unwrap(), 0x400);
    finish(sensor);
}

#[test]
fn rejects_temperature_as_manufacturer_id() {
    let mut sensor = Lm70::new(SpiMock::new(&read_expectation([0x0c, 0x9c])));
    assert_eq!(
        sensor.read_manufacturer_id(),
        Err(Error::InvalidData(0x0c9c))
    );
    finish(sensor);
}
