import generic
import pandas as pd

class Solver(generic.AdventDay):

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        return list(line)

    def preprocess_input(self):
        """
        Convert to dataframe
        """
        self._data = pd.DataFrame(self._raw_data)
        print(self._data)

    def get_trees(self, slope_x, slope_y):
        """
        Get the number of trees you would hit at the given slope
        """
        trees = 0
        cur_x = 0
        cur_y = 0
        while cur_y < len(self._data[0]) - 1:
            cur_x = (cur_x + slope_x) % 31
            cur_y += slope_y
            location = self._data[cur_x][cur_y]
            if location == '#':
                trees += 1
        return trees

    def solve_part_1(self):
        """
        Sled in slope (r3, d1)
        """
        print('--- PART 1 ---')
        print(self.get_trees(3, 1))

    def solve_part_2(self):
        """
        Check a bunch of slopes
        """
        print('--- PART 2 ---')
        slopes = [
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2)
        ]

        trees = [self.get_trees(x,y) for x,y in slopes]
        total = 1
        for x,y in slopes:
            total *= self.get_trees(x,y)
        print(trees, total)
