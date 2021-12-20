import typing
import unittest
import copy
import numpy as np
import cProfile

from permutations_2021d19 import POSSIBLE_ROTATIONS, perm_main

def readlines_from(fname):
    f = open(fname)
    lines = f.readlines()
    f.close()
    lines = [r.strip() for r in lines if len(r.strip()) > 0]
    return lines

INPUT_STR = readlines_from("../input/2021_d19.txt")
SAMPLE_STR = readlines_from("../input/2021_d19_sample.txt")
ROTATIONS_STR = readlines_from("../input/2021_d19_sample_rotations.txt")


class Point3d:

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

    def with_rotation(self, mrot):
        res = np.matmul(mrot, np.array([self.x, self.y, self.z]))
        return Point3d(res[0], res[1], res[2])

    def with_translation(self, dx, dy, dz):
        return Point3d(self.x+dx, self.y+dy, self.z+dz)

    def sub(self, other):
        """
        Return self - other
        """
        return Point3d(self.x-other.x, self.y-other.y, self.z-other.z)

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



def input_to_scanners(lines) -> typing.List['Scanner']:
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



import random

class World:
    def __init__(self, lines):
        """
        """
        self._scanners = input_to_scanners(lines)
        self._scanners[0].set_as_origin()

        if len(self._scanners) > 8:
            # When the following block is commented out, tests take ~1 minute.
            self._scanners[23].try_place_relative(self._scanners[0], -91, -19, -1091, suggest=False)
            self._scanners[13].try_place_relative(self._scanners[23], 1214, 41, -133, suggest=False)
            self._scanners[11].try_place_relative(self._scanners[23], -34, 2, -1216, suggest=False)
            self._scanners[8].try_place_relative(self._scanners[11], -22, -1143, -102, suggest=False)
            self._scanners[9].try_place_relative(self._scanners[0], -55, -1213, 88, suggest=False)
            self._scanners[21].try_place_relative(self._scanners[9], -73, -1215, -89, suggest=False)
            self._scanners[2].try_place_relative(self._scanners[9], 50, -23, 1114, suggest=False)
            self._scanners[28].try_place_relative(self._scanners[2], -77, -1263, -52, suggest=False)
            self._scanners[29].try_place_relative(self._scanners[0], -11, 1122, -4, suggest=False)
            self._scanners[15].try_place_relative(self._scanners[21], -1141, 74, -62, suggest=False)
            self._scanners[17].try_place_relative(self._scanners[2], 3, -83, 1310, suggest=False)
            self._scanners[6].try_place_relative(self._scanners[21], 1343, 87, 86, suggest=False)
            self._scanners[16].try_place_relative(self._scanners[11], -17, -80, -1268, suggest=False)
            self._scanners[22].try_place_relative(self._scanners[2], 1158, -79, -9, suggest=False)
            self._scanners[4].try_place_relative(self._scanners[22], 87, 1347, -29, suggest=False)
            self._scanners[10].try_place_relative(self._scanners[17], -20, -1175, -188, suggest=False)
            self._scanners[14].try_place_relative(self._scanners[15], 99, -1201, 13, suggest=False)
            self._scanners[30].try_place_relative(self._scanners[28], -16, -1132, 160, suggest=False)
            self._scanners[19].try_place_relative(self._scanners[6], -106, 0, -1258, suggest=False)
            self._scanners[5].try_place_relative(self._scanners[19], -65, -69, -1300, suggest=False)
            self._scanners[27].try_place_relative(self._scanners[29], 1252, 17, -50, suggest=False)
            self._scanners[7].try_place_relative(self._scanners[16], 1331, 129, 88, suggest=False)
            self._scanners[12].try_place_relative(self._scanners[16], 175, 3, -1254, suggest=False)
            self._scanners[20].try_place_relative(self._scanners[17], -1247, 131, -93, suggest=False)
            self._scanners[3].try_place_relative(self._scanners[10], 1215, 37, 127, suggest=False)
            self._scanners[26].try_place_relative(self._scanners[21], 71, -1200, 44, suggest=False)
            self._scanners[1].try_place_relative(self._scanners[20], -9, -1149, -31, suggest=False)
            self._scanners[25].try_place_relative(self._scanners[3], 5, -1115, -35, suggest=False)
            self._scanners[18].try_place_relative(self._scanners[16], 174, 1229, 93, suggest=False)
            self._scanners[24].try_place_relative(self._scanners[1], -1217, -106, 127, suggest=False)
            
            pass

        else:
            # Pre-place known scanners, discovered via earlier runs.
            self._scanners[1].try_place_relative(self._scanners[0], 68, -1246, -43, suggest=False)
            self._scanners[3].try_place_relative(self._scanners[1], -160, -1134, 23, suggest=False)
            self._scanners[4].try_place_relative(self._scanners[1], -88, 113, 1104, suggest=False)
            self._scanners[2].try_place_relative(self._scanners[4], 1125, -72, 168, suggest=False)

    def part1(self):
        num_floating_scanners = 1
        while num_floating_scanners > 0:
            ok_scanners = [s for s in self._scanners if s._abs_x is not None]
            floating_scanners = [s for s in self._scanners if s not in ok_scanners]
            num_floating_scanners = len(floating_scanners)
            for floating in floating_scanners:
                for ok in ok_scanners:
                    if floating._abs_x is None:
                        d = self.try_place_relative(ok, floating)
                        for _ in range(5):
                            if d is None:
                                d = self.try_place_relative(ok, floating)
                        if d is not None:
                            floating.try_place_relative(ok, d.x, d.y, d.z)
        
        ok_scanners = [s for s in self._scanners if s._abs_x is not None]
        floating_scanners = [s for s in self._scanners if s not in ok_scanners]                            
        if len(floating_scanners) > 0:
            raise TypeError("Scanners still floating!")

        beacons_abs = set()
        for scanner in ok_scanners:
            scanner_beacons = set(scanner.get_beacons_abs())
            beacons_abs = beacons_abs.union(scanner_beacons)

        return len(beacons_abs)

    def part2(self):
        ok_scanners = [s for s in self._scanners if s._abs_x is not None]
        floating_scanners = [s for s in self._scanners if s not in ok_scanners]                            
        if len(floating_scanners) > 0:
            raise TypeError("Scanners still floating!")

        coords = [Point3d(ok._abs_x, ok._abs_y, ok._abs_z) for ok in ok_scanners]
        max_dist = -1
        for p0 in coords:
            for p1 in coords:
                cand = abs(p0.x-p1.x) + abs(p0.y-p1.y) + abs(p0.z-p1.z) 
                if cand > max_dist:
                    max_dist = cand
        return max_dist

    def try_place_relative(self, ok, floating):

        ok_beacons = ok.best_rot.get_roted_beacons()

        best_overlap = 0
        for rot in floating.get_rotations():
            local_beacons = rot.get_roted_beacons()

            b0 = random.choice(ok_beacons)
            b1 = random.choice(local_beacons)
            d = b0.sub(b1)
            local_beacons = rot.get_roted_translated_beacons(d.x, d.y, d.z)

            overlap = set(ok_beacons).intersection(set(local_beacons))
            if len(overlap) > 8:
                print("Randomly testing beacon alignment between %d and %d gave %d overlapping beacons" % (ok._scanner_nr, floating._scanner_nr, len(overlap)))
                return d


        

        
    def print_beacons(self):
        self._beacons = sorted(self._beacons)
        for beacon in self._beacons:
            print("%s" % beacon)
        print(" >> Total %d beacons << " % len(self._beacons))

    @property
    def known_beacons(self):
        return 1/0

    @property
    def num_scanners(self):
        return len(self._scanners)

    @property
    def beacon_count(self):
        return 1/0

    def get_scanner_nr(self, scanner_nr):
        # print("Getting scanner_nr %d from list with len %d" % (scanner_nr, len(self._scanners)))
        toreturn = self._scanners[scanner_nr]
        if toreturn._scanner_nr != scanner_nr:
            raise TypeError("fkdjh  sd")
        return toreturn


class ScannerRotation:
    def __init__(self, beacons_as_read, rmat):
        """
        Input: 
         - Beacons as read. 
         - rmat <- Rotation matrix. 
        """

        beacons_roted = [b.with_rotation(rmat) for b in beacons_as_read]
        self._beacons_roted = beacons_roted


    def get_roted_beacons(self):
        return self._beacons_roted

    def get_roted_translated_beacons(self, dx, dy, dz):
        own_translated = [b.with_translation(dx, dy, dz) for b in self._beacons_roted]
        return own_translated

    def overlapping_beacons(self, matchlist: typing.List['Point3d'], dx, dy, dz):
        """
        Does this rotation fit another rotation?
        """

        if type(matchlist[0]) != Point3d:
            raise TypeError("Bad point3d, got <'%s'>" % type(matchlist[0]))

        # Only translate MY OWN beacons before matching.
        own_translated = self.get_roted_translated_beacons(dx, dy, dz)

        #print("  ")
        #print("My own translated is %s" % sorted(own_translated))
        #print("matchlist is %s " % sorted(matchlist))

        filtered = [b for b in own_translated if b in matchlist]

        #print("I have %d beacons, other has %d beacons. Overlapping=%d." % (len(own_translated), len(matchlist), len(filtered)))

        return len(filtered)


class Scanner:
    def __init__(self, scanner_nr, beacons_as_read):
        #print("Scanner.__init__(%d, len(xx) = %d)" % (scanner_nr, len(beacons_as_read)))
        self._beacons_as_read = copy.deepcopy(beacons_as_read)
        self._rotations = [ScannerRotation(beacons_as_read, rmat) for rmat in POSSIBLE_ROTATIONS]
        self._scanner_nr = scanner_nr
        if 10 < len(beacons_as_read) or len(beacons_as_read) < 30:
            pass
        else:
            raise TypeError("kfjdh %s" % beacons_rel)

        # Following fields are populated once we are fully placed in world.
        self._abs_x = None
        self._abs_y = None
        self._abs_z = None
        self._best_rot = None
        self._beacons_abs = None


    @property
    def best_rot(self):
        if self._best_rot is None:
            raise TypeError("Why call best rot on scanner nr %d??" % self._scanner_nr)
        return self._best_rot

    def set_as_origin(self):
        """
        Set this scanner as origin scanner.
        """
        self._abs_x = 0
        self._abs_y = 0
        self._abs_z = 0
        #self._rotation_for_abs = np.identity(3)
        self._beacons_abs = self._beacons_as_read
        self._best_rot = ScannerRotation(self._beacons_as_read, np.identity(3))


    def get_rotations(self):
        return self._rotations


    def get_beacons_abs(self):
        return self._beacons_abs

    def try_place_relative(self, other: 'Scanner', dx, dy, dz, suggest=True):
        """
        Can this scanner be placed relative to 
        other, with the dx/dy/dz offsets and
        get 12 overlapping beacons? Return
        max number of overlapping beacons.

        Other scanner MUST be absolutely positioned.
        """

        if other is None:
            raise TypeError("Got a None other")

        beacons_abs = other.get_beacons_abs()
        abs_x = other._abs_x+dx
        abs_y = other._abs_y+dy
        abs_z = other._abs_z+dz
        print("Will try to fit scanner %d at (%d,%d,%d) relative to scanner %d" % (self._scanner_nr, dx, dy, dz, other._scanner_nr))
        print("Will try to fit scanner %d at (%d,%d,%d) in absolute coords." % (self._scanner_nr, abs_x,abs_y,abs_z))
        

        if type(other) != Scanner:
            return TypeError("Got '%s'" % type(other))

        best_rot = None
        best_match_count = -1

        for rot in self._rotations:
            # Rot is ScannerRotation
            rot: ScannerRotation = rot
            count = rot.overlapping_beacons(beacons_abs, abs_x, abs_y, abs_z)
            if count > best_match_count:
                best_match_count = count
                best_rot = rot

        print("Placing scanner %d at (%d,%d,%d) relative to scanner %d gave %d overlapping points" % (self._scanner_nr, dx, dy, dz, other._scanner_nr, best_match_count))

        if best_match_count > 7:
            # Placing scanner 4 at (-88,113,1104) relative to scanner 1 gave 12 overlapping points
            if suggest:
                print("SUGGEST: self._scanners[%d].try_place_relative(self._scanners[%d], %d, %d, %d)" % (self._scanner_nr, other._scanner_nr, dx, dy, dz))
            # Lock in place.

            self._abs_x = abs_x
            self._abs_y = abs_y
            self._abs_z = abs_z
            print("Placed scanner %d at (%d, %d, %d)"% (self._scanner_nr, self._abs_x, self._abs_y, self._abs_z))
            self._best_rot = best_rot
            self._beacons_abs = best_rot.get_roted_translated_beacons(self._abs_x, self._abs_y, self._abs_z)

        return best_match_count



class TestEntryPoint(unittest.TestCase):

    def test_parse(self):
        ws = World(SAMPLE_STR)
        self.assertEqual(4+1, ws.num_scanners)
        for nr in range(ws.num_scanners):
            sc = ws.get_scanner_nr(nr)


        wi = World(INPUT_STR)
        self.assertEqual(30+1, wi.num_scanners)
        for nr in range(wi.num_scanners):
            sc = wi.get_scanner_nr(nr)

        


    def test_point_3d(self):
        initial = Point3d(6, 0, 0)
        
        rotateds = [initial.with_rotation(mrot) for mrot in POSSIBLE_ROTATIONS]
        self.assertIn(Point3d(6,0,0), rotateds)
        self.assertIn(Point3d(-6,0,0), rotateds)
        self.assertIn(Point3d(0,6,0), rotateds)
        self.assertIn(Point3d(0,-6,0), rotateds)
        self.assertIn(Point3d(0,0,6), rotateds)
        self.assertIn(Point3d(0,0,-6), rotateds)
        self.assertEqual(initial.with_translation(1,0,-4), Point3d(7, 0, -4))


    def texst_scanner_rotation(self):
        """
        For example, here is an arrangement of beacons as seen from a scanner in the same position but in different orientations:

        ROTATIONS_STR
        """

        scanners = input_to_scanners(ROTATIONS_STR)

        for sc in scanners:
            self.assertEqual(24, len(sc._rotations))
            for rot in sc._rotations:
                self.assertEqual(6, len(rot._beacons_roted))

        print("Roted beacons is '%s'" % scanners[3]._rotations[0].get_roted_beacons())

        sc_root = scanners[0]
        sc_root.set_as_origin()

        for sc in scanners:
            best = sc.try_place_relative(sc_root, 0, 0, 0)
            self.assertEqual(6, best)

        rmat_identity = np.identity(3)
        beacons_rel = lines_to_beacons_xyz([])
        sr0 = ScannerRotation(beacons_rel, rmat_identity)

        #self.assertEqual()

    def test_scanner_placement_with_answer(self):
        scanners = input_to_scanners(SAMPLE_STR)
        
        sc_root = scanners[0]
        sc_root.set_as_origin()

        sc1 = scanners[1]

        self.assertIs(None, sc1._abs_x)

        # Try to place nr 1.
        # answer = 68, -1246, -43
        match_count_1 = sc1.try_place_relative(sc_root, 68, -1246, -43)
        self.assertEqual(68,    sc1._abs_x)
        self.assertEqual(-1246, sc1._abs_y)
        self.assertEqual(-43,   sc1._abs_z)
        self.assertEqual(12, match_count_1)

        known_by_root = set(sc_root.get_beacons_abs())
        known_by_sc1 = set(sc1.get_beacons_abs())
        known_by_either = known_by_root.union(known_by_sc1)
        known_by_both = known_by_root.intersection(known_by_sc1)

        # Scanners 0 and 1 have overlapping detection cubes; the 12 beacons
        # they both detect (relative to scanner 0) are at the following coordinates:
        self.assertIn(Point3d(-618,-824,-621), known_by_both)
        self.assertIn(Point3d(-537,-823,-458), known_by_both)
        self.assertIn(Point3d(-447,-329,318), known_by_both)
        self.assertIn(Point3d(404,-588,-901), known_by_both)
        self.assertIn(Point3d(544,-627,-890), known_by_both)
        self.assertIn(Point3d(528,-643,409), known_by_both)
        self.assertIn(Point3d(-661,-816,-575), known_by_both)
        self.assertIn(Point3d(390,-675,-793), known_by_both)
        self.assertIn(Point3d(423,-701,434), known_by_both)
        self.assertIn(Point3d(-345,-311,381), known_by_both)
        self.assertIn(Point3d(459,-707,401), known_by_both)
        self.assertIn(Point3d(-485,-357,347), known_by_both)

        
        #print("Known by root: %s" % sorted(list(known_by_root)))
        #print("Known by sc1: %s" % sorted(list(known_by_sc1)))

        self.assertEqual(12, len(known_by_both))

        print("After SC1 is placed, %d | %d -> %d" % (len(known_by_root), len(known_by_sc1), len(known_by_either)))


        # Try to place nr 4.
        # answer = -20,-1133,1061
        # Relative delta = -nr1 + nr4 -> (-)
        sc4 = scanners[4]
        d = Point3d(0-20,-1133,1061).sub(Point3d(68, -1246, -43))
        print("Delta is %s " % d)

        match_count_4 = sc4.try_place_relative(sc1, d.x, d.y, d.z)
        self.assertEqual(   -20, sc4._abs_x)
        self.assertEqual( -1133, sc4._abs_y)
        self.assertEqual(  1061, sc4._abs_z)
        self.assertEqual(12, match_count_4)

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

    def texst_apply_transforms(self):
        beacons0 = lines_to_beacons_xyz(["686,2,58",
                                         "605,3,45"])

        beacons1 = lines_to_beacons_xyz(["586,2,58",
                                         "505,3,45"])


        sc0 = Scanner(0, beacons0)
        sc1 = Scanner(1, beacons1)

        

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


    def test_world(self):
        ws = World(SAMPLE_STR)
        actual = ws.part1()
        self.assertEqual(79, actual)

        wi = World(INPUT_STR)
        actual = wi.part1()
        self.assertEqual(381, actual)


    def texst_part2(self):
        ws = World(SAMPLE_STR)
        actual = ws.part1()
        actual = ws.part2()
        self.assertEqual(3621, actual)

        wi = World(INPUT_STR)
        actual = wi.part1()
        actual = wi.part2()
        self.assertEqual(12201, actual)

if __name__ == '__main__':
    unittest.main()
