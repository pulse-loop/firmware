use crate::bluetooth::configuration::{AttributeIndex, Configuration, GLOBAL_CONFIGURATION};
use esp_idf_sys::*;
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

            let mfr_name = CString::new(Configuration::MANUFACTURER_NAME_STRING).unwrap();
            esp_nofail!(esp_ble_gap_set_device_name(mfr_name.as_ptr()));

            log::info!("Manufacturer name string set.");

            esp_nofail!(esp_ble_gap_config_adv_data(leaky_box_raw!(
                GLOBAL_CONFIGURATION.advertising_data
            ),));

            log::info!("Advertising data set.");

            esp_nofail!(esp_ble_gap_config_adv_data(leaky_box_raw!(
                GLOBAL_CONFIGURATION.scan_response_data
            ),));

            log::info!("Scan response data set.");

            #[allow(clippy::cast_possible_truncation)]
            {
                let attr_tab_without_indices = GLOBAL_CONFIGURATION
                    .gatt_db
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>();

                esp_nofail!(esp_ble_gatts_create_attr_tab(
                    leaky_box_raw!(attr_tab_without_indices.clone()) as *const esp_gatts_attr_db_t,
                    gatts_if,
                    attr_tab_without_indices.len() as u8,
                    0,
                ));
            }
        }
        esp_gatts_cb_event_t_ESP_GATTS_CREAT_ATTR_TAB_EVT => {
            log::info!("Handling attribute table creation event.");

            #[allow(clippy::cast_possible_truncation)]
            if (*param).add_attr_tab.status != esp_gatt_status_t_ESP_GATT_OK {
                log::error!(
                    "Attribute table creation failed, error {}.",
                    (*param).add_attr_tab.status
                );
                // TODO: Panic.
            } else if (*param).add_attr_tab.num_handle != GLOBAL_CONFIGURATION.gatt_db.len() as u16
            {
                log::error!(
                    "Attribute table created with wrong handle {} (instead of {}).",
                    (*param).add_attr_tab.num_handle,
                    GLOBAL_CONFIGURATION.gatt_db.len()
                );
                // TODO: Panic.
            } else {
                log::info!(
                    "Attribute table successfully created, {} handles.",
                    (*param).add_attr_tab.num_handle
                );

                let handles = slice::from_raw_parts(
                    (*param).add_attr_tab.handles,
                    GLOBAL_CONFIGURATION.gatt_db.len(),
                );

                // TODO: Fix zero handles.

                log::info!("Handles: {:?}", handles);

                for service in GLOBAL_CONFIGURATION.services.as_slice() {
                    let handle = handles[service.0.clone() as usize];
                    log::info!("Starting service {:?} with handle {}.", service.0, handle);
                    esp_nofail!(esp_ble_gatts_start_service(handle));
                }
            }
        }
        esp_gatts_cb_event_t_ESP_GATTS_CONNECT_EVT => {
            log::info!("Handling connection event.");
            let conn_params: esp_ble_conn_update_params_t = esp_ble_conn_update_params_t {
                bda: (*param).connect.remote_bda,
                latency: 0,
                max_int: 0x20, // 40ms
                min_int: 0x10, // 20ms
                timeout: 400,  // 4s
            };

            esp_nofail!(esp_ble_gap_update_conn_params(leaky_box_raw!(conn_params)));

            log::info!("Updating connection parameters.");
        }
        esp_gatts_cb_event_t_ESP_GATTS_DISCONNECT_EVT => {
            log::info!("Handling disconnection event.");

            esp_nofail!(esp_ble_gap_start_advertising(leaky_box_raw!(
                GLOBAL_CONFIGURATION.advertising_parameters
            )));

            log::info!("Restarting GAP advertising.");
        }
        _ => {
            log::warn!("Unhandled event #{}.", event);
        }
    }
}
