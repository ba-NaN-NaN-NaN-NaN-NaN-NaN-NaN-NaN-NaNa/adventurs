import unittest
import copy
import json

f = open("../input/2021_d18.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

"""
Samples in text today.

f = open("../input/2021_d18_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]
"""

class SFPair:
    def __init__(self, row=None, left=None, right=None):
        """
        ONE row of input.
        """
        
        if row is not None:
            pair = json.loads(row)
            
            if len(pair) != 2:
                raise TypeError("init pair not 2")

            self._left = pair[0]
            if type(self._left) == list:
                new_left = SFPair(left=self._left[0], right=self._left[1])
                self._left = new_left

            self._right = pair[1]
            if type(self._right) == list:
                new_right = SFPair(left=self._right[0], right=self._right[1])
                self._right = new_right


        else:
            if left is None:
                raise TypeError("kfgidhfgd")
            if right is None:
                raise TypeError("kfgidhfgd")
            self._left = left
            self._right = right
            if type(self._left) == list:
                new_left = SFPair(left=self._left[0], right=self._left[1])
                self._left = new_left

            if type(self._right) == list:
                new_right = SFPair(left=self._right[0], right=self._right[1])
                self._right = new_right


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

    def __str__(self):
        return self.__repr__()

    def __repr__(self):
        lpart = str(self._left)
        rpart = str(self._right)
        return "["+lpart+","+rpart+"]"


    def sf_add(self, other):
        """
        Add two SFPairs together, reducing as needed
        """
        left = copy.deepcopy(self)
        right = copy.deepcopy(other)
        toreturn = SFPair(left=left, right=right)
        #toreturn.repeated_reduce(verbose=True)
        toreturn.repeated_reduce()
        return toreturn


    def __eq__(self, other):
        """
        Operator overload.
        """
        return str(self) == str(other)

    def do_single_reduce(self):
        """
        Returns the action as a string, or None
        """
        if self.needs_to_explode():
            self.do_single_explode()
            return 'explode'

        if self.do_single_split():
            return 'split'

        return None
    
    def repeated_reduce(self, verbose=False):
        """
        Verbose if you want reduction output.
        """
        while True:
            res = self.do_single_reduce()

            if verbose:
                op = str(res)+":"
                op = op.ljust(10, " ")
                print("after %s %s" % (op, self))

            if res is None:
                return




    def needs_to_explode(self, current_depth=1):
        """
        Return true if pair needs to explode.
        """
        if current_depth==4:
            if type(self._left) == SFPair:
                return True

            if type(self._right) == SFPair:
                return True

        if type(self._left) == SFPair:
            if self._left.needs_to_explode(current_depth+1):
                return True

        if type(self._right) == SFPair:
            if self._right.needs_to_explode(current_depth+1):
                return True

        return False

    def add_to_leftmost(self, to_add):
        """
        Add a number to left tree, recursively.
        """
        if type(self._left) == int:
            self._left += to_add
            return

        self._left.add_to_leftmost(to_add)

    def add_to_rightmost(self, to_add):
        """
        Add a number to right tree, recursively.
        """
        if type(self._right) == int:
            self._right += to_add
            return

        self._right.add_to_rightmost(to_add)

    def has_two_ints(self):
        return type(self._left) == int and type(self._right) == int


    def do_single_explode(self, current_depth=1, parent=None):
        """
        Do one explode, then return.

        return [int, int] with values that need to be propagated outwards.
        """

        #print("Will explode %s, current_depth=%d" % (self, current_depth))

        if current_depth >= 4:
            if type(self._left) == SFPair and self._left.has_two_ints():
                # When left explodes:
                #  - Add the right number to the right tree, along left edge.
                #  - Propagate left number upwards.
                #print("My left int pair will explode! %s" % self)
                return_left = self._left._left
                to_add_right = self._left._right
                self._left = 0
                if type(self._right) == int:
                    self._right += to_add_right
                else:
                    self._right.add_to_leftmost(to_add_right)
                toreturn = [return_left, 0]
                #print("Done, am now '%s'. Returning '%s'" % (self, toreturn))
                return toreturn


            if type(self._right) == SFPair and  self._right.has_two_ints():
                #print("My right int pair will explode! %s" % self)
                return_right = self._right._right
                to_add_left = self._right._left
                self._right = 0
                #self.add_to_leftmost(to_add_left)
                if type(self._left) == int:
                    self._left += to_add_left
                else:
                    self._left.add_to_rightmost(to_add_left)
                toreturn = [0, return_right]
                #print("Done, am now '%s'. Returning '%s'" % (self, toreturn))
                return toreturn
                
        #print("My branch types are '%s' '%s'. Parent type is %s" % (type(self._left), type(self._right), type(parent)))
        if type(self._left) == SFPair:
            res = self._left.do_single_explode(current_depth = current_depth+1, parent=self)
            if res:
                #print("I am %s and my LEFT exploded with res '%s" % (self, res))
                if res[1] != 0:
                    if type(self._right) == int:
                        self._right += res[1]
                    else:
                        self._right.add_to_leftmost(res[1])
                    res[1] = 0

                #print("After my LEFT explosion I am %s with res %s" % (self, res))
                return res

        if type(self._right) == SFPair:
            res = self._right.do_single_explode(current_depth = current_depth+1, parent=self)
            if res:
                #print("I am %s and my RIGHT exploded with res '%s" % (self, res))
                if res[0] != 0:
                    if type(self._left) == int:
                        self._left += res[0]
                    else:
                        self._left.add_to_rightmost(res[0])
                    
                    res[0] = 0
                #print("After my RIGHT explosion I am %s with res %s" % (self, res))
                return res
            
        return None


    def do_single_split(self):
        """
        Do at most one split, then return.

        Return true if changed.
        """

        if type(self._left) == int:
            if self._left >= 10:
                sum_of_l = self._left
                new_ll = int(sum_of_l/2)
                new_lr = sum_of_l - new_ll
                # After this new_ll + new_lr == old value
                new_left = SFPair(left=new_ll, right=new_lr)
                self._left = new_left
                return True

        elif self._left.do_single_split():
            return True

        if type(self._right) == int:
            if self._right >= 10:
                sum_of_r = self._right
                new_rl = int(sum_of_r/2)
                new_rr = sum_of_r - new_rl
                # fkhs
                new_right = SFPair(left=new_rl, right=new_rr)
                self._right = new_right
                return True

        elif self._right.do_single_split():
            return True


        return False        

    def magnitude(self):
        toreturn = 0
        if type(self._left) == int:
            toreturn += 3 * self._left
        else:
            toreturn += 3 * self._left.magnitude()

        if type(self._right) == int:
            toreturn += 2 * self._right
        else:
            toreturn += 2 * self._right.magnitude()

        return toreturn

class TestEntryPoint(unittest.TestCase):

    def test_sf_equals(self):
        left=SFPair("[1,2]")
        right=SFPair("[1,2]")
        print("test_equals EQ %s %s" % (left, right))
        self.assertEqual(left, right)
        #self.assertEqual(5,7)


    def test_single_reduce_foo(self):
        """
        pre = "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
        post = "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
        to_reduce = SFPair(pre)
        to_reduce.do_single_reduce()
        self.assertEqual(to_reduce, post)
        """

        pre = "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
        post = "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
        to_reduce = SFPair(pre)
        to_reduce.do_single_reduce()
        self.assertEqual(to_reduce, post)

    def texst_reduce_foo(self):
        to_reduce = SFPair("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")
        to_reduce.repeated_reduce(verbose=True)

        self.assertEqual(5,76)

    def test_sf_add(self):
        """
        left=SFPair("[1,2]")
        right=SFPair("[[3,4],5]")
        summed = left.sf_add(right)
        
        expected = SFPair("[[1,2],[[3,4],5]]")
        self.assertEqual("%s" % summed, "%s" % expected)
        """

        l = SFPair("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
        r = SFPair("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")

        total = SFPair(left=l, right=r)
        print("Now need to reduce %s" % total)

        actual = l.sf_add(r)
        expected = SFPair("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        self.assertEqual(expected, actual)

    def test_parse_repr(self):

        
        self.assertEqual("[3,3]", "%s" % SFPair("[3,3]"))

        for row in INPUT_STR:
            sfp = SFPair(row)
            self.assertEqual("%s" % sfp, row)


    def test_single_explode(self):


        sf = SFPair("[7,[[8,4],9]]")
        self.assertTrue(sf.needs_to_explode(3))
        sf.do_single_explode(3)
        self.assertEqual(sf, SFPair("[15,[0,13]]"))

        #return

        sf = SFPair("[[[[[9,8],1],2],3],4]")
        self.assertTrue(sf.needs_to_explode())
        sf.do_single_explode()
        self.assertEqual(sf, SFPair("[[[[0,9],2],3],4]"))


        sf = SFPair("[7,[6,[5,[4,[3,2]]]]]")
        self.assertTrue(sf.needs_to_explode())
        sf.do_single_explode()
        self.assertEqual(sf, SFPair("[7,[6,[5,[7,0]]]]"))


        sf = SFPair("[[6,[5,[4,[3,2]]]],1]")
        self.assertTrue(sf.needs_to_explode())
        sf.do_single_explode()
        self.assertEqual(sf, SFPair("[[6,[5,[7,0]]],3]"))

        
        sf = SFPair("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
        self.assertTrue(sf.needs_to_explode())
        sf.do_single_explode()
        self.assertEqual(sf, SFPair("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))


        sf = SFPair("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        self.assertTrue(sf.needs_to_explode())
        sf.do_single_explode()
        self.assertEqual(sf, SFPair("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))

        sf = SFPair("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")
        self.assertTrue(sf.needs_to_explode())
        sf.do_single_explode()
        self.assertEqual(sf, SFPair("[[[[0,7],4],[15,[0,13]]],[1,1]]"))

    def test_do_single_reduce(self):

        res1 = SFPair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
        res2 = SFPair("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")
        res3 = SFPair("[[[[0,7],4],[15,[0,13]]],[1,1]]")
        res4 = SFPair("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
        res5 = SFPair("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
        res6 = SFPair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")

        res1_after = copy.deepcopy(res1)
        res1_after.do_single_reduce()
        self.assertEqual(res1_after, res2)

        res2_after = copy.deepcopy(res2)
        res2_after.do_single_reduce()
        self.assertEqual(res2_after, res3)

    def test_repeated_reduce(self):
        """
        From example
        after addition: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
        after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
        after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
        after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
        after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
        after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
        """
        left = [[[[4,3],4],4],[7,[[8,4],9]]]
        right = [1,1]

        sf = SFPair(left=left, right=right)
        print("")
        print("after add:       %s" % sf)
        sf.repeated_reduce(verbose=True)


        self.assertEqual(sf, SFPair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"))


    def test_magnitude(self):
        self.assertEqual(29, SFPair("[9,1]").magnitude())
        
        self.assertEqual(SFPair("[[1,2],[[3,4],5]]").magnitude(), 143)
        self.assertEqual(SFPair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),  1384)
        self.assertEqual(SFPair("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),  445)
        self.assertEqual(SFPair("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),  791)
        self.assertEqual(SFPair("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),  1137)
        self.assertEqual(SFPair("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),  3488)


    def test_add(self):
        """
        Very long add example.
        """


        self.assertEqual(SFPair("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").sf_add(SFPair("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")),
                         SFPair("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"))

        self.assertEqual(SFPair("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").sf_add(SFPair("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")),
                         SFPair("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"))

        self.assertEqual(SFPair("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]").sf_add(SFPair("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]")),
                         SFPair("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"))

        self.assertEqual(SFPair("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]").sf_add(SFPair("[7,[5,[[3,8],[1,4]]]]")),
                         SFPair("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"))

        self.assertEqual(SFPair("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]").sf_add(SFPair("[[2,[2,2]],[8,[8,1]]]")),
                         SFPair("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"))

        self.assertEqual(SFPair("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]").sf_add(SFPair("[2,9]")),
                         SFPair("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"))

        self.assertEqual(SFPair("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]").sf_add(SFPair("[1,[[[9,3],9],[[9,0],[0,7]]]]")),
                         SFPair("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"))

        self.assertEqual(SFPair("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]").sf_add(SFPair("[[[5,[7,4]],7],1]")),
                         SFPair("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"))

        self.assertEqual(SFPair("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]").sf_add(SFPair("[[[[4,2],2],6],[8,7]]")),
                         SFPair("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"))


    def test_part1(self):

        # Slightly larger example
        lines = """[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]""".split("\n")

        final = final_sum(lines)
        self.assertEqual(final, SFPair("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"))

        #return
        # So, given this example homework assignment:
        lines = """[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]""".split("\n")

        # The final sum is:
        # [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]
        final = final_sum(lines)

        self.assertEqual(final, SFPair("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"))
        self.assertEqual(final.magnitude(), 4140)



        # REAL
        final = final_sum(INPUT_STR)
        self.assertEqual(final.magnitude(), 4417)

    def test_part2(self):
        pass
        lines = """[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]""".split("\n")

        self.assertEqual(3993, largest_mag_any_two(lines))
        
        self.assertEqual(4796, largest_mag_any_two(INPUT_STR))
        


def final_sum(assignment_lines):
    sf = SFPair(assignment_lines[0])
    for line in assignment_lines[1:]:
        to_add = SFPair(line)
        sf = sf.sf_add(to_add)
        #added = SFPair(left=sf, right=to_add)
        #added.repeated_reduce()
        print("While summing: %s" % sf)
        #sf = added
    return sf

def largest_mag_any_two(assignment_lines):
    toreturn = 0
    pairs = [SFPair(line) for line in assignment_lines]

    for p1 in pairs:
        for p2 in pairs:
            #line1 = assignment_lines[n]
            #line2 = assignment_lines[n+1]
            mag1 = p1.sf_add(p2).magnitude()
            mag2 = p2.sf_add(p1).magnitude()
            toreturn = max([toreturn, mag1, mag2])

        
    return toreturn

if __name__ == '__main__':
    unittest.main()
