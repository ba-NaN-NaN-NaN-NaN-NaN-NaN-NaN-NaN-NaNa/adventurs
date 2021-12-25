import unittest
import re

def readlines_from(fname):
    f = open(fname)
    lines = f.readlines()
    f.close()
    lines = [r.strip() for r in lines if len(r.strip()) > 0]
    return lines

INPUT_STR = readlines_from("../input/2021_d24.txt")

# Approaches ?
# ?? Replace 'inp w' with 'w=i0'.
# Static single assignment, referencing parent?
# From this, can we derive the expression tree for an assigment?
# I.e. we must be able to ask:
#   if register 'w' must contain '1' at this point, what valid inputs can we have for i0..i13 if i = [6,None,3,...None]?

inp_matcher = re.compile("^inp (?P<register>.)$")
op_matcher = re.compile("^(?P<optype>add|div|eql|inp|mod|mul) (?P<lhs>[^ ]+) (?P<rhs>[^ ]+)$")



EXPR_SIMPLIFICATIONS = {
    #"(i0) + (4) * ((12) == (i0))" 
}

def lines_orig_2arg_ops(lines_orig):
    i_idx = 0
    toreturn = []

    for line in lines_orig:
        res = inp_matcher.match(line)
        if res:
            line = "inp %s i%d" % (res['register'], i_idx)
            i_idx+=1
            toreturn.append(line)
        else:
            toreturn.append(line)
    return toreturn



def token_to_refkey(token, registers):
    """
    Registers must be dict of type
    { 'x': 'const:5',
     'y': 'add@5'}
    """

    return f

class Op:

    @classmethod
    def build(cls, key, opstr, lhs, rhs, registers):
        if opstr == 'mul':
            return Mul(key, opstr, lhs, rhs, registers)
        elif opstr == 'div':
            return Div(key, opstr, lhs, rhs, registers)
        elif opstr == 'add':
            return Add(key, opstr, lhs, rhs, registers)
        elif opstr == 'sub':
            return Sub(key, opstr, lhs, rhs, registers)
        elif opstr == 'eql':
            return Eql(key, opstr, lhs, rhs, registers)
        elif opstr == 'inp':
            return Inp(key, opstr, lhs, rhs, registers)
        elif opstr == 'mod':
            return Mod(key, opstr, lhs, rhs, registers)
        else:
            print("ggf ?? %s" % key)
            return 1/0

    def __init__(self, key, opstr, lhs, rhs, registers):
        return 1/0
        self._key = key
        self._opstr = opstr
        print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._rhs = rhs
        self._rhs_key = rhs._key

    def tostr_as_ssa(self):
        """
        """
        return "%s %s %s" % (self._key, self._lhs_key, self._rhs_key)



class Mul(Op):
    def __init__(self, key, opstr, lhs, rhs, registers):
        self._key = key
        #self._opstr = opstr
        #print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._rhs = rhs
        self._rhs_key = rhs._key
        self._simplified = None

    def simplified(self, tree_elements):
        if self._simplified != None:
            return self._simplified

        self._lhs_simplified = self._lhs.simplified(tree_elements)
        self._rhs_simplified = self._rhs.simplified(tree_elements)
        
        if type(self._lhs_simplified) == Const:
            if self._lhs_simplified._value == 0:
                # 0 * N -> 0
                self._simplified = self._lhs_simplified
                return self._simplified

            if self._lhs_simplified._value == 1:
                # 1 * N -> n
                self._simplified = self._rhs_simplified
                return self._simplified

            if type(self._rhs_simplified) == Const:
                # create and return a new const here??
                print("kjgfdhkfgjhsk")
                return 1/0
                
        if type(self._rhs_simplified) == Const:
            if self._rhs_simplified._value == 0:
                # N * 0 -> n
                return self._rhs_simplified

            if self._rhs_simplified._value == 1:
                # N * 1 -> n
                return self._lhs_simplified


        return self

    def expr(self, tree_elements):
        return "%s * %s" % (self._lhs.simplified(tree_elements).expr(tree_elements), self._rhs.simplified(tree_elements).expr(tree_elements))

class Div(Op):
    def __init__(self, key, opstr, lhs, rhs, registers):
        self._key = key
        #self._opstr = opstr
        #print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._rhs = rhs
        self._rhs_key = rhs._key
        self._simplified = None

    def simplified(self, tree_elements):
        if self._simplified != None:
            return self._simplified

        self._lhs_simplified = self._lhs.simplified(tree_elements)
        self._rhs_simplified = self._rhs.simplified(tree_elements)
        
        if type(self._lhs_simplified) == Const and self._lhs_simplified._value == 0:
            # 0 / N == 0
            return self._lhs_simplified

        if type(self._rhs_simplified) == Const and self._rhs_simplified._value == 1:
            # n / 1 -> n
            return self._lhs_simplified

        return self


    def expr(self, tree_elements):
        return "%s / %s" % (self._lhs.simplified(tree_elements).expr(tree_elements), self._rhs.simplified(tree_elements).expr(tree_elements))

class Mod(Op):
    def __init__(self, key, opstr, lhs, rhs, registers):
        self._key = key
        #self._opstr = opstr
        #print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._rhs = rhs
        self._rhs_key = rhs._key
        self._simplified = None

    def simplified(self, tree_elements):


        if self._simplified != None:
            return self._simplified

        self._lhs_simplified = self._lhs.simplified(tree_elements)
        self._rhs_simplified = self._rhs.simplified(tree_elements)
        
        #print("Mod %s: type(self._lhs_simplified) is %s" % (self._key,type(self._lhs_simplified)))
        if type(self._lhs_simplified) == Const:
            #print("Mod %s: type(self._lhs_simplified) is %s with value '%d " % (self._key,type(self._lhs_simplified), self._lhs_simplified._value))
            if self._lhs_simplified._value == 0:
                # 0 % N == 0
                return tree_elements['const:0']
            #return self._rhs.simplified(tree_elements)
        else:
            #print("Mod %s: type(self._lhs_simplified) is %s" % (self._key,type(self._lhs_simplified)))
            pass

        if type(self._rhs_simplified) == Const and self._rhs_simplified._value == 1:
            return self._lhs.simplified(tree_elements)

        #print("Mod %s: Could not simplify" % self._key)
        return self

    def expr(self, tree_elements):
        return "(%s) %% (%s)" % (self._lhs.simplified(tree_elements).expr(tree_elements), self._rhs.simplified(tree_elements).expr(tree_elements))


class Add(Op):
    def __init__(self, key, opstr, lhs, rhs, registers):
        self._key = key
        #self._opstr = opstr
        #print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._lhs_simplified = None

        self._rhs = rhs
        self._rhs_key = rhs._key
        self._rhs_simplified = None

        self._simplified = None


    def simplified(self, tree_elements):

        if self._simplified != None:
            return self._simplified

        self._lhs_simplified = self._lhs.simplified(tree_elements)
        self._rhs_simplified = self._rhs.simplified(tree_elements)


        # Also case here: const + const

        if type(self._lhs_simplified) == Const and self._lhs_simplified._value == 0:
            self._simplified = self._rhs.simplified(tree_elements)
            return self._simplified

        if type(self._rhs_simplified) == Const and self._rhs_simplified._value == 0:
            self._simplified = self._lhs.simplified(tree_elements)
            return self._simplified

        return self

    def expr(self, tree_elements):
        return "(%s) + (%s)" % (self._lhs.simplified(tree_elements).expr(tree_elements), self._rhs.simplified(tree_elements).expr(tree_elements))


class Sub(Op):
    def __init__(self, key, opstr, lhs, rhs, registers):
        self._key = key
        #self._opstr = opstr
        #print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._rhs = rhs
        self._rhs_key = rhs._key

    def simplified(self, tree_elements):
        if self._rhs_key == 'const:0':
            return self._lhs.simplified(tree_elements)

        return self

    def expr(self, tree_elements):
        return "(%s) - (%s)" % (self._lhs.simplified(tree_elements).expr(tree_elements), self._rhs.simplified(tree_elements).expr(tree_elements))

class Eql(Op):
    def __init__(self, key, opstr, lhs, rhs, registers):
        self._key = key
        #self._opstr = opstr
        #print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._rhs = rhs
        self._rhs_key = rhs._key
        self._simplified = None
        self._expr = None

    def simplified(self, tree_elements):
        if self._simplified is not None:
            return self._simplified

        self._lhs_simplified = self._lhs.simplified(tree_elements)
        self._rhs_simplified = self._rhs.simplified(tree_elements)

        if type(self._lhs_simplified) == Const:
            if type(self._rhs_simplified) == Input:
                # For things like i0 == 12. Not possible, since i0 can only be 1...9
                if self._lhs_simplified.value(tree_elements, None) not in range(1,10):
                    return tree_elements['const:0']


        return self

    def expr(self, tree_elements):
        if self._expr is not None:
            return self._expr

        self.simplified(tree_elements) # Init:s lhs_simplified.

        return "(%s) == (%s)" % ( self._lhs_simplified.expr(tree_elements), self._rhs_simplified.expr(tree_elements))


class Inp(Op):
    def __init__(self, key, opstr, lhs, rhs, registers):
        self._key = key
        #self._opstr = opstr
        #print("lhs is '%s'<%s>" % (lhs, type(lhs)))
        self._lhs = lhs
        self._lhs_key = lhs._key
        self._rhs = rhs
        self._rhs_key = rhs._key

    def simplified(self, tree_elements):

        return self._rhs.simplified(tree_elements)

    def expr(self, tree_elements):
        return self._rhs.simplified(tree_elements)



class Const:
    def __init__(self, key, value):
        pass
        if not key.startswith("const:"):
            return 1/0
        self._key = "const:%d" % value
        self._value = int(value)

    def tostr_as_ssa(self):
        """
        """
        return self._key

    def simplified(self, tree_elements):
        return self

    def expr(self, tree_elements):
        return "%d" % self._value

    def value(self, tree_elements, inputs):
        return self._value


class Input:
    def __init__(self, key, number):
        pass
        if number < 0:
            return 1/0

        if number > 13:
            return 1/0

        key2 = "i%d" % number
        if key != key2:
            return 1/0

        self._key = key

    def tostr_as_ssa(self):
        """
        """
        return self._key


    def simplified(self, tree_elements):
        return self

    def expr(self, tree_elements):
        return self._key


class Machine:
    def __init__(self, lines_orig):
        self._lines_orig = lines_orig
        self._lines = lines_orig_2arg_ops(lines_orig)
        self._tree_elements = {
            'const:0':Const('const:0', 0),
        }
        self._curr_w = self._tree_elements['const:0']
        self._curr_x = self._tree_elements['const:0']
        self._curr_y = self._tree_elements['const:0']
        self._curr_z = self._tree_elements['const:0']

    def init_tree_elements(self):
        """
        """
        for line_nr in range(len(self._lines)):
            line = self._lines[line_nr]
            print("Will regexp line #%d: '%s' " % (line_nr, line))
            res = op_matcher.match(line)
            res = res.groupdict()

            key = "%s@%d" % (res['optype'], line_nr)
           
            lhs_key = None
            rhs_key = None

            registers = {
                'w': self._curr_w,
                'x': self._curr_x,
                'y': self._curr_y,
                'z': self._curr_z,
            }

            lhs_key = res['lhs']
            
            if res['lhs'] in registers:
                lhs = registers[res['lhs']]
            else:
                lhs_int = int(res['lhs'])
                lhs = Const('const:%d' % lhs_int, lhs_int)

            rhs = None
            if res['rhs'] in registers:
                rhs = registers[res['rhs']]
            elif res['rhs'].startswith("i"):
                nr = res['rhs'][1:]
                nr = int(nr)
                rhs = Input(res['rhs'], nr)
            else:
                rhs_int = int(res['rhs'])
                rhs = Const('const:%d' % rhs_int, rhs_int)

            print("Will build op %s,%s,%s,%s,%s," % (key, res['optype'], lhs, rhs, registers))
            op = Op.build(key, res['optype'], lhs, rhs, registers)
            #self._tree_elements[lhs_key] = op
            if lhs_key == 'w':
                self._curr_w = op
            elif lhs_key == 'x':
                self._curr_x = op
            elif lhs_key == 'y':
                self._curr_y = op
            elif lhs_key == 'z':
                self._curr_z = op
            else:
                return 1/0

            print("Tree elements are now '%s'" % self._tree_elements)

            #print(op.tostr_as_ssa())

            print("Simplified: '%s' into '%s' " % (op, op.simplified(self._tree_elements).expr(self._tree_elements)))

        

    



class TestEntryPoint(unittest.TestCase):

    def test_load(self):
        ms = Machine(INPUT_STR)
        ms.init_tree_elements()
       
    def test_lines_orig_2arg_ops(self):
        lines = lines_orig_2arg_ops(INPUT_STR)
        lines = [l for l in lines if 'inp' in l]
        print(lines)

        self.assertEqual(5,7)

    def test_part1(self):
        pass

    def test_part2(self):
        pass

if __name__ == '__main__':
    unittest.main()
