import unittest
import copy

def readlines_from(fname):
    f = open(fname)
    lines = f.readlines()
    f.close()
    lines = [r.rstrip() for r in lines if len(r.strip()) > 0]
    return lines

INPUT_STR = readlines_from("../input/2021_d23.txt")
SAMPLE_STR = readlines_from("../input/2021_d23_sample.txt")

def lines_to_mapdata(lines):
    """
    Input: Input string

    Return 5x11 grid of cells.

    No outer # border can be stripped.
    """
    #print("lines_to_mapdata got:")
    for l in lines:
        pass
        #print(l)

    
    lines = [l + (" " * (13-len(l))) for l in lines]
    lines = [l.replace(" ", "#") for l in lines]

    lines = [l[1:-1] for l in lines] # Trim first and last column.
    lines = lines[1:-1] # Trim first and last line

    lines = [[ch for ch in l ] for l in lines]
    #lines[2] += ["#","#"]

    #print("lines_to_mapdata returning:")
    #for line in lines:
    #    print("".join(line))
    return lines


WANTED_COLUMN = {
    "A":2,
    "B":4,
    "C":6,
    "D":8,
}


LETTER_FOR_COLNR = {
    2:"A",
    4:"B",
    6:"C",
    8:"D",
}

VALID_HALL_COLUMN_NRS = [nr for nr in range(11) if nr not in [2,4,6,8]]
VALID_HALL_COLUMN = [nr in VALID_HALL_COLUMN_NRS for nr in range(11)]

#print(VALID_HALL_COLUMN)


COST_PER_MOVE = {
    "A":1,
    "B":10,
    "C":100,
    "D":1000,
}

def minimum_cost_to_fix(row_nr, col_nr, content):
    """
    LOWER boundary of how much this will cost to fix. Ignore if anything needs to move out of the way etc.
    """
    if content in ["#", "."]:
        return 0

    wanted_col = WANTED_COLUMN[content]

    if row_nr == 0:
        cols_to_move = abs(wanted_col-col_nr)
        """
        if cols_to_move == 0:
            # We need to go down.
            return COST_PER_MOVE[content]
        else:

        """
        return COST_PER_MOVE[content] * (1+cols_to_move)
    else:
        if wanted_col == col_nr:
            return 0
        return COST_PER_MOVE[content] + minimum_cost_to_fix(row_nr-1, col_nr, content)

BURROW_STATE_CACHES = {
    # zobrist hash -> Burrow
}

def hash_for_content(rownr, colnr, content):
    if content in ["#", "."]:
        return 0

    if content in ['A', 'B', 'C', 'D']:
        s = "content_%s_row%d_col%d" % (content, rownr, colnr)
        return hash(s)

def hash_effort(cost):
    s = "effort_%d" % cost
    return hash(s)


class Burrow:
    def __init__(self, lines, partnr):
        """
        State of a board.
        """
        if len(lines) != 5:
            raise TypeError("Bad row count got '%s'" % lines)

        if partnr == 'part1':
            # Act as if the bottom two rows are solved already.
            lines = [
                lines[0],
                lines[1],
                lines[2],
                lines[3],
                "  #A#B#C#D#",
                "  #A#B#C#D#",
                lines[4],
            ]
        elif partnr == 'part2':
            lines = [
                lines[0],
                lines[1],
                lines[2],
                "  #D#C#B#A#",
                "  #D#B#A#C#",
                lines[3],
                lines[4],
            ]
        else:
            raise TypeError("fkjdh")
        self._cells = lines_to_mapdata(lines)

        self._width = len(self._cells[0])
        for row in self._cells:
            if len(row) != self._width:
                raise TypeError("Length mismatch '%s', expected %d" % (row, self._width))


        self._effort_spent = 0



        self._all_moves_generated = False
        self.solved = False
        self.recalc_is_solved()


        #self._total_effort_pessimistic = None
        self.recalc_total_pessimistic()
        self.recalc_effort_optimistic()

        # After this.
        self._zobrist_hash = self.calc_zobrist_hash()

        

    def calc_zobrist_hash(self):
        """
        """
        toreturn = hash_effort(self._effort_spent)
        for rownr in range(len(self._cells)):
            row = self._cells[rownr]
            for colnr in range(len(row)):
                content = self._cells[rownr][colnr]
                partial_hash = hash_for_content(rownr, colnr, content)
                toreturn = toreturn ^ partial_hash
        return toreturn

    def validate_zobrist_hash(self):
        """
        """
        expected = self.calc_zobrist_hash()
        if expected != self._zobrist_hash:
            raise TypeError("fkjdfdjklgfd")

    def recalc_effort_optimistic(self):
        """
        Implement this. Then do worklist sorted by optimistic.

        Asssume: Never need to rest in corners.
        Target is top slot of a room.
        """
        effort_left = 0
        for col_nr in [2,4,6,8]:
            letter_for_col = LETTER_FOR_COLNR[col_nr]
            
            for row_nr in range(4, 0, -1):
                content = self._cells[row_nr][col_nr]
                
                if content == '.':
                    continue
                if content != letter_for_col:
                    dst_col = WANTED_COLUMN[content]
                    hsteps = abs(dst_col-col_nr)
                    vsteps = 1 + row_nr
                    effort_left += (hsteps + vsteps) * COST_PER_MOVE[content]

        for colnr in range(11):
            # What does it cost to place all amphis idling in hallway?
            content = self._cells[0][colnr]
            if content == '.':
                continue
            wanted_col = WANTED_COLUMN[content]
            vsteps = 1
            hsteps = abs(colnr-wanted_col)
            effort_left += (vsteps + hsteps) * COST_PER_MOVE[content]                        

        self._total_effort_optimistic = self._effort_spent + effort_left

    def recalc_total_pessimistic(self):
        """
        Calculate pessimistic cost to place all incorrect pieces + current effort spent.
        """
        self.recalc_is_solved()

        toreturn = 0
        for colnr in [2,4,6,8]:
            first_incorrect = None
            letter_for_col = LETTER_FOR_COLNR[colnr]

            # Determine the start of incorrect letters in a room.
            for row_nr in range(4, 0, -1):
                if first_incorrect is None:
                    if self._cells[row_nr][colnr] != letter_for_col:
                        first_incorrect = row_nr

            if first_incorrect is not None:
                for row_nr in range(first_incorrect, 0 , -1):
                    content = self._cells[row_nr][colnr]
                    if content == '.':
                        continue

                    wanted_col = WANTED_COLUMN[content]
                    
                    # Resting col: The far corner, opposite of column we want to be in.
                    resting_col = None
                    if content == 'A':
                        resting_col = 10
                    if content == 'B':
                        resting_col = 10
                    if content == 'C':
                        resting_col = 0
                    if content == 'D':
                        resting_col = 0

                    vsteps = 4 + row_nr
                    hsteps = abs(resting_col-colnr) + abs(resting_col-wanted_col)
                    toreturn += (vsteps + hsteps) * COST_PER_MOVE[content]

        for colnr in range(11):
            # What does it cost to place all amphis idling in hallway?
            content = self._cells[0][colnr]
            if content == '.':
                continue
            wanted_col = WANTED_COLUMN[content]
            vsteps = 4 + row_nr
            hsteps = abs(colnr-wanted_col)
            toreturn += (vsteps + hsteps) * COST_PER_MOVE[content]

        #if not self.solved:
        #    toreturn += 60 * 8 * 1000

        self._total_effort_pessimistic = self._effort_spent + toreturn
            

    def print(self, label):
        print(" --- %s ---" % label)
        lines = ["".join(r) for r in self._cells]
        to_print = "\n".join(lines)
        print(to_print)


    def distance_to_ok(self):
        """
        Estimate distance to ok.
        """
        distance = 0
        for row_nr in range(len(self._cells)):
            #print("dist: ",end="")
            for col_nr in range(len(self._cells[0])):
                content = self._cells[row_nr][col_nr]
                distance += minimum_cost_to_fix(row_nr, col_nr, content)
                #print(content,end="")
            #print("")
            
        #print("Distance is %d" % distance)
        return distance


    def can_leave_room(self, row_nr, col_nr):
        """
        Criteria (both must be valid):
         - We have a free path out.
         - We or someone below us is in wrong room.
        """
        for r in range(1,row_nr):
            if self._cells[r][col_nr] != ".":
                # Blocked by someone above us.
                return False

        room_letter = LETTER_FOR_COLNR[col_nr]
        for r in range(row_nr, len(self._cells)):
            if self._cells[r][col_nr] != room_letter:
                # Me or someone below me needs to leave.
                return True

        return False


    def can_enter_room(self, content):
        """
        Can an amphi of type 'content' enter its room?

        Does *NOT* consider hallway clearance.

        Criteria:
         - Must not block in anyone.
        """

        col_nr = WANTED_COLUMN[content]

        for r in range(4,0,-1):
            if self._cells[r][col_nr] == '.':
                return r
            if self._cells[r][col_nr] != content:
                return None

        self.print("HUH??")
        raise TypeError("Huh trying to move '%s'??" % content)


    def do_legal_leaves(self, col_nr, cache):
        """
        Do all possible leave actions.

        Top inhabitant of each room (if any) should leave if they can.

        Returns list of boards that are generated.
        """
        toreturn = []
        for row_nr in range(0,5):
            if self._cells[row_nr][col_nr] != '.':
                if self.can_leave_room(row_nr, col_nr):
                    for dst_col in VALID_HALL_COLUMN_NRS:
                        if self.is_hallway_free(col_nr, dst_col):
                            # Move to hallway.
                            generated = self.make_move(row_nr, col_nr, 0, dst_col, cache)
                            if generated is not None:
                                toreturn.append(generated)

                    pass
                break # Check next room.
        return toreturn

    def do_legal_enters(self, cache):
        """
        Do all possible enter actions.

        All inhabitants in hallway should enter their room if they can.

        Returns list of boards that are generated.
        """
        toreturn = []
        for src_col_nr in VALID_HALL_COLUMN_NRS:
            content = self._cells[0][src_col_nr]
            if content == '.':
                continue

            dst_col_nr = WANTED_COLUMN[content]

            check_src = False
            res = self.is_hallway_free(src_col_nr, dst_col_nr, check_src)
            #print("self.is_hallway_free(%d, %d, %s) -> %s" % (src_col_nr, dst_col_nr, check_src, res))

            if res:
                dst_row_nr = self.can_enter_room(content)
                if dst_row_nr is not None:
                    # Move to room.
                    #self.print("Enter room '%s' possible, at row '%d'" % (content, dst_row_nr))
                    generated = self.make_move(0, src_col_nr, dst_row_nr, dst_col_nr, cache)
                    if generated is not None:
                        toreturn.append(generated)

        return toreturn

    def do_legal_moves(self, cache):
        """
        Returns list of boards that are generated.
        """

        if self._all_moves_generated:
            return []

        toreturn = []
        for col_nr in [2,4,6,8]:
            toreturn += self.do_legal_leaves(col_nr, cache)

        toreturn += self.do_legal_enters(cache)
        self._all_moves_generated = True
        return toreturn
        #print("Did legal moves.")


    def make_move(self, src_row, src_col, dst_row, dst_col, cache):
        """
        Return either 'None' or resulting board.
        """
        content = self._cells[src_row][src_col]
        if content not in ['A', 'B', 'C','D']:
            raise TypeError("kgjfd09y35803450 ")

        content_at_dst = self._cells[dst_row][dst_col]
        if content_at_dst != '.':
            msg = "Can not move (src_row, src_col, dst_row, dst_col) = (%d,%d,%d,%d). Content at target is '%s'" % (src_row, src_col, dst_row, dst_col, content_at_dst)
            raise TypeError(msg)

        steps_in_hall = abs(src_col - dst_col)
        steps_in_room = abs(src_row - dst_row)
        cost_to_move = COST_PER_MOVE[content] * (steps_in_hall + steps_in_room)

        new_board_hash = self._zobrist_hash
        # XOR out cost of current effort, then XOR in cost of future effort.
        new_board_hash = new_board_hash ^ hash_effort(self._effort_spent) ^ hash_effort(self._effort_spent + cost_to_move)

        # XOR out cost of content in src location
        # XOR in  cost of content in dst location
        new_board_hash = new_board_hash ^ hash_for_content(src_row, src_col, content) ^ hash_for_content(dst_row, dst_col, content)
        if new_board_hash in cache:
            return None

        #print("Moving '%s' from (%d,%d) to (%d,%d). This costs %d" % (content, src_row, src_col, dst_row, dst_col, cost_to_move))
        new_board = copy.deepcopy(self)
        new_board._cells[dst_row][dst_col] = content
        new_board._cells[src_row][src_col] = '.'
        new_board._effort_spent += cost_to_move
        new_board._prev_hash = self._zobrist_hash
        new_board._zobrist_hash = new_board_hash
        new_board.validate_zobrist_hash()
        if dst_row == 1:
            new_board.recalc_is_solved()

        new_board.recalc_total_pessimistic()
        new_board.recalc_effort_optimistic()

        cache[new_board_hash] = new_board
        return new_board

    def __lt__(self, other):
        return self._total_effort_optimistic < other._total_effort_optimistic

    def gen_legal_moves(self, cache):
        """
        Populate hashes of target moves?

        Cache is 
        """
        debug = False
        if self._all_moves_generated:
            # Don't recalc again and again.
            return

        for room_letter in ['A', 'B', 'C', 'D']:
            
            src_row = None
            if self.is_room_top_leaveable(room_letter):
                src_row = 'top'

            elif self.is_room_bottom_leaveable(room_letter):
                src_row = 'bottom'

            if src_row is None:
                #
                continue

            src_colnr = WANTED_COLUMN[room_letter]
            valid_dst_colnrs = [nr for nr in VALID_HALL_COLUMN_NRS if self.is_hallway_free(src_colnr, nr)]

            if src_row == 'top':
                content = self._cells[1][src_colnr]
                if debug:
                    print("Can leave '%s' top, content is '%s'. Valid destinations is '%s'" % (room_letter, content, valid_dst_colnrs))

                for dst in valid_dst_colnrs:
                    squares_to_move = 1 + abs(dst-src_colnr)
                    cost_to_move = squares_to_move * COST_PER_MOVE[content]
                    new_board_hash = self._zobrist_hash
                    # XOR out cost of current effort, then XOR in cost of future effort.
                    new_board_hash = new_board_hash ^ hash_effort(self._effort_spent) ^ hash_effort(self._effort_spent + cost_to_move)

                    # XOR out cost of content in top row, source column.
                    # XOR in  cost of content in hallway row, dst column.
                    new_board_hash = new_board_hash ^ hash_for_content(1, src_colnr, content) ^ hash_for_content(0, dst, content)
                    if new_board_hash in cache:
                        continue

                    new_board = copy.deepcopy(self)
                    new_board._cells[0][dst] = content
                    new_board._cells[1][src_colnr] = '.'
                    new_board._effort_spent += cost_to_move
                    new_board._zobrist_hash = new_board_hash
                    new_board.validate_zobrist_hash()
                    cache[new_board_hash] = new_board

            else:
                content = self._cells[2][src_colnr]
                if debug:
                    print("Can leave '%s' bottom, content is '%s'. Valid destinations is '%s'" % (room_letter, content, valid_dst_colnrs))                
                for dst in valid_dst_colnrs:
                    squares_to_move = 2 + abs(dst-src_colnr)
                    cost_to_move = squares_to_move * COST_PER_MOVE[content]
                    new_board_hash = self._zobrist_hash
                    # XOR out cost of current effort, then XOR in cost of future effort.
                    new_board_hash = new_board_hash ^ hash_effort(self._effort_spent) ^ hash_effort(self._effort_spent + cost_to_move)

                    # XOR out cost of content in BOTTOM row, source column.
                    # XOR in  cost of content in hallway row, dst column.
                    new_board_hash = new_board_hash ^ hash_for_content(2, src_colnr, content) ^ hash_for_content(0, dst, content)
                    if new_board_hash in cache:
                        continue

                    new_board = copy.deepcopy(self)
                    new_board._cells[0][dst] = content
                    new_board._cells[2][src_colnr] = '.'
                    new_board._effort_spent += cost_to_move
                    new_board._zobrist_hash = new_board_hash
                    new_board.validate_zobrist_hash()
                    cache[new_board_hash] = new_board


        for hall_nr in VALID_HALL_COLUMN_NRS:
            content = self._cells[0][hall_nr]
            if content == '.':
                continue

            dst_room_letter = content
            dst_room_colnr = WANTED_COLUMN[dst_room_letter]

            if not self.is_hallway_free(hall_nr, dst_room_colnr):
                continue

            if self.is_room_top_enterable(dst_room_letter):
                squares_to_move = 1 + abs(dst_room_colnr-hall_nr)
                cost_to_move = squares_to_move * COST_PER_MOVE[content]
                new_board_hash = self._zobrist_hash
                # XOR out cost of current effort, then XOR in cost of future effort.
                new_board_hash = new_board_hash ^ hash_effort(self._effort_spent) ^ hash_effort(self._effort_spent + cost_to_move)

                # XOR in  cost of content in hallway row, src column.
                # XOR out cost of content in TOP row, dst column.
                new_board_hash = new_board_hash ^ hash_for_content(0, hall_nr, content) ^ hash_for_content(1, dst_room_colnr, content)
                if new_board_hash in cache:
                    continue
                                
                if debug:
                    print("Amphi '%s' in col '%d' can enter its room top at a cost of %d." % (dst_room_letter, hall_nr, cost_to_move))
                new_board = copy.deepcopy(self)
                new_board._cells[0][hall_nr] = '.'
                new_board._cells[1][dst_room_colnr] = content
                new_board._effort_spent += cost_to_move
                new_board._zobrist_hash = new_board_hash
                new_board.validate_zobrist_hash()
                cache[new_board_hash] = new_board

                # If we entered a top room, we might be solved!
                new_board.recalc_is_solved()

            if self.is_room_bottom_enterable(dst_room_letter):
                squares_to_move = 2 + abs(dst_room_colnr-hall_nr)
                cost_to_move = squares_to_move * COST_PER_MOVE[content]
                new_board_hash = self._zobrist_hash
                # XOR out cost of current effort, then XOR in cost of future effort.
                new_board_hash = new_board_hash ^ hash_effort(self._effort_spent) ^ hash_effort(self._effort_spent + cost_to_move)

                # XOR in  cost of content in hallway row, src column.
                # XOR out cost of content in BOTTOM row, dst column.
                new_board_hash = new_board_hash ^ hash_for_content(0, hall_nr, content) ^ hash_for_content(2, dst_room_colnr, content)
                if new_board_hash in cache:
                    continue

                if debug:
                    print("Amphi '%s' in col '%d' can enter its room bottom at a cost of %d." % (dst_room_letter, hall_nr, cost_to_move))
                new_board = copy.deepcopy(self)
                new_board._cells[0][hall_nr] = '.'
                new_board._cells[2][dst_room_colnr] = content
                new_board._effort_spent += cost_to_move
                new_board._zobrist_hash = new_board_hash
                new_board.validate_zobrist_hash()
                cache[new_board_hash] = new_board

        self._all_moves_generated = True



    def recalc_is_solved(self):
        """

        """
        for room_letter in ['A', 'B', 'C', 'D']:
            col_nr = WANTED_COLUMN[room_letter]
            for row_nr in [1,2,3,4]:
                if self._cells[row_nr][col_nr] != room_letter:
                    self.solved = False
                    return

        print("Solved! at effort %d"  % self._effort_spent)
        self.solved = True



    """
    OLD IMPLEMENTATION with only 2 holes per room.
    def is_room_top_enterable(self, room_letter):
        col_nr = WANTED_COLUMN[room_letter]
        return self._cells[1][col_nr] == '.' and self._cells[2][col_nr] == room_letter


    def is_room_bottom_enterable(self, room_letter):
        col_nr = WANTED_COLUMN[room_letter]
        return self._cells[1][col_nr] == '.' and self._cells[2][col_nr] == '.'


    def is_room_top_leaveable(self, room_letter):
        " ""
        Can something at the top of the room leave?

        Obs that a amphi in correct room can still leave if the room bottom has a bad amphi.
        " ""
        col_nr = WANTED_COLUMN[room_letter]

        if self._cells[1][col_nr] == '.':
            return False

        if self._cells[2][col_nr] != room_letter:
            return True

        return self._cells[1][col_nr] != room_letter


    def is_room_bottom_leaveable(self, room_letter):
        col_nr = WANTED_COLUMN[room_letter]
        if self._cells[1][col_nr] != '.':
            # Blocked
            return False

        if self._cells[2][col_nr] == '.':
            # Nothing there
            return False

        return self._cells[2][col_nr] != room_letter
    """

    def is_hallway_free(self, src, dst, check_src=True, check_dst=True):
        """
        Set check_src=False if you don't want to include the starting piece.
        """

        if check_src:
            content = self._cells[0][src]
            if content != ".":
                return False

        if check_dst:
            content = self._cells[0][dst]
            if content != ".":
                return False

        if src > dst:
            # Travelling leftwards.
            src, dst = dst, src
        else:
            # Travelling rightwards.
            src, dst = src, dst

        # Check all intermediate steps.
        for col_nr in range(src+1, dst):
            content = self._cells[0][col_nr]
            if content != ".":
                return False
            
        return True


def fixnd_cheapest_solved(cache):
    cheapest = 6798547689745
    for key in cache:
        board = cache[key]
        #print("Is below board solved? %s" % board.solved)
        #board.print("d7657s")
        if board.solved:
            if board._effort_spent < cheapest:
                cheapest = board._effort_spent
    return cheapest

def oxne_solve_pass(sorted_board_list, cache, generate_cap=5006):
    """
    Input board list, sorted by pessimistic total cost.

    Function generates a few new boards, then returns a new sorted board list.
    """
    if len(sorted_board_list) != len(cache):
        raise TypeError("lfkdh %d %d" % (len(sorted_board_list), len(cache)))

    new_this_pass = []
    to_generate_count = int(len(sorted_board_list)/10)

    if to_generate_count < 2110:
        to_generate_count = 2110

    if to_generate_count > generate_cap:
        to_generate_count = generate_cap

    for n in range(len(sorted_board_list)):
        board = sorted_board_list[n]
        new_this_pass += board.do_legal_moves(cache)
        if len(new_this_pass) > to_generate_count:
            break

    sorted_board_list.extend(new_this_pass)
    sorted_board_list.sort()


    print("One solve pass generated %d boards" % len(new_this_pass))
    if len(new_this_pass) == 0:
        raise TypeError("fdh")

    return sorted_board_list



class Solver:
    def __init__(self, first_board):
        int(first_board._zobrist_hash)
        first_board._prev_hash = None
        self._cache = { first_board._zobrist_hash : first_board }
        self._worklist = [first_board]
        self._solved_boards = []

    def do_pass(self):
        created_this_pass = []

        cut_point = len(self._worklist)
        if cut_point > 1000:
            cut_point = int(cut_point/4)

        for b in self._worklist[0:cut_point]:
            created_this_pass += b.do_legal_moves(self._cache)

        new_worklist = []
        for b in created_this_pass:
            if b.solved:
                self._solved_boards.append(b)
            elif b._all_moves_generated:
                pass
            else:
                new_worklist.append(b)
        
        self._worklist = self._worklist[cut_point:] + new_worklist
        self._worklist.sort()
        self._solved_boards.sort()


    def solve(self):
        while(len(self._solved_boards)) == 0:
            self.do_pass()

        return self._solved_boards[0]

    def print_path_to_board(self, board):
        if board is None:
            raise TypeError("gjlrfkhgfljh nil board?")
        
        print("Will print '%s'" % board)
        while board is not None:
            prev_hash = board._prev_hash
            board.print(" -- path to board. Current cost is '%d'. Prev hash is '%s'. Solved is '%s'--" % (board._effort_spent, prev_hash, board.solved))

            if prev_hash is not None:
                board = self._cache[prev_hash]
            else:
                break


class TestEntryPoint(unittest.TestCase):

    def txest_is_in_map(self):
        bs = Burrow(SAMPLE_STR)
        bi = Burrow(INPUT_STR)
        bs.print("fd")
        bs.gen_legal_moves()


    def test_distance_to_ok(self):
        bs = Burrow(SAMPLE_STR, partnr='part1')
        bs.distance_to_ok()
       
    def test_can_leave_room(self):
        b = Burrow("""#############
#...........#
###C#C#C#C###
  #C#C#C#C#
  #########""".split("\n"), partnr='part1')

        for rownr in [1,2,3,4]:
            for colnr in [2,4,6,8]:
                b._cells[rownr][colnr] = 'C'
        #b.print("All C:s")
        self.assertFalse(b.can_leave_room(1, 6))

        # Make C room want to empty out.
        b._cells[4][6] = "A"
        self.assertTrue(b.can_leave_room(1, 6))
        self.assertFalse(b.can_leave_room(2, 6))

        # Free a path to bottom.
        b._cells[1][6] = "."
        self.assertTrue(b.can_leave_room(2, 6))
        self.assertFalse(b.can_leave_room(3, 6))

        b._cells[2][6] = "."
        self.assertTrue(b.can_leave_room(3, 6))
        self.assertFalse(b.can_leave_room(4, 6))

        b._cells[3][6] = "."
        self.assertTrue(b.can_leave_room(4, 6))

        # But even if hallway is free, we cannot leave if we don't need to free ourselves or others.
        b._cells[3][6] = "C"
        b._cells[4][6] = "C"
        self.assertFalse(b.can_leave_room(3, 6))
        self.assertFalse(b.can_leave_room(4, 6))

    def test_can_enter_room(self):
        b = Burrow("""#############
#...........#
###.#.#.#.###
  #C#B#C#.#
  #########""".split("\n"), partnr='part2')

        # D: Can enter, room is empty.
        b._cells[2][8] = '.'
        b._cells[3][8] = '.'
        b.print("Enter room D")
        self.assertEqual(4, b.can_enter_room('D'))


        # C: Can enter, room is not empty.
        b._cells[2][6] = 'C'
        b._cells[3][6] = 'C'
        self.assertEqual(1, b.can_enter_room('C'))

        # B: Can not enter, will block others.
        self.assertEqual(None, b.can_enter_room('B'))

        # A: Can not enter, will block others.
        self.assertEqual(None, b.can_enter_room('A'))



    def test_minimum_cost_to_fix(self):
        b = Burrow("""#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########""".split("\n"), partnr='part1')
        self.assertEqual(0, b.distance_to_ok())

        # Single move to solve.
        b = Burrow("""#############
#.......C...#
###A#B#.#D###
  #A#B#C#D#
  #########""".split("\n"), partnr='part1')
        self.assertEqual(200, b.distance_to_ok())


        b.print("Part 1 to solve with 1 move")
        solver = Solver(b)
        solution = solver.solve()
        solver.print_path_to_board(solution)

        self.assertEqual(200, solution._effort_spent)
        self.assertTrue(solution.solved)


        # Move B out of way. 
        # Move C to hall.
        # Move C to correct.
        # Move B to correct.

        b = Burrow("""#############
#...........#
###A#C#B#D###
  #A#B#C#D#
  #########""".split("\n"), partnr='part1')

        b.print("Part 1 to solve with 4 moves")
        # C needs to go from 1,4 to 1,6.
        # B needs to go from 1,6 to 1,4.
        #self.assertEqual(4 * 10 + 4*100, b.distance_to_ok())
        solver = Solver(b)
        solution = solver.solve()
        #solver.print_path_to_board(solution)

        #self.assertEqual(3,5)

        self.assertEqual((2+4)*10 + 4*100, solution._total_effort_optimistic)


    def texst_part1(self):

        
        bs = Burrow(SAMPLE_STR, partnr='part1')
        solver = Solver(bs)
        solution = solver.solve()        
        self.assertEqual(12521, solution._effort_spent)

        # Foo       
        bi = Burrow(INPUT_STR, partnr='part1')
        solver = Solver(bi)
        solution = solver.solve()
        self.assertEqual(13336, solution._effort_spent)      
        

    def test_part2(self):
        bs = Burrow(SAMPLE_STR, partnr='part2')
        solver = Solver(bs)
        solution = solver.solve()        
        self.assertEqual(44169, solution._effort_spent)

        # Foo       
        bi = Burrow(INPUT_STR, partnr='part2')
        solver = Solver(bi)
        solution = solver.solve()
        self.assertEqual(53308, solution._effort_spent)      
        

if __name__ == '__main__':
    unittest.main()

