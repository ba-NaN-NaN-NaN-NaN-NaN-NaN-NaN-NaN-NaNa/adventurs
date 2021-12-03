import unittest

f = open("../input/2021_d3.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d3_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


def distr_at(lines, n):
    """
    Returns thing like { "0":22, "1":66}
    """
    toreturn = {"0":0, "1":0}
    for line in lines:
        part = line[n:n+1]
        if part in toreturn:
            toreturn[part] += 1
        else:
            toreturn[part] = 1
    #print(toreturn) 
    return toreturn


def gamma(lines) -> str:
    toreturn = ""
    for offset in range(len(lines[0])):
        dist = distr_at(lines, offset)
        if dist["0"] > dist["1"]:
            toreturn += "0"
        else:
            toreturn += "1"
    return toreturn

def epsilon(lines) -> str:
    toreturn = ""
    for offset in range(len(lines[0])):
        dist = distr_at(lines, offset)
        if dist["0"] < dist["1"]:
            toreturn += "0"
        else:
            toreturn += "1"
            
    return toreturn


def ox_rating(lines) -> str:
    worklist = lines[:]
    for offset in range(len(lines[0])):
        if len(worklist) == 1:
            continue
        #worklist = filter_match_maj(worklist, offset)
        dist = distr_at(worklist, offset)
        if dist["0"] == dist["1"]:
            worklist_new = [l for l in worklist if l[offset] == "1"]
        elif dist["0"] > dist["1"]:
            worklist_new = [l for l in worklist if l[offset] == "0"]
        else:
            worklist_new = [l for l in worklist if l[offset] == "1"]

        #print("With dist %s reducing worklist from %s to %s" % (dist, worklist, worklist_new))
        print("Ox: offset %d reduced worklist from %d to %d" % (offset, len(worklist), len(worklist_new)))
        worklist = worklist_new

    if len(worklist) > 1:
        return 1/0
    return worklist[0]

def co2_rating(lines) -> str:
    worklist = lines[:]
    for offset in range(len(lines[0])):
        if len(worklist) == 1:
            continue
        #worklist = filter_match_maj(worklist, offset)
        dist = distr_at(worklist, offset)
        if dist["0"] == dist["1"]:
            worklist_new = [l for l in worklist if l[offset] == "0"]
        elif dist["0"] < dist["1"]:
            worklist_new = [l for l in worklist if l[offset] == "0"]
        else:
            worklist_new = [l for l in worklist if l[offset] == "1"]

        #print("With dist %s reducing worklist from %s to %s" % (dist, worklist, worklist_new))
        print("CO2: Offset %d reduced worklist from %d to %d" % (offset, len(worklist), len(worklist_new)))
        worklist = worklist_new


    if len(worklist) > 1:
        return 1/0
    return worklist[0]    
    

def part1(lines) -> int:
    g = int("0b" + gamma(lines), 2)
    e = int("0b" + epsilon(lines), 2)
    return g * e

def part2(lines) -> int:
    ox = int("0b" + ox_rating(lines), 2)
    co2 = int("0b" + co2_rating(lines), 2)
    return ox * co2

class TestStringMethods(unittest.TestCase):

    def test_gamma(self):

        self.assertEqual(gamma(SAMPLE_STR), "10110")
        self.assertEqual(epsilon(SAMPLE_STR), "01001")

        self.assertEqual(part1(SAMPLE_STR), 198)
        self.assertEqual(part1(INPUT_STR), 4006064)
       
    def test_ox_rating(self):
        #print(INPUT_STR)
        self.assertEqual("10111", ox_rating(SAMPLE_STR))
        self.assertEqual("01010", co2_rating(SAMPLE_STR))

        self.assertEqual(part2(SAMPLE_STR), 230)
        self.assertEqual(part2(INPUT_STR), 5941884)

if __name__ == '__main__':
    unittest.main()
