from collections import defaultdict
import generic

class Solver(generic.AdventDay):
    def __init__(self, filename='input.txt'):
        """
        Init the class
        """
        super().__init__(filename=filename)

        self._groups = []


    def preprocess_input(self):
        """
        Parse lines into groups
        """
        cur_group = defaultdict(int)
        for line in self._raw_data:
            if not line:
                self._groups.append(dict(cur_group))
                cur_group = defaultdict(int)
                continue
            for letter in line:
                cur_group[letter] += 1
            cur_group['total'] += 1
        # forgot the last one!
        self._groups.append(dict(cur_group))

    def solve_part_1(self):
        """
        Count the number of letters
        """
        print('--- PART 1 ---')
        count = 0
        for group in self._groups:
            # skip the total key
            count += len(group.keys()) - 1
        print(count)

    def solve_part_2(self):
        """
        Count the number of letters that everyone in the group has
        """
        print('--- PART 2 ---')
        count = 0
        for group in self._groups:
            total = group.pop('total')
            count += len([v for v in group.values() if v == total])

        print(count)
