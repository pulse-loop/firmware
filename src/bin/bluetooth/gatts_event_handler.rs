use crate::bluetooth::configuration::{AttributeIndex, Configuration};
use esp_idf_sys::{
    esp_ble_gap_config_adv_data, esp_ble_gap_set_device_name, esp_ble_gatts_cb_param_t,
    esp_ble_gatts_create_attr_tab, esp_ble_gatts_start_service, esp_gatt_if_t,
    esp_gatt_status_t_ESP_GATT_OK, esp_gatts_cb_event_t,
    esp_gatts_cb_event_t_ESP_GATTS_CREAT_ATTR_TAB_EVT, esp_gatts_cb_event_t_ESP_GATTS_REG_EVT,
    esp_nofail, ESP_GATT_UUID_PRI_SERVICE,
};
use std::ffi::CString;
use std::slice;

/// GATT server event handler.
///
/// # Arguments
///
/// * `event`: Event identifier.
/// * `gatts_if`: The interface that received the event.
/// * `param`: Generic event parameters.
///
/// returns: ()
pub unsafe extern "C" fn gatts_event_handler(
    event: esp_gatts_cb_event_t,
    gatts_if: esp_gatt_if_t,
    param: *mut esp_ble_gatts_cb_param_t,
) {
    #[allow(non_upper_case_globals)]
    match event {
        esp_gatts_cb_event_t_ESP_GATTS_REG_EVT => {
            log::info!("Handling registration event.");

            let mut configuration = Configuration::default();
            let mfr_name = CString::new(Configuration::MANUFACTURER_NAME_STRING).unwrap();
            esp_nofail!(esp_ble_gap_set_device_name(mfr_name.as_ptr()));

            log::info!("Manufacturer name string set.");

            esp_nofail!(esp_ble_gap_config_adv_data(
                &mut configuration.advertising_data
            ));

            log::info!("Advertising data set.");

            esp_nofail!(esp_ble_gap_config_adv_data(
                &mut configuration.scan_response_data
            ));

            log::info!("Scan response data set.");

            #[allow(clippy::cast_possible_truncation)]
            {
                let attr_tab_without_indices = configuration
                    .gatt_db
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>();

                esp_nofail!(esp_ble_gatts_create_attr_tab(
                    attr_tab_without_indices.as_ptr(),
                    gatts_if,
                    attr_tab_without_indices.len() as u8,
                    0,
                ));
            }
        }
        esp_gatts_cb_event_t_ESP_GATTS_CREAT_ATTR_TAB_EVT => {
            let configuration = Configuration::default();

            #[allow(clippy::cast_possible_truncation)]
            if (*param).add_attr_tab.status != esp_gatt_status_t_ESP_GATT_OK {
                log::error!(
                    "Attribute table creation failed, error {}.",
                    (*param).add_attr_tab.status
                );
                // TODO: Panic.
            } else if (*param).add_attr_tab.num_handle != configuration.gatt_db.len() as u16 {
                log::error!(
                    "Attribute table created with wrong handle {} (instead of {}).",
                    (*param).add_attr_tab.num_handle,
                    configuration.gatt_db.len()
                );
                // TODO: Panic.
            } else {
                log::info!(
                    "Attribute table successfully created, handle {}.",
                    (*param).add_attr_tab.num_handle
                );

                let handles = slice::from_raw_parts(
                    (*param).add_attr_tab.handles,
                    configuration.gatt_db.len(),
                );

                // TODO: Fix zero handles.

                log::info!("Handles: {:?}", handles);

                for index in configuration.active_services {
                    // let handle = handles[index.clone() as usize];
                    // log::info!("Starting service {:?} with handle {}.", index, handle);
                    esp_nofail!(esp_ble_gatts_start_service(index as u16));
                }
            }
        }
        _ => {
            log::warn!("Unhandled event #{}.", event);
        }
    }
}
