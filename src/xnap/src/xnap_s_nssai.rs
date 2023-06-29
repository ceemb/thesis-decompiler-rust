pub struct XnapSNssai {
    pub sst: u32,
    pub sd: Option<u32>,
    pub sd_presence: bool,
}

impl XnapSNssai {
    pub fn new(sst: u32, sd: Option<u32>) -> XnapSNssai {
        let sd_presence = sd.is_some();
        XnapSNssai { sst, sd, sd_presence }
    }
}