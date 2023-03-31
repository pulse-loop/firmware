use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

/// This struct contains all the data that is sent via notifications.
/// All the voltages are expressed in microvolts.
#[derive(Debug, Default, Clone, Copy)]
pub struct RawData {
    // TODO: Remove unused fields.
    pub(crate) ambient_reading: i32,
    pub(crate) led1_reading: i32,
    pub(crate) led2_reading: i32,
    pub(crate) led3_reading: i32,
}

impl RawData {
    pub fn serialise(&self) -> [u8; 16] {
        let mut data = [0; 16];

        data[0..4].copy_from_slice(&self.ambient_reading.to_le_bytes());
        data[4..8].copy_from_slice(&self.led1_reading.to_le_bytes());
        data[8..12].copy_from_slice(&self.led2_reading.to_le_bytes());
        data[12..16].copy_from_slice(&self.led3_reading.to_le_bytes());

        data
    }
}

impl std::ops::AddAssign for RawData {
    fn add_assign(&mut self, rhs: Self) {
        self.ambient_reading += rhs.ambient_reading;
        self.led1_reading += rhs.led1_reading;
        self.led2_reading += rhs.led2_reading;
        self.led3_reading += rhs.led3_reading;
    }
}

impl std::ops::DivAssign<i32> for RawData {
    fn div_assign(&mut self, rhs: i32) {
        self.ambient_reading /= rhs;
        self.led1_reading /= rhs;
        self.led2_reading /= rhs;
        self.led3_reading /= rhs;
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
            0 => self.raw_data.ambient_reading,
            1 => self.raw_data.led1_reading,
            2 => self.raw_data.led2_reading,
            3 => self.raw_data.led3_reading,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

/// This funtion should be called in a separate thread to send the readings from the AFE4404.
pub fn notify_task(
    ble_api: Arc<RwLock<crate::bluetooth::BluetoothAPI>>,
    readings: Arc<Mutex<RawData>>,
) {
    let mut time = std::time::Instant::now();
    loop {
        thread::sleep(Duration::from_millis(10));

        if time.elapsed().as_millis() > 50 {
            if let (Ok(ble_api), Ok(mut readings)) = (ble_api.write(), readings.lock()) {
                ble_api
                    .sensor_data
                    .raw_optical_data_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.serialise());

                time = std::time::Instant::now();
            }
        }
    }
}
