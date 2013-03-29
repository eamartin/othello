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
    *data = *data | (1u64 << pos);
}

#[inline]
fn get_index(pos: Position) -> int {
    let Position(x, y) = pos;
    x + 8 * y
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
        let other = side.other();
        let mut moves: ~[Position] = ~[];

        for int::range(0, 8) |y| {
            for int::range(0, 8) |x| {
                let pos = Position(x, y);
                match self.get(pos) {
                    Some(color) if color == other => {
                        moves.push_all(self.get_moves_around_stone(side, pos))
                    }
                    _ => ()
                };
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

    /*
    Given a spot (x, y) that contains a stone of side.other(), find moves available
    to side.
    */
    fn get_moves_around_stone(&self, side: Color, pos: Position) -> ~[Position] {
        let mut moves: ~[Position] = ~[];

        for int::range(-1, 2) |dy| {
            for int::range(-1, 2) |dx| {
                let my_stones = self.get_stones(side);
                let other_stones = self.get_stones(side.other());
                let mut current_pos = pos.add(Position(dx, dy));
                let this_move = current_pos;

                // can't move there if not in bounds or if occupied
                if (!current_pos.in_bounds() ||
                    self.is_occupied(current_pos)) {
                    loop
                }

                current_pos = pos.add(Position(-dx, -dy));
                while (current_pos.in_bounds() &&
                       get_bit(other_stones, get_index(current_pos))) {

                    current_pos = current_pos.add(Position(-dx, -dy));
                }

                if (current_pos.in_bounds() &&
                    get_bit(my_stones, get_index(current_pos))) {

                    moves.push(this_move);
                }
            };
        };

        return moves;
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
