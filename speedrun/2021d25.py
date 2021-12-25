import unittest
import copy

def readlines_from(fname):
    f = open(fname)
    lines = f.readlines()
    f.close()
    lines = [r.strip() for r in lines if len(r.strip()) > 0]
    return lines

INPUT_STR = readlines_from("../input/2021_d25.txt")
SAMPLE_STR = readlines_from("../input/2021_d25_sample.txt")


class Map:
    def __init__(self, lines, step_nr=0):
        self._lines = lines
        self._width = len(lines[0])

        cells = []
        for line in lines:
            row = [ch for ch in line]
            if len(row) != self._width:
                print("Bad len %d want %d for %s" % (len(row), self._width, line))
                return 1/0
            cells.append(row)


        self._cells = cells
        self._height = len(self._cells)
        print("Init:ed map with height=%d, width=%d" % (self._height, self._width))
        self._step_nr = step_nr
        self._moved_this_step = True

    def get_at(self, x, y):
        """
        Top left is 0,0
        """
        return self._cells[y % self._height][x % self._width]
        """
        offset = x % self._width
        elem = self._cells[offset]
        return elem
        """

    def set_at(self, x, y, new_value):
        self._cells[y % self._height][x % self._width] = new_value
        """
        row = self._lines[y % self._height]
        offset = x % self._width
        self._cells[offset] = new_value
        """

    def get_next_step(self):
        intermediate_map = copy.deepcopy(self)
        for x in range(self._width):
            for y in range(self._height):
                intermediate_map.set_at(x, y, self.next_at_xy_after_east(x, y))


        #print("After eastern movement")
        #intermediate_map.print_map()

        end_map = copy.deepcopy(intermediate_map)
        for x in range(self._width):
            for y in range(self._height):
                end_map.set_at(x, y, intermediate_map.next_at_xy_after_south(x, y))

        #print("After southern movement")
        #end_map.print_map()

        end_map._moved_this_step = False
        end_map._step_nr += 1
        for x in range(self._width):
            for y in range(self._height):
                if self.get_at(x, y) != end_map.get_at(x, y):
                    end_map._moved_this_step = True
                    return end_map

        return end_map

    def print_map(self):
        for row in self._cells:
            #print("Row is '%s" % row)
            toprint = "".join(row)
            print(toprint)

    def next_at_xy_after_east(self, x, y):
        centre = self.get_at(x, y)

        # This herd does not move this step.
        if centre == 'v':
            return 'v'

        right = self.get_at(x+1,y)
        left = self.get_at(x-1,y)

        # Leave if free on right.
        if centre == '>':
            if right == '.':
                return '.'

        # Enter if free, and one is on left.
        if centre == '.':
            if left == '>':
                return '>'

        return centre

    def next_at_xy_after_south(self, x, y):
        centre = self.get_at(x, y)

        # This herd does not move this step.
        if centre == '>':
            return '>'

        above = self.get_at(x,y-1)
        below = self.get_at(x,y+1)

        # Leave if free below.
        if centre == 'v':
            if below == '.':
                return '.'

        # Enter if free, and one is on left.
        if centre == '.':
            if above == 'v':
                return 'v'

        return centre


class TestEntryPoint(unittest.TestCase):

    def test_is_in_map(self):
        ms = Map(SAMPLE_STR)
        mi = Map(INPUT_STR)

    def texst_southern_movement(self):
        ms = Map(["....v",
        ".....",
        "....."])
        for _ in range(5):
            ms = ms.get_next_step()

        return 1/0

    def test_getter(self):
        ms = Map(SAMPLE_STR)
        ms.print_map()

        
        ms1 = ms.get_next_step()
        print("After %d steps" % ms1._step_nr)
        ms1.print_map()


        for _ in range(49):
            ms1 = ms1.get_next_step()
        print("After %d steps" % ms1._step_nr)
        ms1.print_map()

        while ms1._moved_this_step:
            ms1 = ms1.get_next_step()

        self.assertEqual(58, ms1._step_nr)

        return 1/0
       
    def tesxt_ox_ratinxg(self):
        #print(INPUT_STR)
        self.assertEqual("10111", ox_rating(SAMPLE_STR))
        self.assertEqual("01010", co2_rating(SAMPLE_STR))

        self.assertEqual(part2(SAMPLE_STR), 230)
        self.assertEqual(part2(INPUT_STR), 5941884)

    def test_part1(self):
        ms = Map(SAMPLE_STR)        

        while ms._moved_this_step:
            ms = ms.get_next_step()

        self.assertEqual(58, ms._step_nr)


        mi = Map(INPUT_STR)
        while mi._moved_this_step:
            mi = mi.get_next_step()
        self.assertEqual(9999, mi._step_nr)


    def test_part2(self):
        pass

if __name__ == '__main__':
    unittest.main()
