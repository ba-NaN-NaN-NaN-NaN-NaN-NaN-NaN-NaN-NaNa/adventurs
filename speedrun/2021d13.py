import unittest

f = open("../input/2021_d13.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d13_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]



class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def fold_x(self, x):
        if self.x < x:
            return

        overshoot = self.x-x

        #print("Fold %d at %d gives %d overshoot" % (self.x, x, overshoot))
        self.x = x-overshoot

        pass
    def fold_y(self, y):
        if self.y < y:
            return

        overshoot = self.y-y

        #print("Fold %d at %d gives %d overshoot" % (self.y, y, overshoot))
        self.y = y-overshoot

        pass

class Paper:
    def __init__(self, lines):
        self._points = []
        self._lines = []
        self._folds = []
        for line in lines:
            if "," in line:
                parts = line.split(",")
                p = Point(int(parts[0]),int(parts[1]))
                self._points.append(p)

            else:
                l = line.replace('fold along', '').strip()
                parts = l.split("=")
                self._folds.append([parts[0], int(parts[1])])

    def do_folds(self):
        for fold in self._folds:
            print("Folding '%s'" % fold)
            if fold[0] == 'x':
                self.fold_x(fold[1])
            else:
                self.fold_y(fold[1])

    def get_at(self, x, y):
        """
        Top left is 0,0
        """
        row = self._lines[y]
        offset = x % self._width
        elem = row[offset:offset+1]
        return elem

    def is_in_paper(self, x, y):
        """
        Return true if x is in Paper.
        """
        return x < len(self._lines)

    def fold_x(self, x):
        print("x-folding paper %d" % x)
        for p in self._points:
            p.fold_x(x)

    def fold_y(self, y):
        print("y-folding paper %d" % y)
        for p in self._points:
            p.fold_y(y)

    def num_visible_points(self):
        coords = set()
        for p in self._points:
            coords.add("%d,%d" % (p.x, p.y))
        return len(coords)

    def print(self, width, height):
        toreturn = []
        for h in range(height):
            toreturn.append("." * width)


        for p in self._points:
            new_line = toreturn[p.y]
            new_line = new_line[0:p.x] + "#" + new_line[p.x+1:]
            toreturn[p.y] = new_line

        print("\n".join(toreturn))

class TestEntryPoint(unittest.TestCase):

    def texst_is_in_paper(self):
        ms = Paper(SAMPLE_STR)
        mi = Paper(INPUT_STR)
        self.assertTrue(ms.is_in_paper(0,0))
        self.assertTrue(mi.is_in_paper(0,0))
       
    def tesxt_ox_ratinxg(self):
        #print(INPUT_STR)
        self.assertEqual("10111", ox_rating(SAMPLE_STR))
        self.assertEqual("01010", co2_rating(SAMPLE_STR))

        self.assertEqual(part2(SAMPLE_STR), 230)
        self.assertEqual(part2(INPUT_STR), 5941884)


    def test_part1(self):
        ps = Paper(SAMPLE_STR)
        #ps.print(15,15)

        print("After y=7")
        ps.fold_y(7)
        ps.print(15,16)
        self.assertEqual(17, ps.num_visible_points())

        print("After x=5")
        ps.fold_x(5)
        ps.print(15,15)
        self.assertEqual(16, ps.num_visible_points())

        #   INPUT
        pi = Paper(INPUT_STR)
        pi.fold_x(655) # <- Fetched manually from sample text.
        self.assertEqual(827, pi.num_visible_points())

    def test_part2(self):
        pi = Paper(INPUT_STR)
        pi.fold_x(655) # <- Fetched manually from sample text.
        self.assertEqual(827, pi.num_visible_points())
               

        pi.do_folds()
        pi.print(55,8)

if __name__ == '__main__':
    unittest.main()
