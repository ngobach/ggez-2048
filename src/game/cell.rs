type CellValue = u32;

#[derive(Copy, Clone)]
pub struct MoveOffset {
    vertical: u8,
    horizontal: u8,
}

#[derive(Copy, Clone)]
pub enum Cell {
    None,
    Some { value: CellValue },
}
