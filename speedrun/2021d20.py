import unittest

f = open("../input/2021_d20.txt")
INPUT_STR = f.readlines()
f.close()

INPUT_STR = [r.strip() for r in INPUT_STR if len(r.strip()) > 0]

f = open("../input/2021_d20_sample.txt")
SAMPLE_STR = f.readlines()
f.close()

SAMPLE_STR = [r.strip() for r in SAMPLE_STR if len(r.strip()) > 0]


class EnhImage:
    def __init__(self, lines, fill_pixel):
        """
        Out of bound pixel = The pixel gotten by
        accessing image waaay outside of previous area.
        """
        self._algo_raw = lines[0].strip()
        if len(self._algo_raw) != 512:
            raise TypeError("jfldhfd %d" % len(self._algo_raw))

        img_lines = lines[1:]
        self._image = img_lines
        
        self._width = len(self._image[0])
        self._height = len(self._image)
        self._pixel_values = {} # Contains a row. 

        # Render boundaries.
        self._x_min = -2
        self._x_max = 2
        self._y_min = -2
        self._y_max = 2

        if self._width not in [1, 5, 100]:
            print("New image with width=%d, height=%d" % (self._width, self._height))
            return 1/0
        if self._height not in [1, 5, 100]:
            print("New image with width=%d, height=%d" % (self._width, self._height))
            return 1/0

        for x in range(self._width):
            for y in range(self._height):
                new_value = self._image[y][x:x+1]
                self.set_pixel_at(x, y, new_value)

        self._fill_pixel = fill_pixel

    def get_pixel_at(self, x, y):
        """
        Get a pixel at a given position.
        """
        #if self._lit_pixels.has_key(y):
        if y in self._pixel_values:
            row = self._pixel_values[y]
            if x in row:
                #if row.has_key(x):
                return row[x]

        return self._fill_pixel


    def pixel_is_lit(self, x, y):
        """
        Return true if pixel is '#'
        """
        return self.get_pixel_at(x, y) == "#"

    def set_pixel_at(self, x, y, new_value):
        if new_value not in [".", "#"]:
            raise TypeError("set_pixel_at(%d, %d, '%s')" % (x, y, new_value))


        # Adjust render boundaries.
        if x < self._x_min:
            self._x_min = x

        if y < self._y_min:
            self._y_min = y

        if x > self._x_max:
            self._x_max = x

        if y > self._y_max:
            self._y_max = y


        if y not in self._pixel_values:
            #if not self._pixel_values.has_key(y):
            self._pixel_values[y] = dict()
        row = self._pixel_values[y]
        row[x] = new_value

    def render(self, x_min, x_max, y_min, y_max):
        width = x_max - x_min
        toreturn = []
        for y in range(y_min, y_max+1):
            pixels = [self.get_pixel_at(x, y) for x in range(x_min, x_max+1)]
            #print(pixels)
            rendered_row = "".join(pixels)
            if len(rendered_row) != width+1:
                raise TypeError("ofdh?? '%s' %d != %d" % (rendered_row,len(rendered_row),width))

            toreturn.append(rendered_row)
            #print("Got line '%s' for (%d, %d,)" % (rendered_row,x_min, x_max+1))
        
        return toreturn
            

    def print(self):
        """
        """
        lines = self.render(self._x_min, self._x_max, self._y_min, self._y_max)
        print("\n".join(lines))

    def count_lit_pixels(self):
        count = 0
        for row in self._pixel_values.values():
            for cell in row.values():
                if cell == "#":
                    count += 1
                elif cell == ".":
                    pass
                else:
                    raise TypeError("fjdgfkjdgfkhjg  fdsd")
        return count

    def enhance(self):
        """
        Enhance an image into a new image.
        """

        fill_pixel = self.get_next_value(-6948756, 9684576498)

        toreturn = EnhImage([self._algo_raw, "."], fill_pixel=fill_pixel)
        for x in range(self._x_min-2, self._x_max+2):
            for y in range(self._y_min-2, self._y_max+2):
                new_value = self.get_next_value(x, y)
                toreturn.set_pixel_at(x, y, new_value)

        return toreturn

    def get_kernel_str(self, x, y):
        """
        Return 9 pixels around x,y
        """
        #print("Getting kernel at (%d, %d)" % (x,y))
        bits = []
        for dy in [-1, 0, 1]:
            for dx in [-1, 0, 1]:
                pixel = self.get_pixel_at(x+dx, y+dy)
                #print("For kernel, pixel at (%d,%d) -> '%s'" % (x+dx, y+dy, pixel))
                bits.append(pixel)
        return "".join(bits)
    
    def get_next_value(self, x, y):
        kernel_str = self.get_kernel_str(x, y)
        kernel_str = kernel_str.replace("#", "1")
        kernel_str = kernel_str.replace(".", "0")
        next_value_offset = int(kernel_str, 2)
        return self._algo_raw[next_value_offset:next_value_offset+1]



class TestEntryPoint(unittest.TestCase):

    def test_can_set_map(self):
        ms = EnhImage(SAMPLE_STR, fill_pixel=".")
        mi = EnhImage(INPUT_STR, fill_pixel=".")
        self.assertFalse(ms.pixel_is_lit(10, 10))
        self.assertFalse(ms.set_pixel_at(10, 10, "#"))
        self.assertTrue(ms.pixel_is_lit(10, 10))
        #ms.print()

        for x in range(11, 20):
            for y in range(11, 20):
                pix_at = ms.get_pixel_at(x, y)
                self.assertEqual(".", pix_at)
                pix_at = ms.get_pixel_at(x, -y)
                self.assertEqual(".", pix_at)
                pix_at = ms.get_pixel_at(-x, y)
                self.assertEqual(".", pix_at)
                pix_at = ms.get_pixel_at(-x, -y)
                self.assertEqual(".", pix_at)

            
        self.assertEqual("#", ms.get_pixel_at(0, 0))
        self.assertEqual("#", ms.get_pixel_at(0, 1))
        self.assertEqual(".", ms.get_pixel_at(1, 0))

        self.assertEqual("#", ms.get_pixel_at(3, 0))
        self.assertEqual(".", ms.get_pixel_at(3, 1))


        self.assertEqual(".", ms.get_pixel_at(1, 1))
        self.assertEqual(".", ms.get_pixel_at(2, 2))
        self.assertEqual(".", ms.get_pixel_at(3, 3))
        self.assertEqual("#", ms.get_pixel_at(4, 4))
        self.assertEqual(".", ms.get_pixel_at(5, 5))

        # Kernel around centre of sample image.
        self.assertEqual(".", ms.get_pixel_at(1, 1))
        self.assertEqual(".", ms.get_pixel_at(2, 1))
        self.assertEqual(".", ms.get_pixel_at(3, 1))

        self.assertEqual("#", ms.get_pixel_at(1, 2))
        self.assertEqual(".", ms.get_pixel_at(2, 2))
        self.assertEqual(".", ms.get_pixel_at(3, 2))

        self.assertEqual(".", ms.get_pixel_at(1, 3))
        self.assertEqual("#", ms.get_pixel_at(2, 3))
        self.assertEqual(".", ms.get_pixel_at(3, 3))

        #self.assertEqual(5,6)

    def test_kernel_value(self):
        # Sample str is 5x5 image.
        ms = EnhImage(SAMPLE_STR, fill_pixel=".")
        ms.print()
        self.assertEqual(".........", ms.get_kernel_str(2, 200))
        self.assertEqual("...#...#.", ms.get_kernel_str(2, 2))

    def test_pixel_count(self):
        ms = EnhImage(SAMPLE_STR, fill_pixel=".")
        self.assertEqual(10, ms.count_lit_pixels())

        ms = ms.enhance()
        ms.print()
        self.assertEqual(24, ms.count_lit_pixels())

        ms = ms.enhance()
        self.assertEqual(35, ms.count_lit_pixels())

        ms.print()

        # REAL IMAGE
        mi = EnhImage(INPUT_STR, fill_pixel='.')
        mi = mi.enhance()
        #mi.print()
        mi = mi.enhance()
        #mi.print()
        # 5390 too high.
        self.assertLess(mi.count_lit_pixels(), 5390)
        self.assertEqual(4964, mi.count_lit_pixels())

        # 4964 correct.

    def test_part2(self):
        ms = EnhImage(SAMPLE_STR, fill_pixel=".")
        mi = EnhImage(INPUT_STR, fill_pixel='.')

        for n in range(50):
            ms = ms.enhance()
            mi = mi.enhance()

        self.assertEqual(3351, ms.count_lit_pixels())
        self.assertEqual(49649768457896435, mi.count_lit_pixels())




if __name__ == '__main__':
    unittest.main()
    