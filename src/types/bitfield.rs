pub struct BitField {
    pub bits: Vec<u8>,
}

impl BitField {
    pub fn new(bits: Vec<u8>) -> BitField {
        BitField { bits }
    }

    pub fn has_piece(&self, index: u32) -> bool {
        let byte = index / 8;
        let offset = index % 8;

        if byte >= self.bits.len() as u32 {
            return false;
        }

        self.bits[byte as usize] >> (7 - offset) & 1 != 0
    }

    pub fn set_piece(&mut self, index: u32) {
        let byte = index / 8;
        let offset = index % 8;

        if byte >= self.bits.len() as u32 {
            return;
        }

        self.bits[byte as usize] |= 1 << (7 - offset);
    }
}