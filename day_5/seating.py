import re
import generic

class Solver(generic.AdventDay):
    """
    FBFBBFF = 44
    0101100 = 44

    RLR = 5
    101 = 5

    FBFBBFFRLR = 357
    0101100101 = 357
    """
    def preprocess_input(self):
        """
        Turn the seating assignment into an int
        """
        to_binary = {
            'F': '0',
            'B': '1',
            'R': '1',
            'L': '0',
        }
        self.seat_ints = []
        for line in self._raw_data:
            binary_string = re.sub('.', lambda c: to_binary[c[0]] , line)
            self.seat_ints.append(int(binary_string, 2))

    def solve_part_1(self):
        """
        Convert to binary and check for highest seating
        """
        print('--- PART 1 ---')
        print(max(self.seat_ints))

    def solve_part_2(self):
        """
        Find the missing seats
        """
        print('--- PART 2 ---')
        possible = set(range(0, int('1111111111', 2)))
        given = set(self.seat_ints)
        leftover = possible - given

        # find the one that's by itself
        for seat in sorted(list(leftover)):
            if not seat + 1 in leftover and not seat - 1 in leftover:
                print(seat)
                break
