pub(crate) enum CriticalValue {
    None,
    Minimum,
    Maximum,
}

impl CriticalValue {
    pub(crate) fn into_bits(&self) -> u8 {
        match self {
            CriticalValue::None => 0b00000000,
            CriticalValue::Minimum => 0b00000001,
            CriticalValue::Maximum => 0b00000010,
        }
    }
}
