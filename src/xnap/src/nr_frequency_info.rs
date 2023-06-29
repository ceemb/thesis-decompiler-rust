pub struct NrFrequencyInfo {
    pub nr_arfcn: i32,
    pub frequency_band_list: Vec<i32>,
}

impl NrFrequencyInfo {
    pub fn new(nr_arfcn: i32, frequency_band_list: Vec<i32>) -> Self {
        Self {
            nr_arfcn,
            frequency_band_list,
        }
    }
}
