// src/config.rs
pub struct BoardConfig {
    pub led_pins: [u8; 4],
    pub button_pin: u8,
    pub serial_tx_pin: u8,
    pub serial_rx_pin: u8,
}

impl BoardConfig {
    pub const fn default() -> Self {
        Self {
            led_pins: [2, 3, 4, 5],
            button_pin: 9,
            serial_tx_pin: 21,
            serial_rx_pin: 20,
        }
    }
}