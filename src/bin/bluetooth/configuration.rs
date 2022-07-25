//! This file contains all the Self information for the Bluetooth LE advertisement
//! and services. It contains the services table.
//!
//! Every value is set in the `Default` implementation.

use std::slice;

use esp_idf_sys::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_CONFIGURATION: Configuration = Configuration::default();
}

#[derive(Clone, Debug, PartialEq)]
#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum AttributeIndex {
    SVC_DeviceInformation = 1,
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

    /// GATT database.
    pub gatt_db: Vec<(AttributeIndex, esp_gatts_attr_db_t)>,

    /// All the services in the database.
    pub services: Vec<(AttributeIndex, esp_gatts_attr_db_t)>,
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

    /// Base BLE UUID.
    const BLE_BASE_UUID: [u8; 16] = [
        0xfb, 0x34, 0x9b, 0x5f, 0x80, 0x00, 0x00, 0x80, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
}

/// Configuration raw pointers require to be mut and can't be safely shared across threads.
/// This is a workaround to make them Send-able, because we know that we're not modifying them.
unsafe impl Send for Configuration {}
unsafe impl Sync for Configuration {}

impl Default for Configuration {
    fn default() -> Self {
        // GATT server database.
        let gatt_db: Vec<(AttributeIndex, esp_gatts_attr_db_t)> = {
            vec![
                // Service Declaration: Device Information
                (
                    AttributeIndex::SVC_DeviceInformation,
                    esp_gatts_attr_db_t {
                        attr_control: esp_attr_control_t {
                            auto_rsp: ESP_GATT_AUTO_RSP as u8,
                        },
                        att_desc: esp_attr_desc_t {
                            uuid_length: ESP_UUID_LEN_16 as u16,
                            uuid_p: leaky_box_be_bytes!(ESP_GATT_UUID_PRI_SERVICE as u16),
                            perm: ESP_GATT_PERM_READ as u16,
                            max_length: ESP_UUID_LEN_16 as u16,
                            length: ESP_UUID_LEN_16 as u16,
                            value: leaky_box_be_bytes!(ESP_GATT_UUID_DEVICE_INFO_SVC as u16),
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
                            uuid_p: leaky_box_be_bytes!(ESP_GATT_UUID_CHAR_DECLARE as u16),
                            perm: ESP_GATT_PERM_READ as u16,
                            max_length: ESP_UUID_LEN_16 as u16,
                            length: ESP_UUID_LEN_16 as u16,
                            value: leaky_box_be_bytes!(ESP_GATT_CHAR_PROP_BIT_READ as u16),
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
                            uuid_p: leaky_box_be_bytes!(ESP_GATT_UUID_MANU_NAME as u16),
                            perm: ESP_GATT_PERM_READ as u16,
                            max_length: String::from(Self::MANUFACTURER_NAME_STRING).len() as u16,
                            length: String::from(Self::MANUFACTURER_NAME_STRING).len() as u16,
                            value: leaky_box_u8!(Self::MANUFACTURER_NAME_STRING),
                        },
                    },
                ),
            ]
        };

        log::info!("Creating default BLE configuration.");

        // Discover all the services in the GATT database.
        let services: Vec<(AttributeIndex, esp_gatts_attr_db_t)> = gatt_db
            .iter()
            .filter(|(i, attr)| unsafe {
                // Get the UUID from the attribute table.
                let attribute_uuid = slice::from_raw_parts(attr.att_desc.uuid_p, 2);

                // Let's keep this slice in scope during comparisons.
                let pri_slice = (ESP_GATT_UUID_PRI_SERVICE as u16).to_be_bytes().clone();
                let pri_service_uuid = slice::from_raw_parts(pri_slice.as_ptr(), 2);

                // Compare.
                attribute_uuid == pri_service_uuid
            })
            .map(|(i, attr)| (i.clone(), attr.clone()))
            .collect::<Vec<_>>();

        log::info!(
            "Discovered {} services: {:?}.",
            services.len(),
            services.iter().map(|(i, _)| i).collect::<Vec<_>>()
        );

        // For each service, obtain its UUID information.
        let normalised_uuids = services
            .iter()
            .map(|(i, attr)| unsafe {
                slice::from_raw_parts(attr.att_desc.value, attr.att_desc.length as usize)
            })
            .map(|uuid_slice| {
                let mut normalised_uuid: [u8; 16] = Self::BLE_BASE_UUID;

                match uuid_slice.len() as u32 {
                    ESP_UUID_LEN_16 => {
                        normalised_uuid[12] = uuid_slice[1];
                        normalised_uuid[13] = uuid_slice[0];
                    }
                    ESP_UUID_LEN_32 => {
                        normalised_uuid[12] = uuid_slice[3];
                        normalised_uuid[13] = uuid_slice[2];
                        normalised_uuid[14] = uuid_slice[1];
                        normalised_uuid[15] = uuid_slice[0];
                    }
                    ESP_UUID_LEN_128 => unsafe {
                        normalised_uuid.copy_from_slice(&uuid_slice);
                    },
                    _ => {}
                }

                normalised_uuid
            })
            .collect::<Vec<_>>();

        log::info!(
            "The UUIDs of the discovered services are: {:?}.",
            normalised_uuids
                .iter()
                .map(|u| u.to_vec())
                .collect::<Vec<_>>()
        );

        let mut normalised_uuids_data: Vec<u8> = vec![];
        for uuid in normalised_uuids {
            normalised_uuids_data.append(&mut uuid.to_vec());
        }

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
                service_uuid_len: normalised_uuids_data.len() as u16,
                p_service_uuid: leaky_box_u8!(normalised_uuids_data.clone()),
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
                service_uuid_len: normalised_uuids_data.len() as u16,
                p_service_uuid: leaky_box_u8!(normalised_uuids_data),
                flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as u8,
            },
            gatt_db,
            services,
        }
    }
}
