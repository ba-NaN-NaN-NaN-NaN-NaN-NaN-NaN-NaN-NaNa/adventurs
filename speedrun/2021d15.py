import unittest

f = open("../input/2021_d15.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d15_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


class Map:

    def grow_map(self):

        widened = [] # From a NxN map, build a 5NxN map.
        for row in self._local_costs:
            widened_row = []
            for n in range(5):
               higher_value = [x+n for x in row] 
               widened_row = widened_row + higher_value
            widened.append(widened_row)

        also_heightened = [] # From a 5NxN map, build a 5Nx5N map.
        for n in range(5):
            for row in widened:
                new_row = [x+n  for x in row]
                for idx in range(len(new_row)):
                    # Wrap around
                    if new_row[idx] > 9:
                        new_row[idx] -= 9
                also_heightened.append(new_row)

        self._local_costs = also_heightened
       
        self._height = len(self._local_costs)
        self._width = len(self._local_costs[0])

        ## Re-propagate
        self.init_total_risk()


    def init_total_risk(self):
        costs = self._local_costs

        total_risk = []
        for rownr in range(self._height):
            total_risk.append([95384579834587436] * self._width)

        total_risk[self._height-1][self._width-1] = costs[self._height-1][self._width-1]
        self._total_risk = total_risk

        for rownr in reversed(range(self._height-1)):
            self.propagate_risk_to(rownr+1, rownr)
            self.propagate_risk_to(rownr, rownr+1)
            self.propagate_risk_to(rownr, rownr)
            #self._total_risk[rownr][rownr] = self._local_costs[rownr][rownr+1] + self._local_costs[rownr][rownr] + self._total_risk[rownr+1][rownr+1]
        

    def __init__(self, lines):
        #self._lines = lines
        self._height = len(lines)
        self._width = len(lines[0])

        costs = []
        for rownr in range(self._height):
            costs.append([int(ch) for ch in lines[rownr]])
        self._local_costs = costs

        self.init_total_risk()

    def get_start_location_risk(self):
        return self._total_risk[0][0] - self._local_costs[0][0]

    def propagate_risk_to(self, x, y):
        if x == self._width-1 and y == self._height-1:
            return False
        changed = False

        total_a = self.get_local_cost_at(x, y) + self.get_total_risk_at(x+1,y)
        total_b = self.get_local_cost_at(x, y) + self.get_total_risk_at(x-1,y)
        total_c = self.get_local_cost_at(x, y) + self.get_total_risk_at(x,y+1)
        total_d = self.get_local_cost_at(x, y) + self.get_total_risk_at(x,y-1)
        minimum = min([total_a, total_b, total_c, total_d])
        if self._total_risk[y][x] != minimum:
            changed = True

        self._total_risk[y][x] = minimum
        return changed
        
    def propagate_risk(self):
        """
        Do 1 pass of entire map.
        """
        changed = False
        # Optimisation: Do 1 pass, but also build a worklist?
        for x in reversed(range(self._height)):
            for y in reversed(range(self._width)):
                if self.propagate_risk_to(x, y):
                    changed = True
        return changed

    def print_local_costs(self):
        strings = []
        for row in self._local_costs:
            print("Will format '%s'" % row)
            formatted = "".join(["%2d" % nr for nr in row])
            strings.append(formatted)
        print("\n".join(strings))


    def print_total_risk(self):
        strings = []
        for row in self._total_risk:
            print("Will format '%s'" % row)
            formatted = "".join(["%3d" % nr for nr in row])
            strings.append(formatted)
        print("\n".join(strings))

    def get_local_cost_at(self, x, y):
        if x >= self._width:
            return 69864958674598
        if y >= self._height:
            return 69864958674598

        if x < 0:
            return 5689067896987

        if y < 0:
            return 5689067896987

        return self._local_costs[y][x]


    def get_total_risk_at(self, x, y):
        if x >= self._width:
            return 69864958674598
        if y >= self._height:
            return 69864958674598

        if x < 0:
            return 5689067896987

        if y < 0:
            return 5689067896987

        return self._total_risk[y][x]




class TestEntryPoint(unittest.TestCase):

    def test_part1(self):
        ms = Map(SAMPLE_STR)
        ms.propagate_risk()
        #ms.print_total_risk()

        mi = Map(INPUT_STR)
        mi.propagate_risk()
        #mi.print_total_risk()
        self.assertEqual(ms.get_start_location_risk(), 40)
        self.assertEqual(mi.get_start_location_risk(), 673)


    def test_part2(self):
        ms = Map(SAMPLE_STR)
        ms.grow_map()
        #ms.print_local_costs()
        ms.propagate_risk()
        while ms.propagate_risk():
            ms.propagate_risk()

        self.assertEqual(ms.get_start_location_risk(), 315)

        mi = Map(INPUT_STR)
        mi.grow_map()
        #mi.print_local_costs()
        while mi.propagate_risk():
            # Once not enough.
            mi.propagate_risk()
        
        self.assertEqual(mi.get_start_location_risk(), 2893)
        

if __name__ == '__main__':
    unittest.main()
