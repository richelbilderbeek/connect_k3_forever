/*

BOOST_AUTO_TEST_CASE(play_two_player_game)
{
  //Can a 2x2 game be played without exceptions thrown
  const int n_rows = 2;
  const int n_cols = 2;
  std::bitset<3> is_player_human;
  is_player_human[0] = true;
  is_player_human[1] = true;
  is_player_human[2] = true;
  Game c(n_cols,n_rows);
  assert(c.GetCols() == n_cols);
  assert(c.GetRows() == n_rows);
  assert(c.GetActivePlayer() == Player::player1);
  assert( c.CanDoMove(0,0));
  assert( c.CanDoMove(0,1));
  assert( c.CanDoMove(1,0));
  assert( c.CanDoMove(1,1));
  assert(!c.CanDoMove(0,n_rows));
  assert(!c.CanDoMove(n_rows,0));
  assert(c.SuggestMove(is_player_human).GetX() >= 0); //It just shouldn't throw
  assert(c.GetWinner() == Winner::no_winner); //No winner yet
  c.DoMove(0,0);
  assert(c.GetActivePlayer() == Player::player2);
  assert(!c.CanDoMove(0,0));
  assert( c.CanDoMove(0,1));
  assert( c.CanDoMove(1,0));
  assert( c.CanDoMove(1,1));
  assert(c.SuggestMove(is_player_human).GetX() >= 0);//It just shouldn't throw
  assert(c.GetWinner() == Winner::no_winner); //No winner yet
  c.DoMove(0,1);
  assert(c.GetActivePlayer() == Player::player3);
  assert(!c.CanDoMove(0,0));
  assert(!c.CanDoMove(0,1));
  assert( c.CanDoMove(1,0));
  assert( c.CanDoMove(1,1));
  assert(c.SuggestMove(is_player_human).GetX() >= 0);//It just shouldn't throw
  assert(c.GetWinner() == Winner::no_winner); //No winner yet
  c.DoMove(1,0);
  assert(c.GetActivePlayer() == Player::player1);
  assert(!c.CanDoMove(0,0));
  assert(!c.CanDoMove(0,1));
  assert(!c.CanDoMove(1,0));
  assert( c.CanDoMove(1,1));
  assert(c.SuggestMove(is_player_human).GetX() >= 0);//It just shouldn't throw
  assert(c.GetWinner() == Winner::no_winner); //No winner yet
  c.DoMove(1,1);
  assert(c.GetActivePlayer() == Player::player2);
  assert(!c.CanDoMove(0,0));
  assert(!c.CanDoMove(0,1));
  assert(!c.CanDoMove(1,0));
  assert(!c.CanDoMove(1,1));
  try {
    c.SuggestMove(is_player_human);
  }
  catch (std::logic_error& ) {
    //OK
  }
  assert(c.GetWinner() == Winner::draw);

}

*/