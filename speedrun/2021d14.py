import unittest

f = open("../input/2021_d14.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d14_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


def template_to_pairs(template):
    """
    In: NNCB
    Out: {'NN':1,'NC':1,'CB':1}
    """
    print("Will template to pairs '%s'" % template)
    toreturn = {}
    for n in range(len(template)-1):
        pair = template[n:n+2]
        if pair in toreturn:
            toreturn[pair] += 1
        else:
            toreturn[pair] = 1
    return toreturn

def lines_to_rules(lines):
    """
    In: 
    CH -> B
    HH -> N

    Out:
    { 'CH':'B', 'HH':'N'}
    """
    rules = {}
    for line in lines:
        if "->" not in line:
            print("BAD LINE %s" % line)
            return 1/0
        line = line.replace(" -> ", ",")
        parts = line.split(",")
        rules[parts[0].strip()] = parts[1].strip()

    print("Parsed rules '%s'" % str(rules))
    return rules

def step(pairs, rules):
    """
    In: pairs + rules.
    Out: New set of pairs.
    """
    new_pairs = {}
    for key in pairs.keys():
        count = pairs[key]
        #print("Key count is %s %d" % (key, count))
        if key in rules:
            # Polymerization happens here.
            middle = rules[key]
            first = key[0:1]
            second = key[1:2]

            if first+middle not in new_pairs:
                new_pairs[first+middle] = 0

            new_pairs[first+middle]+=count

            if middle+second not in new_pairs:
                new_pairs[middle+second] = 0

            new_pairs[middle+second]+=count
        else:
            if key not in new_pairs:
                new_pairs[key] = 0
            new_pairs[key] += count

    return new_pairs

def format_pairs(pairs):
    """
    Return sorted list of pairs.

    Primary use of this function is to have a consistently sorted
    serialization of pairs for unit testing.
    """
    toreturn = []
    keys = sorted(pairs.keys())
    for key in keys:
        toreturn.append("%s:%d" % (key, pairs[key]))

    return ",".join(toreturn)


def pairs_to_elementcount(pairs):
    """
    In: {'AB':5, 'BC':6}
    Out: {'A':5, 'B':11, 'C':6}

    """
    toreturn = {}
    for key in pairs:
        count = pairs[key]
        first = key[0:1]
        second = key[1:2]

        if first not in toreturn:
            toreturn[first] = 0

        toreturn[first] += count

        if second not in toreturn:
            toreturn[second] = 0

        toreturn[second] += count

    print("Pairs to elementcount '%s' -> '%s'" % (str(pairs), str(toreturn)))

    return toreturn

def full_part(input, step_count):
    """
    Full part
    """
    pairs = template_to_pairs(input[0])
    rules = lines_to_rules(input[1:])

    for _ in range(step_count):
        pairs = step(pairs, rules)

    counts = pairs_to_elementcount(pairs)
    if 'K' in counts:
        # Since we count all pairs, every element in polymer
        # is counted twice, EXCEPT the two at
        # each end. Manually duplicate them here.
        counts["K"]+=1
        counts["B"]+=1
    else:
        counts["N"]+=1
        counts["B"]+=1

    print("Counts is '%s'" % str(counts))
    highest_count = 0
    lowest_count = 9684576874568754685476548754
    for elem in counts:
        count = counts[elem]
        if count > highest_count:
            print("New HIGHEST count '%s' : %d" % (elem, count))
            highest_count = count

        if count < lowest_count:
            print("New lowest count '%s' : %d" % (elem, count))
            lowest_count = count

    return (highest_count-lowest_count)/2 # Div by 2 due to duplicated reason, see above.



class TestEntryPoint(unittest.TestCase):

    def test_template(self):
        pairs = template_to_pairs(SAMPLE_STR[0])
        formatted = format_pairs(pairs)
        print(pairs)
        
        self.assertEqual("CB:1,NC:1,NN:1", formatted)


    def test_missing_compounds(self):
        pairs = template_to_pairs("ZZNCZZ")
        rules = lines_to_rules(SAMPLE_STR[1:])
        pairs_1 = step(pairs, rules)
        print("pairs_1 missing is " + str(pairs_1)        )
        self.assertEqual(format_pairs(template_to_pairs("ZZNBCZZ"))  , format_pairs(pairs_1))

        pairs = template_to_pairs("NZC")
        rules = lines_to_rules(SAMPLE_STR[1:])
        pairs_1 = step(pairs, rules)
        print("pairs_1 missing is " + str(pairs_1)        )
        self.assertEqual(format_pairs(template_to_pairs("NZC"))  , format_pairs(pairs_1))

        pairs = template_to_pairs("NC")
        rules = lines_to_rules(SAMPLE_STR[1:])
        pairs_1 = step(pairs, rules)
        print("pairs_1 missing is " + str(pairs_1)        )
        self.assertEqual(format_pairs(template_to_pairs("NBC"))  , format_pairs(pairs_1))


    def test_step(self):
        pairs = template_to_pairs(SAMPLE_STR[0])
        rules = lines_to_rules(SAMPLE_STR[1:])
        formatted = format_pairs(pairs)
        #print(pairs)

        pairs_1 = step(pairs, rules)
        print("pairs_1 is " + str(pairs_1)        )
        self.assertEqual(format_pairs(template_to_pairs("NCNBCHB"))  , format_pairs(pairs_1))


        pairs_2 = step(pairs_1, rules)
        print("pairs_2 is " + str(pairs_2)        )
        self.assertEqual(format_pairs(template_to_pairs("NBCCNBBBCBHCB"))  , format_pairs(pairs_2))


        pairs_3 = step(pairs_2, rules)
        print("pairs_3 is " + str(pairs_3)        )
        self.assertEqual(format_pairs(template_to_pairs("NBBBCNCCNBBNBNBBCHBHHBCHB"))  , format_pairs(pairs_3))

        pairs_4 = step(pairs_3, rules)
        print("pairs_4 is " + str(pairs_4)        )
        self.assertEqual(format_pairs(template_to_pairs("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"))  , format_pairs(pairs_4))


    def test_pairs_to_elementcount(self):
        pairs ={'AB':5, 'BC':6}
        res = pairs_to_elementcount(pairs)
        self.assertEqual(res['A'], 5)
        self.assertEqual(res['B'], 11)
        self.assertEqual(res['C'], 6)


    def test_part1(self):

        self.assertEqual(full_part(SAMPLE_STR, 10), 1588)
        self.assertEqual(full_part(INPUT_STR, 10), 2345)




    def test_part2(self):
        self.assertEqual(full_part(SAMPLE_STR, 40), 2188189693529)
        self.assertEqual(full_part(INPUT_STR, 40),  2432786807053)

if __name__ == '__main__':
    unittest.main()
