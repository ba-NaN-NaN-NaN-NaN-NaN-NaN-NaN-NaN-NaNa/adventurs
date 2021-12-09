import unittest

f = open("../input/2021_d9.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d9_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y


class Map:
    def __init__(self, lines):
        self._lines = lines
        ints = []
        for line in lines:
            ints.append([int(s) for s in line])

        print("Map is %d high, %d wide" % (len(ints), len(ints[0])))
        self._ints = ints
        self._low_points = None
        self._sum_risk = 0

        basins = []
        for line in lines:
            basins.append([None] * len(line))
        self._basins = basins


    def find_low_points(self):
        sum_risk = 0
        low_points = []
        for x in range(len(self._lines[0])):
            for y in range(len(self._lines)):
                if self.is_low_point(x, y):
                    #print("Is low point (%d, %d)" % (x, y))
                    sum_risk += self._ints[y][x] + 1
                    low_points.append(Point(x,y))

        self._sum_risk = sum_risk
        self._low_points = low_points
        #print("Sum risk is '%d'" % sum_risk)

    def propagate_basin_at(self, x, y, height):
        
        if self.get_basin_at(x, y) is not None:
            # Already done.
            return
        
        local_height = self.get_at(x, y)
        if local_height != height:
            # Not doing in this pass.
            return
        
        if local_height == 9:
            # Because edges count as height 888. We don't want the 9:s along the bottom
            # edge of the sample to count as a basin.
            return

        adjecent_basins = []

        # I think there is a failure mode since each height only gets to propagate to closes neigbours.
        #
        # 8888888
        # 8144448
        # 8888888
        # 
        # Then I think only the 4 adjecent to the 1 will be filled. But a real gas will flow
        # and fill all 4:s.
        #
        if not self.is_left_edge(x, y):
            if self.get_at(x-1, y) < local_height:
                adjecent_basins.append(self.get_basin_at(x-1, y))

        if not self.is_right_edge(x, y):
            if self.get_at(x+1, y) < local_height:
                adjecent_basins.append(self.get_basin_at(x+1, y))

        if not self.is_top_edge(x, y):
            if self.get_at(x, y-1) < local_height:
                adjecent_basins.append(self.get_basin_at(x, y-1))

        if not self.is_bottom_edge(x, y):
            if self.get_at(x, y+1) < local_height:
                adjecent_basins.append(self.get_basin_at(x, y+1))


        adjecent_basins = [b for b in adjecent_basins if b is not None]
        adjecent_basins = list(set(adjecent_basins))
        if len(adjecent_basins) == 1:
            self.set_basin_at(x, y, adjecent_basins[0])


    def get_basin_sizes(self):
        """
        return sorted list of ints.
        """
        basins_with_size = {}
        toreturn = []
        for x in range(len(self._lines[0])):
            for y in range(len(self._lines)):
                basin_at_xy = self.get_basin_at(x, y)
                if basin_at_xy is None:
                    continue

                if basin_at_xy not in basins_with_size:
                    basins_with_size[basin_at_xy] = 1
                else:
                    basins_with_size[basin_at_xy] += 1


        for k in basins_with_size:
            size = basins_with_size[k]
            toreturn.append(size)

        return sorted(toreturn)


    def propagate_basins(self):
        """

        """
        for n in range(len(self._low_points)):
            low_point = self._low_points[n]
            x = low_point.x
            y = low_point.y
            self._basins[y][x] = n
        
        for height in range(10):
            for x in range(len(self._lines[0])):
                for y in range(len(self._lines)):
                    self.propagate_basin_at(x, y, height)


    def print_basins(self):
        """
        Use for visual verification vs the bolded basin samples in problem description.
        """
        for row in self._basins:
            def f(n):
                if n is None:
                    return '?'
                return "%d" % n
            formatted = " ".join([f(n) for n in row])
            print(formatted)

    def is_left_edge(self, x, y):
        return x == 0


    def is_right_edge(self, x, y):
        return x == len(self._lines[0]) - 1


    def is_top_edge(self, x, y):
        return y == 0

    def is_bottom_edge(self, x, y):
        return y == len(self._lines) - 1


    def get_at(self, x, y):
        try:
            # I use setters / getters for maps since this
            # type of [y][x] vs [x][y] trips me up.
            return self._ints[y][x]
        except:
            return 888

    def get_basin_at(self, x, y):
        """
        A basin is all locations that eventually flow downward to a single low point.
        """
        try:
            return self._basins[y][x]
        except:
            return None   


    def set_basin_at(self, x, y, basin):
        """
        A basin is all locations that eventually flow downward to a single low point.
        """
        try:
            self._basins[y][x] = basin
        except:
            pass


    def is_low_point(self, x, y):
        """
        Top left is 0,0

        x = col
        y = row
        """
        at_location = self.get_at(x, y)

        return (at_location < self.get_at(x-1, y) and
                at_location < self.get_at(x+1, y) and
                at_location < self.get_at(x, y-1) and
                at_location < self.get_at(x, y+1))


class TestEntryPoint(unittest.TestCase):

    def test_part1(self):
        ms = Map(SAMPLE_STR)
        ms.find_low_points()
        self.assertEqual(15, ms._sum_risk)

        mi = Map(INPUT_STR)
        mi.find_low_points()
        self.assertNotEqual(0, mi._sum_risk)
        self.assertEqual(566, mi._sum_risk)
        

    def test_part2(self):
        ms = Map(SAMPLE_STR)
        ms.find_low_points()
        self.assertEqual(15, ms._sum_risk)
        ms.propagate_basins()
        ms.print_basins()
        sizes = ms.get_basin_sizes()
        print("Sizes are %s " % sizes)

        mulsizes = sizes[-1] * sizes[-2] * sizes[-3]
        self.assertEqual([9, 9, 14], sizes[-3:])
        self.assertEqual(mulsizes, 1134)

        mi = Map(INPUT_STR)
        mi.find_low_points()
        mi.propagate_basins()
        sizes = mi.get_basin_sizes()
        #print("Sizes are %s " % sizes)
        mulsizes = sizes[-1] * sizes[-2] * sizes[-3]
        self.assertEqual(mulsizes, 891684)

if __name__ == '__main__':
    unittest.main()
