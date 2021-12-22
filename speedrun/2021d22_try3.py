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

line_matcher = re.compile("^(?P<new_state>on|off) x=(?P<x_min>[0-9\-]*)\.\.(?P<x_max>[0-9\-]*),y=(?P<y_min>[0-9\-]*)\.\.(?P<y_max>[0-9\-]*),z=(?P<z_min>[0-9\-]*)\.\.(?P<z_max>[0-9\-]*)$")


def sub_cuboid(existing, new_cuboid):
    """
    Given a list of existing cuboids, what is left if we remove a new_cuboid?
    """
    worklist = existing[:]
    toreturn = []
    while len(worklist) > 0:
        exists = worklist.pop(0)
    


def add_cuboid(existing, new_cuboid):
    return existing

class World:
    def __init__(self, lines):
        #off x=-40..-22,y=-38..-28,z=23..41

        cuboids = []
        for line in lines:
            cuboid = Cuboid.from_line(line)
            cuboids.append(cuboid)
        self._cuboids = cuboids
        self._part1_ans = None

        for n in range(len(self._cuboids)-1):
            self._cuboids[n].next_step = self._cuboids[n+1]

    def part1(self):
        to_perform = [c.crop(-50, 50, -50, 50, -50, 50) for c in self._cuboids]
        to_perform = [c for c in to_perform if c is not None]

        worklist = []
        for c in to_perform:
            worklist = add_cuboid(worklist, c)
            print("Part1 step now has %d cuboids" % len(worklist))
        
        lights = [c.volume() for c in worklist]
        return sum(lights)



    def part2(self):
        pass

    def part1_bad(self):
        """
        Union/intersect etc fails.

        Quickly reaches 100k+ cuboids and grinds to halt.
        """
        worklist = set([self._cuboids[0]])
        for c in self._cuboids[1:]:
            worklist = self.apply_a_cuboid(worklist, c)

        lights_on = 0
        for c in worklist:
            lights_on += c.volume()

        return lights_on

    def part1_bad2(self):
        """
        Volumetric overlap fails? Problem with sub-children?
        """
        return self.lights_on(-50, 50, -50, 50, -50, 50)


    

    def lights_on(self, x_min, x_max, y_min, y_max, z_min, z_max):
        return self._cuboids[0].lights_on(x_min, x_max, y_min, y_max, z_min, z_max)

    def apply_a_cuboid(self, existing_cuboids, new_cuboid):
        toreturn = set()
        for exist in existing_cuboids:
            if new_cuboid._new_state == 'on':
                to_add = exist.union_with(new_cuboid)
                toreturn = toreturn | set(to_add)
            elif new_cuboid._new_state == 'off':
                to_add = exist.sub_other(new_cuboid)
                toreturn = toreturn | set(to_add)
            else:
                raise TypeError("jfd")

        print("Apply a cuboid went from %d cuboids in world to %d" % (len(existing_cuboids), len(toreturn)))

        try_merge = True

        toreturn = sorted(list(toreturn))

        while try_merge:
            try_merge = False
            n = 0
            while n < len(toreturn)-1:
                cub0 = toreturn[n]
                cub1 = toreturn[n+1]
                merge_res = cub0.try_merge(cub1)
                if merge_res is None:
                    n += 1
                else:
                    toreturn = toreturn[0:n] + [merge_res] + toreturn[n+2:]
                    try_merge = True


        #print("Apply a cuboid '%s' went from cuboid set '%s' in world to set '%s'" % (new_cuboid, existing_cuboids, toreturn))
        return toreturn

        


def cuboid_repr(new_state, x_min, x_max, y_min, y_max, z_min, z_max):
    return "%s: x=%d..%d,y=%d..%d,z=%d..%d" % (new_state, x_min, x_max, y_min, y_max, z_min, z_max)



class Cuboid:


    def __hash__(self):
        return hash(self.__str__())

    @classmethod
    def from_line(cls, line):

        parse_res = line_matcher.match(line)
        if parse_res is None:
            raise TypeError("Unparseable '%s'" % line)
        #print(parse_res.groupdict())
        new_state = parse_res['new_state']
        x_min = parse_res['x_min']
        x_max = parse_res['x_max']
        y_min = parse_res['y_min']
        y_max = parse_res['y_max']
        z_min = parse_res['z_min']
        z_max = parse_res['z_max']

        x_min = int(x_min)
        x_max = int(x_max)
        y_min = int(y_min)
        y_max = int(y_max)
        z_min = int(z_min)
        z_max = int(z_max)

        if x_max < x_min:
            raise TypeError("kfldhfds xfor line '%s'" % line)
        if y_max < y_min:
            raise TypeError("kfldhfds yfor line '%s'" % line)
        if z_max < z_min:
            raise TypeError("kfldhfds zfor line '%s'" % line)

        return Cuboid(new_state, x_min, x_max, y_min,y_max,z_min,z_max)
        

    def __init__(self, new_state, 
                 x_min, x_max, y_min,y_max,z_min,z_max):
        """
        Parse a line.
        """
        key = cuboid_repr(new_state, x_min, x_max, y_min, y_max, z_min, z_max)
        self._repr = key

        self._new_state = new_state
        self._x_min = x_min
        self._x_max = x_max
        self._y_min = y_min
        self._y_max = y_max
        self._z_min = z_min
        self._z_max = z_max

        self.next_step = None # Next row in reboot steps.
        #self._cubes_on_memo = {}

    def volume(self):
        """
        A.k.a. volume.
        """
        dx = self._x_max - self._x_min + 1
        dy = self._y_max - self._y_min + 1
        dz = self._z_max - self._z_min + 1

        #print("self._x_max %d - self._x_min %d -> dx %d" % (self._x_max, self._x_min, dx))  
        #print("dx,dy,dz = (%d,%d,%d)" % (dx,dy,dz))

        self._num_cuboids = dx * dy * dz

        return self._num_cuboids

    def get_at(self, x, y):
        """
        Top left is 0,0
        """
        row = self._lines[y]
        offset = x % self._width
        elem = row[offset:offset+1]
        return elem

    def is_in_map(self, x, y):
        """
        Return true if x is in map.
        """
        return x < len(self._lines)

    def crop(self, x_min, x_max, y_min, y_max, z_min, z_max):
        """
        Crop cube.

        Return None if nothing is left.
        """
        new_x_min = self._x_min
        new_x_max = self._x_max
        new_y_min = self._y_min
        new_y_max = self._y_max
        new_z_min = self._z_min
        new_z_max = self._z_max

        if new_x_min < x_min:
            new_x_min = x_min
        if new_y_min < y_min:
            new_y_min = y_min
        if new_z_min < z_min:
            new_z_min = z_min

        if x_max < new_x_max:
            new_x_max = x_max
        if y_max < new_y_max:
            new_y_max = y_max
        if z_max < new_z_max:
            new_z_max = z_max

        if x_max < x_min:
            return None
        if y_max < y_min:
            return None
        if z_max < z_min:
            return None

        return Cuboid(self._new_state, new_x_min, new_x_max, new_y_min, new_y_max, new_z_min, new_z_max)

    
    def lights_on(self):
        if self.next_step is not None:
            raise TypeError("Not supported")

        return self.volume()


    def lighxts_on(self, x_min, x_max, y_min, y_max, z_min, z_max):
        """
        Arguments are boundaries to consider.
        """
        memo_key = "%d,%d,%d,%d,%d,%d" % (x_min, x_max, y_min, y_max, z_min, z_max)
        if memo_key in self._cubes_on_memo:
            return self._cubes_on_memo[memo_key]

        cropped_self = self.crop(x_min, x_max, y_min, y_max, z_min, z_max)
        cropped_next = None
        if self.next_step is not None:
            cropped_next = self.next_step.crop(x_min, x_max, y_min, y_max, z_min, z_max)

        print("lights_on(%d,%d,%d,%d,%d,%d) self='%s', cropped_next='%s'  " % (x_min, x_max, y_min, y_max, z_min, z_max, cropped_self, cropped_next))

        if cropped_self is None:
            if cropped_next is None:
                # Neither I nor next step exists within boundary.
                print("Neither exist in boundary, returning 0")
                self._cubes_on_memo[memo_key] = 0
                return 0
            else:
                # Only next step exists within boundary.
                res = cropped_next.lights_on(x_min, x_max, y_min, y_max, z_min, z_max)
                self._cubes_on_memo[memo_key] = res
                print("Only other exists in boundary. Returning %d" % res)
                return res

        else: # cropped_self is valid
            if cropped_next is None:
                # Only I exist within boundary. All on-cubes match my own state.
                if cropped_self._new_state == 'on':
                    to_set = cropped_self.volume()
                    print("Only I (%s) exist. Lights in boundary = %d" % (self, to_set))
                    self._cubes_on_memo[memo_key] = to_set
                    return self._cubes_on_memo[memo_key]

                elif cropped_self._new_state == 'off':
                    print("Only I (%s) exist, but am off. 0 lights in boundary." % (self))
                    self._cubes_on_memo[memo_key] = 0
                    return 0
                else:
                    raise TypeError("Self neither 'on' nor 'off'?? %s" % cropped_self._new_state)

            else:
                # Both of us exist within boundary. Tricky here.
                if cropped_self._new_state == 'off':
                    # I am off. Any cubes on are due to other cube.
                    to_set = cropped_next.lights_on(x_min, x_max, y_min, y_max, z_min, z_max)
                    print("Both (%s and %s) exist, but I am off. Other has %d lights." % (cropped_self, cropped_next, to_set))
                    self._cubes_on_memo[memo_key] = to_set
                    return self._cubes_on_memo[memo_key]

                elif cropped_self._new_state == 'on':
                    # I am on. Return: a few cases
                    #
                    # +----+    +-----+
                    # | me |    | next|  <- No overlap.
                    # +----+    +-----+
                    #
                    # +-----+----+------+
                    # | me' | I  | next'|    <- Some overlap.
                    # +-----+----+------+
                    #
                    # In the second case, the lights in the following volumes are equal:
                    #  - I+next'
                    #  - next
                    # 
                    intersect = cropped_self.intersect(cropped_next)

                    #print("Calculating intersect of cropped_self=%s, cropped_next=%s, intersect=%s " % (cropped_self, cropped_next, intersect))
                    if intersect is None:
                        lights_self = cropped_self.volume()
                        lights_next = cropped_next.lights_on(x_min, x_max, y_min, y_max, z_min, z_max)
                        lights_total = lights_self + lights_next
                        print("NO intersect. my contribution=%d, next contribution=%d -> %d" % (lights_self, lights_next, lights_total))
                        self._cubes_on_memo[memo_key] = lights_total
                        return self._cubes_on_memo[memo_key]

                    else:
                        lights_only_self = cropped_self.volume() - intersect.volume()
                        print("lights_only_self=%d <- cropped_self.volume()=%d - intersect.volume()=%d" % (lights_only_self, cropped_self.volume(),intersect.volume()))
                        lights_next = self.next_step.lights_on(x_min, x_max, y_min, y_max, z_min, z_max)
                        lights_total = lights_only_self + lights_next
                        print("INTERSECT - lights_total=%d <- lights_only_self=%d + lights_next=%d" % (lights_total, lights_only_self,lights_next))
                        self._cubes_on_memo[memo_key] = lights_total
                        return self._cubes_on_memo[memo_key]


                else:
                    raise TypeError("Self neither 'on' nor 'off'?? %s" % cropped_self._new_state)





    def slice_x(self, x):
        """
        Slicing will split such that 
        _RIGHT_ cube will include x.
        """
        #print("Will slice x at %d" % x)

        if x == self._x_min:
            # Edge case., but we only return self.
            return [None, self]

        if x > self._x_max:
            # Split is to the right of us. We are 100% on the left.
            return [self, None]

        if x < self._x_min:
            # Split is to the left of us. We are 100% on the right.
            return [None, self]

        cub0 = Cuboid(self._new_state, self._x_min, x-1,         self._y_min, self._y_max, self._z_min, self._z_max)
        cub1 = Cuboid(self._new_state, x,           self._x_max, self._y_min, self._y_max, self._z_min, self._z_max)
        return [cub0, cub1]

    def slice_y(self,y):
        if y == self._y_min:
            # Edge case., but we only return self.
            return [None, self]

        if y > self._y_max:
            # Split is to the right of us. We are 100% on the left.
            return [self, None]

        if y < self._y_min:
            # Split is to the left of us. We are 100% on the right.
            return [None, self]

        cub0 = Cuboid(self._new_state, self._x_min, self._x_max, self._y_min,         y-1, self._z_min, self._z_max)
        cub1 = Cuboid(self._new_state, self._x_min, self._x_max, y,           self._y_max, self._z_min, self._z_max)
        return [cub0, cub1]

    def slice_z(self,z):
        if z == self._z_min:
            # Edge case., but we only return self.
            return [None, self]

        if z > self._z_max:
            # Split is to the right of us. We are 100% on the left.
            return [self, None]

        if z < self._z_min:
            # Split is to the left of us. We are 100% on the right.
            return [None, self]

        cub0 = Cuboid(self._new_state, self._x_min, self._x_max, self._y_min, self._y_max, self._z_min,         z-1)
        cub1 = Cuboid(self._new_state, self._x_min, self._x_max, self._y_min, self._y_max,           z, self._z_max)
        return [cub0, cub1]




    def __repr__(self):
        return self.__str__()

    def __str__(self):
        return self._repr
        
    def intersect(self, other):
        """

        """
        return self.crop(other._x_min, other._x_max, other._y_min, other._y_max, other._z_min, other._z_max)


    def __eq__(self, other):
        if self._x_min != other._x_min:
            return False
        if self._x_max != other._x_max:
            return False
        if self._y_min != other._y_min:
            return False
        if self._y_max != other._y_max:
            return False
        if self._x_min != other._z_min:
            return False
        if self._z_max != other._z_max:
            return False

        return self.__str__() == other.__str__()

    def __lt__(self, other):
        if self._x_min != other._x_min:
            return True
        if self._x_max != other._x_max:
            return True
        if self._y_min != other._y_min:
            return True
        if self._y_max != other._y_max:
            return True
        if self._x_min != other._z_min:
            return True
        if self._z_max != other._z_max:
            return True
        
        return False


    def sub(self, other):
        """
        Sub one cuboid from another.
        """
        pass
           

class TestEntryPoint(unittest.TestCase):

    def texst_is_in_map(self):
        ms = Map(SAMPLE_STR, 1, 3)
        mi = Map(INPUT_STR, 1, 3)
        self.assertTrue(ms.is_in_map(0,0))
        self.assertTrue(mi.is_in_map(0,0))
       
    def tesxt_ox_ratinxg(self):
        #print(INPUT_STR)
        self.assertEqual("10111", ox_rating(SAMPLE_STR))
        self.assertEqual("01010", co2_rating(SAMPLE_STR))

        self.assertEqual(part2(SAMPLE_STR), 230)
        self.assertEqual(part2(INPUT_STR), 5941884)

    def tesxt_cuboid_volume(self):
        cuboid = Cuboid.from_line("on x=10..10,y=10..10,z=10..10")
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=0..0,y=0..0,z=0..0")
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=1..1,y=1..1,z=1..1")
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=-1..-1,y=-1..-1,z=-1..-1")
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1")
        self.assertEqual(cuboid.volume(), 3*3*3)


        cuboid = Cuboid.from_line("on x=10..12,y=10..12,z=10..12")
        self.assertEqual(cuboid.volume(), 27)

        cuboid = Cuboid.from_line("on x=-54112..-39298,y=-85059..-49293,z=-27449..7877")
        self.assertEqual(cuboid.volume(), 0)


        cuboid = Cuboid.from_line("on x=-110..-110,y=10..10,z=10..10")
        self.assertEqual(cuboid.volume(), 0)
        cuboid = Cuboid.from_line("on x=110..110,y=10..10,z=10..10")
        self.assertEqual(cuboid.volume(), 0)

        cuboid = Cuboid.from_line("on x=-110..-110,y=10..10,z=10..10")
        self.assertEqual(cuboid.volume(), 1)
        cuboid = Cuboid.from_line("on x=110..110,y=10..10,z=10..10")
        self.assertEqual(cuboid.volume(), 1)

    def texst_cubes_on(self):
        cuboid = Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1")
        self.assertEqual(3*3*3, cuboid.lights_on(-1, 1, -1, 1, -1, 1))
        self.assertEqual(3*3*3, cuboid.lights_on(-10, 10, -10, 10, -10, 10))
        self.assertEqual(2*2*2, cuboid.lights_on(0, 1, 0, 1, 0, 1))

        self.assertEqual(1*1*1, cuboid.lights_on(1,4,1,5,1,6))
        self.assertEqual(1*1*1, cuboid.lights_on(-4, -1,-6,-1,-7,-1))

        # Recreate to dump memoization cache
        cuboid = Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1")
        cuboid.next_step = Cuboid.from_line("off x=1..5,y=1..5,z=1..5")
        self.assertEqual(3*3*3 - 1, cuboid.lights_on(-1, 1, -1, 1, -1, 1))

        # Recreate to dump memoization cache
        cuboid = Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1")
        cuboid.next_step = Cuboid.from_line("off x=2..5,y=1..5,z=1..5") # Outsize of area.
        self.assertEqual(3*3*3, cuboid.lights_on(-1, 1, -1, 1, -1, 1))

        # Recreate to dump memoization cache
        cuboid = Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1")
        cuboid.next_step = Cuboid.from_line("on x=0..5,y=0..0,z=0..0") # Overlaps in centre, sticks out at end.
        self.assertEqual(3*3*3 + 4 , cuboid.lights_on(-10, 10, -10, 10, -10, 10))

    def test_sub(self):
        cuboid = Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1")
        frags = cuboid.sub(Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1"))
        self.assertEqual(0, len(frags))


    def test_part1(self):
        non_overlapping = """on x=0..0,y=0..0,z=0..0
on x=1..1,y=0..0,z=0..0
on x=2..2,y=0..0,z=0..0
on x=3..3,y=0..0,z=0..0
on x=4..4,y=0..0,z=0..0""".split("\n")
        non_overlapping = [s.strip() for s in non_overlapping]

        print("TEST PART 1 -> 5")
        ws = World(non_overlapping)
        self.assertEqual(5, ws.part1())

        #return 1/0

        print("TEST PART 10 -> 8")
        overlapping = """on x=0..9,y=0..0,z=0..0
off x=4..5,y=-1..0,z=0..0""".split("\n")
        overlapping = [s.strip() for s in overlapping]
        ws = World(overlapping)
        self.assertEqual(10-2, ws.part1())

        print("TEST PART 10 - 2 - 1 -> 7")
        overlapping = """on x=0..9,y=0..0,z=0..0
off x=4..5,y=0..1,z=0..0
off x=5..6,y=0..1,z=0..0""".split("\n")
        # On = size 20.
        # Off = size 4, but only 4 overlap.
        overlapping = [s.strip() for s in overlapping]
        ws = World(overlapping)
        self.assertEqual(7, ws.part1())

        #return 1/0

        ws = World(SAMPLE_STR)
        self.assertEqual(590784, ws.part1())

        wi = World(INPUT_STR)
        self.assertEqual(666, wi.part1())

    def test_part2(self):
        ws = World(SAMPLE_STR)
        pass

if __name__ == '__main__':
    unittest.main()
