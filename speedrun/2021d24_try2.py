import unittest

import re

def readlines_from(fname):
    f = open(fname)
    lines = f.readlines()
    f.close()
    lines = [r.strip() for r in lines if len(r.strip()) > 0]
    return lines

INPUT_STR = readlines_from("../input/2021_d24.txt")


def perform_op(line, w, x, y, z, inputs_left):
    """
    """
    parts = line.strip().split(" ")
    #print(parts)

    op = parts[0]
    lhs = parts[1]

    if op == 'inp':
        # Deal with our only binary operation.
        bool_as_int = inputs_left[0]
        #print("READ input. Was '%s' is now '%s'" % (inputs_left, inputs_left[1:]))
        inputs_left = inputs_left[1:]

        if lhs == 'w':
            return bool_as_int, x, y, z, inputs_left
        elif lhs == 'x':
            return w, bool_as_int, y, z, inputs_left
        elif lhs == 'y':
            return w, x, bool_as_int, z, inputs_left
        elif lhs == 'z':
            return w, x, y, bool_as_int, inputs_left
        else:
            return 1/0


    rhs = parts[2]

    if parts[0] == 'add':
        if rhs in ['w', 'x', 'y', 'z']:
            rhs_val = {'w':w, 'x':x, 'y':y, 'z':z}[rhs]
        else:
            rhs_val = int(rhs)

        if lhs == 'w':
            return w+rhs_val, x, y, z, inputs_left
        elif lhs == 'x':
            return w, x+rhs_val, y, z, inputs_left
        elif lhs == 'y':
            return w, x, y+rhs_val, z, inputs_left
        elif lhs == 'z':
            return w, x, y, z+rhs_val, inputs_left
        else:
            return 1/0

    if parts[0] == 'mul':
        if rhs in ['w', 'x', 'y', 'z']:
            rhs_val = {'w':w, 'x':x, 'y':y, 'z':z}[rhs]
        else:
            rhs_val = int(rhs)

        if lhs == 'w':
            return w*rhs_val, x, y, z, inputs_left
        elif lhs == 'x':
            return w, x*rhs_val, y, z, inputs_left
        elif lhs == 'y':
            return w, x, y*rhs_val, z, inputs_left
        elif lhs == 'z':
            return w, x, y, z*rhs_val, inputs_left
        else:
            return 1/0


    if parts[0] == 'mod':
        if rhs in ['w', 'x', 'y', 'z']:
            rhs_val = {'w':w, 'x':x, 'y':y, 'z':z}[rhs]
        else:
            rhs_val = int(rhs)

        if lhs == 'w':
            return w%rhs_val, x, y, z, inputs_left
        elif lhs == 'x':
            return w, x%rhs_val, y, z, inputs_left
        elif lhs == 'y':
            return w, x, y%rhs_val, z, inputs_left
        elif lhs == 'z':
            return w, x, y, z%rhs_val, inputs_left
        else:
            return 1/0


    if parts[0] == 'div':
        if rhs in ['w', 'x', 'y', 'z']:
            rhs_val = {'w':w, 'x':x, 'y':y, 'z':z}[rhs]
        else:
            rhs_val = int(rhs)

        if lhs == 'w':
            return int(w/rhs_val), x, y, z, inputs_left
        elif lhs == 'x':
            return w, int(x/rhs_val), y, z, inputs_left
        elif lhs == 'y':
            return w, x, int(y/rhs_val), z, inputs_left
        elif lhs == 'z':
            return w, x, y, int(z/rhs_val), inputs_left
        else:
            return 1/0


    if parts[0] == 'eql':
        if rhs in ['w', 'x', 'y', 'z']:
            rhs_val = {'w':w, 'x':x, 'y':y, 'z':z}[rhs]
        else:
            rhs_val = int(rhs)

        lhs_val = {'w':w, 'x':x, 'y':y, 'z':z}[lhs]
        if lhs_val == rhs_val:
            bool_as_int = 1
        else:
            bool_as_int = 0

        if lhs == 'w':
            return bool_as_int, x, y, z, inputs_left
        elif lhs == 'x':
            return w, bool_as_int, y, z, inputs_left
        elif lhs == 'y':
            return w, x, bool_as_int, z, inputs_left
        elif lhs == 'z':
            return w, x, y, bool_as_int, inputs_left
        else:
            return 1/0



    print("Not implemented '%s'" % line)
    return 1/0
    return w, x, y, z, inputs_left

def perform_program(lines, w, x, y, z, inputs_left):
    """
    """
    w, x, y, z, inputs_left =  w, x, y, z, inputs_left
    lines_left = lines[:]
    while len(lines_left) > 0:
        line = lines_left[0]
        lines_left = lines_left[1:]
        w, x, y, z, inputs_left = perform_op(line, w, x, y, z, inputs_left)
    return w, x, y, z, inputs_left



def lines_into_chunks(lines):
    """
    """

    toreturn = []
    this_chunk = []
    for line in lines:
        if line.startswith('inp'):
            toreturn.append(this_chunk)
            this_chunk = [line]
        else:
            this_chunk.append(line)
            continue

    toreturn.append(this_chunk)
    toreturn = toreturn[1:]

    for chunknr in range(len(toreturn)):
        chunk = toreturn[chunknr]
        #print("Chunk #%d: %s" % (chunknr, chunk))
    return toreturn



# Given z_in_n and iN which valid z_out do we get?
# Given z_in_n and iN and  valid z_out:s do we get?
#
# Given list of valid z_out_N:s,
# then for each possible input N
# What possible pairs of 
#    (z_in_N, iN)
# exist?

def valid_inputs_chunk_13(program):
    return get_valid_inputs(program, 13, [0])



def valid_inputs_chunk_12(program):
    """
    Obsolete.
    """
    valid_output_pairs = valid_inputs_chunk_13(program)
    print("valid_output_pairs for chunk 12 = '%s'" % valid_output_pairs)

    valid_output_zs = set()
    for z, _ in valid_output_pairs:
        valid_output_zs.add(z)

    return get_valid_inputs(program, 12, valid_output_zs)
    
def valid_inputs_chunk_N(program, chunk_nr):
    if chunk_nr == 13:
        return valid_inputs_chunk_13(program)

    valid_output_pairs = valid_inputs_chunk_N(program, chunk_nr+1)
    print("valid_output_pairs for chunk %d = '%s'" % (chunk_nr, valid_output_pairs))

    valid_output_zs = set()
    for z, _ in valid_output_pairs:
        valid_output_zs.add(z)

    return get_valid_inputs(program, chunk_nr, valid_output_zs)    

def nr_to_stack(n):
    n_orig = n
    toreturn = []
    while n > 0:
        toreturn.append(n%26)
        n = int(n/26)
    if len(toreturn) == 0:
        toreturn = [0]
    #print("%d -> %s" % (n_orig, toreturn))
    return toreturn


def get_valid_inputs(program, chunk_nr, valid_outputs):
    """
    Return list of 
    (z_in, digit input)
    """

    print("get_valid_inputs(). Chunk_nr is %d, valid_outputs is %s" % (chunk_nr, valid_outputs))

    # In general, max range to test is 26 ** N, where N is steps remaining.

    ranges_to_test = [range(1, 2**32) for _ in range(14)] 


    #    c9 -> div z 1     <- w_in cap is 26**3. This is  17576, but in practice this is   9820 
    #    c10 -> div z 26   <- w_in cap is 26**4. This is 456976, but in practice this is 255334
    #    c11 -> div z 26   <- w_in cap is 26**3. This is  17576, but in practice this is   9821 

    #ranges_to_test[10] = range(0, 26**4) # for 10, max z_in=255334 experimentally.
    ranges_to_test[9] =  range(0, 9820+100) 
    ranges_to_test[10] = range(0, 255334+100)
    ranges_to_test[11] = range(0, 9820+100)
    ranges_to_test[12] = range(0, 555)
    ranges_to_test[13] = range(0, 30)



    range_to_test = ranges_to_test[chunk_nr]
    lines = program._chunks[chunk_nr]

    toreturn = []
    

    possible_outputs = set() # A chunk can only give limited outputs?

    for z_in in range_to_test:
        for indigit in range(1, 9+1):
            chunk_input = [indigit]
            _, _, _, z_out, _ = perform_program(lines, 2**20, 2**20, 2**20, z_in, chunk_input)
            if z_out in valid_outputs:
                print("Chunk %d found valid input pair. z_in=%d and indigit=%d" % (chunk_nr, z_in, indigit))
                toreturn.append((z_in, indigit))
                possible_outputs.add(z_out)

    outputs = sorted(list(possible_outputs))
    for out in outputs:
        nr_to_stack(out)

    #print("Possible outputs for chunk_nr %d is %s" % (chunk_nr, outputs))

    return toreturn


class Program:
    def __init__(self, lines, inputs):
        self._lines = lines
        self._inputs = inputs
        self._chunks = lines_into_chunks(lines)
        #self._registers_before_chunk = [None] * 14
        self._registers_after_chunk = [None] * 14

        #self._registers_before_chunk[0] = [0,0,0,0]

        if len(inputs) != 14:
            return 1/0
        if len(self._chunks) != 14:
            return 1/0





    def run_chunk(self, chunk_nr):
        if chunk_nr == 0:
            registers_before = [0,0,0,0]
        else:
            registers_before = self.get_registers_after_chunk(chunk_nr-1)
        w, x, y, z = registers_before        
        lines_in_chunk = self._chunks[chunk_nr]
        input_to_chunk = [self._inputs[chunk_nr]] # Yeah, chunk is expected to eat 1 and exactly 1 input.

        w, x, y, z, inputs_left = perform_program(lines_in_chunk, w, x, y, z, input_to_chunk)
        if len(inputs_left) != 0:
            print("Performing lines '%s' for chunk %d with input '%s' gave inputs_left = '%s'" % (lines_in_chunk, chunk_nr, input_to_chunk, inputs_left))
            return 1/0

        self._registers_after_chunk[chunk_nr] = [w, x, y, z]


    def brute_force_chunk_states(self, chunk_nr, possible_z_input_map):
        """
        Given all possible inputs to a chunk, what outputs can we have?
        """

        lines_in_chunk = self._chunks[chunk_nr]
        outputs = {}


        # We need to divide w by 26 each time at the end if we want to be able to reach 0.
        # That means we can limit how many states we need to check here. 
        #
        # for 10, max z_in=237081 experimentally. max_z_out = 9820.
        # for 11, max z_in=9821 experimentally. max_z_out = 377
        # for 12, max z_in=377 experimentally. max_z_out = 14
        # for 13, max z_in=14 experimentally. max_z_out = 0
        #  


        # However, this is not completely true. Let us look at the div:s in the running program.

        # 11881376
        #   616734
        """
        c0 -> div z 1      <- w_in cap is 26**0
        c1 -> div z 1     <- w_in cap is 26**1
        c2 -> div z 1     <- w_in cap is 26**2
        c3 -> div z 1     <- w_in cap is 26**3
        c4 -> div z 1     <- w_in cap is 26**4
        c5 -> div z 26    <- w_in cap is 26**5
        c6 -> div z 1     <- w_in cap is 26**4
        c7 -> div z 26    <- w_in cap is 26**5
        c8 -> div z 26    <- w_in cap is 26**4. This is 456976, but in practice this is ??
        c9 -> div z 1     <- w_in cap is 26**3. This is  17576, but in practice this is   9820 
        c10 -> div z 26   <- w_in cap is 26**4. This is 456976, but in practice this is 255334
        c11 -> div z 26   <- w_in cap is 26**3. This is  17576, but in practice this is   9821 
        c12 -> div z 26   <- w_in cap is 26**2. This is    676, but in practice this is    377
        c13 -> div z 26   <- w_in cap is 26**1. This is     26, but in practice this is     14   
        """

        #if chunk_nr == 6:
        #    reject_z_over = 920 * 26 * 26

        reject_zouts_over = 36**6

        #reject_zouts_over = 9820 * 26 * 26 * 26 + 100
        if chunk_nr == 0:
            reject_zouts_over = 15 # Can be at most i0+4

        if chunk_nr == 1:
            reject_zouts_over = 9999 # Can be at most 26*i0 + i1 + 115. Constants here maaaan...

        """
        Outputs for chunk_nr 0 has 9 possible.
        Outputs for chunk_nr 1 has 81 possible.
        Outputs for chunk_nr 2 has 729 possible.
        Outputs for chunk_nr 3 has 6561 possible.
        Outputs for chunk_nr 4 has 59049 possible.
        Outputs for chunk_nr 5 has 65610 possible.
        Outputs for chunk_nr 6 has 590490 possible.
        Outputs for chunk_nr 7 has 616734 possible.
        Outputs for chunk_nr 8 has 607986 possible.  <- Getting to here = 5 minutes
        Outputs for chunk_nr 9 has 0 possible.
        Outputs for chunk_nr 10 has 0 possible.
        Outputs for chunk_nr 11 has 0 possible.
        Outputs for chunk_nr 12 has 0 possible.
        Outputs for chunk_nr 13 has 0 possible.
        """            

        """
        Outputs for chunk_nr 7 has 616734 possible.
        Outputs for chunk_nr 8 has 607986 possible.
        Outputs for chunk_nr 9 has 6561 possible.
        Outputs for chunk_nr 10 has 7290 possible.
        Outputs for chunk_nr 11 has 7938 possible.
        Outputs for chunk_nr 12 has 1575 possible.
        Outputs for chunk_nr 13 has 127 possible.
        """

        if chunk_nr == 9:
            reject_zouts_over = (9820 + 100) * 26
        if chunk_nr == 10:
            reject_zouts_over = (255334 + 100) * 26
        if chunk_nr == 11:
            reject_zouts_over = (9821 + 100) * 26
        if chunk_nr == 12:
            reject_zouts_over = (377 + 100) * 26
        if chunk_nr == 13:
            reject_zouts_over = (14 + 100) * 26

        #print("Doing chunk nr %d with input map %s" % (chunk_nr, str(possible_z_input_map)))

        for inval in range(1, 10):
            input_numbers = [inval]
            for z_in, digits_to_get_here in possible_z_input_map.items():
                _, _ , _, z_out, inputs_left = perform_program(lines_in_chunk, 0, 0, 0, z_in, input_numbers)

                if z_out < reject_zouts_over:
                    #print("z_out < reject_zouts_over = %d %d" % (z_out, reject_zouts_over))

                    # outputs =
                    # {
                    #   digit_sequence: max_z_value,
                    # }

                    digit_seq = digits_to_get_here * 10 + inval
                    if z_out in outputs:
                        # We have seen sequences which lead to this z_out before.
                        prev_known_digit_seq = outputs[z_out]
                        """
                        if digit_seq > prev_known_digit_seq: -> 92915979999498
                            outputs[z_out] = digit_seq    
                        """
                        if digit_seq < prev_known_digit_seq:
                            outputs[z_out] = digit_seq    

                    else:
                        outputs[z_out] = digit_seq
                else:
                    #print("Rejecting due to z_out < reject_zouts_over = %d %d" % (z_out, reject_zouts_over))
                    pass

        #print("Outputs for chunk_nr %d is %s" % (chunk_nr, str(outputs)))
        print("Outputs for chunk_nr %d has %d possible." % (chunk_nr, len(outputs)))
        return outputs


    def f(self): 
        """
        states_before {
            # state nr, best sequence of digits we can use to get there.
            0:0,
        }

        for z_in in states_before:
            for input_nr in for i in range(1,10):


        :


        {
            5555: 123,
            5556: 125,
        }
        """



    def get_registers_after_chunk(self, chunk_nr):
        toreturn = self._registers_after_chunk[chunk_nr]
        if toreturn is None:
            self.run_chunk(chunk_nr)

        return self._registers_after_chunk[chunk_nr]





class TestEntryPoint(unittest.TestCase):

    def test_ops(self):

        w, x, y, z, inputs_left = perform_op('add x y', 111, 5, 11, 111, [])
        self.assertEqual(x, 16)

        w, x, y, z, inputs_left = perform_op('add w 5', 1, 2, 3, 4, [])
        self.assertEqual([1+5, 2, 3, 4], [w, x, y, z])


    def text_program(self):

        for i0 in range(1, 10):
            for i1 in range(1, 10):
                for i2 in range(1, 10):
                    chunknr = 13
                    inputs = [1]*14
                    inputs[0] = i0
                    inputs[1] = i1
                    inputs[2] = i2
                    p = Program(INPUT_STR, inputs)
                    w, x, y, z = p.get_registers_after_chunk(chunknr)
                    print("For inputs '%s', chunk #%d returned -> %d, %d, %d, %d" % (inputs, chunknr, w,x,y,z))
            

    def test_foo(self):
        p = Program(INPUT_STR, [0]*14)
        valid_outputs_13 = get_valid_inputs(p, 13, set([0]))
        print("Got valid_outputs_13='%s'" % valid_outputs_13)
        return 1/0

    def test_how_ok_after_chunk_13(self):
        # From this: Input is valid if:
        #   z going into chunk nr 13 = i13 + 5
        # zout13 = zout12 == i13+5
        # So any possible legal z_in_13 is in the range 6...14   <- 9 possible values
        # And each i13 must be 1:1 for possible inputs.

        for fiddle_z in range(200):
            for i13 in range(1, 10):
                chunknr = 13
                inputs = [2]*14
                inputs[13] = i13
                p = Program(INPUT_STR, inputs)
                p._registers_after_chunk[chunknr-1] = [2**32,2**32,2**32, fiddle_z]
                w, x, y, z = p.get_registers_after_chunk(chunknr)
                if z == 0:
                    #print("For inputs '%s' and z=%d before running chunk, chunk #%d returned -> %d, %d, %d, %d" % (inputs, fiddle_z, chunknr, w,x,y,z))
                    pass
            
    def texst_valid_inputs_chunk_13(self):
        inputs = [None]*14
        p = Program(INPUT_STR, inputs)
        pairs = valid_inputs_chunk_13(p)
        self.assertEqual(9, len(pairs))

    def test_what_outputs_after_chunk_11(self):
        #
        # Constraint from #12: Valid output from here must be in range 6...377
        #     
        # z_divved  = int(z_in_11/26)
        """
        p = Program(INPUT_STR, [-10000]*14)
        res = valid_inputs_chunk_N(p, 12)

        max_ok = 0
        for pair in res:
            if pair[0] > max_ok:
                max_ok = pair[0]
        self.assertEqual(max_ok, 377)


        p = Program(INPUT_STR, [-10000]*14)
        res = valid_inputs_chunk_N(p, 11)

        max_ok = 0
        for pair in res:
            if pair[0] > max_ok:
                max_ok = pair[0]
        self.assertEqual(max_ok, 9820)
        

        p = Program(INPUT_STR, [-10000]*14)
        res = valid_inputs_chunk_N(p, 10)

        max_ok = 0
        for pair in res:
            if pair[0] > max_ok:
                max_ok = pair[0]
        self.assertEqual(max_ok, 255334)


        """
        """
        p = Program(INPUT_STR, [-10000]*14)
        res = valid_inputs_chunk_N(p, 6)

        max_ok = 0
        for pair in res:
            if pair[0] > max_ok:
                max_ok = pair[0]
        self.assertEqual(max_ok, 255344434)

        """

        return 1/0

        self.assertEqual(64, len(res))

    def texst_output_lengths(self):
        p = Program(INPUT_STR, [-10000]*14)

        res = valid_inputs_chunk_N(p, 13)
        self.assertEqual(9, len(res))

        res = valid_inputs_chunk_N(p, 12)
        self.assertEqual(81, len(res))

        #res = valid_inputs_chunk_N(p, 11)
        #self.assertEqual(999999999999, len(res))
        
        #res = valid_inputs_chunk_N(p, 10)
        #self.assertEqual(9999999999999999999999, len(res))


    def texst_what_outputs_after_chunk_12(self):
        #
        # Constraint from #13: Valid output from here must be in range 6...14
        # 
        # z_divved  = int(z_in_12/26)
        # z_modded = z % 26
        # if z_modded - 4 == i12:
        #     valid return z_divved
        # else:
        #     Not valid.
        #
        # Given z_divved range, valid z_in_12 range is 6*26 ... 14*26
        # Also: z_modded must be in range 5...13
        # Also: z_in_12 must be z_modded - 4
        #
        # This means there exists 
        #

        for fiddle_z in range(520):
            for i12 in range(1, 10):
                chunknr = 12
                inputs = [2]*14
                inputs[12] = i12
                p = Program(INPUT_STR, inputs)
                p._registers_after_chunk[chunknr-1] = [2**32,2**32,2**32, fiddle_z]
                w, x, y, z = p.get_registers_after_chunk(chunknr)

                if 6 <= z and z <= 14:
                    print("For inputs '%s' and z=%d before running chunk. z/26=%d. z%%26=%d, chunk #%d returned -> %d, %d, %d, %d" % (inputs, fiddle_z, int(fiddle_z/26), fiddle_z%26, chunknr, w,x,y,z))
                    pass

        print("DETERMINING VALID INPUTS CHUNK 12")
        p = Program(INPUT_STR, [-10000]*14)
        res1 = valid_inputs_chunk_N(p, 12)
        res2 = valid_inputs_chunk_12(p)
        print("DETERMINED %d of %d VALID INPUTS CHUNK 12" % (len(res1), len(res2)))

        #res2 = valid_inputs_chunk_N(p, 0)
        return 1/0

    def tesxt_fooooo(self):
        
        p = Program(INPUT_STR, [2]*14)
        states = p.brute_force_chunk_states(0, {0:0})
        
        states = p.brute_force_chunk_states(1, states)
        """
        states = p.brute_force_chunk_states(2, states)
        states = p.brute_force_chunk_states(3, states)
        states = p.brute_force_chunk_states(4, states)
        states = p.brute_force_chunk_states(5, states)
        states = p.brute_force_chunk_states(6, states)
        states = p.brute_force_chunk_states(7, states)
        states = p.brute_force_chunk_states(8, states)
        states = p.brute_force_chunk_states(9, states)
        states = p.brute_force_chunk_states(10, states)
        states = p.brute_force_chunk_states(11, states)
        """
        states = p.brute_force_chunk_states(12, states)
        
        states = p.brute_force_chunk_states(13, states)
        self.assertEqual(states[0], 66666666666666666666666666666) # What sequence gets us back to state 0?
        self.assertEqual(3, states)

    def test_fooooo(self):
        
        p = Program(INPUT_STR, [2]*14)
        states = p.brute_force_chunk_states(0, {0:0})
        states = p.brute_force_chunk_states(1, states)
        states = p.brute_force_chunk_states(2, states)
        states = p.brute_force_chunk_states(3, states)
    
        states = p.brute_force_chunk_states(4, states)
        states = p.brute_force_chunk_states(5, states)
        states = p.brute_force_chunk_states(6, states)
        states = p.brute_force_chunk_states(7, states)
        states = p.brute_force_chunk_states(8, states)
        states = p.brute_force_chunk_states(9, states)

        states = p.brute_force_chunk_states(10, states)
        states = p.brute_force_chunk_states(11, states)
        states = p.brute_force_chunk_states(12, states)        
        states = p.brute_force_chunk_states(13, states)
        self.assertEqual(states[0], 66666666666666666666666666666) # What sequence gets us back to state 0?
        self.assertEqual(3, states)

        # 21611513911181 is too low. Perhaps answer to nr 2? (Yes it was.)
       
    def tesxt_ox_ratinxg(self):
        #print(INPUT_STR)
        self.assertEqual("10111", ox_rating(SAMPLE_STR))
        self.assertEqual("01010", co2_rating(SAMPLE_STR))

        self.assertEqual(part2(SAMPLE_STR), 230)
        self.assertEqual(part2(INPUT_STR), 5941884)

    def test_part1(self):
        pass

    def test_part2(self):
        pass

if __name__ == '__main__':
    unittest.main()
