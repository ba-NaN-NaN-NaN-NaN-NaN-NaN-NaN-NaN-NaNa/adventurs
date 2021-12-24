


import unittest
import re
import numpy as np

def readlines_from(fname):
    f = open(fname)
    lines = f.readlines()
    f.close()
    lines = [r.strip() for r in lines if len(r.strip()) > 0]
    return lines

INPUT_STR = readlines_from("../input/2021_d22.txt")
SAMPLE_STR = readlines_from("../input/2021_d22_sample.txt")

INPUT_STR2 = readlines_from("../input/2021_d22_p2.txt")
SAMPLE_STR2 = readlines_from("../input/2021_d22_p2_sample.txt")

line_matcher = re.compile("^(?P<new_state>on|off) x=(?P<x_min>[0-9\-]*)\.\.(?P<x_max>[0-9\-]*),y=(?P<y_min>[0-9\-]*)\.\.(?P<y_max>[0-9\-]*),z=(?P<z_min>[0-9\-]*)\.\.(?P<z_max>[0-9\-]*)$")

# Rename cuboid to volume. Or AABB?
# Try: get_all_x:es. Pre-slice every volume. This causes a 480x480x480 sparse key space.


def filter_part1(line):
    """
    Return trimmed line so that ranges are at most -50...50.

    Return empty string if nothing remains.
    """
    res = line_matcher.match(line)
    x_min = int(res['x_min'])
    x_max = int(res['x_max'])
    y_min = int(res['y_min'])
    y_max = int(res['y_max'])
    z_min = int(res['z_min'])
    z_max = int(res['z_max'])

    # Completely out-of-bounds.
    if x_max < -50:
        return ""
    if y_max < -50:
        return ""
    if z_max < -50:
        return ""

    if 50 < x_min:
        return ""
    if 50 < y_min:
        return ""
    if 50 < z_min:
        return ""

    # Just needs a trim.
    if 50 < x_max:
        x_max = 50
    if 50 < y_max:
        y_max = 50
    if 50 < z_max:
        z_max = 50

    if x_min < -50:
        x_min = -50
    if y_min < -50:
        y_min = -50
    if z_min < -50:
        z_min = -50

    new_line = "%s x=%d..%d,y=%d..%d,z=%d..%d" % (res['new_state'], x_min, x_max, y_min, y_max, z_min, z_max)
    if line != new_line:
        print("Altered line! '%s' -> '%s'")
    return new_line


def calc_intersect(chunk_a, chunk_b):
    x_min_a, x_max_a, y_min_a, y_max_a, z_min_a, z_max_a = chunk_a
    x_min_b, x_max_b, y_min_b, y_max_b, z_min_b, z_max_b = chunk_b

    if x_max_a < x_min_b:
        # a completely to left of b
        return None

    if x_max_b < x_min_a:
        # b completely to left of a
        return None

    if y_max_a < y_min_b:
        # a completely to left of b
        return None

    if y_max_b < y_min_a:
        # b completely to left of a
        return None

    if z_max_a < z_min_b:
        # a completely to left of b
        return None

    if z_max_b < z_min_a:
        # b completely to left of a
        return None

    x_min = max(x_min_a, x_min_b)
    x_max = min(x_max_a, x_max_b)
    y_min = max(y_min_a, y_min_b)
    y_max = min(y_max_a, y_max_b)
    z_min = max(z_min_a, z_min_b)
    z_max = min(z_max_a, z_max_b)

    return x_min, x_max, y_min, y_max, z_min, z_max

class Reactor:
    def __init__(self, lines):
        
        self._all_xs = None
        self._all_ys = None
        self._all_zs = None

        self._lit_chunks = set()
        self._lines = lines
        self.populate_all_chunks()
        #self.chunks = None
        #self.intersecting_chunks = None

    def populate_all_chunks(self):
        self.chunks = set()
        for line in self._lines:
            res = line_matcher.match(line)
            """
            xs.add(int(res['x_min']))
            xs.add(int(res['x_max']))
            ys.add(int(res['y_min']))
            ys.add(int(res['y_max']))
            zs.add(int(res['z_min']))
            zs.add(int(res['z_max']))
            """

            x_min = int(res['x_min'])
            x_max = int(res['x_max'])
            y_min = int(res['y_min'])
            y_max = int(res['y_max'])
            z_min = int(res['z_min'])
            z_max = int(res['z_max'])
            volume = (x_min, x_max, y_min, y_max, z_min, z_max)
            self.chunks.add(volume)


    def calculate_fenceposts(self, chunk):
        """
        Every x/y/z value (both lowest included and highest included)
        for chunk, as intersecting with all chunks.
        """
        xs = set()
        ys = set()
        zs = set()

        intersecting_chunks = set()
        for other in self.chunks:
            intersect = calc_intersect(chunk, other)
            if intersect:
                # Since we 1:1 intersect with outselves, fenceposts automatically 
                # contain values appropate for a cube without any intersections otherwise.
                #intersecting_chunks.add(intersect)
                intersecting_chunks.add(chunk)
                intersecting_chunks.add(other)
                #self.intersecting_chunks.add(chunk_b)


        # Old way. <- Why is this needed?
        for chunk_a in self.chunks:
            for chunk_b in self.chunks:
                intersect = calc_intersect(chunk_a, chunk_b)
                if intersect:
                    # Since we 1:1 intersect with outselves, fenceposts automatically 
                    # contain values appropate for a cube without any intersections otherwise.
                    #intersecting_chunks.add(intersect)
                    intersecting_chunks.add(chunk_a)
                    intersecting_chunks.add(chunk_b)


        print("Chunk %s intersected with %d other chunks" % (chunk, len(intersecting_chunks)))
        for x_min, x_max, y_min, y_max, z_min, z_max in intersecting_chunks:
            xs.add(x_min)
            xs.add(x_max)
            ys.add(y_min)
            ys.add(y_max)
            zs.add(z_min)
            zs.add(z_max)

        return sorted(list(xs)), sorted(list(ys)), sorted(list(zs))

    def line_into_xyz_ranges(self, line):
        """
        Parse and turn on all chunks in a line.
        """
        res = line_matcher.match(line)
        #if res['new_state'] != 'on':
        #    return 1/0
        x_min = int(res['x_min'])
        x_max = int(res['x_max'])
        y_min = int(res['y_min'])
        y_max = int(res['y_max'])
        z_min = int(res['z_min'])
        z_max = int(res['z_max'])

        """
        if x_max not in self._all_xs:
            raise TypeError("fdkd")
        if x_min not in self._all_xs:
            raise TypeError("fdkd")
        if y_max not in self._all_ys:
            raise TypeError("fdkd")
        if y_min not in self._all_ys:
            raise TypeError("fdkd")
        if z_max not in self._all_zs:
            raise TypeError("fdkd")
        if z_min not in self._all_zs:
            raise TypeError("fdkd")
        """

        chunk = x_min, x_max, y_min, y_max, z_min, z_max
        fencepost_xs, fencepost_ys, fencepost_zs = self.calculate_fenceposts(chunk)

        print("Checking fencepost_xs='%s'" % fencepost_xs)
        xs = [x for x in fencepost_xs if x_min <= x and x <= x_max]
        ys = [y for y in fencepost_ys if y_min <= y and y <= y_max]
        zs = [z for z in fencepost_zs if z_min <= z and z <= z_max]

        # Assume x-range for line is 5-15
        # and xs contains 5,9,10,15.
        # Then we want segments
        # 5-5, 6-8, 9-9, 10-10, 11-14, 15-15.
        #  ^         ^     ^             ^
        # Fenceposts must be added specifically.   <- Vast optimization potential here? Only add fenceposts for any coord that exists in any volume/volume intersection?
        # Without fencepost optimization we reach 96GB ram usage by row 145 / 420.

        #
        # Second vast optimization potential here? Calc intersect xs/ys/zs for _each_ line.
        #
        """
        endcap_x = xs[-1] + 1
        xs.append(endcap_x)

        endcap_y = ys[-1] + 1
        ys.append(endcap_y)

        endcap_z = zs[-1] + 1
        zs.append(endcap_z)
        """

        print("Fenceposts for line '%s' -> xs = %s, ys= %s, zs= %s" % (line, xs,ys,zs))

        xrange_pairs = set()
        yrange_pairs = set()
        zrange_pairs = set()

        xrange_pairs.add((xs[0],xs[0]))
        yrange_pairs.add((ys[0],ys[0]))
        zrange_pairs.add((zs[0],zs[0]))

        # Manually add since they might not exist in slice-induced fenceposts.
        xrange_pairs.add((x_min,x_min))
        xrange_pairs.add((x_max,x_max))
        yrange_pairs.add((y_min,y_min))
        yrange_pairs.add((y_max,y_max))
        zrange_pairs.add((z_min,z_min))
        zrange_pairs.add((z_max,z_max))

        for x_idx in range(len(xs)-1):
            x_min, x_max = xs[x_idx], xs[x_idx+1]
            xrange_pairs.add((x_min,x_min))
            xrange_pairs.add((x_max,x_max))

            insert_x_range = (x_max - x_min) > 1
            if insert_x_range:
                xrange_pairs.add((x_min+1, x_max-1))

        print("Xrange pairs '%s'" % str(xrange_pairs))


        for y_idx in range(len(ys)-1):
            y_min, y_max = ys[y_idx], ys[y_idx+1]
            yrange_pairs.add((y_min,y_min))
            yrange_pairs.add((y_max,y_max))

            insert_y_range = (y_max - y_min) > 1
            if insert_y_range:
                yrange_pairs.add((y_min+1, y_max-1))

        #print("Yrange pairs '%s'" % str(yrange_pairs))

        for z_idx in range(len(zs)-1):
            z_min, z_max = zs[z_idx], zs[z_idx+1]
            zrange_pairs.add((z_min,z_min))
            zrange_pairs.add((z_max,z_max))

            insert_z_range = (z_max - z_min) > 1
            if insert_z_range:
                zrange_pairs.add((z_min+1, z_max-1))

        #print("Zrange pairs '%s'" % str(zrange_pairs))

        return xrange_pairs, yrange_pairs, zrange_pairs

    def do_line(self, line):
        if line.startswith("on "):
            self.turn_on_line(line)
        else:
            self.turn_off_line(line)



    def turn_on_line(self, line):
        """
        Parse and turn on all chunks in a line.
        """
        xrange_pairs, yrange_pairs, zrange_pairs = self.line_into_xyz_ranges(line)
        for x_min, x_max in xrange_pairs:
            for y_min, y_max in yrange_pairs:
                for z_min, z_max in zrange_pairs:
                    volume = (x_min, x_max, y_min, y_max, z_min, z_max)
                    #print("Turning on %4d..%4d, %4d..%4d, %4d..%4d" % volume)
                    self._lit_chunks.add(volume)


    def turn_off_line(self, line):
        """
        Parse and turn on all chunks in a line.
        """
        xrange_pairs, yrange_pairs, zrange_pairs = self.line_into_xyz_ranges(line)
        for x_min, x_max in xrange_pairs:
            for y_min, y_max in yrange_pairs:
                for z_min, z_max in zrange_pairs:
                    volume = (x_min, x_max, y_min, y_max, z_min, z_max)
                    #print("Turning off %4d..%4d, %4d..%4d, %4d..%4d" % volume)
                    if volume in self._lit_chunks:
                        self._lit_chunks.remove(volume)


    def tally_all_on(self):
        """
        Sum all that are on.
        """
        toreturn = 0
        for x_min, x_max, y_min, y_max, z_min, z_max in self._lit_chunks:
            
            dx = x_max - x_min + 1
            dy = y_max - y_min + 1
            dz = z_max - z_min + 1
            vol = dx * dy * dz
            #print("Tally %4d..%4d, %4d..%4d, %4d..%4d -> %6d" % (x_min, x_max, y_min, y_max, z_min, z_max, vol))
            toreturn += vol

        return toreturn


class TestEntryPoint(unittest.TestCase):

    def texst_is_in_map(self):
        ms = Map(SAMPLE_STR, 1, 3)
        mi = Map(INPUT_STR, 1, 3)
        self.assertTrue(ms.is_in_map(0,0))
        self.assertTrue(mi.is_in_map(0,0))

    def texst_parse(self):
        rs = Reactor(SAMPLE_STR)
        ri = Reactor(INPUT_STR)

        # Sample segments are around 1.9 times the number of rows, which are 22.
        self.assertGreater(len(rs._all_xs), 30)
        self.assertGreater(len(rs._all_ys), 30)
        self.assertGreater(len(rs._all_zs), 30)

        self.assertGreater(44, len(rs._all_xs))
        self.assertGreater(44, len(rs._all_ys))
        self.assertGreater(44, len(rs._all_zs))
        

        # Sample segments are around 1.9 times the number of rows, which are 420.
        self.assertGreater(len(ri._all_xs), 700)
        self.assertGreater(len(ri._all_ys), 700)
        self.assertGreater(len(ri._all_zs), 700)

        self.assertGreater(844, len(ri._all_xs))
        self.assertGreater(844, len(ri._all_ys))
        self.assertGreater(844, len(ri._all_zs))


    def test_turn_on_line(self):
        rs = Reactor(SAMPLE_STR)

        """
        self.assertIn(-5, rs._all_xs)
        self.assertIn(2, rs._all_xs)

        self.assertIn(-6, rs._all_ys)
        self.assertIn(6, rs._all_ys)

        self.assertIn(-7, rs._all_zs)
        self.assertIn(7, rs._all_zs)
        """
        
        """
        rs.turn_on_line("on x=-5..2,y=-6..6,z=-2..3")
        dx =   2 - (-5) + 1
        dy =   6 - (-6) + 1
        dz =   3 - (-2) + 1
        expected = dx*dy*dz

        self.assertEqual(13, dy)
        self.assertEqual(expected, rs.tally_all_on())
        """

        # Span one dimension at a time. We need to '-2' due to common corner only counting once for 3 axles.
        dx = 1
        dy = 1
        dz = 1
        rs.turn_on_line("on x=-20..-20,y=-36..-36,z=-47..-47")
        self.assertEqual(dx, rs.tally_all_on())

        print("ENABLING SPAN OF X")
        dx = 20 + 26 + 1
        rs.turn_on_line("on x=-20..26,y=-36..-36,z=-47..-47")
        self.assertEqual(dx, rs.tally_all_on())

        dy = 36 + 1 + 17
        rs.turn_on_line("on x=-20..-20,y=-36..17,z=-47..-47")
        self.assertEqual(dx + dy - 1, rs.tally_all_on())

        dz = 47 + 1 + 7
        rs.turn_on_line("on x=-20..-20,y=-36..-36,z=-47..7")
        self.assertEqual(dx + dy + dz - 2, rs.tally_all_on())

        print("FOOOO")

        # This line completely overlaps the one above
        rs.turn_on_line("on x=-20..26,y=-36..17,z=-47..7")
        print("FOOOO")

        # Total on volume
        dx =  26 - (-20) + 1
        dy =  17 - (-36) + 1
        dz =   7 - (-47) + 1
        expected = dx*dy*dz
        print("Chunk turn on is dx,dy,dz=%d,%d,%d" % (dx,dy,dz))
        self.assertEqual(expected, rs.tally_all_on())
        

    def test_chunk_intersect(self):
        chunk_a =  (-20, 20, -20, 20, -20,   20)
        chunk_b =  (-15, 20,  20, 20, -200, 200)
        expected = (-15, 20,  20, 20, -20,   20)

        self.assertEqual(expected, calc_intersect(chunk_a, chunk_b))
        self.assertEqual(expected, calc_intersect(chunk_b, chunk_a))
        #return 1/0


    def test_part1(self):
        
        lines = [filter_part1(line) for line in SAMPLE_STR]
        lines = [l for l in lines if len(l) > 1]
        rs = Reactor(lines)
        for line in lines:
            rs.do_line(line)
        self.assertEqual(590784, rs.tally_all_on())


        lines = [filter_part1(line) for line in INPUT_STR]
        lines = [l for l in lines if len(l) > 1]
        rs = Reactor(lines)
        for line in lines:
            rs.do_line(line)
        self.assertEqual(590467, rs.tally_all_on())


    def test_part2(self):
        
        #lines = [filter_part1(line) for line in SAMPLE_STR]
        lines = SAMPLE_STR2
        lines = [l for l in lines if len(l) > 1]
        print("Part2 sample lines got %d lines" % len(lines))
        rs = Reactor(lines)
        for line in lines:
            rs.do_line(line)
        self.assertEqual(2758514936282235, rs.tally_all_on())

        return 1/0

        #lines = [filter_part1(line) for line in INPUT_STR]

        # Workaround here??? 
        # for xcrop(-2**30,-50k,0,50k,2**30)
        #   for ycrop..
        #      for zctop...
        #        lines_cropped = filter_crop(lines)
        #        Reactor(lines_cropped)
        #         sum_each_subreactor = rs.tally_all_on()
        # print(sum_each_subreactor)
        #
        lines = INPUT_STR2
        lines = [l for l in lines if len(l) > 1]
        print("Part2 Input lines got %d lines" % len(lines))
        rs = Reactor(lines)
        for line in lines:
            rs.do_line(line)
        self.assertEqual(5555, rs.tally_all_on())


if __name__ == '__main__':
    unittest.main()
