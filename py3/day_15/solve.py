import re
import generic


class Solver(generic.AdventDay):
    def __init__(self, filename="input.txt"):
        super().__init__(filename)
        self.reset()

    def reset(self):
        self.turn = 1
        self.prev_number = False
        self.seen = {}

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        return [int(n) for n in line.split(",")]

    def preprocess_input(self):
        pass

    def prime(self, start_sequence):
        for idx, num in enumerate(start_sequence):
            self.seen[num] = idx
            self.prev_number = num
        self.turn = idx + 1

    def next_number(self):
        # print(f"{self.turn} {self.prev_number} {self.seen}")
        if self.prev_number not in self.seen:
            next_num = 0
        else:
            # diff between when last seen
            next_num = self.turn - 1 - self.seen[self.prev_number]
        self.seen[self.prev_number] = self.turn - 1
        self.prev_number = next_num
        self.turn += 1
        return next_num

    def solve(self, sequence, number):
        self.reset()
        print(sequence)
        self.prime(sequence)
        for _ in range(number - len(sequence)):
            if self.turn % 500000 == 0:
                print(f"{self.turn} {len(self.seen)}")
            num = self.next_number()

        print(f"{self.turn}: {num}")

    def solve_part_1(self):
        print("--- PART 1 ---")
        for line in self._raw_data:
            self.solve(line, 2020)

    def solve_part_2(self):
        print("--- PART 2 ---")
        for line in self._raw_data:
            self.solve(line, 30000000)
