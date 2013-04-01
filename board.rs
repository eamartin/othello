use utils::debug;

#[deriving(Eq)]
pub struct Position(int, int);

pub const DEFAULT_POSITION: Position = Position(-1, -1);

pub impl Position {
    fn in_bounds(&self) -> bool {
        let Position(x, y) = *self;
        x >=0 && x < 8 && y >= 0 && y < 8
    }

    fn add(&self, other: Position) -> Position {
        let Position(x, y) = *self;
        let Position(ox, oy) = other;
        Position(x + ox, y + oy)
    }
}

impl ToStr for Position {
    fn to_str(&self) -> ~str {
        let Position(x, y) = *self;
        fmt!("Position(%d, %d)", x, y)
    }
}

#[deriving(Eq)]
pub enum Color {
    Black,
    White
}

pub impl Color {
    fn other(&self) -> Color {
        match *self {
            Black => White,
            White => Black
        }
    }
}

#[inline]
fn get_bit(data: u64, pos: int) -> bool {
    data & (1u64 << pos) != 0
}

#[inline]
fn set_bit(data: &mut u64, pos: int) {
    *data |= (1u64 << pos);
}

#[inline]
fn get_index(pos: Position) -> int {
    let Position(x, y) = pos;
    x + 8 * y
}

fn get_row(board_side: u64, row_num: int) -> u8 {
    let offset = row_num * 8;
    ((board_side & (255 << offset)) >> offset) as u8
}

fn get_col(board_side: u64, col_num: int) -> u8 {
    let mut col = 0u64;
    for int::range(0, 8) |row_num| {
        let index = 8 * row_num + col_num;
        col += (board_side & (1u64 << index)) >> (index - row_num);
    }
    return col as u8;
}

fn get_positive_diag(board_side: u64, start: Position) -> u8 {
    let mut pos = start;
    let mut diag = 0u64;
    let mut offset = 0;
    while pos.in_bounds() {
        let index = get_index(pos);
        diag += (board_side & (1u64 << index)) >> (index - offset);

        offset += 1;
        pos = start.add(Position(offset, offset));
    }
    return diag as u8;
}

fn get_negative_diag(board_side: u64, start: Position) -> u8 {
    let mut pos = start;
    let mut diag = 0u64;
    let mut offset = 0;
    while pos.in_bounds() {
        let index = get_index(pos);
        diag += (board_side & (1u64 << index)) >> (index - offset) as u8;

        offset += 1;
        pos = start.add(Position(offset, -offset));
    }

    return diag as u8;
}


fn get_linear_moves(my_row: u8, other_row: u8) -> ~[int] {
    let get_bit = |data: u8, idx: int| (data & (1u8 << idx)) != 0;
    let mut moves: ~[int] = vec::with_capacity(8);

    for int::range(1, 7) |index| {
        if get_bit(other_row, index) {
            // first lets look for move on the left
            if !get_bit(my_row | other_row, index - 1) {
                for int::range(index + 1, 8) |j| {
                    if get_bit(my_row, j) {
                        moves.push(index - 1);
                        break;
                    }
                    if !get_bit(my_row | other_row, j) {
                        break;
                    }
                }
            }

            // look for move on the right
            if !get_bit(my_row | other_row, index + 1) {
                for int::range_rev(index - 1, -1) |j| {
                    if get_bit(my_row, j) {
                        moves.push(index + 1);
                        break;
                    }
                    if !get_bit(my_row | other_row, j) {
                        break;
                    }
                }
            }
        }
    }
    return moves;
}

#[deriving(Eq)]
pub struct Board {
    black: u64,
    white: u64
}

pub impl Board {
    fn new() -> Board {
        let mut black = 0u64;
        let mut white = 0u64;
        set_bit(&mut black, get_index(Position(4, 3)));
        set_bit(&mut black, get_index(Position(3, 4)));
        set_bit(&mut white, get_index(Position(3, 3)));
        set_bit(&mut white, get_index(Position(4, 4)));
        Board { black: black, white: white }
    }

    fn get(&self, pos: Position) -> Option<Color> {
        let index = get_index(pos);
        if get_bit(self.black, index) {
            Some(Black)
        } else if get_bit(self.white, index) {
            Some(White)
        } else {
            None
        }
    }

    fn is_occupied(&self, pos: Position) -> bool {
        let index = get_index(pos);
        get_bit(self.black | self.white, index)
    }

    fn get_stones(&self, side: Color) -> u64 {
        match side {
            Black => self.black,
            White => self.white
        }
    }

    /*
    Get moves that are available to side.
    */
    fn get_moves(&self, side: Color) -> ~[Position] {
        debug("=======================");
        let mut moves: ~[Position] = vec::with_capacity(32);

        let my_stones = self.get_stones(side);
        let other_stones = self.get_stones(side.other());

        // rows
        debug("ROW MOVES");
        for int::range(0, 8) |row_num| {
            let my_row = get_row(my_stones, row_num);
            let other_row = get_row(other_stones, row_num);

            for get_linear_moves(my_row, other_row).each |col_num| {
                moves.push(Position(*col_num, row_num));
                debug(moves[moves.len() - 1]);
            }
        }

        // column
        debug("COL MOVES");
        for int::range(0, 8) |col_num| {
            let my_col = get_col(my_stones, col_num);
            let other_col = get_col(other_stones, col_num);

            for get_linear_moves(my_col, other_col).each |row_num| {
                moves.push(Position(col_num, *row_num));
                debug(moves[moves.len() - 1]);
            }
        }

        // positive diagonals
        debug("POS DIAG");
        let pos_starts = vec::from_fn(6, |x| Position(x as int, 0)) +
            vec::from_fn(5, |y| Position(0, y as int + 1));
        for pos_starts.each |start| {
            let my_diag = get_positive_diag(my_stones, *start);
            let other_diag = get_positive_diag(other_stones, *start);

            for get_linear_moves(my_diag, other_diag).each |offset| {
                debug(*start);
                debug(*offset);
                moves.push(start.add(Position(*offset, *offset)));
                debug(moves[moves.len() - 1]);
            }
        }

        // negative diagonals
        debug("NEG DIAG");
        let neg_starts = vec::from_fn(6, |x| Position(x as int, 7)) +
            vec::from_fn(5, |y| Position(0, 6 - y as int));
        for neg_starts.each |start| {
            let my_diag = get_negative_diag(my_stones, *start);
            let other_diag = get_positive_diag(other_stones, *start);

            for get_linear_moves(my_diag, other_diag).each |offset| {
                moves.push(start.add(Position(*offset, -*offset)));
                debug(moves[moves.len() - 1]);
            }
        }

        return moves;
    }

    fn print_board(&self) {
        for int::range(0, 8) |x| {
            for int::range(0, 8) |y| {
                let pos = Position(x, y);
                match self.get(pos) {
                    Some(x) => debug(fmt!("%s: %?", pos.to_str(), x)),
                    _ => ()
                };
            };
        };
    }

    fn make_move(&self, side: Color, pos: Position) -> Board {
        fail_unless!(pos.in_bounds());
        let Position(x, y) = pos;

        let mut flip_stones = 0u64;
        set_bit(&mut flip_stones, get_index(pos));

        for int::range(-1, 2) |dy| {
            for int::range(-1, 2) |dx| {
                let mut offset = 1;
                let mut test = Position(x + offset * dx, y + offset * dy);
                while test.in_bounds() {
                    match self.get(test) {
                        Some(col) if col == side.other() => {
                            offset += 1;
                            test = Position(x + offset * dx, y + offset * dy);
                        }
                        Some(col) if (offset != 1 && col == side) => {
                            for int::range(1, offset) |offset_replay| {
                                let old_pos = Position(x + offset_replay * dx,
                                                       y + offset_replay * dy);

                                set_bit(&mut flip_stones, get_index(old_pos));
                            }
                            break;
                        }
                        _ => break
                    }
                }
            }
        }

        match side {
            Black => Board { black: self.black | flip_stones,
                             white: self.white & (!flip_stones)},

            White => Board { white: self.white | flip_stones,
                             black: self.black & (!flip_stones)}
        }
    }
}
