import unittest

f = open("../input/2021_d11.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d11_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


SMALL_SAMPLE = [r.strip() for r in """11111
19991
19191
19991
11111""".split("\n") if len(r.strip()) > 0]


def lines_to_intfield(lines):
    toreturn = []
    for line in lines:
        row = [int(n) for n in line]
        toreturn.append(row)
    return toreturn
        
class Dumbos:
    def __init__(self, lines, step_nr=0):
        self._energies = lines_to_intfield(lines)
        self._width = len(lines[0])
        self._height = len(lines)
        self._step_nr = step_nr
        self._flashes = 0
        self._first_synced_flash = 66666555

    def to_string(self):
        toreturn = ""
        for row in self._energies:
            toreturn += "".join("%d" % n for n in row) + "\n"
        return toreturn

    def flash_until_step_nr(self, n):
        while self._step_nr < n:
            self.do_step()

    def print(self):
        print(" == At step %d ==" % self._step_nr) 
        print(self.to_string())


    def do_step(self):
        flashed_bools = []
        for _ in range(self._height):
            flashed_bools.append([False] * self._width)

        self._flashed_this_step = flashed_bools

        for x in range(self._width):
            for y in range(self._height):
                self.inc_energy_level(x, y)

        # After all flashes done, reduce energy levels of everyone that flashed
        flashcount_this_step = 0
        for x in range(self._width):
            for y in range(self._height):
                if self._flashed_this_step[x][y]:
                    self._energies[x][y] = 0
                    self._flashes += 1
                    flashcount_this_step += 1

        self._step_nr += 1
        if flashcount_this_step == self._width * self._height:
            if self._first_synced_flash > self._step_nr:
                print("FIRST FLASH %d" % self._step_nr)
                self._first_synced_flash = self._step_nr

    def inc_energy_level(self, x, y):
        """
        Increase energy level (I.e. because gets affected by a flash, or have time pass)
        """
        if not (0 <= x and x < self._width):
            return 0

        if not (0 <= y and y < self._height):
            return 0

        self._energies[x][y] += 1
        if self._energies[x][y] > 9 and not self._flashed_this_step[x][y]:
            self._flashed_this_step[x][y] = True
            self.inc_energy_level(x-1, y-1)
            self.inc_energy_level(x,   y-1)
            self.inc_energy_level(x+1, y-1)

            self.inc_energy_level(x-1, y)
            # self.inc_energy_level(x,   y) <- This is our current location.
            self.inc_energy_level(x+1, y)

            self.inc_energy_level(x-1, y+1)
            self.inc_energy_level(x,   y+1)
            self.inc_energy_level(x+1, y+1)





class TestEntryPoint(unittest.TestCase):


    def test_sample_map(self):
        ds = Dumbos(SMALL_SAMPLE)
        self.assertEqual(0, ds._flashes)

        ds.print()
        ds.do_step()
        self.assertEqual(9, ds._flashes)

        ds.print()
        ds.do_step()
        self.assertEqual(9, ds._flashes)
        
        #self.assertEqual(2,3)



    def test_part1(self):
        ds = Dumbos(SAMPLE_STR)
        """
        ds.print()
        ds.flash_until_step_nr(5)
        ds.print()
        ds.flash_until_step_nr(6)
        ds.print()
        ds.flash_until_step_nr(80)
        ds.print()
        ds.flash_until_step_nr(90)
        ds.print()
        """
        ds.flash_until_step_nr(100)
        #ds.print()
        self.assertEqual(1656, ds._flashes)

        ds = Dumbos(INPUT_STR)
        ds.flash_until_step_nr(100)
        #ds.print()
        self.assertEqual(1585, ds._flashes)



    def test_part2(self):
        ds = Dumbos(SAMPLE_STR)
        ds.flash_until_step_nr(200)        
        self.assertEqual(195, ds._first_synced_flash)


        ds = Dumbos(INPUT_STR)
        ds.flash_until_step_nr(1300)        
        self.assertEqual(382, ds._first_synced_flash)

if __name__ == '__main__':
    unittest.main()
