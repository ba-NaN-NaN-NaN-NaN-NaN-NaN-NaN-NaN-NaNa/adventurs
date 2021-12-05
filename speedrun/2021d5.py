import unittest
import re

f = open("../input/2021_d5.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d5_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


class Segment:
    def __init__(self, toparse):
        toparse = toparse.replace("->", ",")
        toparse = toparse.replace(" ", "")
        toparse = toparse.replace(" ", "")
        toparse = toparse.replace(" ", "")
        toparse = toparse.replace(" ", "")
        parts = toparse.split(",")
        #print(parts)
        self.x1 = int(parts[0])
        self.y1 = int(parts[1])
        self.x2 = int(parts[2])
        self.y2 = int(parts[3])

        #print("%s becomes %d, %d -> %d, %d" % (parts, self.x1, self.y1, self.x2, self.y2))

    def __repr__(self):
        return "%d, %d -> %d, %d" % (self.x1, self.y1, self.x2, self.y2)


def initial_map(max_x, max_y):
    """
    Zeroes of given size.
    """
    toreturn = []
    for _ in range(0, max_y+1):
        single_line = [0] * (max_x+1) 
        toreturn.append(single_line)

    return toreturn


class VentMap:
    def __init__(self, lines):
        self._segments = [Segment(l) for l in lines]
        self._intfield = None
        

    def init_map(self):
        x, y = self.get_max_xy()
        self._intfield = initial_map(x, y)


    def get_danger_count(self):
        count = 0
        for row in self._intfield:
            for n in row:
                if n >= 2:
                    count += 1

        return count


    def apply_segment(self, seg):
        if seg.x1 == seg.x2:
            x = seg.x1
            y1 = seg.y1
            y2 = seg.y2
            if y2 < y1:
                y1, y2 = y2, y1

            y = y1
            y_stop = y2
            while y <= y_stop:
                self._intfield[y][x] += 1
                y += 1

        elif seg.y1 == seg.y2:
            #print("Seg is %s" % seg)
            y = seg.y1
            x1 = seg.x1
            x2 = seg.x2
            if x2 < x1:
                x1, x2 = x2, x1

            x = x1
            x_stop = x2
            while x <= x_stop:
                self._intfield[y][x] += 1
                x += 1

        else:
            pass
            # For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
            
            y1 = seg.y1
            y2 = seg.y2
            x1 = seg.x1
            x2 = seg.x2

            if x2 < x1:
                x1, x2 = x2, x1
                y1, y2 = y2, y1

            if abs(y2-y1) != abs(x2-x1):            
                raise TypeError("Out of bounds '%s'" % seg)

            for off in range(0, x2-x1+1):
                if y1 > y2:
                    # Negative slope
                    self._intfield[y1-off][x1+off] += 1
                else:
                    self._intfield[y1+off][x1+off] += 1


    def apply_all_segments(self):
        for seg in self._segments:
            self.apply_segment(seg)

    def print_map(self):
        for line in self._intfield:
            print(" ".join(["%2d" % n for n in line]))

    def get_max_xy(self):
        """
        Get maximum map size.
        """
        max_x = self._segments[0].x2
        max_y = self._segments[0].y2
        for seg in self._segments:
            if seg.x1 > max_x:
                #print("Segment %s gave new max x" % seg)
                max_x = seg.x1

            if seg.y1 > max_y:
                max_y = seg.y1

            if seg.x2 > max_x:
                #print("Segment %s gave new max x" % seg)
                max_x = seg.x2

            if seg.y2 > max_y:
                max_y = seg.y2

        return max_x, max_y



class TestStringMethods(unittest.TestCase):

    def tesxt_map_size(self):
        svm = VentMap(SAMPLE_STR)
        max_x, max_y = svm.get_max_xy()
        self.assertEqual(9, max_x)
        self.assertEqual(9, max_y)
        svm.init_map()
        svm.print_map()
        print("fd")
        svm.apply_all_segments()
        svm.print_map()

    def test_part1(self):
        svm = VentMap(SAMPLE_STR)
        ivm = VentMap(INPUT_STR)
        
        svm.init_map()
        svm.apply_all_segments()        

        ivm.init_map()
        ivm.apply_all_segments()        

        svm.print_map()
        svm_danger_count = svm.get_danger_count()
        self.assertEqual(12, svm_danger_count)

        ivm_danger_count = ivm.get_danger_count()
        self.assertEqual(16793, ivm_danger_count)


    def texst_map_basics(self):
        #svm = VentMap(["0,0 -> 5,5", "10,5 -> 5,5", "2,0 -> 5,0", "1,1 -> 4,4", "5,2 -> 5,4", "3,3 -> 3,3", "3,3 -> 4,3"])
        svm = VentMap(["0,0 -> 5,5", "10,5 -> 5,5", "8,3 -> 5,0"])
        max_x, max_y = svm.get_max_xy()
        self.assertEqual(10, max_x)
        self.assertEqual(5, max_y)
        svm.init_map()
        svm.print_map()
        svm.apply_all_segments()
        svm.print_map()
        
        


if __name__ == '__main__':
    unittest.main()
