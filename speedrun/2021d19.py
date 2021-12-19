import unittest
import copy
import numpy as np
import cProfile

from permutations_2021d19 import POSSIBLE_TRANSFORMS, perm_main

f = open("../input/2021_d19.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d19_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


class Point3d:
    pass
    def __init__(self, x, y, z):
        """
        Class exists mainly for typing.
        """
        self.x = x
        self.y = y
        self.z = z
        #self._repr = "%d,%d,%d" % (self.x, self.y, self.x)

    def __hash__(self):
        return hash(self.__repr__())

    def __lt__(self, other):
        #print("LT %s < %s ?" % (self, other))
        if self.x < other.x:
            return True

        if self.x == other.x:
            # Need to continue and also check y,z
            pass
        else: 
            return False


        if self.y < other.y:
            return True

        if self.y == other.y:
            # Need to continue and also check z
            pass
        else: 
            return False


        if self.z < other.z:
            return True


        print("LT %s < %s is False" % (self, other))
        return False

    def __gt__(self, other):
        #print("GT %s < %s ?" % (self, other))
        if self.x > other.x:
            return True

        if self.x == other.x:
            # Need to continue and also check y,z
            pass
        else: 
            return False


        if self.y > other.y:
            return True

        if self.y == other.y:
            # Need to continue and also check z
            pass
        else: 
            return False

        if self.z > other.z:
            return True

        return False

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __str__(self):
        return self.__repr__()

    def __repr__(self):
        self._repr = "%d,%d,%d" % (self.x, self.y, self.z)
        return self._repr

"""
def coords_to_projection_1d(ints):
    "  ""
    Input: List of ints.
    Output: String like 'B..BB'
    "  ""
    min_val = min(ints)
    max_val = max(ints)
    toreturn = ["."]*(max_val - min_val + 1)
    for val in ints:
        offset = val - min_val
        toreturn[offset] = "B"
    return "".join(toreturn)
"""

def lines_to_beacons_xyz(lines):
    """
    Input: a bunch of lines 
    Output: list of point 3Ds
    """
    #print("Building beacons from %s" % lines)
    toreturn = []
    for line in lines:
        vals = line.strip().split(",")
        p3d = Point3d(int(vals[0]),int(vals[1]),int(vals[2]),)
        toreturn.append(p3d)
    return toreturn

def input_to_scanners(lines):
    """
    Input: Either sample or real input file.
    Output: List of scanners
    """
    toreturn = []
    coord_worklist = []
    scanner_nr = -1
    for line in lines:
        if "scanner " in line:
            if scanner_nr >= 0:
                beacons_xyz = lines_to_beacons_xyz(coord_worklist)
                #print("Building scanner %d from worlkist %s" % (scanner_nr, coord_worklist))
                coord_worklist = []
                scanner = Scanner(scanner_nr, beacons_xyz)
                toreturn.append(scanner)

            scanner_nr += 1
    
        else:
            #print("Eating coord '%s' for scanner nr '%d'" % (line, scanner_nr))
            coord_worklist.append(line)


    beacons_xyz = lines_to_beacons_xyz(coord_worklist)
    scanner = Scanner(scanner_nr, beacons_xyz)
    toreturn.append(scanner)
    return toreturn


def get_overlapping(ref_str, new_str, offset):
    """
    Return string parts that overlap.
    """
    len_ref = len(ref_str)
    len_new = len(new_str)

    ref_shifted = ref_str
    new_shifted = new_str

    if offset > 0:
        ref_shifted = ref_shifted[offset:]
    else:
        new_shifted = new_shifted[-offset:]


    #print("For offset %d then (%s,%s) -> (%s,%s)" % (offset, ref_str, new_str, ref_shifted, new_shifted))

    len_ref = len(ref_shifted)
    len_new = len(new_shifted)

    if len_ref == len_new:
        return ref_shifted, new_shifted

    if len_ref < len_new:
        return ref_shifted, new_shifted[0:len_ref]

    return ref_shifted[0:len_new], new_shifted


def guess_offset(ref_str, new_str):
    """
    Return offset of new_str to ref_str.
    """
    overlap_to_test = min([
        int(len(ref_str)*0.75),
        int(len(new_str)*0.75),
    ])

    
    for guess in range(-overlap_to_test, overlap_to_test):
        ref_part, new_part = get_overlapping(ref_str, new_str, guess)
        if ref_part == new_part:
            if ref_part.count("B") < 4:
                continue
            return guess

    return None
"""
def beacons_to_1d_projections(beacons):
    ""xx"
    All 1d projections from a given list of beacons.

    input: List[Beacon3d]
    output: List of ["B...B", "B...B",...]
    "xxx""
    xs = [b.x for b in beacons]
    ys = [b.y for b in beacons]
    zs = [b.z for b in beacons]
    x_str = coords_to_projection_1d(xs)
    y_str = coords_to_projection_1d(ys)
    z_str = coords_to_projection_1d(zs)


    toreturn = [
        x_str, y_str, z_str, 
        x_str[::-1], y_str[::-1], z_str[::-1], 
    ]

    return toreturn
"""

import random

class World:
    def __init__(self, lines):
        """
        """
        self._scanners = input_to_scanners(lines)
        beacons = self._scanners[0].get_beacons_abs()
        self._beacons = copy.deepcopy(beacons) # All coords relative to beacon 0
        random.shuffle(self._beacons) # To verify that sorted works later.
        # This is only the initial set, relative to beacon 0
        #self.recalc_projections() # NOT USEFUL?
        
    def print_beacons(self):
        self._beacons = sorted(self._beacons)
        for beacon in self._beacons:
            print("%s" % beacon)
        print(" >> Total %d beacons << " % len(self._beacons))

    def recaxxxlc_projections(self):
        """
        Recalc projections.
        
        Call this after a beacon is added.
        """
        xs = [b.x for b in self._beacons]
        ys = [b.y for b in self._beacons]
        zs = [b.z for b in self._beacons]
        self.known_beacon_xs = sorted(list(set(xs)))
        self.known_beacon_ys = sorted(list(set(ys)))
        self.known_beacon_zs = sorted(list(set(zs)))

        self._projection_x = coords_to_projection_1d(self.known_beacon_xs)
        self._projection_y = coords_to_projection_1d(self.known_beacon_ys)
        self._projection_z = coords_to_projection_1d(self.known_beacon_zs)


    def find_projection(self, projection):
        """
        Find a given projection in the world.

        NOT USEFUL?
        """
        x_guess = guess_offset(self._projection_x, projection)
        if x_guess is not None:
            return 'x_forward', x_guess

        y_guess = guess_offset(self._projection_y, projection)
        if y_guess is not None:
            return 'y_forward', y_guess

        z_guess = guess_offset(self._projection_z, projection)
        if z_guess is not None:
            return 'z_forward', z_guess

        # Also check reverse
        projection = projection[::-1]
        x_guess = guess_offset(self._projection_x, projection)
        if x_guess is not None:
            return 'x_reverse', x_guess

        y_guess = guess_offset(self._projection_y, projection)
        if y_guess is not None:
            return 'y_reverse', y_guess

        z_guess = guess_offset(self._projection_z, projection)
        if z_guess is not None:
            return 'z_reverse', z_guess

        if False:
            print("world_x: %s" % self._projection_x)
            print("world_y: %s" % self._projection_y)
            print("world_z: %s" % self._projection_z)
            print("Could not find forward '%s'" % projection[::-1])
            print("Could not find reverse '%s'" % projection)

        return None, None

    @property
    def known_beacons(self):
        return self._beacons

    @property
    def num_scanners(self):
        return len(self._scanners)

    @property
    def beacon_count(self):
        return len(self._beacons)

    def get_scanner_nr(self, scanner_nr):
        # print("Getting scanner_nr %d from list with len %d" % (scanner_nr, len(self._scanners)))
        toreturn = self._scanners[scanner_nr]
        if toreturn._scanner_nr != scanner_nr:
            raise TypeError("fkdjh  sd")
        return toreturn

    def known_beacons_with_mdists(self, mdists):
        toreturn = set()
        for b0 in self._beacons:
            for b1 in self._beacons:
                actual_mdists = [abs(b0.x-b1.x),abs(b0.y-b1.y),abs(b0.z-b1.z),]
                actual_mdists = sorted(actual_mdists)
                if actual_mdists == mdists:
                    print("Matched beacons %s and %s with mdists %s" % (b0, b1, mdists))
                    toreturn.add(b0)
                    toreturn.add(b1)

        #if len(toreturn) == 0:
        #    return 1/0
        return list(toreturn)



APPLY_REL_CACHE = None        
APPLY_ROT_CACHE = None
    
def apply_transforms(beacons_rel, beacons_abs, rot_matrix, dx, dy, dz):
    """
    Rotate beacon around relative coords, then apply (dx, dy, dz) translation.
    """
    global APPLY_REL_CACHE
    global APPLY_ROT_CACHE
    #print("kjfhd")
    if len(beacons_rel) != len(beacons_abs):
        raise TypeError("kjfdgkfjgd")

    #print("Will apply transform < %d, %d, %d> to %s" % ( dx, dy, dz, beacons_rel))
    for idx in range(len(beacons_rel)):
        b_rel = beacons_rel[idx]
        b_abs = beacons_abs[idx]

        if APPLY_REL_CACHE is None:
            APPLY_REL_CACHE = np.array([b_rel.x, b_rel.y, b_rel.z,])
        else:
            APPLY_REL_CACHE[0] = b_rel.x
            APPLY_REL_CACHE[1] = b_rel.y
            APPLY_REL_CACHE[2] = b_rel.z

        # Rotated = apply rotation, but not yet translation
        if APPLY_ROT_CACHE is None:
            #rotated_xyz = np.matmul(APPLY_REL_CACHE, rot_matrix)
            APPLY_ROT_CACHE = np.matmul(APPLY_REL_CACHE, rot_matrix)
        else:
            np.matmul(APPLY_REL_CACHE, rot_matrix, out=APPLY_ROT_CACHE)
            
        
        b_abs.x = APPLY_ROT_CACHE[0] + dx
        #print("into.x = new_xyz[0] + dx is %d %d %d "  % (into.x, new_xyz[0], dx))
        b_abs.y = APPLY_ROT_CACHE[1] + dy
        b_abs.z = APPLY_ROT_CACHE[2] + dz

        #print("Transformed %s -> %s " % (b_rel, b_abs))

        pass
    #print("Applied transform. %s -> %s, <matri>, %d, %d, %d" % (beacons_abs, beacons_rel,  dx, dy, dz))
    return beacons_abs


class Scanner:
    def __init__(self, scanner_nr, beacons_rel):
        self._scanner_nr = scanner_nr
        if 10 < len(beacons_rel) or len(beacons_rel) < 30:
            self._beacons_rel = copy.deepcopy(beacons_rel)
        else:
            raise TypeError("kfjdh %s" % beacons_rel)

        #self._beacon_projections_1d = beacons_to_1d_projections(self._beacons_rel)
        self._beacons_abs = copy.deepcopy(beacons_rel)
        self.set_transforms(np.identity(3),0,0,0)

        # self._rotated_cache = {} <- Can this work??

        self._middlest_beacon_memo = None

    def get_beacons_abs(self):
        return self._beacons_abs

    def set_transforms(self, rot_matrix, x, y, z):
        """
        For each original beacon, set the transformed beacon
        according to the arguments.
        """
        #rel_before = copy.deepcopy(self._beacons_rel)
        #abs_before = copy.deepcopy(self._beacons_abs)

        # Assume beacon abs at x=650
        # Assume scanner at x=100
        # Then beacon relative should be 550
        # beacon_abs = beacon_relative + scanner_location
        self._beacons_abs = apply_transforms(self._beacons_rel,
                                             self._beacons_abs,
                                             rot_matrix, x, y, z) # OBS MINUS
        self._rot_matrix = rot_matrix
        self._x = x
        self._y = y
        self._z = z


        #rel_after = copy.deepcopy(self._beacons_rel)
        #abs_after = copy.deepcopy(self._beacons_abs)

        # Rel must not change.
        # Abs may.
        #print("XXXXX rel %s -> %s" % (rel_before, rel_after))
        #print("XXXXX abs %s -> %s" % (abs_before, abs_after))
        pass


    #def get_1d_projections(self):
    #    return self._beacon_projections_1d



    def fits_as_transformed(self, world):
        """
        Does this scanner fit in the world, assuming transformation is correct?
        """
        known_beacons = world.known_beacons
        beacon_fit_count = 0
        for beacon in self.get_beacons_abs():
            if beacon in known_beacons:
                beacon_fit_count += 1

        return beacon_fit_count  >= 12


    def try_fit_to_world(self, world):
        """
        Try to fit in to the world.
        """
        if self.fits_as_transformed(world):
            return

        candidates = []
        if self._scanner_nr == 0:
            transf = np.array([[1, 0, 0],
                               [0, 1, 0],
                               [0, 0, 1]])
            candidates.append([transf, 0, 0, 0])
        
        for cand in candidates:
            transf, dx, dy, dz = cand
            self.set_transforms(transf, dx, dy, dz)
            if self.fits_as_transformed(world):
                return
        
        #b, mdists = self.middlest_beacon()
        b, mdists = self.random_beacon()
        offsets = self.gen_possible_offsets(world, b, mdists)
        print("Trying %d offsets: %s for scannr nr %d" % (len(offsets), offsets, self._scanner_nr))

        for dx in offsets:
            for dy in offsets:
                for dz in offsets:
                    for transf in POSSIBLE_TRANSFORMS:
                        self.set_transforms(transf, dx, dy, dz)
                        if self.fits_as_transformed(world):
                            print("NEW HIT Scanner %d fits with %s, %s, %s, %s" % (self._scanner_nr, transf, dx, dy, dz))
                            return


        return
        return 1/0
        # Brute force approach below.
        # search_space = 5 -> takes several seconds to search.
        search_space = 5
        for dx in range(-search_space, search_space):
            for dy in range(-search_space, search_space):
                for dz in range(-search_space, search_space):
                    for transf in POSSIBLE_TRANSFORMS:
                        self.set_transforms(transf, dx, dy, dz)
                        if self.fits_as_transformed(world):
                            print("NEW HIT Brue force %d fits with %s, %s, %s, %s" % (self._scanner_nr, transf, dx, dy, dz))
                            return

    def gen_possible_offsets(self, world, b, mdists):
        """
        Instead of _EVERY_ int in search space, take a beacon
        and generate all possible offsets in the world.
        """
        offsets = set()
        #b, mdists = self.middlest_beacon()
        known_list = world.known_beacons_with_mdists(mdists)
        print("for mdists %s, possible world beacons is %s" % (mdists, known_list))
        for known in known_list:
            offsets.add(abs(b.x-known.x))
            offsets.add(abs(b.x-known.y))
            offsets.add(abs(b.x-known.z))
            offsets.add(abs(b.y-known.x))
            offsets.add(abs(b.y-known.y))
            offsets.add(abs(b.y-known.z))
            offsets.add(abs(b.z-known.x))
            offsets.add(abs(b.z-known.y))
            offsets.add(abs(b.z-known.z))

        found = list(offsets)
        offsets = [int(nr) for nr in found]
        negative = [-nr for nr in offsets]

        offsets = offsets + negative
        print("  -> offsets are %s" % offsets)

        return offsets

    def random_beacon(self):
        """
        Return a random beacon + its closest beacon mdists.

        Similar to middlest_beacon, but useful if there are non-overlapping spaces. I.e. as below.

        +-----+
        |     |
        |     |
        |  m  |
        | +---+--- 
        | |   |
        +-+---+
          |
        """

        r_beak = random.choice(self._beacons_rel)
        print("Randomly chose r_beak %s" % r_beak)
        mdists, mdists_sum = self.least_mdists_to_rel(r_beak)
        print("Returning random %s %s " % (r_beak, mdists))
        return r_beak, mdists
    
    def least_mdists_to_rel(self, beacon):
        """
        Given a beacon, what are mdists to the closest one?

        Must not be the beacon itself.
        """
        found_beacon = False

        mdists = None
        mdists_sum = 96876947865497
        for cand in self._beacons_rel[:]:
            if beacon == cand:
                # Skip this one. Mdists [0,0,0] is pointless.
                found_beacon = True
                continue
            #print("Will calc mdists between %s <%s> and %s <%s>" % (beacon, type(beacon), cand, type(cand)))
            cand_mdists = [abs(cand.x-beacon.x),abs(cand.y-beacon.y),abs(cand.z-beacon.z)]
            cand_mdists_sum = sum(cand_mdists)
            if cand_mdists_sum < mdists_sum:
                mdists = sorted(cand_mdists)
                mdists_sum = cand_mdists_sum
            
        return mdists, mdists_sum

    def middlest_beacon(self):
        """
        return beacon that is closest to middle.
        Also list of distances to second closest, in abs-valued, sorted, axis-aligned distances.
        I.e. this function returns [<beacon>, [6,10,20]]
        """
        if self._middlest_beacon_memo is not None:
            return self._middlest_beacon_memo

        worklist = self._beacons_rel[:]
        toreturn = worklist[0]
        toreturn_dist = abs(toreturn.x) + abs(toreturn.y) + abs(toreturn.z)
        toreturn_2 = worklist[0]

        class BCalc:
            def __init__(self, b):
                self.b = b
                self.dist = abs(b.x) + abs(b.y) + abs(b.z)

            def __repr__(self):
                return "%d away is %s" % (self.dist, self.b)

        bcalcs = [BCalc(b) for b in worklist]
        bcalcs = sorted(bcalcs, key=lambda x: x.dist)
        #print("\n".join([str(b) for b in bcalcs]))

        """
        for cand in worklist[1:]:
            cand_dist = abs(cand.x) + abs(cand.y) + abs(cand.z) 
            if cand_dist < toreturn_dist:
                toreturn = cand
                toreturn_dist = cand_dist

        """

        b0 = bcalcs[0]
        b1 = bcalcs[1]
        dists = [abs(b0.b.x-b1.b.x), abs(b0.b.y-b1.b.y) , abs(b0.b.z-b1.b.z)]
        dists = sorted(dists)

        toreturn = [b0.b, dists]

        print("middlest beacon returning %s" % toreturn)
        self._middlest_beacon_memo = toreturn
        return toreturn



class TestEntryPoint(unittest.TestCase):

    def texst_coords_to_projection_1d(self):
        self.assertEqual("B...B", coords_to_projection_1d([0, 4]))
        self.assertEqual("B...B", coords_to_projection_1d([0, -4]))
        self.assertEqual("B...B", coords_to_projection_1d([0, -4]))


    def txest_parse(self):
        ws = World(SAMPLE_STR)
        self.assertEqual(4+1, ws.num_scanners)
        for nr in range(ws.num_scanners):
            sc = ws.get_scanner_nr(nr)


        wi = World(INPUT_STR)
        self.assertEqual(30+1, wi.num_scanners)
        for nr in range(wi.num_scanners):
            sc = wi.get_scanner_nr(nr)

    def txest_determine_projection_offset(self):
        """
        DOES NOT WORK???

        1d projection doesn't result in scanners finding
        themselves, except for the first one and a stray scanner on a single axle.
        """

        ref, new = get_overlapping("abcdef", "bcdefghij", 0)
        self.assertEqual(ref, "abcdef")
        self.assertEqual(new, "bcdefg")

        ref, new = get_overlapping("abcdef", "bcdefghij", -1)
        self.assertEqual(ref, "abcdef")
        self.assertEqual(new, "cdefgh")

        ref, new = get_overlapping("abcdef", "bcdefghij", 3)
        self.assertEqual(ref, "def")
        self.assertEqual(new, "bcd")


        self.assertEqual(0,
            guess_offset("...B.............B..................B...........B...........................................B....",
                         "...B.............B..................B...........B...........................................B...."))

        self.assertEqual(-1,
            guess_offset("...B.............B..................B...........B...........................................B....",
                        "....B.............B..................B...........B...........................................B...."))


        self.assertEqual(7,
            guess_offset("...B.............B..................B...........B...........................................B....",
                                "..........B..................B...........B...........................................B...."))

        self.assertEqual(None,
            guess_offset("...B....B........B..................B...........B...........................................B....",
                                "..........B..................B...........B...........................................B...."))

        self.assertEqual(None,
            guess_offset("...B.............B..................B...........B...........................................B....",
                                "..........B..................B...........B...........................................B..B."))

    def texst_projections(self):
        ws = World(SAMPLE_STR)
        sc0 = ws.get_scanner_nr(0)
        projections0 = sc0.get_1d_projections()
        sc1 = ws.get_scanner_nr(1)
        projections1 = sc1.get_1d_projections()
        #print(projections1[0])
        #self.assertEqual("v", projections1[0])


    def texst_find_projection(self):
        """
        DOES NOT WORK???

        1d projection doesn't result in scanners finding
        themselves, except for the first one and a stray scanner on a single axle.
        """
        ws = World(SAMPLE_STR)

        """
        sc0 = ws.get_scanner_nr(0)
        projections0 = sc0.get_1d_projections()
        for projection in projections0:
            direction, offset = ws.find_projection(projection)
            self.assertEqual(0, offset)
            print("direction %s, offset %s" % (direction, offset))
        """

        """
        found = False
        for nr in range(ws.num_scanners):
            sc1 = ws.get_scanner_nr(nr)
            projections1 = sc1.get_1d_projections()
            for projection in projections1:
                direction, offset = ws.find_projection(projection)
                print("direction %s, offset %s" % (direction, offset))
                if offset is not None:
                    found = True
        self.assertTrue(found)
        self.assertEqual(5,7)
        """

        """
        sc4 = ws.get_scanner_nr(4)
        projections4 = sc4.get_1d_projections()
        for projection in projections4:
            direction, offset = ws.find_projection(projection)
            print("direction %s, offset %s" % (direction, offset))
        self.assertEqual(5,7)
        """


    def test_fits_in_world(self):
        ws = World(SAMPLE_STR)
        sc0 = ws.get_scanner_nr(0)
        self.assertTrue(sc0.fits_as_transformed(ws))

        # Can we transform offset 1, then notice that we don't fit?
        sc0.set_transforms(np.identity(3),1,0,0)
        self.assertFalse(sc0.fits_as_transformed(ws))

        # Can we transform back?
        sc0.set_transforms(np.identity(3),0,0,0)
        self.assertTrue(sc0.fits_as_transformed(ws))

        # Can we rotate, then notice that we don't fit?
        sc0.set_transforms(POSSIBLE_TRANSFORMS[4],0,0,0)
        self.assertFalse(sc0.fits_as_transformed(ws))

        # Can we transform back?
        sc0.set_transforms(np.identity(3),0,0,0)
        self.assertTrue(sc0.fits_as_transformed(ws))


        # Can we rotate, then notice that we don't fit?
        sc0.set_transforms(POSSIBLE_TRANSFORMS[4],0,0,0)
        self.assertFalse(sc0.fits_as_transformed(ws))

        #with cProfile.Profile() as prof:
        #    sc0.try_fit_to_world(ws)
        #    prof.print_stats()

        sc0.try_fit_to_world(ws)
        self.assertTrue(sc0.fits_as_transformed(ws))

        ns = [n for n in range(ws.num_scanners)]
        for _ in range(10):
            for n in ns:
                sc = ws.get_scanner_nr(n)
                sc.try_fit_to_world(ws)

        sc4 = ws.get_scanner_nr(4)
        sc4.try_fit_to_world(ws)

        self.assertTrue(sc0.fits_as_transformed(ws))
        self.assertEqual(34,5)

    def test_lines_to_beacons_xyz(self):
        beacons = lines_to_beacons_xyz(["686,2,58",
                                        "-605,3,45"])
        b0 = beacons[0]
        b1 = beacons[1]
        self.assertEqual(b0.x, 686)
        self.assertEqual(b0.y, 2)
        self.assertEqual(b0.z, 58)
        self.assertEqual(b1.x, -605)
        self.assertEqual(b1.y, 3)
        self.assertEqual(b1.z, 45)
        self.assertEqual("686,2,58", "%s" % b0)
        self.assertEqual("-605,3,45", "%s" % b1)

    def test_apply_transforms(self):
        beacons0 = lines_to_beacons_xyz(["686,2,58",
                                         "605,3,45"])

        beacons1 = lines_to_beacons_xyz(["586,2,58",
                                         "505,3,45"])

        #print("beacons0 is %s" % beacons0)
        #print("beacons1 is %s" % beacons1)

        sc0 = Scanner(0, beacons0)
        sc1 = Scanner(1, beacons1)

        #print("XXXXXX")
        for _ in range(1):
            sc1.set_transforms(np.identity(3), 100, 0, 0)
        
        sc0_abs = sc0.get_beacons_abs()
        sc1_abs = sc1.get_beacons_abs()
        
        #print("sc0_abs is '%s'" % sc0_abs)
        #print("sc1_abs is '%s'" % sc1_abs)
        
        self.assertEqual(sc0_abs, beacons0)
        self.assertEqual(sc0_abs, sc1_abs)

    def texst_apply_transforms(self):
        beacons0 = lines_to_beacons_xyz(["-618,-824,-621",
                                         "-537,-823,-458"])

        beacons1 = lines_to_beacons_xyz(["686,422,578",
                                         "605,423,415"])

        sc0 = Scanner(0, beacons0)
        sc1 = Scanner(1, beacons1)
        sc1.set_transforms(np.identity(3), 68, -1246, -43)
        sc1_transformed = sc1.get_beacons_abs()
        
        self.assertEqual(beacons0, sc1_transformed)

    def tezst_part1(self):
        ws = World(SAMPLE_STR)
        ws.print_beacons()
        self.assertEqual(79, ws.beacon_count)

    def test_part2(self):
        pass

if __name__ == '__main__':
    unittest.main()
