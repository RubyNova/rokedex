mod gen1;

pub enum RamLoadError {
    InvalidRangeConstant,
    DataBufferTooLarge,
    DataBufferTooSmall,
    HallOfFameNotFoundOrInvalid,
    BoxesNotFoundOrInvalid,
    PlayerNameNotFoundOrInvalid,
    PartyDataNotFoundOrInvalid,
    BoxDataNotFoundOrInvalid,
    ChecksumNotFoundOrInvalid,
    ChecksumSaveDataMismatch,
}

pub enum BoxRangeRetrievalError {
    IndexOutOfRange //TODO: idk if we'll need more errors but w/e
}

pub enum ByteRangeError {
    OutOfRange //TODO: Do we need more errors...?
}

trait RamMap {
    const ADDRESS_RANGE: std::ops::Range<usize>;
    const RAM_SIZE: usize;
    const HALL_OF_FAME_DATA_RANGE: std::ops::Range<usize>;
    const PLAYER_NAME_DATA_RANGE: std::ops::Range<usize>;
    const PARTY_DATA_RANGE: std::ops::Range<usize>;
    const CURRENT_BOX_DATA_RANGE: std::ops::Range<usize>;
    const DATA_CHECKSUM_DATA_RANGE: std::ops::Range<usize>;
    const POKEDEX_OWNED_DATA_RANGE: std::ops::Range<usize>;
    const POKEDEX_SEEN_DATA_RANGE: std::ops::Range<usize>;
    const BAG_ITEMS_DATA_RANGE: std::ops::Range<usize>;
    const MONEY_DATA_RANGE: std::ops::Range<usize>;
    const RIVAL_NAME_DATA_RANGE: std::ops::Range<usize>;
    const GAME_OPTIONS_DATA_RANGE: std::ops::Range<usize>;
    const BADGES_DATA_RANGE: std::ops::Range<usize>;
    const BOXES_DATA_RANGE: std::ops::Range<usize>;

    fn load_save_buffer(&mut self, data: &[u8]) -> Result<(), RamLoadError>;
    fn get_data_range_for_box(&self, box_number: usize) -> Result<std::ops::Range<usize>, BoxRangeRetrievalError>;
    fn get_byte_subrange(&self, data_range: std::ops::Range<usize>) -> Result<&mut [u8], ByteRangeError>;
    fn get_impl_specific_data_ranges(&self) -> &std::collections::HashMap<String, std::ops::Range<usize>>;
}

const NULL_TERMINATOR: u8 = 0;