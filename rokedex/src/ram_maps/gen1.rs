use super::{RamLoadError, RamMap, NULL_TERMINATOR};

struct Gen1Map {
    save_data: [u8; Gen1Map::RAM_SIZE],    
}

impl Gen1Map {
    fn is_correct_size_for_ram_range(&self, size: &usize) -> Result<(), super::RamLoadError>{
        if size > &Gen1Map::RAM_SIZE {
            return Err(super::RamLoadError::DataBufferTooLarge);
        }

        if size < &Gen1Map::RAM_SIZE {
            return Err(super::RamLoadError::DataBufferTooSmall);
        }

        Ok(())
    }
}

impl RamMap for Gen1Map {
    const ADDRESS_RANGE: std::ops::Range<usize> = 0x0000..(0x7A53 + 0x5AD);
    const RAM_SIZE: usize = Gen1Map::ADDRESS_RANGE.end - Gen1Map::ADDRESS_RANGE.start;
    const HALL_OF_FAME_DATA_RANGE: std::ops::Range<usize> = 0x0598..(0x0598 + 0x12C0);
    const PLAYER_NAME_DATA_RANGE: std::ops::Range<usize> = 0x2598..(0x2598 + 0xB);
    const PARTY_DATA_RANGE: std::ops::Range<usize> = 0x2F2C..(0x2F2C + 0x194);
    const CURRENT_BOX_DATA_RANGE: std::ops::Range<usize> = 0x30C0..(0x30C0 + 0x462);
    const DATA_CHECKSUM_DATA_RANGE: std::ops::Range<usize> = 0x3523..(0x3523 + 0x1);
    const POKEDEX_OWNED_DATA_RANGE: std::ops::Range<usize> = 0x25A3..(0x25A3 + 0x13);
    const POKEDEX_SEEN_DATA_RANGE: std::ops::Range<usize> = 0x25B6..(0x25B6 + 0x13);
    const BAG_ITEMS_DATA_RANGE: std::ops::Range<usize> = 0x25C9..(0x25C9 + 0x2A);
    const MONEY_DATA_RANGE: std::ops::Range<usize> = 0x25F3..(0x25F3 + 0x3);
    const RIVAL_NAME_DATA_RANGE: std::ops::Range<usize> = 0x25F6..(0x25F6 + 0xB);
    const GAME_OPTIONS_DATA_RANGE: std::ops::Range<usize> = 0x2601..(0x2601 + 0x1);
    const BADGES_DATA_RANGE: std::ops::Range<usize> = 0x2602..(0x2602 + 0x1);
    const BOXES_DATA_RANGE: std::ops::Range<usize> = 0x4000..(0x7A53 + 0x5AD);

    fn load_save_buffer(&mut self, data: &[u8]) -> Result<(), super::RamLoadError> {
        let data_length = data.len();

        if let Err(e) = self.is_correct_size_for_ram_range(&data_length) {
            return Err(e);
        }


        // this should never panic. If it does, our checks are set up wrong before it gets here. This is a bug!
        self.save_data = data.try_into().expect("If you are seeing this message, it means this implementation of RamMap is bugged! Please provide reproduction steps in a ticket at <insert GH URL here>");

        // now we check if it is a valid gen 1 save as best we can
        let last: Option<&u8> = {
                let this = self.get_byte_subrange(Gen1Map::PLAYER_NAME_DATA_RANGE);
                match this {
                    Ok(t) => t,
                    Err(_) => panic!("Invalid data range provided for PLAYER_NAME_DATA_RANGE for this implementation of the pokemon save file structure! Please submit a bug report at <insert github URL here>"),
                }
            }.last();

        match last {
                Some(some_last) => {
                    if &NULL_TERMINATOR == some_last {
                        return Err(super::RamLoadError::PlayerNameNotFoundOrInvalid);
                    }
                }
                None => panic!("Invalid binary data slice occured when using PLAYER_NAME_DATA_RANGE! This implemenation is broken! Please submit a bug report at <insert GiHub URL here>"),
            }

        Ok(())
    }

    fn get_data_range_for_box(
        &self,
        box_number: usize,
    ) -> Result<std::ops::Range<usize>, super::BoxRangeRetrievalError> {
        todo!()
    }

    fn get_byte_subrange(
        &self,
        data_range: std::ops::Range<usize>,
    ) -> Result<&mut [u8], super::ByteRangeError> {
        todo!()
    }

    fn get_impl_specific_data_ranges(
        &self,
    ) -> &std::collections::HashMap<String, std::ops::Range<usize>> {
        todo!()
    }
}
