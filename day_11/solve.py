from collections import defaultdict
import generic
from pprint import pprint as pp

class Solver(generic.AdventDay):
    def __init__(self, filename='input.txt'):
        super().__init__(filename)
        self.seats = []
        self.seat_dict = {}
        self.nearest_neighbors = defaultdict(list)
        self.visible_neighbors = defaultdict(list)

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        line = super().process_line(line)
        return list(line)

    def preprocess_input(self):
        """
        Turn the 2D chart into a dict of (x, y) -> character
        and make 2 graphs, one for immediate neighbors and one for visible neighbors
        """
        # convert to address -> value dict
        max_x = max_y = 0
        for x, row in enumerate(self._raw_data):
            max_x = x if x > max_x else max_x
            for y, seat in enumerate(row):
                max_y = y if y > max_y else max_y
                if seat == '.':
                    # don't care about non-seat coords
                    continue
                self.seat_dict[(x, y)] = seat

        # All the directions in a 1d list
        directions = []
        for d_x in range(-1, 2):
            for d_y in range(-1, 2):
                if d_x == 0 and d_y == 0:
                    continue
                directions.append((d_x, d_y))

        # create neighbor graphs
        for coord, seat in self.seat_dict.items():
            for delta in directions:
                # nearest
                test_coord = (coord[0] + delta[0], coord[1] + delta[1])
                if test_coord in self.seat_dict:
                    self.nearest_neighbors[coord].append(test_coord)
                    self.visible_neighbors[coord].append(test_coord)
                    continue

                # nothing immediately close, so check for line of sight
                while (0 <= test_coord[0] <= max_x) and (0 <= test_coord[1] <= max_y):
                    test_coord = (test_coord[0] + delta[0], test_coord[1] + delta[1])
                    if test_coord in self.seat_dict:
                        self.visible_neighbors[coord].append(test_coord)
                        break

    @staticmethod
    def update_seat_chart(seat_chart, neighbors, max_visible=4):
        next_chart = {}
        for coord, seat in seat_chart.items():
            full = 0
            for neighbor in neighbors[coord]:
                if seat_chart[neighbor] == '#':
                    full += 1
            if seat == 'L' and full == 0:
                next_chart[coord] = '#'
            elif seat == '#' and full >= max_visible:
                next_chart[coord] = 'L'
            else:
                next_chart[coord] = seat
        return next_chart

    def solve_part_1(self):
        """
        Find the joltage differences
        """
        print('--- PART 1 ---')

        cur_chart = self.seat_dict
        cur_str = ''.join([x for x in cur_chart.values()])
        for rnd in range(1000):
            print(f'--- {rnd}')
            cur_chart = self.update_seat_chart(cur_chart, self.nearest_neighbors)
            next_str = ''.join([x for x in cur_chart.values()])
            if next_str == cur_str:
                print(f'FOUND! {rnd}')
                break
            cur_str = next_str
        print(len([c for c in next_str if c == '#'] ))


    @staticmethod
    def print_grid(chart, size_x=10, size_y=10):
        lines = []
        for x in range(size_x):
            cur_line = ''
            for y in range(size_y):
                cur_line += chart.get((x, y), '.')
            lines.append(cur_line)
        pp(lines)

    def solve_part_2(self):
        """
        Find all possible ways to get to our end goal
        """
        cur_chart = self.seat_dict
        cur_str = ''.join([x for x in cur_chart.values()])
        for rnd in range(1000):
            print(f'--- {rnd}')
            self.print_grid(cur_chart, size_x=92, size_y=91)
            cur_chart = self.update_seat_chart(cur_chart, self.visible_neighbors, max_visible=5)
            next_str = ''.join([x for x in cur_chart.values()])
            if next_str == cur_str:
                print(f'FOUND! {rnd}')
                break
            cur_str = next_str
        print(len([c for c in next_str if c == '#'] ))
