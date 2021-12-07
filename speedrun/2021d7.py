import unittest

f = open("../input/2021_d7.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d7_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


def cost_single_p1(x, y):
    return abs(x-y)

def cost_single_p2(x, y):
    diff = abs(x-y)
    cost_total = 0
    cost_one = 1
    # This part can be done better. return n + (n-1) * (n-1) ?
    while diff > 0:
        diff -= 1
        cost_total += cost_one
        cost_one += 1

    return cost_total


def fuel_costs_p1(gatherpoint, locations):
    cost = 0
    for loc in locations:
        cost += cost_single_p1(gatherpoint, loc)
    return cost

def fuel_costs_p2(gatherpoint, locations):
    cost = 0
    for loc in locations:
        cost += cost_single_p2(gatherpoint, loc)
    return cost

def input_to_ints(input):
    return [int(s) for s in input.split(",")]

class TestEntryPoint(unittest.TestCase):


    def test_cost_one_move(self):
        self.assertEqual(5, cost_single_p1(7,2))
        self.assertEqual(5, cost_single_p1(2,7))
        self.assertEqual(66, cost_single_p2(16,5))

    def test_fuel_costs_p1(self):
        sample_locs = input_to_ints(SAMPLE_STR[0])
        self.assertEqual(37, fuel_costs_p1(2, sample_locs))
        self.assertEqual(41, fuel_costs_p1(1, sample_locs))
        self.assertEqual(71, fuel_costs_p1(10, sample_locs))

        real_locs = input_to_ints(INPUT_STR[0])

        best_n = 0
        best_cost = 5987589746
        for n in range(300, 350):
            cost = fuel_costs_p1(n, real_locs)

            if cost < best_cost:
                best_n = n
                best_cost = cost
                print("New best part 1 n=%d -> cost=%d" % (n, best_cost))

        # Correct ans = 329389
        self.assertEqual(329389, best_cost)       


    def test_fuel_costs_p2(self):
        sample_locs = input_to_ints(SAMPLE_STR[0])
        self.assertEqual(206, fuel_costs_p2(2, sample_locs))
        self.assertEqual(168, fuel_costs_p2(5, sample_locs))

        real_locs = input_to_ints(INPUT_STR[0])

        best_n = 0
        best_cost = 5987589746
        for n in range(400, 500):
            cost = fuel_costs_p2(n, real_locs)

            if cost < best_cost:
                best_n = n
                best_cost = cost
                print("New best part 2 n=%d -> cost=%d" % (n, best_cost))
        # Correct ans = 86397080
        self.assertEqual(86397080, best_cost)       


if __name__ == '__main__':
    unittest.main()
# 329389
# 86397080