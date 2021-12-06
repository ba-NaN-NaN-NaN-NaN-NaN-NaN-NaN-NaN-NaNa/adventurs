import unittest

f = open("../input/2021_d6.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d6_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]

def ints_into_counts(as_ints):
    return {
                -1: 0,
                0: as_ints.count(0),
                1: as_ints.count(1),
                2: as_ints.count(2),
                3: as_ints.count(3),
                4: as_ints.count(4),
                5: as_ints.count(5),
                6: as_ints.count(6),
                7: as_ints.count(7),
                8: as_ints.count(8),
            }

class School:
    def __init__(self, initial_ages, use_counts = False):
        if "," not in initial_ages:
            raise TypeError("fgd %s" % initial_ages)

        as_ints = [int(n) for n in initial_ages.split(",")]
        if use_counts:
            self._count_at_ages = [ints_into_counts(as_ints)]
        else:
            self._fish_states = [as_ints]
        
        self._use_counts = use_counts

    @classmethod
    def iterate_day(cls, ages):
        """
        From one day to next.
        """
        next_ages = [n-1 for n in ages]
        spawncount = 0

        for i in range(len(next_ages)):
            if next_ages[i] == -1:
                spawncount += 1
                next_ages[i] = 6


        return next_ages + [8] * spawncount

    @classmethod
    def iterate_day_counts(cls, counts):
        """
        Distr is dict.
        """
        new_counts = {
            -1: 0,
            0: counts[1],
            1: counts[2],
            2: counts[3],
            3: counts[4],
            4: counts[5],
            5: counts[6],
            6: counts[0] + counts[7],
            7: counts[8],
            8: counts[0],
        }
        #print("Iterating '%s' into '%s'" % (counts, new_counts))
        return new_counts

    def fishcount_on_day(self, day):
        if self._use_counts:
            count = 0
            day_count = self._count_at_ages[day]
            print("fishcount_on_day %d basis %s" % (day, day_count))
            for n in range(9):
                count += day_count[n]
            return count                
        else:
            return len(self._fish_states[day])


    def age_until(self, target_age):
        if self._use_counts:
            while len(self._count_at_ages) <= target_age:
                day_to_calc = len(self._count_at_ages)
                new_counts = School.iterate_day_counts(self._count_at_ages[day_to_calc-1])
                self._count_at_ages.append(new_counts)
        else:
            while len(self._fish_states) <= target_age:
                day_to_calc = len(self._fish_states)
                new_states = School.iterate_day(self._fish_states[day_to_calc-1])
                self._fish_states.append(new_states)

    def print_days(self, day_nrs):
        for n in range(day_nrs):
            states = self._fish_states[n]
            state_str = ",".join([str(s) for s in states])
            print("After %2d days:  %s   = %d fish" % (n, state_str, len(states)))


class TestEntryPoint(unittest.TestCase):

    def test_part1(self):
        school = School(SAMPLE_STR[0])
        school.age_until(5)
        school.age_until(10)
        school.age_until(15)
        school.age_until(82)

        self.assertEqual(26, school.fishcount_on_day(18))
        self.assertEqual(5934, school.fishcount_on_day(80))


        school = School(INPUT_STR[0])
        school.age_until(82)

        # Answer part 1
        self.assertEqual(377263, school.fishcount_on_day(80))

    def test_iter_counts(self):
        # Verify day 1 becomes day 2
        self.assertEqual(School.iterate_day_counts(ints_into_counts([2,3,2,0,1])),
                                                   ints_into_counts([1,2,1,6,0,8]))

    def test_part2(self):
        school_sample = School(SAMPLE_STR[0], use_counts = True)
        school_sample.age_until(5)
        school_sample.age_until(10)
        school_sample.age_until(15)
        school_sample.age_until(82)

        self.assertEqual(26, school_sample.fishcount_on_day(18))
        self.assertEqual(5934, school_sample.fishcount_on_day(80))

        school_input = School(INPUT_STR[0], use_counts = True)
        self.assertEqual(5, school_sample.fishcount_on_day(0))
        school_input.age_until(82)

        self.assertEqual(377263, school_input.fishcount_on_day(80))

        school_input.age_until(257)

        # Answer part 2
        self.assertEqual(1695929023803, school_input.fishcount_on_day(256))

if __name__ == '__main__':
    unittest.main()
