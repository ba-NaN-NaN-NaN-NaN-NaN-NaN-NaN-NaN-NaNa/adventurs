import numpy as np
import unittest
import copy

cos90 = 0
sin90 = 1


rot90x = np.array([
    [1,      0,      0],
    [0,  cos90, -sin90],
    [0,  sin90,  cos90],
])

rot90y = np.array([
    [  cos90, 0, sin90],
    [      0, 1,     0],
    [ -sin90, 0, cos90],
])

rot90z = np.array([
    [ cos90, -sin90, 0],
    [ sin90,  cos90, 0],
    [     0,      0, 1],
])




def permute_rows(identity):
    toreturn = [
        identity[:, [0,1,2]],
        identity[:, [0,2,1]],

        identity[:, [1,0,2]],
        identity[:, [1,2,0]],

        identity[:, [2,1,0]],
        identity[:, [2,0,1]],
    ]
    #print("permute_rows gave '%s'" % toreturn)
    return toreturn
    #return [identity]

def permute_signs(matrix):
    toreturn = []   
    for flip_0 in [True, False]:
        for flip_1 in [True, False]:
            for flip_2 in [True, False]:
                m = copy.deepcopy(matrix)
                if flip_0:
                    m[0] = -m[0]
                if flip_1:
                    m[1] = -m[1]
                if flip_2:
                    m[2] = -m[2]

                toreturn.append(m)

    return toreturn

def generate_transforms():
    #x_transforms = x_forward, x_reverse, y_forward, y_reverse, z_forward, z_reverse
    
    identity = np.identity(3)
    permuted_rows = permute_rows(identity)
    
    #identity = identity[:, [0,2,1]]
    #print(permuted_rows)

    toreturn = []
    for rperm in permuted_rows:
        permuted_signs = permute_signs(rperm)
        #print("Got %d new permuted signs" % len(permuted_signs))
        """
        for m in permuted_signs:
            if m not in toreturn:
                toreturn.append(m)
        """
        toreturn += permuted_signs

    #return sorted(list(set(toreturn)))
    return toreturn

def apply_matrix_times(m, to_apply, times):
    toreturn = m
    for _ in range(times):
        transformed = np.matmul(toreturn, to_apply)
        #print("Transformed matrix from %s to %s" % (toreturn, transformed))
        toreturn = transformed
    return toreturn


def uniq_matrixes(to_uniq):
    toreturn = []
    candidates = to_uniq[:]

    while len(candidates) > 0:
        can = candidates.pop()
        found = False
        for exists in toreturn:
            if found is False:
                matching_cells = can == exists
                all_match = np.all(matching_cells)
                if all_match:
                    found = True

        #found =False
        if not found:
            toreturn.append(can)

    return toreturn


def generate_transforms_rot90():
    # Second attempt, calc matrixes myself.
    identity = np.identity(3)
    toreturn = []
    for xrot in range(5):
        for yrot in range(5):
            for zrot in range(5):
                m = np.identity(3)
                m = apply_matrix_times(m, rot90x, xrot)
                m = apply_matrix_times(m, rot90y, yrot)
                m = apply_matrix_times(m, rot90z, zrot)
                toreturn.append(m)

    candidates = toreturn
    toreturn = uniq_matrixes(candidates)



    print("Generated %d transforms" % len(toreturn))
    #print(toreturn)
    return toreturn

"""

def generate_transforms_rot90():
    # This one is incorrect.
    # rot90 will *NOT* generate rotation matrixes.
    # It will only rotate a matrix, which happens to be an identity matrix.
    raise TypeError("BAD APPROACH glkdfhs")
    identity = np.identity(3)
    toreturn = []
    for xrot in range(5):
        for yrot in range(5):
            for zrot in range(5):
                m = np.identity(3)
                m = np.rot90(m, xrot, axes=(1,2))
                m = np.rot90(m, yrot, axes=(0,2))
                m = np.rot90(m, zrot, axes=(0,1))

                toreturn.append(m)

    candidates = toreturn
    toreturn = []
    while len(candidates) > 0:
        can = candidates.pop()
        found = False
        for exists in toreturn:
            if found is False:
                matching_cells = can == exists
                all_match = np.all(matching_cells)
                if all_match:
                    found = True

        
        if not found:
            toreturn.append(can)


    print("Generated %d transforms" % len(toreturn))
    #print(toreturn)
    return toreturn
"""

POSSIBLE_ROTATIONS = generate_transforms_rot90()




class TestEntryPoint(unittest.TestCase):
    def test_np_transforms(self):
        self.assertEqual(24, len(POSSIBLE_ROTATIONS))
        # Accept that we do mirrored transforms well for now.
        #self.assertEqual(48, len(POSSIBLE_TRANSFORMS))

def perm_main():
    unittest.main()

if __name__ == '__main__':
    perm_main()

