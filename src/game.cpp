/*


#include "connectthreegame.h"

#include <algorithm>
#include <cassert>
#include <ctime>
#include <iostream>

#include <boost/scoped_ptr.hpp>

#include "connectthreemove.h"
#include "connectthreemovefactory.h"


ribi::con3::Winner ribi::con3::Game::GetWinner() const noexcept
{
  const int n_rows = GetRows();
  for (int y=0; y!=n_rows; ++y)
  {
    const int n_cols = GetCols();
    for (int x=0; x!=n_cols; ++x)
    {
      if (m_area[x][y] == Square::empty) continue;
      //Horizontal
      if (x + 2 < n_cols
        && m_area[x  ][y] == m_area[x+1][y]
        && m_area[x+1][y] == m_area[x+2][y])
      {
        return SquareToWinner(m_area[x][y]);
      }
      //Vertical
      if (y + 2 < n_rows
        && m_area[x][y  ] == m_area[x][y+1]
        && m_area[x][y+1] == m_area[x][y+2])
      {
        return SquareToWinner(m_area[x][y]);
      }
    }
  }
  //Check for draw: if all
  return IsDraw() ? Winner::draw : Winner::no_winner;
}

ribi::con3::Move ribi::con3::Game::SuggestMove(
  const std::bitset<3>& is_player_human
) const
{
  try {
    const auto m = CheckTwoHorizontalOwn();
    if (CanDoMove(m)) return m;
  }
  catch (std::logic_error&) {  }

  try {
    const auto m = CheckTwoVerticalOwn();
    if (CanDoMove(m)) return m;
  }
  catch (std::logic_error&) {  }

  try {
    const auto m = CheckTwoOther(is_player_human);
    if (CanDoMove(m)) return m;
  }
  catch (std::logic_error&) {  }

  try {
    const auto m = CheckTwoDiagonally();
    if (CanDoMove(m)) return m;
  }
  catch (std::logic_error&) {  }

  try {
    const auto m = CheckOneOther(is_player_human);
    if (CanDoMove(m)) return m;
  }
  catch (std::logic_error&) {  }

  return MakeRandomMove();
}

ribi::con3::Move ribi::con3::Game::CheckTwoHorizontalOwn() const
{
  const int n_rows = GetRows();
  for (int y=0; y!=n_rows; ++y)
  {
    const int n_cols = GetCols();
    for (int x=0; x!=n_cols-1; ++x) //-1 to prevent out of range
    {
      //Two consequtive selfs
      if (m_area[x][y] == PlayerToSquare(m_player) && m_area[x+1][y] == PlayerToSquare(m_player))
      {
        if (x >= 1)
        {
          if (m_area[x-1][y] == Square::empty)
          {
            const Move p{ MoveFactory().Create(x-1,y,m_player) };
            assert(CanDoMove(p));
            return p;
          }
        }
        if (x < n_cols-2 && m_area[x+2][y] == Square::empty)
        {
          const Move p { MoveFactory().Create(x+2,y,m_player) };
          assert(CanDoMove(p));
          return p;
        }
      }
      //Perhaps a gap?
      if (x < n_cols-2)
      {
        if (m_area[x][y] == PlayerToSquare(m_player) && m_area[x+1][y] == Square::empty && m_area[x+2][y] == PlayerToSquare(m_player))
        {
          const Move p { MoveFactory().Create(x+1,y,m_player) };
          assert(CanDoMove(p));
          return p;
        }
      }
    }
  }
  throw std::logic_error("no move");
}

ribi::con3::Move ribi::con3::Game::CheckTwoVerticalOwn() const
{
  const int n_rows = GetRows();
  assert(n_rows > 1);
  for (int y=0; y!=n_rows-1; ++y) //-1 to prevent out of range
  {
    const int n_cols = GetCols();
    assert(n_cols > 1);
    for (int x=0; x!=n_cols; ++x)
    {
      //Two consequtive selfs?
      if (m_area[x][y] == PlayerToSquare(m_player) && m_area[x][y+1] == PlayerToSquare(m_player))
      {
        if (y >= 1)
        {
          if (m_area[x][y-1] == Square::empty)
          {
            const Move p { MoveFactory().Create(x,y-1,m_player) };
            assert(CanDoMove(p));
            return p;
          }
        }
        if (y < n_rows-2)
        {
          if (m_area[x][y+2] == Square::empty)
          {
            const Move p { MoveFactory().Create(x,y+2,m_player) };
            assert(CanDoMove(p));
            return p;
          }
        }
      }
      //Perhaps a gap?
      if (y < n_rows-2)
      {
        if (m_area[x][y] == PlayerToSquare(m_player) && m_area[x][y+1] == Square::empty && m_area[x][y+2] == PlayerToSquare(m_player))
        {
          const Move p { MoveFactory().Create(x,y+1,m_player) };
          assert(CanDoMove(p));
          return p;
        }
      }
    }
  }
  throw std::logic_error("no move");
}

ribi::con3::Move ribi::con3::Game::CheckTwoOther(const std::bitset<3>& is_player_human) const
{
  const auto moves(GetAllPossibleMoves());

  const int nMoves = moves.size();
  if (nMoves==0)
  {
    throw std::logic_error("no move");
  }

  {
    //Get anti-human moves
    std::vector<Move> v;
    for(const auto m: moves)
    {
      assert(CanDoMove(m));
      //Player is human
      if (IsPlayerHuman(m.GetPlayer(),is_player_human))
      {
        v.push_back(m);
      }
    }
    //If there are anti-player moves, choose one at random
    if (!v.empty())
    {
      const auto move = v[std::rand() % v.size()];
      assert(CanDoMove(move));
      return move;
    }
  }
  {
    //Get moves anti-next-player
    const Player next_player_index = GetNextPlayer();
    std::vector<Move> v;
    for(const auto& move: moves)
    {
      assert(CanDoMove(move));
      if (move.GetPlayer() == next_player_index)
        v.push_back(move);
    }
    //If there are anti-next-player moves, choose one at random
    if (!v.empty())
    {
      const auto move = v[std::rand() % v.size()];
      assert(CanDoMove(move));
      return move;
    }
  }
  //Choose a move at random
  {
    const auto move = moves[std::rand() % moves.size()];
    assert(CanDoMove(move));
    return move;
  }
}

std::vector<ribi::con3::Move>
  ribi::con3::Game::GetAllPossibleMoves() const noexcept
{
  std::vector<Move> v(GetTwoHorizontalOtherMoves());
  const std::vector<Move> w(GetTwoVerticalOtherMoves());
  std::copy(w.begin(),w.end(),std::back_inserter(v));
  return v;
}

std::vector<ribi::con3::Move> ribi::con3::Game::GetTwoHorizontalOtherMoves() const noexcept
{
  const int n_rows = GetRows();
  std::vector<Move> moves;
  for (int y=0; y!=n_rows; ++y)
  {
    const int n_cols = GetCols();
    for (int x=0; x!=n_cols-1; ++x) //-1 to prevent out of range
    {
      //Check consequtive
      if (m_area[x][y]!=Square::empty && m_area[x][y] == m_area[x+1][y])
      {
        //Check A X B
        if (x > 0 && m_area[x-1][y] == Square::empty)
        {
          const Move m { MoveFactory().Create(
            x-1,y,SquareToPlayer(m_area[x][y]))
          };
          assert(CanDoMove(m));
          moves.push_back(m);
        }
        //Check X B C
        if (x < n_cols-2 && m_area[x+2][y] == Square::empty)
        {
          const Move m { MoveFactory().Create(
            x+2,y,SquareToPlayer(m_area[x][y]))
          };
          assert(CanDoMove(m));
          moves.push_back(m);
        }
      }
      //Check gap, also X B C
      if (m_area[x][y] != Square::empty && x + 2 < n_cols && m_area[x+1][y] == Square::empty && m_area[x][y] == m_area[x+2][y])
      {
        const Move m { MoveFactory().Create(
          x+1,y,SquareToPlayer(m_area[x][y]))
        };
        assert(CanDoMove(m));
        moves.push_back(m);
      }

    }
  }
  return moves;
}

//A X B C (x is focus of for loop)
std::vector<ribi::con3::Move>
  ribi::con3::Game::GetTwoVerticalOtherMoves() const noexcept
{
  const int n_rows = GetRows();
  std::vector<Move> v;

  for (int y=0; y!=n_rows-1; ++y) //-1 to prevent out of range
  {
    const int n_cols = GetCols();
    for (int x=0; x!=n_cols; ++x)
    {
      //Check consequtive
      if (m_area[x][y] != Square::empty && m_area[x][y] == m_area[x][y+1])
      {
        //Check A X B
        if (y > 0 && m_area[x][y-1] == Square::empty)
        {
          const Move m { MoveFactory().Create(
            x,y-1,SquareToPlayer(m_area[x][y]))
          };
          assert(CanDoMove(m));
          v.push_back(m);
        }
        //Check X B C
        if (y < n_rows-2 && m_area[x][y+2] == Square::empty)
        {
          const Move m { MoveFactory().Create(
            x,y+2,SquareToPlayer(m_area[x][y]))
          };
          assert(CanDoMove(m));
          v.push_back(m);
        }
      }
      //Check gap, also X B C
      if (m_area[x][y] != Square::empty && y < n_rows && m_area[x][y+1] == Square::empty && m_area[x][y] == m_area[x][y+2])
      {
        const Move m { MoveFactory().Create(
          x,y+1,SquareToPlayer(m_area[x][y]))
        };
        assert(CanDoMove(m));
        v.push_back(m);
      }
    }
  }
  return v;
}

ribi::con3::Move ribi::con3::Game::CheckTwoDiagonally() const
{
  std::vector<Move> v;

  const int n_rows = GetRows();
  for (int y=0; y!=n_rows-1; ++y) //-1 To prevent out of range
  {
    const int n_cols = GetCols();
    for (int x=0; x!=n_cols-1; ++x) //-1 to prevent out of range
    {
      if (m_area[x][y] == PlayerToSquare(m_player) && m_area[x+1][y+1] == PlayerToSquare(m_player))
      {
        if (m_area[x+1][y] == Square::empty)
        {
          const Move m { MoveFactory().Create(
            x+1,y,SquareToPlayer(m_area[x][y]))
          };
          assert(CanDoMove(m));
          v.push_back(m);
        }
        if (m_area[x][y+1] == Square::empty)
        {
          const Move m { MoveFactory().Create(
            x,y+1,SquareToPlayer(m_area[x][y]))
          };
          assert(CanDoMove(m));
          v.push_back(m);
        }
      }
    }
  }
  if (v.empty())
  {
    throw std::logic_error("no move");
  }
  const auto m = v[std::rand() % v.size()];
  assert(CanDoMove(m));
  return m;
}

ribi::con3::Move ribi::con3::Game::CheckOneOther(const std::bitset<3>& is_player_human) const
{
  std::vector<Move> v;

  const int n_rows = GetRows();

  for (int y=0; y!=n_rows; ++y)
  {
    const int n_cols = GetCols();
    for (int x=0; x!=n_cols; ++x)
    {
      if (m_area[x][y] != Square::empty)
      {
        if (y >= 1 && m_area[x][y-1] == Square::empty)
        {
          const Move m { MoveFactory().Create(
            x,y-1,SquareToPlayer(m_area[x][y]))
          };
          assert(CanDoMove(m));
          v.push_back(m);
        }
        if (y < n_rows-1)
        {
          if (m_area[x][y+1] == Square::empty)
          {
            const Move m { MoveFactory().Create(
              x,y+1,SquareToPlayer(m_area[x][y]))
            };
            assert(CanDoMove(m));
            v.push_back(m);
          }
        }
        if (x >= 1)
        {
          if (m_area[x-1][y] == Square::empty)
          {
            const Move m { MoveFactory().Create(
              x-1,y,SquareToPlayer(m_area[x][y]))
            };
            assert(CanDoMove(m));
            v.push_back(m);
          }
        }
        if (x < n_cols-1)
        {
          if (m_area[x+1][y] == Square::empty)
          {
            const Move m { MoveFactory().Create(
              x+1,y,SquareToPlayer(m_area[x][y]))
            };
            assert(CanDoMove(m));
            v.push_back(m);
          }
        }
      }
    }
  }
  if (v.empty())
  {
    throw std::logic_error("no move");
  }

  {
    //Get anti-human moves
    std::vector<Move> w;
    for(const auto m: v)
    {
      assert(CanDoMove(m));
      if (IsPlayerHuman(m.GetPlayer(),is_player_human))
        w.push_back(m);
    }
    //If there are anti-player moves, choose one at random
    if (!w.empty())
    {
      const auto m = w[std::rand() % w.size()]; //ex-bug ('w.size()' was 'v.size()')
      assert(CanDoMove(m));
      return m;
    }
  }

  {
    //Get moves anti-next-player
    const Player next_player_index = GetNextPlayer();
    std::vector<Move> w;
    for(const auto m: v)
    {
      assert(CanDoMove(m));
      if (m.GetPlayer() == next_player_index)
        w.push_back(m);
    }
    //If there are anti-next-player moves, choose one at random
    if (!w.empty())
    {
      const auto m = w[std::rand() % w.size()];
      assert(CanDoMove(m));
      return m;
    }
  }
  //Choose a move at random
  {
    assert(v.size() != 0);
    const auto m = v[std::rand() % v.size()];
    assert(CanDoMove(m));
    return m;
  }
}

ribi::con3::Move ribi::con3::Game::MakeRandomMove() const
{
  std::vector<Move> v;
  const int n_cols = GetCols();
  const int n_rows = GetRows();

  for (int y=0; y!=n_rows; ++y)
  {
    for (int x=0; x!=n_cols; ++x)
    {
      if (this->GetSquare(x,y) == Square::empty)
      {
        const Move m { MoveFactory().Create(
          x,y,m_player)
        };
        assert(CanDoMove(m));
        v.push_back(m);
      }
    }
  }
  if (v.empty())
  {
    throw std::logic_error("no move");
  }
  const int index = std::rand() % v.size();
  return v[index];
}

*/