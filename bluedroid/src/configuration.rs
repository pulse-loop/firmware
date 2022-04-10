//! This file contains all the configuration information for the Bluetooth LE advertisement
//! and services. It contains the services table.
//!
//! Every value is set in the `Default` implementation.

use esp_idf_sys::*;

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
/// Global Bluetooth LE configuration.
pub struct Configuration {
    /// Data advertised before the connection.
    pub advertising_data: esp_ble_adv_data_t,

    /// Data advertised on scan.
    pub scan_response_data: esp_ble_adv_data_t,

    /// A list of possible callbacks from the GATT server.
    pub gatts_event_callbacks: super::gatts_event_handler::GATTSEventCallbacks,

    /// The GATT server attribute table.
    pub gatt_db: Vec<esp_gatts_attr_db_t>,
}

unsafe impl Send for Configuration {}

impl Configuration {
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Converts an unsigned integer to a `u8` array.
    ///
    /// # Arguments
    ///
    /// * `val`: The unsigned integer.
    ///
    /// returns: *mut u8
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 0x12345678;
    /// let mut buf = [0u8; 4];
    /// let ptr = BluetoothConfiguration::u32_to_u8_array(x);
    /// let slice = unsafe { std::slice::from_raw_parts(ptr, 4) };
    /// assert_eq!(slice, &[0x12, 0x34, 0x56, 0x78]);
    /// ```
    pub fn esp_uuid_as_u8_ptr(val: u32) -> *mut u8 {
        // Convert the value to a u8 array.
        let mut u8_array: [u8; 4] = [0; 4];
        u8_array[0] = (val >> 24) as u8;
        u8_array[1] = (val >> 16) as u8;
        u8_array[2] = (val >> 8) as u8;
        u8_array[3] = val as u8;

        // Return the pointer to the array.
        u8_array.as_mut_ptr()
    }
}
