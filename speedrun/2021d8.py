import unittest
import copy

f = open("../input/2021_d8.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d8_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


def knockout(seq_a, seq_b):
    """
    If seq_a = 'abcdefg' and 'seq_b' = 'bcf'
    then return 'adeg'
    """
    toreturn = []
    #bad = seq_b.split("")
    for a in seq_a:
        if a in seq_b:
            continue

        toreturn.append(a)
    return "".join(sorted(list(set(toreturn))))


class Row:
    def __init__(self, line):
        line_parts = line.split("|")

        mystery_inputs = []
        all_inputs = set()

        for seg in line_parts[0].strip().split(" "):
            seg = "".join(sorted(seg))
            all_inputs.add(seg)

        for seg in line_parts[1].strip().split(" "):
            seg = "".join(sorted(seg))
            mystery_inputs.append(seg)
            all_inputs.add(seg)
            #print("Seg is '%s'" % seg)

        self._mystery_inputs = mystery_inputs
        self._all_inputs = all_inputs

        self._seq_to_int = {}
        self._int_to_seq = {}

        #print("Formatted row is '%s | %s'" % (" ".join(self._all_inputs), " ".join(self._mystery_inputs)))
        self.iterate_mapping()

    def get_mystery_int(self):
        """
        Return int.
        """
        parts = ["%d" % self._seq_to_int[s] for s in self._mystery_inputs]
        to_parse = "".join(parts)
        return int(to_parse)


    def iterate_mapping(self):
        to_int = self._seq_to_int
        to_seq = self._int_to_seq

        for seg in self._all_inputs:
            if len(seg) == 2:
                to_int[seg] = 1
                to_seq[1] = seg

            if len(seg) == 3:
                to_int[seg] = 7
                to_seq[7] = seg

            if len(seg) == 4:
                to_int[seg] = 4
                to_seq[4] = seg

            if len(seg) == 7:
                to_int[seg] = 8
                to_seq[8] = seg



        for seg in self._all_inputs:
            if len(seg) == 6:
                if len(knockout(seg, to_seq[7])) == 4:
                    to_int[seg] = 6
                    to_seq[6] = seg

                elif len(knockout(seg, to_seq[4])) == 2:
                    to_int[seg] = 9
                    to_seq[9] = seg

                else:
                    to_int[seg] = 0
                    to_seq[0] = seg


        for seg in self._all_inputs:
            if len(seg) == 5:
                if (len(knockout(seg, to_seq[1])) == 4 and
                    len(knockout(seg, to_seq[4])) == 3 and
                    len(knockout(seg, to_seq[7])) == 3):
                    to_int[seg] = 2
                    to_seq[2] = seg

                if (len(knockout(seg, to_seq[1])) == 3 and
                    len(knockout(seg, to_seq[4])) == 2 and
                    len(knockout(seg, to_seq[7])) == 2):
                    to_int[seg] = 3
                    to_seq[3] = seg

                if (len(knockout(seg, to_seq[1])) == 4 and
                    len(knockout(seg, to_seq[4])) == 2 and
                    len(knockout(seg, to_seq[7])) == 3):
                    to_int[seg] = 5
                    to_seq[5] = seg


        #print("After iterating, to_int mapping is now '%s'" % to_int)
        



def part1(lines):
    """
    """
    toreturn = 0
    for line in lines:
        out_part = line.split("|")[1].strip()
        output_values = out_part.split(" ")

        unique_output_values = [val.strip() for val in output_values if len(val.strip()) in [7,3,2,4]]
        #print(output_values)
        toreturn += len(unique_output_values)

    return toreturn


class TestEntryPoint(unittest.TestCase):

    def test_knockout(self):
        self.assertEqual("adeg", knockout("abcdefg", "bcf"))
        self.assertEqual("ade", knockout("abcdefg", "bcfg"))
        self.assertEqual("abcdefg", knockout("abcdefg", ""))

    def test_row(self):
        row = Row("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe")
        signals = [set([n for n in "be"]), 
                   set([n for n in "ebd"])]
        #self.assertEqual(row.sig_seq_to_digit(signals), "17")

    def test_part1(self):
        self.assertEqual(part1(SAMPLE_STR), 26)
        self.assertEqual(part1(INPUT_STR), 495)

    def test_part2(self):
        i = 0
        for string in SAMPLE_STR:
            row = Row(string)
            i += row.get_mystery_int()
        self.assertEqual(i, 61229)


        i = 0
        for string in INPUT_STR:
            row = Row(string)
            i += row.get_mystery_int()
        self.assertEqual(i, 1055164)

if __name__ == '__main__':
    unittest.main()
