

#[cfg(test)]
///The game logic.
///The game does not know which players are human,
///except for when suggestion a move.
pub struct Game {

    /// The area. It's cells are:
    /// - -1: empty
    /// - 0: player 1
    /// - 1: player 2
    /// - 2: player 3
    m_area: Vec<Vec<i8>>,

    /// The player that is making a move.
    /// - 0: player 1
    /// - 1: player 2
    /// - 2: player 3
    m_player: i8,
}


#[cfg(test)]
impl Game {

    #[cfg(test)]
    pub fn default() -> Game {
        Game::new(16, 12)
    }

    #[cfg(test)]
    pub fn new(n_cols: usize, n_rows: usize) -> Game {
        assert!(n_cols > 1);
        assert!(n_rows > 1);
        Game{
            m_area: vec![vec![-1; n_cols]; n_rows],
            m_player: 0,
        }
    }

    /// Does this square exist? Can it get read?
    #[cfg(test)]
    pub fn can_get_square(&self, x: usize, y: usize) -> bool {
          x < self.get_n_cols() && y < self.get_n_rows()
    }

    #[cfg(test)]
    pub fn can_do_move(&self, x: usize, y: usize) -> bool {
        self.can_get_square(x,y)
            && self.is_empty(x, y)
    }

    #[cfg(test)]
    pub fn do_move(&mut self, x: usize, y: usize) -> () {
        assert!(self.can_do_move(x, y));
        self.set_square(x, y, self.get_active_player());
        self.m_player = ((self.m_player) + 1) % crate::band::get_band_size() as i8;
        assert!(self.m_player >= 0);
        assert!(self.m_player < crate::band::get_band_size() as i8);
    }

    #[cfg(test)]
    pub fn get_active_player(&self) -> i8 {
        self.m_player
    }

    #[cfg(test)]
    pub fn get_n_cols(&self) -> usize {
        self.m_area[0].len()
    }

    #[cfg(test)]
    pub fn get_n_rows(&self) -> usize {
        self.m_area.len()
    }

    #[cfg(test)]
    /// Get the value of a square
    /// @return value of the square:
    /// - -1: empty
    /// - 0: player 1
    /// - 1: player 2
    /// - 2: player 3
    pub fn get_square(&self, x: usize, y: usize) -> i8 {
        assert!(self.can_get_square(x, y));
        self.m_area[y][x]
    }

    /// Is the game a draw?
    #[cfg(test)]
    pub fn is_draw(&self) -> bool {
        for x in 0..self.get_n_cols() {
            for y in 0..self.get_n_rows() {
              if self.is_empty(x, y) {
                  return false;
              }
            }
        }
        true
    }

    #[cfg(test)]
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        assert!(self.can_get_square(x,y));
        self.get_square(x, y) == -1
    }

    /// Restart the game, maintaining the arena's size
    #[cfg(test)]
    pub fn restart(&mut self) -> () {
        self.m_area = vec![vec![-1; self.get_n_cols()]; self.get_n_rows()];
        self.m_player = 0;
    }

    #[cfg(test)]
    /// Set the value of a square
    /// @param square_value value of the square:
    /// - -1: empty
    /// - 0: player 1
    /// - 1: player 2
    /// - 2: player 3
    pub fn set_square(&mut self, x: usize, y: usize, square_value: i8) -> () {
        assert!(self.can_get_square(x, y));
        self.m_area[y][x] = square_value;
        assert_eq!(self.get_square(x, y), square_value)
    }
}

/*
struct Game
{
  void DoMove(const int x, const int y);
  void DoMove(const Move& p) noexcept;
  Square GetSquare(const int x, const int y) const noexcept;

  ///GetWinner returns the index of the winner.
  ///Returns draw if the board is full without a winner
  ///Returns no_player if there are still free squares
  Winner GetWinner() const noexcept;

  void Restart() noexcept;

  ///SuggestMove suggests a good move.
  ///If the game is a draw, it will throw a logic error
  Move SuggestMove(const std::bitset<3>& is_player_human) const;

private:

  Move CheckOneOther(const std::bitset<3>& is_player_human) const;
  Move CheckTwoDiagonally() const;
  Move CheckTwoHorizontalOwn() const;
  Move CheckTwoOther(const std::bitset<3>& is_player_human) const;
  Move CheckTwoVerticalOwn() const;

  ///Returns all possible moves
  std::vector<Move> GetAllPossibleMoves() const noexcept;
  std::vector<Move> GetTwoHorizontalOtherMoves() const noexcept;
  std::vector<Move> GetTwoVerticalOtherMoves() const noexcept;

  ///Are all the squares occupied?
  bool IsDraw() const noexcept;
  bool IsPlayerHuman(const Player player, const std::bitset<3>& is_player_human) const noexcept;

  ///Throws std::logic_error if there is no move left
  Move MakeRandomMove() const;
  Square PlayerToSquare(const Player player) const noexcept;
  Player SquareToPlayer(const Square square) const noexcept;
  Winner SquareToWinner(const Square square) const noexcept;
};

std::ostream& operator<<(std::ostream& os, const Game& c);
*/

#[cfg(test)]
mod tests {
    use crate::band::get_band_size;
    use super::*;

    #[test]
    fn test_default_construction() {
        let game = Game::default();
        assert_eq!(game.get_active_player(), 0);
        assert_eq!(game.get_n_cols(), 16);
        assert_eq!(game.get_n_rows(), 12);
    }


    #[test]
    fn test_construction_of_3_x_2() {
        let n_cols = 3;
        let n_rows = 2;
        let game = Game::new(n_cols, n_rows);
        assert_eq!(game.get_n_cols(), n_cols);
        assert_eq!(game.get_n_rows(), n_rows);
    }
    #[test]
    fn test_can_get_square() {
        let game = Game::default();
        assert!(game.can_get_square(0, 0));
        assert!(!game.can_get_square(100, 0));
        assert!(!game.can_get_square(0, 100));
    }

    #[test]
    fn test_is_empty() {
        let game = Game::default();
        assert!(game.is_empty(0, 0));
        assert!(game.is_empty(12, 0));
        assert!(game.is_empty(0, 8));
    }

    #[test]
    fn test_set_square() {
        let x = 3;
        let y = 2;
        let player_2 = 1; // Square value
        let empty = -1; // Square value
        assert!(x != y);
        let mut game = Game::default();
        assert!(game.is_empty(x, y));
        game.set_square(x, y, player_2);
        assert!(!game.is_empty(x, y));
        game.set_square(x, y, empty);
        assert!(game.is_empty(x, y));
    }
    #[test]
    fn test_get_square() {
        let x = 3;
        let y = 2;
        let player_2 = 1; // Square value
        let empty = -1; // Square value
        assert!(x != y);
        let mut game = Game::default();
        assert!(game.get_square(x, y) != player_2);
        game.set_square(x, y, player_2);
        assert!(game.get_square(x, y) == player_2);
        game.set_square(x, y, empty);
        assert!(game.get_square(x, y) != player_2);
    }


    #[test]
    fn test_can_do_move() {
        let game = Game::default();
        assert!(game.can_do_move(0, 0));
        assert!(!game.can_do_move(100, 0));
        assert!(!game.can_do_move(0, 100));
    }

    #[test]
    fn test_do_move_once() {
        let x = 7;
        let y = 3;
        let mut game = Game::default();
        assert_eq!(game.get_active_player(), 0);
        assert!(game.is_empty(x, y));
        game.do_move(x, y);
        assert_eq!(game.get_active_player(), 1);
        assert!(!game.is_empty(x, y));
        assert_eq!(game.get_square(x, y), 0); // Player 1
    }

    #[test]
    fn test_do_move_changes_player() {
        let mut game = Game::default();
        assert_eq!(game.get_active_player(), 0);
        game.do_move(0, 0);
        assert_eq!(game.get_active_player(), 1);
        game.do_move(1, 0);
        assert_eq!(game.get_active_player(), 2);
        game.do_move(2, 0);
        assert_eq!(game.get_active_player(), 0);
    }

    #[test]
    fn test_is_draw() {
        let mut game = Game::new(2, 2);
        assert!(!game.is_draw());
        game.do_move(0, 0);
        assert!(!game.is_draw());
        game.do_move(0, 1);
        assert!(!game.is_draw());
        game.do_move(1, 0);
        assert!(!game.is_draw());
        game.do_move(1, 1);
        assert!(game.is_draw());
    }

    #[test]
    fn test_restart() {
        let mut game = Game::new(2, 2);
        assert!(!game.is_draw());
        game.do_move(0, 0); // done by player 1
        game.do_move(0, 1); // done by player 2
        game.do_move(1, 0); // done by player 3
        game.do_move(1, 1); // done by player 1
        assert!(game.is_draw());
        assert_eq!(game.get_active_player(), 1); // 1 == player 2
        game.restart();
        assert!(!game.is_draw());
        assert_eq!(game.get_active_player(), 0); // 0 == player 1
    }

    #[test]
    fn test_play_2_x_2_match() {
        let n_cols = 2;
        let n_rows = 2;
        let c = Game::new(n_cols, n_rows);
        assert_eq!(c.get_n_cols(), n_cols);
        assert_eq!(c.get_n_rows(), n_rows);
        assert_eq!(c.get_active_player(), 0);
        let is_player_human = [true, true, true];

        assert_eq!(is_player_human.len(), get_band_size());
        assert!(is_player_human[0]);
        assert!(is_player_human[1]);
        assert!(is_player_human[2]);
        assert!( c.can_do_move(0,0));
        assert!( c.can_do_move(0,1));
        assert!( c.can_do_move(1,0));
        assert!( c.can_do_move(1,1));
        assert!(!c.can_do_move(0,n_rows));
        assert!(!c.can_do_move(n_rows,0));

        /*
        assert!(c.SuggestMove(is_player_human).GetX() >= 0); //It just shouldn't throw
        assert!(c.GetWinner() == Winner::no_winner); //No winner yet
        c.DoMove(0,0);
        assert(c.GetActivePlayer() == Player::player2);
        assert(!c.can_do_move(0,0));
        assert( c.can_do_move(0,1));
        assert( c.can_do_move(1,0));
        assert( c.can_do_move(1,1));
        assert(c.SuggestMove(is_player_human).GetX() >= 0);//It just shouldn't throw
        assert(c.GetWinner() == Winner::no_winner); //No winner yet
        c.DoMove(0,1);
        assert(c.GetActivePlayer() == Player::player3);
        assert(!c.can_do_move(0,0));
        assert(!c.can_do_move(0,1));
        assert( c.can_do_move(1,0));
        assert( c.can_do_move(1,1));
        assert(c.SuggestMove(is_player_human).GetX() >= 0);//It just shouldn't throw
        assert(c.GetWinner() == Winner::no_winner); //No winner yet
        c.DoMove(1,0);
        assert(c.GetActivePlayer() == Player::player1);
        assert(!c.can_do_move(0,0));
        assert(!c.can_do_move(0,1));
        assert(!c.can_do_move(1,0));
        assert( c.can_do_move(1,1));
        assert(c.SuggestMove(is_player_human).GetX() >= 0);//It just shouldn't throw
        assert(c.GetWinner() == Winner::no_winner); //No winner yet
        c.DoMove(1,1);
        assert(c.GetActivePlayer() == Player::player2);
        assert(!c.can_do_move(0,0));
        assert(!c.can_do_move(0,1));
        assert(!c.can_do_move(1,0));
        assert(!c.can_do_move(1,1));
        try {
            c.SuggestMove(is_player_human);
        }
        catch (std::logic_error& ) {
            //OK
        }
        assert(c.GetWinner() == Winner::draw);
        */
    }

}
