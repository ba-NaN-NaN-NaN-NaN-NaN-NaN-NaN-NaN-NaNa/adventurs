import unittest
import copy

f = open("../input/2021_d12.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d12_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]



# TYPEDEF PATH -> List[Cave]
def format_path(cavelist):
    cavenames = [c._name for c in cavelist]
    return ",".join(cavenames)

class Cave:
    def __init__(self, name):
        self._direct_edges = []
        self._name = name
        self._is_small_cave = name == name.lower()

    def add_direct_edge(self, dst):
        if dst == 'start':
            return
        edges = sorted(self._direct_edges + [dst])
        self._direct_edges = edges

    def get_paths_to(self, cave_dict, banlist, dst):
        """
        banlist = do not visit. Add self IFF i am a small cave.

        Returns: List[List[Cave]]

        Each entry in return list:
         - Includes self at start
         - Includes dst at the end.
         - Has at most 1 small cave.
        """
        if self == dst:
            return [[self]]

        if self._name in banlist and banlist[self._name] == 2:
            return []

        banlist_copy = copy.deepcopy(banlist)
        if self._is_small_cave:
            if self._name in banlist_copy:
                banlist_copy[self._name] += 1
            else:
                banlist_copy[self._name] = 1

        # Do we have two caves with two visits?
        visited_twice = []
        for k in banlist_copy:
            if banlist_copy[k] == 2:
                visited_twice.append(k)

        #print("visited_twice is %s" % visited_twice)
        if len(visited_twice) > 1:
            return []

        toreturn = []
        direct_caves = [cave_dict[name] for name in self._direct_edges]
        #print("Searching direct_caves from '%s' -> %s" % (self._name, direct_caves))
        for edge_name in self._direct_edges:
            #if edge_name in banlist:
            #    continue
            #print("Searching %s->%s?" % (self._name, dst))
            next_cave = cave_dict[edge_name]
            paths_beyond = next_cave.get_paths_to(cave_dict, banlist_copy, dst)
            #print("Going from '%s' to '%s' will then allow us %d further paths" % (self._name, edge_name, len(paths_beyond)))
            for p in paths_beyond:
                found = [self] + p
                #print("Found path: %s" % format_path(found))
                toreturn.append(found)

        #print(" == %s.get_paths_to(%s) == " % (self._name, dst))
        for path in toreturn:
            #print(path)
            pass
        return toreturn

    def __str__(self):
        return "%s" % self._name

    def __repr__(self):
        return self.__str__()

class Paths:
    def __init__(self, lines):
        # Dict. If from->to exists in this dict, we have a path.
        caves = {}
        for line in lines:
            l = line.split("-")
            if l[0] not in caves:
                caves[l[0]] = Cave(l[0])

            if l[1] not in caves:
                caves[l[1]] = Cave(l[1])

            print("Split line into '%s'" % l)
            caves[l[0]].add_direct_edge(l[1])
            if l[0] != 'start':
                caves[l[1]].add_direct_edge(l[0])

        self._caves = caves
        print(self._caves)

    def get_paths(self, src_name, dst_name):
        src_name = str(src_name)
        dst_name = str(dst_name)
        start_cave = self._caves[src_name]
        end_cave = self._caves[dst_name]
        return start_cave.get_paths_to(self._caves, {}, end_cave)





class TestEntryPoint(unittest.TestCase):


    def test_part1(self):

        small_ex = """start-A
start-b
A-c
A-b
b-d
A-end
b-end"""
        SAMPLE_SMALL = [r.strip() for r in small_ex.split("\n") if len(r.strip()) > 0]

        #p = Paths(SAMPLE_SMALL)
        p = Paths(SAMPLE_STR) #Slightly larger example
        p = Paths(INPUT_STR)
        paths = p.get_paths('start', 'end')
        
        #print(paths)
        for path in paths:
            #print("One path: %s" % path)# -> ".join(path.__repr__()))
            print("One path: %s" % ",".join([str(p) for p in path]))
        pass
        print("part1: %d" % len(paths))

    def test_part2(self):
        pass

if __name__ == '__main__':
    unittest.main()
