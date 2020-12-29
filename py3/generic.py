from pprint import pprint as pp


class AdventDay:
    def __init__(self, filename='input.txt'):
        """
        Create the day's solver
        """
        self._raw_data = self.load_file(filename)

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        return line.strip()

    def load_file(self, filename):
        """
        Load all the lines from the input file
        """
        input_data = []
        with open(filename, 'r') as inp:
            for line in inp:
                input_data.append(self.process_line(line.strip()))

        return input_data

    def preprocess_input(self):
        """
        Change the raw data into something better, if needed
        """
        print('--- RAW DATA ---')
        pp(self._raw_data)

    def solve_part_1(self):
        """
        Solve the day's first problem!
        """
        print('Part 1')

    def solve_part_2(self):
        """
        Solve the day's second problem
        """
        print('Part 2')

    def run(self):
        """
        Solve the thing!
        """
        self.preprocess_input()
        self.solve_part_1()
        self.solve_part_2()
