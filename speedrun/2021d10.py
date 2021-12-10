import unittest

f = open("../input/2021_d10.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d10_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


openers = "([{<"
closers = ")]}>"

# Expected closers for given item on stack
matcher = {
    "(":")",
    "[":"]",
    "{":"}",
    "<":">",
}

score_for = {
    ")":3,
    "]":57,
    "}":1197,
    ">":25137,
}

c_score_for = {
    ")":1,
    "]":2,
    "}":3,
    ">":4,
}

def str_to_completion_score(input):
    """
    }}]])})] - 288957 total points.
    """
    score = 0
    for ch in input:
        ch_score = c_score_for[ch]
        score = score * 5 + ch_score
    return score

class Row:
    def __init__(self, row):
        self._row = row.strip()


    def error_score(self):
        stack = []
        for ch in self._row:
            if ch in openers:
                stack.append(ch)
            elif ch in closers:
                if len(stack) == 0:
                    return score_for[ch]

                if matcher[stack[-1]] == ch:
                    stack.pop(-1)
                else:
                    return score_for[ch]

        return 0

    def is_corrupted(self):
        return self.error_score() != 0


    def completion_score(self):
        toreturn = self.completion_required()
        if len(toreturn) > 0:
            return str_to_completion_score(toreturn)
        return None

    def completion_required(self):
        stack = []
        for ch in self._row:
            if ch in openers:
                stack.append(ch)
            elif ch in closers:
                if len(stack) == 0:
                    return None

                if matcher[stack[-1]] == ch:
                    stack.pop(-1)
                else:
                    return None

        # We are now left with unmatched characters. I will use letters in this example.
        # Assume "a" closes "A", "b" closes "B" etc. So string "ABCDEF" should close by "fedcba".
        # 
        # We must do two things:
        #   Find the closer for each individual character:   "ABCDEF" -> "abcdef"
        #   Reverse the closer string.                       "abcdef" -> "fedcba"
        # Note that we can do the two things in either order.

        toreturn = []
        while len(stack) > 0:
            last = stack.pop(-1)
            toreturn.append(matcher[last])

        toreturn = "".join(toreturn)
        # print("For row %s completion required is %s" % (self._row, toreturn))
        return toreturn



class TestEntryPoint(unittest.TestCase):

    def test_error_score(self):

        self.assertEqual(Row(")").error_score(), 3)
        self.assertEqual(Row("]").error_score(), 57)
        self.assertEqual(Row("}").error_score(), 1197)
        self.assertEqual(Row(">").error_score(), 25137)
        self.assertEqual(Row("{([(<{}[<>[]}>{[]{[(<()>").error_score(), 1197)
       

    def test_part1(self):
        total_score = 0
        for row in SAMPLE_STR:
            r = Row(row)
            total_score += r.error_score()
        self.assertEqual(total_score, 26397)

        total_score = 0
        for row in INPUT_STR:
            r = Row(row)
            total_score += r.error_score()
        self.assertEqual(total_score, 392139)


    def test_str_to_completion_score(self):
        self.assertEqual(str_to_completion_score("}}>}>))))"), 1480781)

    def test_part2(self):
        scores = []
        for row in SAMPLE_STR:
            r = Row(row)
            if r.is_corrupted():
                continue
            scores.append(r.completion_score())

        scores = sorted(scores)
        median = scores[int(len(scores)/2)]
        print("Completion scores are '%s' -> %d" % (str(scores), median))
        self.assertEqual(median, 288957)

        scores = []
        for row in INPUT_STR:
            r = Row(row)
            if r.is_corrupted():
                continue
            scores.append(r.completion_score())

        scores = sorted(scores)
        median = scores[int(len(scores)/2)]
        print("Completion scores are '%s' -> %d" % (str(scores), median))
        self.assertEqual(median, 4001832844)


if __name__ == '__main__':
    unittest.main()
