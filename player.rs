use board::*;

pub struct Player {
    board: Board,
    color: Color
}

pub impl Player {
    fn new(side: Color) -> Player {
        Player { board: Board::new(), color: side }
    }

    fn choose_move(&mut self, opponents_move: Position) -> Position {
        if opponents_move != DEFAULT_POSITION {
            self.board = self.board.make_move(self.color.other(),
                                              opponents_move);
        }

        let my_move = self.board.get_moves(self.color)[0];
        self.board = self.board.make_move(self.color, my_move);
        return my_move;
    }
}
