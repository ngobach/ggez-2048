type CellValue = u32;

#[derive(Copy, Clone)]
pub struct MoveOffset {
    vertical: u8,
    horizontal: u8,
}

#[derive(Copy, Clone)]
pub enum Cell {
    None,
    Some {
        value: CellValue,
    },
    Changing {
        old: Option<CellValue>,
        new: CellValue,
        frames: u32,
    },
    Moving {
        value: CellValue,
        offset: MoveOffset,
        new: Option<CellValue>,
        frames: u32,
    },
}
