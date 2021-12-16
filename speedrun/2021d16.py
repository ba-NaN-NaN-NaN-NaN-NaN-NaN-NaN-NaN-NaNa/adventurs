import unittest

f = open("../input/2021_d16.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d16_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]

def hex_to_bits(hexstr):
    toreturn = ""
    for ch in hexstr:
        int_val = int(ch, 16)
        bitst = bin(int_val)
        bitst = bitst[2:]
        while len(bitst) < 4:
            bitst = "0" + bitst
        toreturn += bitst

        #print("%s -> %s" % (ch, bitst))
    return toreturn


def eat_literal_value(bitstr):
    """
    Eat a literal value.

    Must contain a bunch of 5-bit chunks.

    OBS not packet parser. Do not pass in header in bitstr

    Return (int, remainder)
    """
    to_int = ""
    left_to_eat = bitstr[:]
    print("eat_literal_value(%s)" % bitstr)
    while len(left_to_eat) > 0 and left_to_eat[0] == "1":
        chunk = left_to_eat[0:5]
        print("Ate chunk '%s'" % chunk)
        to_int += left_to_eat[1:5]
        left_to_eat = left_to_eat[5:]

    # Final should end in "0"
    chunk = left_to_eat[0:5]
    print("Ate last chunk '%s'" % chunk)
    to_int += left_to_eat[1:5]
    left_to_eat = left_to_eat[5:]

    print("Eat literal(%s) -> int(%s, 2)" % (bitstr, to_int))
    return int(to_int, 2), left_to_eat


def eat_a_packet(bitstr):
    """
    Return Packet, remainder
    """
    version_bits = bitstr[0:3]
    type_id_bits = bitstr[3:6]
    payload = bitstr[6:]
    if type_id_bits == "100":
        # Literal value
        lit_value, remainder = eat_literal_value(payload)
        print("eat_a_packet(%s) -> Ate a packet. Lit_value =%d, remainder = '%s'" % (bitstr, lit_value, remainder))
        return Packet(version_bits=version_bits, type_id_bits=type_id_bits, lit_value=lit_value), remainder


    else:
        # Operator

        subpackets, remainder = eat_subpackets(payload)
        print("eat_a_packet(%s) -> Ate an operator packet. subpacket count =%d, remainder = '%s'" % (bitstr, len(subpackets), remainder))
        return Packet(version_bits=version_bits, type_id_bits=type_id_bits, subpackets=subpackets), remainder

def eat_operator_packet(bitstr):
    """
    Return Packet, remainder

    MUST inclide version and type id bits.    
    """
    version_bits = bitstr[0:3]
    type_id_bits = bitstr[3:6]
    payload = bitstr[7:]
    subpackets, remainder = eat_subpackets(payload)


def eat_subpackets(payload):
    """
    An operator packet contains one or more packets. To indicate which subsequent binary data 
    represents its sub-packets, an operator packet can use one of two modes indicated by the bit
    immediately after the packet header; this is called the length type ID.

    returns subpackets, remainder
    """
    subtype_id = payload[0:1]
    plen_bits = None
    packets_bits = None
    remainder = None
    subpackets = []
    if subtype_id == "0":
        # Len bits tell us how many bits are packets
        pkt_len_bits = payload[1:16]
        if len(pkt_len_bits) != 15:
            raise TypeError("fds for '%s'" % payload)
        pkt_len = int(pkt_len_bits, 2)

        to_eat = payload[16:]
        packets_bits = to_eat[0:pkt_len]
        remainder = to_eat[pkt_len:]
        print("")
        print("0 Payload '%s' -> pkt_len_bits '%s' and packets_bits '%s'" % (payload, pkt_len_bits, packets_bits))
        print("0          I" + ("L"*len(pkt_len_bits)) + ("^"*len(packets_bits))  )

        packets = []

        while "1" in packets_bits:
            # I.e. while we don't only have padding left.
            print("Will eat a packet from bitstream '%s'" % packets_bits)
            packet, packets_bits = eat_a_packet(packets_bits)
            packets.append(packet)

        return packets, remainder
    
    elif subtype_id == "1":
        pkt_count_bits = payload[1:12]
        if len(pkt_count_bits) != 11:
            raise TypeError("fds")

        pkt_count = int(pkt_count_bits, 2)
        to_eat = payload[12:]

        print("")
        print("1 Payload '%s' -> pkt_count_bits '%s' and packets_bits '%s'" % (payload, pkt_count_bits, packets_bits))
        print("1          I" + ("L"*len(pkt_count_bits)))

        sub_packets = []
        for _ in range(pkt_count):
            packet, to_eat = eat_a_packet(to_eat)
            sub_packets.append(packet)

        return sub_packets, to_eat


        
            

    else:
        return 1/0

    print("Operator has %d packet_bits" % len(packets_bits))
    


class Packet:
    def __init__(self, version_bits, type_id_bits, lit_value=None, subpackets=None):
        self._version_bits = version_bits
        self._version = int(version_bits, 2)

        self._type_id_bits = type_id_bits
        self._type_id = int(type_id_bits, 2)

        self._lit_value = lit_value
       
        self._subpackets = subpackets

        

    def packet_type(self):
        if self._type_id == 4:
            return 'literal'

        return 'operator'

    def version_sum(self):
        print("Version sum is '%s'" % self._version)
        toreturn = self._version
        if self._subpackets is not None:
            for p in self._subpackets:
                toreturn += p.version_sum()

        return toreturn

    def num_sub_packets(self):
        """
        How many subpackets do we have?
        """
        return len(self._subpackets)

    def print_packet_tree(self, prefix):
        this_pretty = "<%s v%2d>" % (self.packet_type(), self._version)
        print(prefix + this_pretty)
        if self._subpackets is not None:
            for p in self._subpackets:
                p.print_packet_tree(prefix + "  |-")

    def expr_value(self):
        if self._type_id == 0:
            # type ID 0 are sum packets
            toreturn = 0
            for p in self._subpackets:
                toreturn += p.expr_value()
            return toreturn

        if self._type_id == 1:
            # type ID 1 are product packets
            toreturn = 1
            for p in self._subpackets:
                toreturn *= p.expr_value()
            return toreturn

        if self._type_id == 2:
            toreturn = 658936596534973654
            for p in self._subpackets:
                val = p.expr_value()
                if val < toreturn:
                    toreturn = val
    
            return toreturn

        if self._type_id == 3:
            # ID 3 are maximum packets
            toreturn = -658936596534973654
            for p in self._subpackets:
                val = p.expr_value()
                if val > toreturn:
                    toreturn = val
    
            return toreturn

        if self._type_id == 4:
            return self._lit_value

        if self._type_id == 5:
            # ID 5 are greater than packets
            if len(self._subpackets) != 2:
                raise TypeError("fkjdfdhgjfkdh")

            if self._subpackets[0].expr_value() > self._subpackets[1].expr_value():
                return 1
            else:
                return 0

        if self._type_id == 6:
            # ID 6 are less than packet
            if len(self._subpackets) != 2:
                raise TypeError("fkjdfdhgjfkdh")

            if self._subpackets[0].expr_value() < self._subpackets[1].expr_value():
                return 1
            else:
                return 0
    
        if self._type_id == 7:
            # ID 7 are equal to packets
            if len(self._subpackets) != 2:
                raise TypeError("fkjdfdhgjfkdh")

            if self._subpackets[0].expr_value() == self._subpackets[1].expr_value():
                return 1
            else:
                return 0

        return 'operator'        


class TestEntryPoint(unittest.TestCase):

    def test_bitstr(self):
        self.assertEqual("0000110001010000", hex_to_bits("0C50"))
        

    def test_eat_literal_value(self):
        i, left = eat_literal_value("101111111000101000")
        self.assertEqual(2021, i)
        self.assertEqual("000", left)


        i, left = eat_literal_value("01010")
        self.assertEqual(10, i)
        self.assertEqual("", left)



    def test_eat_packet(self):
        bitstr = "1101000101001010010001001000000000" # 11 first bits -> literal value with int number 10
        packets, remainder = eat_a_packet(bitstr)
        self.assertEqual("01010010001001000000000", remainder)

    def test_eat_subpackets(self):
        bitstr = "00000000000110111101000101001010010001001000000000"
        packets, remainder = eat_subpackets(bitstr)
        
        self.assertEqual(2, len(packets))
        self.assertEqual("0000000", remainder)

    def test_part1(self):
        p, left = eat_a_packet(hex_to_bits("38006F45291200"))
        p.print_packet_tree("ss  ")
        self.assertEqual(p.num_sub_packets(), 2)

        print(" ==========================")
        p, left = eat_a_packet(hex_to_bits("8A004A801A8002F478"))
        p.print_packet_tree("ss  ")
        self.assertEqual(p.version_sum(), 16)
        self.assertEqual(p.num_sub_packets(), 1)

        p, left = eat_a_packet(hex_to_bits("620080001611562C8802118E34"))
        self.assertEqual(p.version_sum(), 12)
        self.assertEqual(p.num_sub_packets(), 2)


        p, left = eat_a_packet(hex_to_bits("C0015000016115A2E0802F182340"))
        self.assertEqual(p.version_sum(), 23)
        self.assertEqual(p.num_sub_packets(), 2)


        p, left = eat_a_packet(hex_to_bits("A0016C880162017C3686B18A3D4780"))
        self.assertEqual(p.version_sum(), 31)
        self.assertEqual(p.num_sub_packets(), 1)


        p, left = eat_a_packet(hex_to_bits("EE00D40C823060"))
        self.assertEqual(p.num_sub_packets(), 3)


        p, left = eat_a_packet(hex_to_bits(INPUT_STR[0]))
        self.assertEqual(p.version_sum(), 989)

        pass

    def test_part2(self):
        p, left = eat_a_packet(hex_to_bits("C200B40A82"))
        self.assertEqual(p.expr_value(), 3)
        
        p, left = eat_a_packet(hex_to_bits("04005AC33890"))
        self.assertEqual(p.expr_value(), 54)

        p, left = eat_a_packet(hex_to_bits("880086C3E88112"))
        self.assertEqual(p.expr_value(), 7)

        p, left = eat_a_packet(hex_to_bits("CE00C43D881120"))
        self.assertEqual(p.expr_value(), 9)

        p, left = eat_a_packet(hex_to_bits("D8005AC2A8F0"))
        self.assertEqual(p.expr_value(), 1)

        p, left = eat_a_packet(hex_to_bits("F600BC2D8F"))
        self.assertEqual(p.expr_value(), 0)
        p, left = eat_a_packet(hex_to_bits("9C005AC2F8F0"))
        self.assertEqual(p.expr_value(), 0)
        p, left = eat_a_packet(hex_to_bits("9C0141080250320F1802104A08"))
        self.assertEqual(p.expr_value(), 1)

        p, left = eat_a_packet(hex_to_bits(INPUT_STR[0]))
        self.assertEqual(p.expr_value(), 7936430475134)

if __name__ == '__main__':
    unittest.main()
