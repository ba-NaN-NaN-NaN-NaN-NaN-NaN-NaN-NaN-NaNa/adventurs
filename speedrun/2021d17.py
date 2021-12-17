import unittest
import re

f = open("../input/2021_d17.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0][0]

f = open("../input/2021_d17_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0][0]


def part1(area):
    ints = area_to_ints(area)
    best_probe = Probe(area, 0, 0)
    best_probe.step_until_pointless()

    ever_in_area_count = 0

    vels_ever_in_area = []

    # What range for y?
    # Sample input: New best probe. dx, dy = (6, 9) reached 45
    # Real input: New best probe. dx, dy = (22, 117) reached 6903
    
    range_until_dx = max([ints[0], -ints[0], ints[1], -ints[1]])
    range_until_dy = max([ints[3], -ints[3], ints[2], -ints[2]])

    # For sample area, dx ranges from 6 to 30.
    for dx in range(0, range_until_dx+10):
        # For sample area, dy ranges from -10 to +10.
        #for dy in range(-15, 15): # <- y area starts at -10
        for dy in range(-range_until_dy - 10, range_until_dy + 10): # <- y area starts at -118
            p = Probe(area, dx, dy)
            p.step_until_pointless()
            if p._ever_entered_area:
                ever_in_area_count += 1
                vels_ever_in_area.append([dx, dy])
                if p._record_y > best_probe._record_y:
                    print("New best probe. dx, dy = (%d, %d) reached %d" % (dx, dy, p._record_y))
                    best_probe = p


    print("ever_in_area_count is %d" % ever_in_area_count)
    vels_ever_in_area_str = []
    for vels in vels_ever_in_area:
        s = "%d,%d" % (vels[0], vels[1])
        vels_ever_in_area_str.append(s)


    vels_ever_in_area_str = sorted(vels_ever_in_area_str)
    with open("/tmp/actual_vels_in_area", "w") as fhandle:
        # UUuugh. Needed to print this to realize that I was range:ing
        # to 250, too low to hit the target area which
        # goes to 259 in the sample.
        fhandle.writelines("\n".join(vels_ever_in_area_str))

    #print("vels_ever_in_area is '%s'" % " ".join(vels_ever_in_area_str))
    return ever_in_area_count, best_probe



def area_to_ints(area):
    """
    x=20..30, y=-10..-5 -> values as int

    Could regexp parse this, but nah.
    """

    if area.endswith("x=20..30, y=-10..-5"):
        return [20, 30, -10, -5]

    if area.endswith("x=235..259, y=-118..-62"):
        return [235, 259, -118, -62]


class Probe:
    def __init__(self, target_area, dx, dy):
        """
        dx, dy = speed of proble
        """
        self._target_area = target_area

        ints = area_to_ints(target_area)


        self._area_left = ints[0]
        self._area_right = ints[1]
        self._area_top = ints[2]
        self._area_bottom = ints[3]

        if self._area_top < self._area_bottom:
            self._area_top, self._area_bottom = self._area_bottom, self._area_top
        

        self._curr_x = 0
        self._curr_y = 0
        self._record_y = 0 # How high have we ever reached?

        self._step_nr = 0 # Did not end up using this for part 2, but was still useful for debugging.

        self._dx = dx
        self._dy = dy
        self._ever_entered_area = False

    def is_in_area(self):
        """
        Are we inside the area?
        """

        if self._area_left <= self._curr_x and self._curr_x <= self._area_right:
            if self._area_bottom <= self._curr_y and self._curr_y <= self._area_top:
                return True

        return False

    def step(self):
        """
        Move one step.
        """
        new_x = self._curr_x + self._dx
        new_y = self._curr_y + self._dy
        new_dx = 0
        if self._dx < 0:
            new_dx = self._dx + 1

        if self._dx > 0:
            new_dx = self._dx - 1

        

        #print("Step %d: (%d, %d) -> (%d, %d). dx changed from %d to %d" % (self._step_nr, self._curr_x, self._curr_y, new_x, new_y, self._dx, new_dx))
        self._curr_x = new_x
        self._curr_y = new_y
        self._dx = new_dx
        self._dy = self._dy - 1

        # Save variables for post-analysis
        if self._curr_y > self._record_y:
            self._record_y = self._curr_y

        if self.is_in_area():
            self._ever_entered_area = True

    def step_until_pointless(self):
        """
        Run steps until we entered area or missed area.
        """
        while True:
            self.step()
            if self.has_passed_area():
                #print("Has passed area self._curr_y < self._area_bottom = (%d, %d)" % (self._curr_y, self._area_bottom))
                return
            if self.is_in_area():
                return


    def has_passed_area(self):
        """
        Return True if we can never reach the area.
        """
        return self._curr_y < self._area_bottom

class TestEntryPoint(unittest.TestCase):


    def test_step(self):
        p = Probe(SAMPLE_STR, 7, 2)
        p.step()
        p.step()
        p.step()
        p.step()
        p.step()
        p.step()
        p.step()
        #p.step()
        self.assertTrue(p.is_in_area())


    def test_paths(self):
        p = Probe(SAMPLE_STR, 6, 9)
        p.step_until_pointless()
        self.assertEqual(p._record_y, 45)
        self.assertTrue(p._ever_entered_area)

        p = Probe(SAMPLE_STR, 17, -4)
        p.step_until_pointless()
        self.assertEqual(p._record_y, 0)
        self.assertFalse(p._ever_entered_area)

        p = Probe(SAMPLE_STR, 7,0)
        p.step_until_pointless()
        self.assertTrue(p._ever_entered_area)


    def test_paths_60(self):
        p = Probe(SAMPLE_STR, 6,0)
        p.step_until_pointless()
        self.assertTrue(p._ever_entered_area)


    def test_part1(self):
        count, best = part1(SAMPLE_STR)
        self.assertEqual(best._record_y, 45)
        self.assertEqual(count, 112)
        self.assertTrue(best._ever_entered_area)

        
        count, best = part1(INPUT_STR)
        self.assertNotEqual(best._record_y, 27495) # 27495 not answer to part 1
        self.assertEqual(best._record_y, 6903)
        self.assertTrue(best._ever_entered_area)
        self.assertNotEqual(count, 67) # 67 is not answer to part 2
        self.assertGreater(count, 1797)
        self.assertGreater(count, 1895)
        # 1797 is too low for part 2
        # 1895 is too low for part 2
        self.assertEqual(count, 2351)
    def test_part2(self):
        pass

if __name__ == '__main__':
    unittest.main()
