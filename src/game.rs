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

  bool CanDoMove(const int x, const int y) const noexcept;
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

  bool CanDoMove(const Move& p) const noexcept;
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

}
