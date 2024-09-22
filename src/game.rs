
///The game logic.
///The game does not know which players are human,
///except for when suggestion a move.
pub struct Game {

    // The area
    m_area: Vec<Vec<i8>>,

    // The player that is making a move
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
            m_area: vec![vec![0; n_cols]; n_rows],
            m_player: 0,
        }
    }
    pub fn get_active_player(&self) -> i8 {
        self.m_player
    }
    pub fn get_n_cols(&self) -> usize {
        self.m_area[0].len()
    }
    pub fn get_n_rows(&self) -> usize {
        self.m_area.len()
    }
}

/*
struct Game
{
  explicit Game(
    const int n_cols = 16,
    const int n_rows = 12
  );

  bool can_do_move(const int x, const int y) const noexcept;
  int CanGetSquare(const int x, const int y) const noexcept;
  void DoMove(const int x, const int y);
  void DoMove(const Move& p) noexcept;
  Square GetSquare(const int x, const int y) const noexcept;
  static std::string GetVersion() noexcept;
  static std::vector<std::string> GetVersionHistory() noexcept;

  ///GetWinner returns the index of the winner.
  ///Returns draw if the board is full without a winner
  ///Returns no_player if there are still free squares
  Winner GetWinner() const noexcept;

  void Restart() noexcept;

  ///SuggestMove suggests a good move.
  ///If the game is a draw, it will throw a logic error
  Move SuggestMove(const std::bitset<3>& is_player_human) const;

private:

  bool can_do_move(const Move& p) const noexcept;
  Move CheckOneOther(const std::bitset<3>& is_player_human) const;
  Move CheckTwoDiagonally() const;
  Move CheckTwoHorizontalOwn() const;
  Move CheckTwoOther(const std::bitset<3>& is_player_human) const;
  Move CheckTwoVerticalOwn() const;

  ///Returns all possible moves
  std::vector<Move> GetAllPossibleMoves() const noexcept;
  Player GetNextPlayer() const noexcept;
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
    fn test_play_2_x_2_match() {
        let n_cols = 2;
        let n_rows = 2;
        let c = Game::new(n_cols, n_rows);
        assert_eq!(c.get_n_cols(), n_cols);
        assert_eq!(c.get_n_rows(), n_rows);
        assert_eq!(c.get_active_player(), 0);
        let is_player_human = [true, true, true];

        assert_eq!(is_player_human.len(), get_band_size());
        assert_eq!(is_player_human[0], true);
        assert_eq!(is_player_human[1], true);
        assert_eq!(is_player_human[2], true);

        /*
        assert!( c.can_do_move(0,0));
        assert!( c.can_do_move(0,1));
        assert!( c.can_do_move(1,0));
        assert!( c.can_do_move(1,1));
        assert!(!c.can_do_move(0,n_rows));
        assert!(!c.can_do_move(n_rows,0));
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
