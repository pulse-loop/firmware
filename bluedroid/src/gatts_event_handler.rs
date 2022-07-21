//! The internal event handling loop for the GATT server.
//!
//! It dispatches various events to user-defined callbacks in the [`GATTSEventCallbacks`] struct.

use esp_idf_sys::{
    esp_ble_gatts_cb_param_t, esp_ble_gatts_cb_param_t_gatts_add_attr_tab_evt_param,
    esp_ble_gatts_cb_param_t_gatts_add_char_descr_evt_param,
    esp_ble_gatts_cb_param_t_gatts_add_char_evt_param,
    esp_ble_gatts_cb_param_t_gatts_add_incl_srvc_evt_param,
    esp_ble_gatts_cb_param_t_gatts_cancel_open_evt_param,
    esp_ble_gatts_cb_param_t_gatts_close_evt_param, esp_ble_gatts_cb_param_t_gatts_conf_evt_param,
    esp_ble_gatts_cb_param_t_gatts_congest_evt_param,
    esp_ble_gatts_cb_param_t_gatts_connect_evt_param,
    esp_ble_gatts_cb_param_t_gatts_create_evt_param,
    esp_ble_gatts_cb_param_t_gatts_delete_evt_param,
    esp_ble_gatts_cb_param_t_gatts_disconnect_evt_param,
    esp_ble_gatts_cb_param_t_gatts_exec_write_evt_param,
    esp_ble_gatts_cb_param_t_gatts_mtu_evt_param, esp_ble_gatts_cb_param_t_gatts_open_evt_param,
    esp_ble_gatts_cb_param_t_gatts_read_evt_param, esp_ble_gatts_cb_param_t_gatts_reg_evt_param,
    esp_ble_gatts_cb_param_t_gatts_rsp_evt_param,
    esp_ble_gatts_cb_param_t_gatts_send_service_change_evt_param,
    esp_ble_gatts_cb_param_t_gatts_set_attr_val_evt_param,
    esp_ble_gatts_cb_param_t_gatts_start_evt_param, esp_ble_gatts_cb_param_t_gatts_stop_evt_param,
    esp_ble_gatts_cb_param_t_gatts_write_evt_param, esp_gatts_cb_event_t,
    esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_DESCR_EVT, esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_ADD_INCL_SRVC_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_CANCEL_OPEN_EVT, esp_gatts_cb_event_t_ESP_GATTS_CLOSE_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_CONF_EVT, esp_gatts_cb_event_t_ESP_GATTS_CONGEST_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_CONNECT_EVT, esp_gatts_cb_event_t_ESP_GATTS_CREATE_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_CREAT_ATTR_TAB_EVT, esp_gatts_cb_event_t_ESP_GATTS_DELETE_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_DISCONNECT_EVT, esp_gatts_cb_event_t_ESP_GATTS_EXEC_WRITE_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_LISTEN_EVT, esp_gatts_cb_event_t_ESP_GATTS_MTU_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_OPEN_EVT, esp_gatts_cb_event_t_ESP_GATTS_READ_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_REG_EVT, esp_gatts_cb_event_t_ESP_GATTS_RESPONSE_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_SEND_SERVICE_CHANGE_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_SET_ATTR_VAL_EVT, esp_gatts_cb_event_t_ESP_GATTS_START_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_STOP_EVT, esp_gatts_cb_event_t_ESP_GATTS_UNREG_EVT,
    esp_gatts_cb_event_t_ESP_GATTS_WRITE_EVT,
};

#[derive(Copy, Clone)]
/// GATT server events callback definitions.
pub struct GATTSEventCallbacks {
    /// Event called when an application is registered.
    pub application_registered:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_reg_evt_param)>,

    /// Event called when the client requests to read an attribute.
    pub read_request:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_read_evt_param)>,

    /// Event called when the client requests to write an attribute.
    pub write_request:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_write_evt_param)>,

    /// Event called when the client requests to execute a write operation.
    pub execute_write_request:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_exec_write_evt_param)>,

    /// Event called when MTU changes.
    pub mtu_set: Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_mtu_evt_param)>,

    /// Event called when a receive confirmation occurred.
    pub receive_confirmation:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_conf_evt_param)>,

    /// Event called when an application is unregistered.
    pub application_unregistered: Option<fn(gatts_if: u8)>,

    /// Event called when a service is registered.
    pub service_created:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_create_evt_param)>,

    /// Event called when an include service is added.
    pub service_include_added:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_add_incl_srvc_evt_param)>,

    /// Event called when a characteristic is added.
    pub characteristic_added:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_add_char_evt_param)>,

    /// Event called when a characteristic descriptor is added.
    pub characteristic_descriptor_added:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_add_char_descr_evt_param)>,

    /// Event called when a service is deleted.
    pub service_deleted:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_delete_evt_param)>,

    /// Event called when a service is started.
    pub service_started:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_start_evt_param)>,

    /// Event called when a service is stopped.
    pub service_stopped:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_stop_evt_param)>,

    /// Event called when a client is connected.
    pub client_connected:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_connect_evt_param)>,

    /// Event called when a client is disconnected.
    pub client_disconnected:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_disconnect_evt_param)>,

    /// Event called when a peer is connected.
    pub peer_connected:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_open_evt_param)>,

    /// Event called when a peer is disconnected.
    pub peer_disconnected:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_cancel_open_evt_param)>,

    /// Event called when the server closes.
    pub server_closed:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_close_evt_param)>,

    /// Event called when the server listens for connections.
    pub server_listen: Option<fn(gatts_if: u8)>,

    /// Event called when a congestion occurs.
    pub congestion_occurred:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_congest_evt_param)>,

    /// Event called when a response is sent.
    pub response_sent:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_rsp_evt_param)>,

    /// Event called when the attribute table is created.
    pub attribute_table_created:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_add_attr_tab_evt_param)>,

    /// Event called when an attribute value is set.
    pub attribute_value_set:
        Option<fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_set_attr_val_evt_param)>,

    /// Event called when a service change indication is sent.
    pub service_change_indication_sent: Option<
        fn(gatts_if: u8, param: esp_ble_gatts_cb_param_t_gatts_send_service_change_evt_param),
    >,
}

/// The default event handler for the GATT server.
///
/// # Arguments
///
/// * `event`: The event.
/// * `gatts_if`: The interface on which the event occurred.
/// * `param`: Event-based parameters.
///
/// returns: ()
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub unsafe extern "C" fn gatts_event_handler(
    event: esp_gatts_cb_event_t,
    gatts_if: u8,
    param: *mut esp_ble_gatts_cb_param_t,
) {
    // Get callbacks from static crate mutex.
    let callbacks: Option<GATTSEventCallbacks> = {
        if let Ok(config) = crate::CONFIG.lock() {
            config.clone().map(|config| config.gatts_event_callbacks)
        } else {
            None
        }
    };

    if callbacks.is_none() {
        return;
    }

    let callbacks = callbacks.unwrap();

    #[allow(non_upper_case_globals)]
    match event {
        // Application registered.
        esp_gatts_cb_event_t_ESP_GATTS_REG_EVT => {
            let param = (*param).reg;
            if let Some(callback) = callbacks.application_registered {
                callback(gatts_if, param);
            }
        }
        // Client requested read operation.
        esp_gatts_cb_event_t_ESP_GATTS_READ_EVT => {
            let param = (*param).read;
            if let Some(callback) = callbacks.read_request {
                callback(gatts_if, param);
            }
        }
        // Client requested write operation.
        esp_gatts_cb_event_t_ESP_GATTS_WRITE_EVT => {
            let param = (*param).write;
            if let Some(callback) = callbacks.write_request {
                callback(gatts_if, param);
            }
        }
        // Client requested execute write operation.
        esp_gatts_cb_event_t_ESP_GATTS_EXEC_WRITE_EVT => {
            let param = (*param).exec_write;
            if let Some(callback) = callbacks.execute_write_request {
                callback(gatts_if, param);
            }
        }
        // MTU setting completed.
        esp_gatts_cb_event_t_ESP_GATTS_MTU_EVT => {
            let param = (*param).mtu;
            if let Some(callback) = callbacks.mtu_set {
                callback(gatts_if, param);
            }
        }
        // Receive confirmation.
        esp_gatts_cb_event_t_ESP_GATTS_CONF_EVT => {
            let param = (*param).conf;
            if let Some(callback) = callbacks.receive_confirmation {
                callback(gatts_if, param);
            }
        }
        // Application unregistered.
        esp_gatts_cb_event_t_ESP_GATTS_UNREG_EVT => {
            if let Some(callback) = callbacks.application_unregistered {
                callback(gatts_if);
            }
        }
        // Service created.
        esp_gatts_cb_event_t_ESP_GATTS_CREATE_EVT => {
            let param = (*param).create;
            if let Some(callback) = callbacks.service_created {
                callback(gatts_if, param);
            }
        }
        // Service include added.
        esp_gatts_cb_event_t_ESP_GATTS_ADD_INCL_SRVC_EVT => {
            let param = (*param).add_incl_srvc;
            if let Some(callback) = callbacks.service_include_added {
                callback(gatts_if, param);
            }
        }
        // Characteristic added.
        esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_EVT => {
            let param = (*param).add_char;
            if let Some(callback) = callbacks.characteristic_added {
                callback(gatts_if, param);
            }
        }
        // Characteristic descriptor added.
        esp_gatts_cb_event_t_ESP_GATTS_ADD_CHAR_DESCR_EVT => {
            let param = (*param).add_char_descr;
            if let Some(callback) = callbacks.characteristic_descriptor_added {
                callback(gatts_if, param);
            }
        }
        // Service deleted.
        esp_gatts_cb_event_t_ESP_GATTS_DELETE_EVT => {
            let param = (*param).del;
            if let Some(callback) = callbacks.service_deleted {
                callback(gatts_if, param);
            }
        }
        // Service started.
        esp_gatts_cb_event_t_ESP_GATTS_START_EVT => {
            let param = (*param).start;
            if let Some(callback) = callbacks.service_started {
                callback(gatts_if, param);
            }
        }
        // Service stopped.
        esp_gatts_cb_event_t_ESP_GATTS_STOP_EVT => {
            let param = (*param).stop;
            if let Some(callback) = callbacks.service_stopped {
                callback(gatts_if, param);
            }
        }
        // Client connected.
        esp_gatts_cb_event_t_ESP_GATTS_CONNECT_EVT => {
            let param = (*param).connect;
            if let Some(callback) = callbacks.client_connected {
                callback(gatts_if, param);
            }
        }
        // Client disconnected.
        esp_gatts_cb_event_t_ESP_GATTS_DISCONNECT_EVT => {
            let param = (*param).disconnect;
            if let Some(callback) = callbacks.client_disconnected {
                callback(gatts_if, param);
            }
        }
        // Peer connected.
        esp_gatts_cb_event_t_ESP_GATTS_OPEN_EVT => {
            let param = (*param).open;
            if let Some(callback) = callbacks.peer_connected {
                callback(gatts_if, param);
            }
        }
        // Peer disconnected.
        esp_gatts_cb_event_t_ESP_GATTS_CANCEL_OPEN_EVT => {
            let param = (*param).cancel_open;
            if let Some(callback) = callbacks.peer_disconnected {
                callback(gatts_if, param);
            }
        }
        // Server closed.
        esp_gatts_cb_event_t_ESP_GATTS_CLOSE_EVT => {
            let param = (*param).close;
            if let Some(callback) = callbacks.server_closed {
                callback(gatts_if, param);
            }
        }
        // Server listens for connections.
        esp_gatts_cb_event_t_ESP_GATTS_LISTEN_EVT => {
            if let Some(callback) = callbacks.server_listen {
                callback(gatts_if);
            }
        }
        // Congestion occurred.
        esp_gatts_cb_event_t_ESP_GATTS_CONGEST_EVT => {
            let param = (*param).congest;
            if let Some(callback) = callbacks.congestion_occurred {
                callback(gatts_if, param);
            }
        }
        // Response sent.
        esp_gatts_cb_event_t_ESP_GATTS_RESPONSE_EVT => {
            let param = (*param).rsp;
            if let Some(callback) = callbacks.response_sent {
                callback(gatts_if, param);
            }
        }
        // Attribute table created.
        esp_gatts_cb_event_t_ESP_GATTS_CREAT_ATTR_TAB_EVT => {
            let param = (*param).add_attr_tab;
            if let Some(callback) = callbacks.attribute_table_created {
                callback(gatts_if, param);
            }
        }
        // Attribute value set.
        esp_gatts_cb_event_t_ESP_GATTS_SET_ATTR_VAL_EVT => {
            let param = (*param).set_attr_val;
            if let Some(callback) = callbacks.attribute_value_set {
                callback(gatts_if, param);
            }
        }
        // Service change indication sent.
        esp_gatts_cb_event_t_ESP_GATTS_SEND_SERVICE_CHANGE_EVT => {
            let param = (*param).service_change;
            if let Some(callback) = callbacks.service_change_indication_sent {
                callback(gatts_if, param);
            }
        }
        // Unhandled events.
        _ => {}
    }
}
