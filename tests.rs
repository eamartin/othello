use board::*;
use utils::debug;

/*
#[test]
fn test_bits() {
    let mut x = 0u64;
    set_bit(&mut x, 5);
    fail_unless!(get_bit(x, 5));
    set_bit(&mut x, 7);
    fail_unless!(get_bit(x, 5) && get_bit(x, 7));

    fail_unless!(!get_bit(x, 8));
}
*/

#[test]
fn test_init() {
    let b = Board::new();
    let black = [Position(3, 4), Position(4, 3)];
    let white = [Position(3, 3), Position(4, 4)];

    fail_unless!(black.all(|p| match b.get(*p) {
        Some(Black) => true,
        _ => false
    }));

    fail_unless!(white.all(|p| match b.get(*p) {
        Some(White) => true,
        _ => false
    }));

    fail_unless!(b.is_occupied(Position(3, 3)));
}

#[test]
fn test_get_moves() {
    let mut b = Board::new();

    let mut moves = b.get_moves(Black);
    fail_unless!(moves.contains(&Position(3, 2)));
    fail_unless!(moves.contains(&Position(2, 3)));
    fail_unless!(moves.contains(&Position(4, 5)));
    fail_unless!(moves.contains(&Position(5, 4)));
    fail_unless!(moves.len() == 4);

    b = b.make_move(Black, Position(3, 2));
    moves = b.get_moves(White);
    fail_unless!(moves.contains(&Position(2, 4)));
    fail_unless!(moves.contains(&Position(4, 2)));
    fail_unless!(moves.contains(&Position(2, 2)));
    fail_unless!(moves.len() == 3);

    b = b.make_move(White, Position(4, 2));
    moves = b.get_moves(Black);
    debug(copy moves);
    fail_unless!(moves.contains(&Position(5, 1)));
    fail_unless!(moves.contains(&Position(5, 2)));
    fail_unless!(moves.contains(&Position(5, 3)));
    fail_unless!(moves.contains(&Position(5, 4)));
    fail_unless!(moves.contains(&Position(5, 5)));
    fail_unless!(moves.len() == 5);
}

fn test_board_correctness(b: Board, black: ~[Position], white: ~[Position]) {
    for int::range(0, 8) |x| {
        for int::range(0, 8) |y| {
            let pos = Position(x, y);
            let pass = match b.get(pos) {
                Some(Black) => black.contains(&pos),
                Some(White) => white.contains(&pos),
                None => !(black.contains(&pos) || white.contains(&pos))
            };

            if !pass {
                debug(fmt!("%s %?", pos.to_str(), b.get(pos)));
            }
            fail_unless!(pass);
        };
    };
}

#[test]
fn test_make_moves() {
    let mut b = Board::new();

    b = b.make_move(Black, Position(3, 2));
    let mut black = ~[Position(3, 2), Position(3, 3), Position(3, 4), Position(4, 3)];
    let mut white = ~[Position(4, 4)];
    test_board_correctness(b, black, white);


    b = b.make_move(White, Position(4, 2));
    black = ~[Position(3, 2), Position(3, 3), Position(3, 4)];
    white = ~[Position(4, 2), Position(4, 3), Position(4, 4)];
    test_board_correctness(b, black, white);
}
