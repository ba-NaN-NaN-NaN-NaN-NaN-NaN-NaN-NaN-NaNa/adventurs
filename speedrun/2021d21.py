import unittest
import re

def readlines_from(fname):
    f = open(fname)
    lines = f.readlines()
    f.close()
    lines = [r.strip() for r in lines if len(r.strip()) > 0]
    return lines

INPUT_STR = readlines_from("../input/2021_d21.txt")
SAMPLE_STR = readlines_from("../input/2021_d21_sample.txt")


class Dice:
    def __init__(self):
        self._state = 0
        self._total_rolls = 0

    def roll(self):
        self._state += 1
        if self._state > 100:
            self._state = 1

        self._total_rolls += 1
        return self._state


class Player:
    def __init__(self, label, position):
        self._label = label
        self._score = 0
        self._position = int(position)

    def make_move(self, dice):
        to_move1 = dice.roll()
        to_move2 = dice.roll()
        to_move3 = dice.roll()
        self._position = self._position + to_move1 + to_move2 + to_move3
        while self._position > 10:
            self._position -= 10

        self._score += self._position
        #print("Player %s rolls %d+%d+%d and moves to space %d for a total score of %d." % (self._label, to_move1, to_move2,to_move3,self._position,self._score))
        pass

class Game:
    def __init__(self, lines):
        self._lines = lines
        self._dice = Dice()
        matcher = re.compile("Player (?P<label>.*) starting position: (?P<start_pos>[0-9]+)$")
        res = matcher.match(lines[0])
        self._player1 = Player(res.groupdict()['label'], res.groupdict()['start_pos'])

        res = matcher.match(lines[1])
        self._player2 = Player(res.groupdict()['label'], res.groupdict()['start_pos'])

        #print(res.groupdict())

        self._looser_score = None

    def calc_score(self):
        return self._looser_score * self._dice._total_rolls

    def play_until_done(self):
        pass
        while self._player1._score < 1000 and self._player2._score < 1000:
            self._player1.make_move(self._dice)
            if self._player1._score < 1000:
                self._player2.make_move(self._dice)

        if self._player1._score < 1000:
            self._looser_score = self._player1._score
        else:
            self._looser_score = self._player2._score
        

def gen_roll_outcomes():
    results = [d1+d2+d3 for d1 in [1,2,3] for d2 in [1,2,3] for d3 in [1,2,3]]
    #print(results)

    toreturn = {}
    for res in results:
        if res in toreturn:
            toreturn[res]+=1
        else:
            toreturn[res]=1

    #print(toreturn)
    return toreturn


ROLL_OUTCOMES = gen_roll_outcomes()

OUTCOMES_CACHE = []

WORLD_STATE_CACHE = {}

class WorldState:

    @classmethod
    def get_world_with_params(cls, p1pos, p2pos, p1score, p2score, next_player):
        # key = "%d,%d,%d,%d,%d,%d" % (p1pos, p2pos, p1score, p2score, p1_rolls_left, p2_rolls_left)
        key = "%d,%d,%d,%d,%d" % (p1pos, p2pos, p1score, p2score, next_player)
        if key not in WORLD_STATE_CACHE:
            w = WorldState(p1pos, p2pos, p1score, p2score, next_player)

        #print("Size of WORLD_STATE_CACHE is now %d" % len(WORLD_STATE_CACHE))
        return WORLD_STATE_CACHE[key]

    def __init__(self, p1pos, p2pos, p1score, p2score, next_player):
        self.p1pos = p1pos
        self.p2pos = p2pos
        self.p1score = p1score
        self.p2score = p2score
        self.next_player = next_player

        # Memoization key here for SELF?
        key = key = "%d,%d,%d,%d,%d" % (p1pos, p2pos, p1score, p2score, next_player)
        self._state_str = key

        if key in WORLD_STATE_CACHE:
            print("DOUBLE ADD of WORLD_STATE_CACHE key %s" % key)
            return 1/0
        else:
            WORLD_STATE_CACHE[key] = self
            #print("WORLD_STATE_CACHE is now %s" % WORLD_STATE_CACHE)

        # Memoized next worlds.
        self._next_worlds = None
        self.result_memo = None

    def p1_has_won(self):
        if self.p1score >= 21:
            if self.p2score >= 21:
                # Sanity check
                return 1/0

            return True

        return False


    def p2_has_won(self):
        if self.p2score >= 21:
            if self.p1score >= 21:
                # Sanity check
                return 1/0

            return True

        return False

    def next_worlds(self):
        """
        What are our next immediate worlds?
        """
        if self._next_worlds is not None:
            return self._next_worlds

        if self.p1_has_won():
            # No further branches
            self._next_worlds = {}
            return self._next_worlds

        if self.p2_has_won():
            # No further branches
            self._next_worlds = {}
            return self._next_worlds
                
        to_set = {} # World key -> number of worlds

        if self.next_player == 1:
            for roll_res in ROLL_OUTCOMES:
                roll_freq = ROLL_OUTCOMES[roll_res]
                p1pos_new = self.p1pos + roll_res
                if p1pos_new > 10:
                    p1pos_new -= 10

                p1score_new = self.p1score + p1pos_new

                p2pos_new = self.p2pos
                p2score_new = self.p2score
                w = WorldState.get_world_with_params(p1pos_new, p2pos_new, 
                                                     p1score_new, p2score_new, 2)

                if w._state_str not in to_set:
                    to_set[w._state_str] = 0

                to_set[w._state_str] += roll_freq
        elif self.next_player == 2:
            for roll_res in ROLL_OUTCOMES:
                roll_freq = ROLL_OUTCOMES[roll_res]
                p2pos_new = self.p2pos + roll_res
                if p2pos_new > 10:
                    p2pos_new -= 10

                p2score_new = self.p2score + p2pos_new

                p1pos_new = self.p1pos
                p1score_new = self.p1score

                w = WorldState.get_world_with_params(p1pos_new, p2pos_new, 
                                                     p1score_new, p2score_new, 1)

                if w._state_str not in to_set:
                    to_set[w._state_str] = 0

                to_set[w._state_str] += roll_freq            
        else:
            return 1/0


        self._next_worlds = to_set
        return self._next_worlds

    def win_results(self):
        """
        Return (player1_wins, player2_wins)
        """
        if self.result_memo != None:
            return self.result_memo

        if self.p1_has_won():
            self.result_memo = (1,0)
            return self.result_memo

        if self.p2_has_won():
            self.result_memo = (0, 1)
            return self.result_memo


        p1_wins = 0
        p2_wins = 0

        next_worlds = self.next_worlds()
        for world_key in next_worlds:
            freq = next_worlds[world_key]
            world = WORLD_STATE_CACHE[world_key]
            world_res = world.win_results()
            p1_wins += world_res[0] * freq
            p2_wins += world_res[1] * freq

        self.result_memo = p1_wins, p2_wins
        return self.result_memo



class GamePart2:
    def __init__(self, lines):
        for line in lines:
            matcher = re.compile("Player (?P<label>.*) starting position: (?P<start_pos>[0-9]+)$")
            res = matcher.match(line)
            label = int(res.groupdict()['label'])
            start_pos = res.groupdict()['start_pos']
            if label == 1:
                self._player1_pos = start_pos
            else:
                self._player2_pos = start_pos

        self._player1_score = 0
        self._player2_score = 0
        # self._total_die_rolls = 0 <- Not used in part2.
        self._looser_score = None # Populated once we are done.


class TestEntryPoint(unittest.TestCase):

    def test_dice(self):
        d = Dice()
        self.assertEqual(1, d.roll())
        self.assertEqual(2, d.roll())
        for _ in range(96):
            d.roll()
        self.assertEqual(99, d.roll())
        self.assertEqual(100, d.roll())
        self.assertEqual(1, d.roll())

    def test_part1(self):
        gs = Game(SAMPLE_STR)
        gs.play_until_done()
        self.assertEqual(gs.calc_score(), 739785)
        
        gi = Game(INPUT_STR)
        gi.play_until_done()
        self.assertEqual(gi.calc_score(), 926610)

    def txest_outcomes(self):
        res = outcomes(6, 7, 21, 3, 2)
        self.assertEqual(res, (1,0))

        # Player 1 guaranteed to win.
        res = outcomes(6, 7, 20, 3, 1)
        self.assertEqual(res, (3,0))

        # Player 1 guaranteed to win, even if player 2 goes first.
        res = outcomes(6, 7, 20, 3, 2)
        self.assertEqual(res, (9,0))


        # Player 2 guaranteed to win.
        res = outcomes(6, 7, 2, 20, 1)
        self.assertEqual(res, (0,9))

        # Player 2 guaranteed to win, but sometimes needs two rolls to do so.
        # Sometimes =
        #  1,* <- p2 rolls 1. Then is at pos 2, score 19, p1 can roll any 3, p2 can roll any 3.
        #  2 <- p2 wins
        #  3 <- p2 wins
        res = outcomes(1, 1, 2, 17, 2)
        self.assertEqual(res, (0,19))


        #res = outcomes(1, 1, 2, 15, 2, verbose=" ")
        #self.assertEqual(res, (0,19))

        # Sample: positions 4 and 8
        res = outcomes(4, 8, 0, 0, 1)
        #self.assertEqual(res, (444356092776315, 341960390180808))

        #return 1/0
        # SAMPLE_STR

    def test_worlds(self):
        world = WorldState.get_world_with_params(1, 1, 21, 0, 1)
        next_worlds = len(world.next_worlds())
        self.assertEqual(0, next_worlds)

        world = WorldState.get_world_with_params(1, 1, 20, 0, 1)
        next_worlds = world.next_worlds()
        print("Next worlds is '%s'" % next_worlds)
        #self.assertEqual(3, len(next_worlds))

        #self.assertEqual(world.win_results(), (27,0))


        #world = WorldState.get_world_with_params(1, 1, 2, 17, 2)
        #self.assertEqual(world.win_results(), (0,19))


        world = WorldState.get_world_with_params(4,8,0,0,1)
        self.assertEqual(world.win_results(), (444356092776315, 341960390180808))

        world = WorldState.get_world_with_params(6,2,0,0,1)
        self.assertEqual(world.win_results(), (555,555))


    def test_try2(self):
        w = WorldState.get_world_with_params(6, 7, 20, 3, 1)

if __name__ == '__main__':
    unittest.main()
