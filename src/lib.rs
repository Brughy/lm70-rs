#![deny(missing_docs, unsafe_code)]
#![no_std]

//! Platform-independent driver for the LM70 temperature sensor.
//!
//! The driver uses an [`embedded_hal::spi::SpiDevice`], which owns chip-select
//! handling for each LM70 transfer.

/// All errors returned by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
    /// SPI device error.
    Spi(E),
    /// The LM70 status bits do not identify the expected register.
    InvalidData(u16),
}

/// LM70 temperature sensor driver.
#[derive(Debug)]
pub struct Lm70<SPI> {
    spi: SPI,
}

mod conversion;
mod lm70;
