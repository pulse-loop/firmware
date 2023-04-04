use esp_idf_sys::time;
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

/// This struct contains the raw readings from the frontend that will be sent to the application via notifications.
/// All the voltages are expressed in microvolts.
#[derive(Debug, Default, Clone, Copy)]
pub struct RawData {
    pub(crate) ambient: i32,
    pub(crate) led1: i32,
    pub(crate) led2: i32,
    pub(crate) led3: i32,
}

impl RawData {
    pub fn serialise(&self) -> [u8; 16] {
        let mut data = [0; 16];

        data[0..4].copy_from_slice(&self.ambient.to_le_bytes());
        data[4..8].copy_from_slice(&self.led1.to_le_bytes());
        data[8..12].copy_from_slice(&self.led2.to_le_bytes());
        data[12..16].copy_from_slice(&self.led3.to_le_bytes());

        data
    }
}

impl std::ops::AddAssign for RawData {
    fn add_assign(&mut self, rhs: Self) {
        self.ambient += rhs.ambient;
        self.led1 += rhs.led1;
        self.led2 += rhs.led2;
        self.led3 += rhs.led3;
    }
}

impl std::ops::DivAssign<i32> for RawData {
    fn div_assign(&mut self, rhs: i32) {
        self.ambient /= rhs;
        self.led1 /= rhs;
        self.led2 /= rhs;
        self.led3 /= rhs;
    }
}

impl<'a> IntoIterator for &'a RawData {
    type Item = i32;
    type IntoIter = RawDataIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RawDataIntoIterator {
            raw_data: self,
            index: 0,
        }
    }
}

pub struct RawDataIntoIterator<'a> {
    raw_data: &'a RawData,
    index: usize,
}

impl<'a> Iterator for RawDataIntoIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let result = match self.index {
            0 => self.raw_data.ambient,
            1 => self.raw_data.led1,
            2 => self.raw_data.led2,
            3 => self.raw_data.led3,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

/// This struct contains the filtered readings in the format (dc, ac), that will be sent to the application via notifications.
/// All the voltages are expressed in microvolts.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilteredData {
    pub(crate) led1: (i32, i32), // (dc, ac).
    pub(crate) led2: (i32, i32), // (dc, ac).
    pub(crate) led3: (i32, i32), // (dc, ac).
}

impl FilteredData {
    pub fn serialise(&self) -> [u8; 24] {
        let mut data = [0; 24];

        data[0..4].copy_from_slice(&self.led1.0.to_le_bytes());
        data[4..8].copy_from_slice(&self.led1.1.to_le_bytes());
        data[8..12].copy_from_slice(&self.led2.0.to_le_bytes());
        data[12..16].copy_from_slice(&self.led2.1.to_le_bytes());
        data[16..20].copy_from_slice(&self.led3.0.to_le_bytes());
        data[20..24].copy_from_slice(&self.led3.1.to_le_bytes());

        data
    }
}

impl std::ops::Index<usize> for FilteredData {
    type Output = (i32, i32);
    fn index(&self, i: usize) -> &(i32, i32) {
        match i {
            0 => &self.led1,
            1 => &self.led2,
            2 => &self.led3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<usize> for FilteredData {
    fn index_mut(&mut self, i: usize) -> &mut (i32, i32) {
        match i {
            0 => &mut self.led1,
            1 => &mut self.led2,
            2 => &mut self.led3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// This funtion should be called in a separate thread to send the readings from the AFE4404.
pub fn notify_task(
    ble_api: Arc<RwLock<crate::bluetooth::BluetoothAPI>>,
    raw_data: Arc<Mutex<RawData>>,
    filtered_data: Arc<Mutex<FilteredData>>,
) {
    let mut notify_timer = super::timer::Timer::new(50);
    loop {
        thread::sleep(Duration::from_millis(10));

        if notify_timer.is_expired() {
            if let (Ok(ble_api), Ok(raw_data), Ok(filtered_data)) =
                (ble_api.write(), raw_data.lock(), filtered_data.lock())
            {
                ble_api
                    .sensor_data
                    .raw_optical_data_characteristic
                    .write()
                    .unwrap()
                    .set_value(raw_data.serialise());
                ble_api
                    .sensor_data
                    .filtered_optical_data_characteristic
                    .write()
                    .unwrap()
                    .set_value(filtered_data.serialise());
                notify_timer.reset();
            }
        }
    }
}
