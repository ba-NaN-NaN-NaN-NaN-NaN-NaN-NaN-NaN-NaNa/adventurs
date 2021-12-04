import unittest
import copy

f = open("../input/2021_d4.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d4_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


class Board:
    def __init__(self, label, rows):
        self._label = label
        parsed_rows = []
        for row in rows:
            row = row.replace("  ", " ").replace("  ", " ").strip()
            parsed_rows.append([int(n) for n in row.split(" ")])

        self._rows = parsed_rows

        marked = []
        for _ in range(5):
            marked.append([False] * 5)
        self._marked = marked
        self._winning_move = None
        self._moves = 0
        self._moves_to_win = 0
        self._final_score = 0

    def apply_moves(self, moves):
        """
        Sample moves = "5,22"
        """
        ints = [int(n) for n in moves.split(",")]
        for n in ints:
            self.mark(n)


    def play_board_until_win(self, moves, verbose = False):
        ints = [int(n) for n in moves.split(",")]
        move_nr = 0
        for n in ints:
            if not self.has_won():
                if verbose:
                    self.print_board("verbose after %02d moves" % move_nr)
                self.mark(n)
                move_nr += 1
        if verbose:
            self.print_board("verbose done after %02d moves" % move_nr)

    def mark(self, n):
        won_before = self.has_won()
        self._moves += 1
        for rownr in range(len(self._rows)):
            for colnr in range(len(self._rows[rownr])):
                if self._rows[rownr][colnr] == n:
                    try:
                        self._marked[rownr][colnr] = True
                    except Exception as ex:
                        print("Can not mark rownr %d, colnr %d, " % (rownr, colnr))

        won_after = self.has_won()

        if won_after and not won_before:
            self._winning_move = n
            self._moves_to_win = self._moves
            self._final_score = self.current_score()
            #self.print_board("Board '%s' won after %02d moves!! %d. Final score = %d" % (self._label, self._moves, n, self.final_score()))

    def print_board(self, title):
        print(" vv %s vv " % title )
        for rownr in range(len(self._rows)):
            num_part = " ".join(["%03s" % n for n in self._rows[rownr]])
            marked_part = " ".join(["%05s" % n for n in self._marked[rownr]])
            print("%s - %s" % (num_part, marked_part))

        print(" ^^ %s ^^ " % title )

    def has_won(self):
        if [True,True,True,True,True] in self._marked:
            return True

        for colnr in range(5):
            if (self._marked[0][colnr] and 
                self._marked[1][colnr] and 
                self._marked[2][colnr] and 
                self._marked[3][colnr] and 
                self._marked[4][colnr]):
                return True

        return False

    def sum_all_unmarked(self):
        toreturn = 0
        for rownr in range(len(self._rows)):
            for colnr in range(len(self._rows[rownr])):
                n = self._rows[rownr][colnr]
                marked = self._marked[rownr][colnr]
                if not marked:
                    toreturn += n
                
        return toreturn

    def current_score(self):
        sum_unmarked = self.sum_all_unmarked()
        return sum_unmarked * self._winning_move

    def final_score(self):
        return self._final_score
        
    def print_play_result(self):
        print("After %03d moves, board %s won with score %d" % ( self._moves_to_win,self._label, self.final_score()))

def lines_to_boards(label, lines):
    """
    Ingest a number of boards from full input into boards.
    """
    toreturn = []
    lines = lines[1:]
    board_nr = 0
    while len(lines) > 0:
        rows = lines[0:5]
        lines = lines[5:]
        board = Board(label + " #%d" % board_nr, rows)
        toreturn.append(board)
        board_nr += 1

    return toreturn


class Game:
    def __init__(self, label, lines):
        self._lines = lines
        self._moves = self._lines[0]
        self._boards = lines_to_boards(label, self._lines)
        self._label = label
        self._has_played_until_win = False

    def num_boards(self):
        return len(self._boards)

    def play_boards_until_win(self):
        if self._has_played_until_win:
            return
        for boardnr in range(len(self._boards)):
            board = self._boards[boardnr]
            board.play_board_until_win(self._moves)
            self._boards[boardnr] = board
        self._has_played_until_win = True



    def get_winning_board(self):
        toreturn = self._boards[0]
        for candidate_nr in range(len(self._boards)):
            candidate = self._boards[candidate_nr]
            if candidate._moves_to_win < toreturn._moves_to_win:
                toreturn = candidate

        return toreturn

    def get_loosing_board(self):
        toreturn = self._boards[0]
        for candidate_nr in range(len(self._boards)):
            candidate = self._boards[candidate_nr]
            if candidate._moves_to_win > toreturn._moves_to_win:
                toreturn = candidate

        return toreturn        

    def print_boards_results(self):
        for boardnr in range(len(self._boards)):
            board = self._boards[boardnr]

            print("Game '%s' board %03d - moves %03d, score %d" % (self._label, boardnr, board._moves_to_win, board.final_score()))

class TestStringMethods(unittest.TestCase):

    def test_moves(self):
        gs = Game("test_moves", SAMPLE_STR)
        sboard1 = copy.deepcopy(gs._boards[2])
        sboard1.apply_moves("7,4,9,5,11")
        #sboard1.print_board("fdfd")
        self.assertFalse(sboard1.has_won())

        sboard1.apply_moves("17,23,2,0,14,21")
        #sboard1.print_board("next 6")
        self.assertFalse(sboard1.has_won())

        sboard1.apply_moves("24")
        #sboard1.print_board("final")
        self.assertTrue(sboard1.has_won())

        self.assertEqual(188, sboard1.sum_all_unmarked())
        self.assertEqual(24, sboard1._winning_move)
        self.assertEqual(4512, sboard1.final_score())

    def test_column_moves(self):
        """
        Verify that a column win is possible.
        """
        gs = Game("test_col_win", SAMPLE_STR)
        board = gs._boards[0]
        board.apply_moves("11,16,18,15,9")
        self.assertFalse(board.has_won())
        board.apply_moves("4")
        self.assertTrue(board.has_won())
        
    def test_game_moves(self):
        gs = Game("test_moves", SAMPLE_STR)
        gs.play_boards_until_win()
        sboard1 = copy.deepcopy(gs._boards[2])
        self.assertEqual(188, sboard1.sum_all_unmarked())
        self.assertEqual(24, sboard1._winning_move)
        self.assertEqual(4512, sboard1.final_score())

    def test_real_moves(self):
        gi = Game("test_real_moves", INPUT_STR)
        iboard1 = copy.deepcopy(gi._boards[2])
        iboard1.apply_moves("85,2,51,76,69")
        
        self.assertFalse(iboard1.has_won())
        iboard1.apply_moves("52")
        self.assertTrue(iboard1.has_won())

        #iboard1.print_board("next 6")
        iboard1_unmarked = "31,49,21,84,83,18,86,53,75,29,48,28,24,12,5,87,67,95,82"
        # Sum -> 977
        self.assertEqual(31+49+21+84+83+18+86+53+75+29+48+28+24+12+5+87+67+95+82, iboard1.sum_all_unmarked())
        self.assertEqual(977, iboard1.sum_all_unmarked())
        self.assertEqual(52, iboard1._winning_move)
        self.assertTrue(iboard1.has_won())

        self.assertEqual(52 * 977, iboard1.final_score())
        self.assertEqual(50804, iboard1.final_score())
        


    def test_input(self):

        self.assertEqual(16, len(SAMPLE_STR))
        self.assertEqual(501, len(INPUT_STR))

        self.assertEqual(70, len(SAMPLE_STR[0]))
        self.assertEqual(14, len(SAMPLE_STR[1]))
        self.assertEqual(289, len(INPUT_STR[0]))
       
        gs = Game("test_input sample", SAMPLE_STR)
        gi = Game("test_input input", INPUT_STR)

        self.assertEqual(3, gs.num_boards())
        self.assertEqual(100, gi.num_boards())


    def test_game(self):
        gs = Game("test_game sample", SAMPLE_STR)
        gi = Game("test_game input", INPUT_STR)


        gs.play_boards_until_win()
        gi.play_boards_until_win()

        #gi.print_boards_results()


        winning_board = gs.get_winning_board()
        self.assertEqual(4512, winning_board.final_score())        
        
        # Part 1
        winning_board = gi.get_winning_board()
        winning_board.print_board("Winning board for skarp input")
        self.assertEqual(4662, winning_board.final_score())

        # Part 2
        loosing_board = gi.get_loosing_board()
        loosing_board.print_board("Loosing board for skarp input")
        self.assertEqual(12080, loosing_board.final_score())


if __name__ == '__main__':
    unittest.main()
