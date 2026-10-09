//! Value conversions.

/// Converts the signed 11-bit temperature field to degrees Celsius.
pub(crate) fn convert_temperature(raw: u16) -> f32 {
    f32::from((raw as i16) >> 5) * 0.25
}

/// Returns whether D4-D2 identify a temperature register value.
pub(crate) fn is_temperature(raw: u16) -> bool {
    raw & 0x1c == 0x1c
}

/// Extracts the 11-bit manufacturer/device ID field.
pub(crate) fn convert_manufacturer_id(raw: u16) -> u16 {
    raw >> 5
}

/// Returns whether D4-D2 identify a manufacturer/device ID value.
pub(crate) fn is_manufacturer_id(raw: u16) -> bool {
    raw & 0x1c == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn converts_positive_and_negative_temperatures() {
        assert_eq!(convert_temperature(0x001c), 0.0);
        assert_eq!(convert_temperature(0x003c), 0.25);
        assert_eq!(convert_temperature(0x0c9c), 25.0);
        assert_eq!(convert_temperature(0x7ffc), 255.75);
        assert_eq!(convert_temperature(0xfffc), -0.25);
        assert_eq!(convert_temperature(0xe71c), -50.0);
        assert_eq!(convert_temperature(0x8000), -256.0);
    }

    #[test]
    fn identifies_temperature_data() {
        assert!(is_temperature(0x0c9c));
        assert!(is_temperature(0x0c9f));
        assert!(!is_temperature(0x0c80));
    }

    #[test]
    fn identifies_and_converts_manufacturer_id() {
        assert!(is_manufacturer_id(0x8000));
        assert!(is_manufacturer_id(0x8003));
        assert!(!is_manufacturer_id(0x801c));
        assert_eq!(convert_manufacturer_id(0x8003), 0x400);
    }
}
