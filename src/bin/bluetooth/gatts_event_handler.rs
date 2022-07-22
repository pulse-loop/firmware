use crate::bluetooth::configuration::Configuration;
use esp_idf_sys::{
    esp_ble_gap_config_adv_data, esp_ble_gap_set_device_name, esp_ble_gatts_cb_param_t,
    esp_ble_gatts_create_attr_tab, esp_ble_gatts_start_service, esp_gatt_if_t,
    esp_gatt_status_t_ESP_GATT_OK, esp_gatts_cb_event_t,
    esp_gatts_cb_event_t_ESP_GATTS_CREAT_ATTR_TAB_EVT, esp_gatts_cb_event_t_ESP_GATTS_REG_EVT,
    esp_nofail, ESP_GATT_UUID_PRI_SERVICE,
};
use std::ffi::CString;
use std::ptr;

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
                esp_nofail!(esp_ble_gatts_create_attr_tab(
                    configuration.gatt_db.as_ptr(),
                    gatts_if,
                    configuration.gatt_db.len() as u8,
                    0,
                ));
            }
        }
        esp_gatts_cb_event_t_ESP_GATTS_CREAT_ATTR_TAB_EVT => {
            let configuration = Configuration::default();

            #[allow(clippy::cast_possible_truncation)]
            if (*param).add_attr_tab.status != esp_gatt_status_t_ESP_GATT_OK {
                log::error!("Attribute table creation failed.");
                // TODO: Panic.
            } else if (*param).add_attr_tab.num_handle != configuration.gatt_db.len() as u16 {
                log::error!("Attribute table created with wrong handle.");
                // TODO: Panic.
            } else {
                log::info!("Attribute table successfully created.");

                configuration
                    .gatt_db
                    .iter()
                    .enumerate()
                    .filter(|tuple| {
                        // Filter all the attributes that actually are primary services.
                        let (_i, item) = tuple;
                        log::info!("Checking if attribute is a primary service.");
                        item.att_desc.uuid_p
                            == Configuration::esp_uuid_as_u8_ptr(ESP_GATT_UUID_PRI_SERVICE)
                    })
                    .map(|tuple| {
                        // Get the index.
                        let (i, _item) = tuple;
                        log::info!("Attribute #{} is a primary service.", i);
                        i
                    })
                    .for_each(|index| {
                        // For each index, register the service.
                        #[allow(clippy::ptr_offset_with_cast, clippy::cast_possible_wrap)]
                        let handle = *(*param).add_attr_tab.handles.offset(index as isize);
                        esp_ble_gatts_start_service(hand);

                        log::info!(
                            "Starting service from attribute #{} with handle {}.",
                            index,
                            handle
                        );
                    });
            }
        }
        _ => {
            log::warn!("Unhandled event #{}.", event);
        }
    }
}
