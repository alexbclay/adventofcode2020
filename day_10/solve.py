from copy import deepcopy
import re
import generic


class Solver(generic.AdventDay):
    def __init__(self, filename='input.txt'):
        super().__init__(filename)
        self.sorted_lines = []
        self.series = []
        self.graph = {}

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        line = super().process_line(line)
        return int(line)

    def preprocess_input(self):
        self._raw_data.append(0)
        self.sorted_lines = sorted(self._raw_data)

        # create a graph of the possible connections
        series = []
        graph = {}
        cur_series = set()
        for idx, num in enumerate(self.sorted_lines[:-1]):
            diff = self.sorted_lines[idx+1] - num
            cur_series.add(num)
            if diff == 3:
                series.append(cur_series)
                cur_series = set()
            graph[num] = []
            for diff in range(1,4):
                if (num - diff) in graph:
                    graph[num - diff].append(num)
        cur_series.add(self.sorted_lines[-1])
        if cur_series:
            series.append(cur_series)
        self.series = series
        self.graph = graph

    def find_all_paths(self, start, end, indent=0):
        """
        Find all paths from the node with label 'start' to node with label 'end'
        I used this to generate the lookup table below
        """
        paths = []
        if start == end:
            return [[start]]
        for child in self.graph[start]:
            if child > end:
                continue
            for path in self.find_all_paths(child, end, indent + 1):
                paths.append([start] + path)
        return paths

    def solve_part_1(self):
        """
        Find the joltage differences
        """
        print('--- PART 1 ---')
        ones = 0
        threes = 0
        for idx, num in enumerate(self.sorted_lines[:-1]):
            diff = self.sorted_lines[idx+1] - num
            if diff == 1:
                ones += 1
            elif diff == 3:
                threes += 1
        threes += 1
        print(f'ones: {ones}')
        print(f'threes: {threes}')
        print(f'SOLUTION: {ones * threes}')

    def solve_part_2(self):
        """
        Find all possible ways to get to our end goal
        """
        print('--- PART 2 ---')
        # each node can only connect to ones less than 3 value from them
        # so when the nodes are 3 apart, there's only one valid connection
        # we can treat series of contiguous numbers as their own sub-graph
        # and multiply the number of paths through their sub-graph into the whole
        lookup = {
            1: 1,
            2: 1,
            3: 2,
            4: 4,
            5: 7,
        }
        total = 1
        for ser in self.series:
            total *= lookup[len(ser)]
        print(total)
