from copy import deepcopy
import re
import generic


class Solver(generic.AdventDay):
    def __init__(self, filename='input.txt', lookback_size=25):
        super().__init__(filename)
        self.lookback_size = lookback_size

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        line = super().process_line(line)
        return int(line)

    def preprocess_input(self):
        pass

    def find_invalid(self):
        for idx, number in enumerate(self._raw_data):
            if idx < self.lookback_size:
                continue
            # we enumerate starting from [1], so the first index is 0
            # which gets the previous group to the one that's being enumerated
            prev_group = self._raw_data[idx - self.lookback_size:idx]
            possible_set = set()
            for x, num_1 in enumerate(prev_group):
                for num_2 in prev_group[x:]:
                    possible_set.add(num_1 + num_2)
            if number not in possible_set:
                return number

    def solve_part_1(self):
        """
        Find the invalid number
        """
        print('--- PART 1 ---')
        print(self.find_invalid())

    def solve_part_2(self):
        """
        From the invalid number, find a set of indexes that add up to it
        """
        print('--- PART 2 ---')
        invalid_number = self.find_invalid()

        found = False
        for idx, start_number in enumerate(self._raw_data):
            cur_sum = start_number
            offset = 1
            while cur_sum < invalid_number and idx + offset < len(self._raw_data):
                cur_sum += self._raw_data[idx + offset]
                if cur_sum == invalid_number:
                    found = self._raw_data[idx:idx+offset]
                    break
                offset += 1
        if not found:
            raise Exception("WTF??")
        print(min(found) + max(found))
