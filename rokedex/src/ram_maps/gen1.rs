use super::RamMap;

struct Gen1 {

}

impl RamMap for Gen1 {
    const ADDRESS_RANGE: std::ops::Range<usize> = 0x0000..0x6000;
}