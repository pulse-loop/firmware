//! This file contains all the Self information for the Bluetooth LE advertisement
//! and services. It contains the services table.
//!
//! Every value is set in the `Default` implementation.

use esp_idf_sys::{
    esp_attr_control_t, esp_attr_desc_t, esp_ble_addr_type_t_BLE_ADDR_TYPE_PUBLIC,
    esp_ble_adv_channel_t_ADV_CHNL_ALL, esp_ble_adv_data_t,
    esp_ble_adv_filter_t_ADV_FILTER_ALLOW_SCAN_ANY_CON_ANY, esp_ble_adv_params_t,
    esp_ble_adv_type_t_ADV_TYPE_IND, esp_gatts_attr_db_t, ESP_BLE_ADV_FLAG_BREDR_NOT_SPT,
    ESP_BLE_ADV_FLAG_GEN_DISC, ESP_BLE_APPEARANCE_PULSE_OXIMETER_WRIST, ESP_GATT_AUTO_RSP,
    ESP_GATT_PERM_READ, ESP_GATT_UUID_CHAR_DECLARE, ESP_GATT_UUID_DEVICE_INFO_SVC,
    ESP_GATT_UUID_MANU_NAME, ESP_GATT_UUID_PRI_SERVICE, ESP_UUID_LEN_16,
};

#[derive(Clone, Debug)]
#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum AttributeIndex {
    SVC_DeviceInformation,
    CHD_DeviceInformation_ManufacturerNameString,
    CHV_DeviceInformation_ManufacturerNameString,
}

#[derive(Clone)]
/// Global Bluetooth LE Self.
pub struct Configuration {
    /// Data advertised in GAP.
    pub advertising_data: esp_ble_adv_data_t,

    /// Parameters for GAP advertising.
    pub advertising_parameters: esp_ble_adv_params_t,

    /// Data advertised on scan.
    pub scan_response_data: esp_ble_adv_data_t,

    /// Active GATT services.
    pub active_services: Vec<AttributeIndex>,

    /// The GATT server attribute table.
    pub gatt_db: Vec<(AttributeIndex, esp_gatts_attr_db_t)>,
}

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
    /// let id = 0x12345678;
    /// let ptr = esp_uuid_as_u8_ptr(id);
    /// assert_eq!(ptr.as_ref(), &[0x12, 0x34, 0x56, 0x78]);
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

impl Configuration {
    #[allow(clippy::cast_possible_wrap)]
    /// The default appearance for our device is a wrist-worn pulse-oximeter.
    pub const APPEARANCE: i32 = ESP_BLE_APPEARANCE_PULSE_OXIMETER_WRIST as i32;

    /// Slave connection interval, lower boundary.
    pub const MIN_INTERVAL_MS: f32 = 7.5;

    /// Slave connection interval, upper boundary.
    pub const MAX_INTERVAL_MS: f32 = 12.5;

    /// Device name.
    pub const MANUFACTURER_NAME_STRING: &'static str = "pulse.loop";
}

impl Default for Configuration {
    fn default() -> Self {
        let mut mfr_name: String = String::from(Self::MANUFACTURER_NAME_STRING);

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Self {
            advertising_data: esp_ble_adv_data_t {
                set_scan_rsp: false,
                include_name: true,
                include_txpower: true,
                min_interval: ((Self::MIN_INTERVAL_MS / 1.25) as u16).into(),
                max_interval: ((Self::MAX_INTERVAL_MS / 1.25) as u16).into(),
                appearance: Self::APPEARANCE,
                manufacturer_len: 0,
                p_manufacturer_data: std::ptr::null_mut(),
                service_data_len: 0,
                p_service_data: std::ptr::null_mut(),
                service_uuid_len: 0,
                p_service_uuid: std::ptr::null_mut(),
                flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as u8,
            },
            advertising_parameters: esp_ble_adv_params_t {
                adv_int_min: 0x20, // 0x20 * 0.625ms = 20ms
                adv_int_max: 0x40, // 0x40 * 0.625ms = 40ms
                adv_type: esp_ble_adv_type_t_ADV_TYPE_IND,
                own_addr_type: esp_ble_addr_type_t_BLE_ADDR_TYPE_PUBLIC,
                channel_map: esp_ble_adv_channel_t_ADV_CHNL_ALL,
                adv_filter_policy: esp_ble_adv_filter_t_ADV_FILTER_ALLOW_SCAN_ANY_CON_ANY,
                ..Default::default()
            },
            scan_response_data: esp_ble_adv_data_t {
                set_scan_rsp: true,
                include_name: true,
                include_txpower: true,
                min_interval: ((Self::MIN_INTERVAL_MS / 1.25) as u16).into(), // Slave connection min interval, Time = min_interval * 1.25 msec
                max_interval: ((Self::MAX_INTERVAL_MS / 1.25) as u16).into(), // Slave connection max interval, Time = max_interval * 1.25 msec
                appearance: Self::APPEARANCE,
                manufacturer_len: 0,
                p_manufacturer_data: std::ptr::null_mut(),
                service_data_len: 0,
                p_service_data: std::ptr::null_mut(),
                service_uuid_len: 0,
                p_service_uuid: std::ptr::null_mut(),
                flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as u8,
            },
            active_services: vec![AttributeIndex::SVC_DeviceInformation],
            gatt_db: vec![
                // Service Declaration: Device Information
                (
                    AttributeIndex::SVC_DeviceInformation,
                    esp_gatts_attr_db_t {
                        attr_control: esp_attr_control_t {
                            auto_rsp: ESP_GATT_AUTO_RSP as u8,
                        },
                        att_desc: esp_attr_desc_t {
                            uuid_length: ESP_UUID_LEN_16 as u16,
                            uuid_p: Self::esp_uuid_as_u8_ptr(ESP_GATT_UUID_PRI_SERVICE),
                            perm: ESP_GATT_PERM_READ as u16,
                            max_length: std::mem::size_of_val(&ESP_GATT_UUID_DEVICE_INFO_SVC)
                                as u16,
                            length: std::mem::size_of_val(&ESP_GATT_UUID_DEVICE_INFO_SVC) as u16,
                            value: Self::esp_uuid_as_u8_ptr(ESP_GATT_UUID_DEVICE_INFO_SVC),
                        },
                    },
                ),
                // Characteristic Declaration: Manufacturer Name String
                (
                    AttributeIndex::CHD_DeviceInformation_ManufacturerNameString,
                    esp_gatts_attr_db_t {
                        attr_control: esp_attr_control_t {
                            auto_rsp: ESP_GATT_AUTO_RSP as u8,
                        },
                        att_desc: esp_attr_desc_t {
                            uuid_length: ESP_UUID_LEN_16 as u16,
                            uuid_p: Self::esp_uuid_as_u8_ptr(ESP_GATT_UUID_CHAR_DECLARE),
                            perm: ESP_GATT_PERM_READ as u16,
                            max_length: std::mem::size_of_val(&ESP_GATT_UUID_MANU_NAME) as u16,
                            length: std::mem::size_of_val(&ESP_GATT_UUID_MANU_NAME) as u16,
                            value: Self::esp_uuid_as_u8_ptr(ESP_GATT_UUID_MANU_NAME),
                        },
                    },
                ),
                // Characteristic Value: Manufacturer Name String
                (
                    AttributeIndex::CHV_DeviceInformation_ManufacturerNameString,
                    esp_gatts_attr_db_t {
                        attr_control: esp_attr_control_t {
                            auto_rsp: ESP_GATT_AUTO_RSP as u8,
                        },
                        att_desc: esp_attr_desc_t {
                            uuid_length: ESP_UUID_LEN_16 as u16,
                            uuid_p: Self::esp_uuid_as_u8_ptr(ESP_GATT_UUID_MANU_NAME),
                            perm: ESP_GATT_PERM_READ as u16,
                            max_length: mfr_name.len() as u16,
                            length: mfr_name.len() as u16,
                            value: mfr_name.as_mut_ptr(),
                        },
                    },
                ),
            ],
        }
    }
}
