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

CUBOID_CACHE = {}

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
            self._cuboids[n].next = self._cuboids[n+1]

    def part1_bad(self):
        """
        Union/intersect etc fails.

        Quickly reaches 100k+ cuboids.
        """
        worklist = set([self._cuboids[0]])
        for c in self._cuboids[1:]:
            worklist = self.apply_a_cuboid(worklist, c)

        lights_on = 0
        for c in worklist:
            lights_on += c.volume()

        return lights_on

    def part1(self):

        return self.cubes_on(-50, 50, -50, 50, -50, 50)

    def cubes_on(self, x_min, x_max, y_min, y_max, z_min, z_max):
        return self._cuboids[0].cubes_on(x_min, x_max, y_min, y_max, z_min, z_max)


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

        
    

        

def crop50(n):
    if n < -50:
        return -50

    if n > 50:
        return 50

    return n

def any_same_boundaries(one, many):
    for c in many:
        if one.same_boundaries(c):
            return True
    return False


def cuboid_repr(new_state, x_min, x_max, y_min, y_max, z_min, z_max, limit50):
    return "%s: x=%d..%d,y=%d..%d,z=%d..%d,%s" % (new_state, x_min, x_max, y_min, y_max, z_min, z_max, limit50)



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

        key = cuboid_repr(new_state, x_min, x_max, y_min, y_max, z_min, z_max)
        return Cuboid.from_params(new_state, x_min, x_max, y_min,y_max,z_min,z_max)

    def from_params(new_state, x_min, x_max, y_min,y_max,z_min,z_max, limit50):
        key = cuboid_repr(new_state, x_min, x_max, y_min, y_max, z_min, z_max, limit50)
        if key not in CUBOID_CACHE:
            return Cuboid(new_state, x_min, x_max, y_min, y_max, z_min, z_max, limit50)

        #return CUBOID_CACHE[key]
        

    def __init__(self, new_state, 
                 x_min, x_max, y_min,y_max,z_min,z_max,
                 limit50):
        """
        Parse a line.
        Limit to 50?
        """
        key = cuboid_repr(new_state, x_min, x_max, y_min, y_max, z_min, z_max, limit50)
        #if key in CUBOID_CACHE: # <- Disable cacheing for chaining purposes
        #    raise TypeError("Duplicate construction called") # <- Disable cacheing for chaining purposes
        self._repr = key
        # "%s: x=%d..%d,y=%d..%d,z=%d..%d" % (self._new_state, self._x_min, self._x_max, self._y_min, self._y_max, self._z_min, self._z_max,)

        self._new_state = new_state
        self._x_min = x_min
        self._x_max = x_max
        self._y_min = y_min
        self._y_max = y_max
        self._z_min = z_min
        self._z_max = z_max

        self._out_of_bounds = False

        if limit50:

            if (50 < self._x_min or
                50 < self._y_min or
                50 < self._z_min or
                self._x_max < -50 or
                self._y_max < -50 or
                self._z_max < -50):
                self._out_of_bounds = True


            self._x_min = crop50(self._x_min)
            self._x_max = crop50(self._x_max)
            self._y_min = crop50(self._y_min)
            self._y_max = crop50(self._y_max)
            self._z_min = crop50(self._z_min)
            self._z_max = crop50(self._z_max)

        self._limit50 = limit50
        self._num_cuboids = None
        #CUBOID_CACHE[key] = self # <- Disable cacheing for chaining purposes

    def volume(self):
        """
        A.k.a. volume.
        """
        if self._out_of_bounds:
            return 0

        if self._num_cuboids is not None:
            return self._num_cuboids

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


    def slice_x(self, x):
        """
        Slicing will split such that 
        _RIGHT_ cube will include x.
        """
        #print("Will slice x at %d" % x)

        if x == self._x_min:
            # Edge case., but we only return self.
            return [self]

        if x > self._x_max:
            # Ignore.
            return [self]

        if x < self._x_min:
            return [self]

        cub0 = Cuboid.from_params(self._new_state, self._x_min, x-1,         self._y_min, self._y_max, self._z_min, self._z_max, self._limit50)
        cub1 = Cuboid.from_params(self._new_state, x,           self._x_max, self._y_min, self._y_max, self._z_min, self._z_max, self._limit50)
        return [cub0, cub1]

    def slice_y(self,y):
        if y == self._y_min:
            return [self]

        if y > self._y_max:
            return [self]

        if y < self._y_min:
            return [self]

        cub0 = Cuboid.from_params(self._new_state, self._x_min, self._x_max, self._y_min,         y-1, self._z_min, self._z_max, self._limit50)
        cub1 = Cuboid.from_params(self._new_state, self._x_min, self._x_max, y,           self._y_max, self._z_min, self._z_max, self._limit50)
        return [cub0, cub1]

    def slice_z(self,z):
        if z == self._z_min:
            return [self]

        if z > self._z_max:
            return [self]

        if z < self._z_min:
            return [self]

        cub0 = Cuboid.from_params(self._new_state, self._x_min, self._x_max, self._y_min, self._y_max, self._z_min,         z-1, self._limit50)
        cub1 = Cuboid.from_params(self._new_state, self._x_min, self._x_max, self._y_min, self._y_max,           z, self._z_max, self._limit50)
        return [cub0, cub1]


    def slice(self, x, y, z):
        """
        Slice self in x,y,z ranges.
        """
        done_x = self.slice_x(x)
        done_xy = []
        for cuboid in done_x:
            done_xy += cuboid.slice_y(y)

        done_xyz = []
        for cuboid in done_xy:
            done_xyz += cuboid.slice_z(z)

        return done_xyz


    def try_merge(self, other):
        """
        Need to do this due to crazy explosion.
        """
        if self._new_state != other._new_state:
            return None

        if self._limit50 != other._limit50:
            return None

        # Try merge in x-axle.
        if (self._y_min == other._y_min and
            self._y_min == other._y_max and
            self._z_max == other._z_min and
            self._z_max == other._z_max):
            x_vals = sorted([self._x_min, self._x_max, other._x_min, other._x_min])
            if x_vals[1] <= x_vals[2]:
                return Cuboid.from_params(self._new_state, x_vals[0], x_vals[3], self._y_min, self._y_max, self._z_min, self._z_max, self._limit50)

        # Try merge in y-axle.
        if (self._x_min == other._x_min and
            self._x_min == other._x_max and
            self._z_max == other._z_min and
            self._z_max == other._z_max):
            y_vals = sorted([self._y_min, self._y_max, other._y_min, other._y_min])
            if y_vals[1] <= y_vals[2]:
                return Cuboid.from_params(self._new_state,  self._x_min, self._x_max, y_vals[0], y_vals[3], self._z_min, self._z_max, self._limit50)


        # Try merge in z-axle.
        if (self._y_min == other._y_min and
            self._y_min == other._y_max and
            self._x_max == other._x_min and
            self._x_max == other._x_max):
            z_vals = sorted([self._z_min, self._z_max, other._z_min, other._z_min])
            if z_vals[1] <= z_vals[2]:
                return Cuboid.from_params(self._new_state, self._x_min, self._x_max, self._y_min, self._y_max, z_vals[0], z_vals[3], self._limit50)


        return None





    def union_with(self, other):
        """
        Union two on-cuboids.

        Both cuboids must be 'on'.
        """
        if self._new_state != 'on':
            raise TypeError("fgljdhghfsd")
        if other._new_state != 'on':
            raise TypeError("fgljdhghfsd")

        # Slice myself to have subcuboids with boundaries matching other cuboid.
        # We need to do this twice: To match each opposing corner in the other cuboid.
        worklist = self.slice(other._x_min, other._y_min, other._z_min)
        my_cuboids = []
        for c in worklist:
            my_cuboids += c.slice(other._x_max+1, other._y_max+1, other._z_max+1)


        # Slice other to have subcuboids with boundaries matching self.
        worklist = other.slice(self._x_min, self._y_min, self._z_min)
        other_cuboids = []
        for c in worklist:
            other_cuboids += c.slice(self._x_max+1, self._y_max+1, self._z_max+1)


        toreturn = my_cuboids[:]
        for frag_other in other_cuboids:
            if any_same_boundaries(frag_other, toreturn):
                pass
            else:
                toreturn.append(frag_other)

        return toreturn

    def sub(self, other):
        """
        Sub one cuboid from another.
        """
        # Slice myself to have subcuboids with boundaries matching other cuboid.
        # We need to do this twice: To match each opposing corner in the other cuboid.
        worklist = self.slice(other._x_min, other._y_min, other._z_min)
        my_cuboids = []
        for c in worklist:
            my_cuboids += c.slice(other._x_max+1, other._y_max+1, other._z_max+1)

        # Slice other to have subcuboids with boundaries matching self.
        worklist = other.slice(self._x_min, self._y_min, self._z_min)
        other_cuboids = []
        for c in worklist:
            other_cuboids += c.slice(self._x_max+1, self._y_max+1, self._z_max+1)

        toreturn = []
        for frag_self in my_cuboids:
            if any_same_boundaries(frag_self, other_cuboids):
                pass
            else:
                toreturn.append(frag_self)

        return toreturn


    def same_boundaries(self, other):
        """
        Returns true of two cubes occupy same location (i.e. both same volume and position in world)
        """
        if self._x_min != other._x_min:
            return False

        if self._x_max != other._x_max:
            return False

        if self._y_min != other._y_min:
            return False

        if self._y_max != other._y_max:
            return False

        if self._z_min != other._z_min:
            return False

        if self._z_max != other._z_max:
            return False

        return True


    def __repr__(self):
        return self.__str__()

    def __str__(self):
        return self._repr
        


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
        cuboid = Cuboid.from_line("on x=10..10,y=10..10,z=10..10", limit50=True)
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=0..0,y=0..0,z=0..0", limit50=True)
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=1..1,y=1..1,z=1..1", limit50=True)
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=-1..-1,y=-1..-1,z=-1..-1", limit50=True)
        self.assertEqual(cuboid.volume(), 1)

        cuboid = Cuboid.from_line("on x=-1..1,y=-1..1,z=-1..1", limit50=True)
        self.assertEqual(cuboid.volume(), 3*3*3)


        cuboid = Cuboid.from_line("on x=10..12,y=10..12,z=10..12", limit50=True)
        self.assertEqual(cuboid.volume(), 27)

        cuboid = Cuboid.from_line("on x=-54112..-39298,y=-85059..-49293,z=-27449..7877", limit50=True)
        self.assertEqual(cuboid.volume(), 0)


        cuboid = Cuboid.from_line("on x=-110..-110,y=10..10,z=10..10", limit50=True)
        self.assertEqual(cuboid.volume(), 0)
        cuboid = Cuboid.from_line("on x=110..110,y=10..10,z=10..10", limit50=True)
        self.assertEqual(cuboid.volume(), 0)

        cuboid = Cuboid.from_line("on x=-110..-110,y=10..10,z=10..10", limit50=False)
        self.assertEqual(cuboid.volume(), 1)
        cuboid = Cuboid.from_line("on x=110..110,y=10..10,z=10..10", limit50=False)
        self.assertEqual(cuboid.volume(), 1)

    def test_cuboid_slice(self):
        cuboid = Cuboid.from_line("on x=10..10,y=10..10,z=10..10", limit50=True)
        sliced = cuboid.slice_x(10)
        self.assertEqual(1, len(sliced))

        cuboid = Cuboid.from_line("on x=10..10,y=10..10,z=10..10", limit50=True)
        sliced = cuboid.slice_x(20)
        self.assertEqual(1, len(sliced))


        # Slice at right place? See graphic
        cuboid = Cuboid.from_line("on x=-1..1,y=1..1,z=1..1", limit50=True)
        self.assertEqual(3, cuboid.volume())
        cuboid = Cuboid.from_line("on x=-1..4,y=1..1,z=1..1", limit50=True)
        self.assertEqual(6, cuboid.volume())
        sliced = cuboid.slice_x(2)
        self.assertEqual(2, len(sliced))
        self.assertEqual(3, sliced[0].volume())
        self.assertEqual(3, sliced[1].volume())

        # Slice at right place? Slice at 3
        cuboid = Cuboid.from_line("on x=-10..10,y=-10..10,z=-10..10", limit50=True)
        self.assertEqual(21*21*21, cuboid.volume()) # <- Quick check.

        # After this sliced_x = -10..2 and 3..10
        sliced = cuboid.slice_x(3)
        self.assertEqual(2, len(sliced))

        cub0, cub1 = sliced[0], sliced[1]
        if cub0._x_min > cub1._x_min:
            cub1, cub0 = cub0, cub1

        self.assertEqual(cub0._x_max, 2)
        self.assertEqual(cub1._x_min, 3)
        self.assertEqual(13*21*21, cub0.volume())
        self.assertEqual(8*21*21, cub1.volume())


        # Slice into even parts.

    def test_slice_xyz(self):
        cuboid = Cuboid.from_line("on x=-3..3,y=-3..3,z=-3..3", limit50=True)

        # See graphic 
        sliced = cuboid.slice(20, 0, 1)
        volumes = sorted([c.volume() for c in sliced])
        expected = sorted([4*4*7, 3*3*7, 3*4*7,3*4*7,])
        self.assertEqual(expected, volumes)
        #print(volumes)
        #return 1/0


        cuboid = Cuboid.from_line("on x=-2..1,y=-2..1,z=-2..1", limit50=True)

        # See graphic 
        sliced = cuboid.slice(0, 0, 0)
        volumes = sorted([c.volume() for c in sliced])
        expected = sorted([2*2*2]*8)
        self.assertEqual(expected, volumes)


    def test_union(self):

        # Simple-ish case.
        a = Cuboid.from_line("on x=0..1,y=0..1,z=0..1", limit50=True)
        b = Cuboid.from_line("on x=2..3,y=0..1,z=0..1", limit50=True)
        union_results = a.union_with(b) # Zero overlap.
        self.assertEqual(2, len(union_results))
        sum_volumes = sum([c.volume() for c in union_results])
        self.assertEqual(4*2*2, sum_volumes)

        # Simple-ish case.
        a = Cuboid.from_line("on x=0..1,y=0..1,z=0..1", limit50=True)
        b = Cuboid.from_line("on x=1..3,y=0..1,z=0..1", limit50=True)
        union_results = a.union_with(b) # One overlap. Causes one slicing.
        self.assertEqual(3, len(union_results))
        sum_volumes = sum([c.volume() for c in union_results])
        self.assertEqual(4*2*2, sum_volumes)

        # See graphic 'union test'
        red = Cuboid.from_line("on x=-3..4,y=2..3,z=-1..0", limit50=True)
        red_volume = 2 * 2 * 8
        self.assertEqual(red_volume, red.volume())

        blue = Cuboid.from_line("on x=0..3,y=-3..4,z=0..3", limit50=True)
        blue_volume = 4 * 8 * 4
        self.assertEqual(blue_volume, blue.volume())

        purple = Cuboid.from_line("on x=0..3,y=2..3,z=0..0", limit50=True)
        purple_volume = 4 * 2 * 1
        self.assertEqual(purple_volume, purple.volume())

        union_results = red.union_with(blue)
        #for c in union_results:
        #    print("Union test got fragment: '%s' with volume %d" % (c, c.volume()))
        union_volumes = [c.volume() for c in union_results]
        self.assertEqual(red_volume+blue_volume-purple_volume, sum(union_volumes))


    def test_sub(self):
        # Simple-ish case.
        a = Cuboid.from_line("on x=0..1,y=0..1,z=0..1", limit50=True)
        b = Cuboid.from_line("off x=2..3,y=0..1,z=0..1", limit50=True)
        sub_results = a.sub_other(b) # Zero overlap.
        self.assertEqual(1, len(sub_results))
        sum_volumes = sum([c.volume() for c in sub_results])
        self.assertEqual(2*2*2, sum_volumes)

        # Simple-ish case.
        a = Cuboid.from_line("on x=0..1,y=0..1,z=0..1", limit50=True)
        b = Cuboid.from_line("off x=1..3,y=0..1,z=0..1", limit50=True)
        sub_results = a.sub_other(b) # One overlap. Causes one slicing.
        self.assertEqual(1, len(sub_results))
        sum_volumes = sum([c.volume() for c in sub_results])
        self.assertEqual(1*2*2, sum_volumes)

    def test_set(self):
        cuboids = [
            Cuboid.from_line("on x=0..1,y=0..1,z=0..1", limit50=False),
            Cuboid.from_line("on x=0..1,y=0..1,z=0..1", limit50=False)
        ]

        self.assertEqual(cuboids[0], cuboids[1])
        s = set(cuboids)


        self.assertEqual(1, len(s))


    def test_part1(self):
        non_overlapping = """on x=0..0,y=0..0,z=0..0
on x=1..1,y=0..0,z=0..0
on x=2..2,y=0..0,z=0..0
on x=3..3,y=0..0,z=0..0
on x=4..4,y=0..0,z=0..0""".split("\n")
        non_overlapping = [s.strip() for s in non_overlapping]
        ws = World(non_overlapping)
        self.assertEqual(5, ws.part1())



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
