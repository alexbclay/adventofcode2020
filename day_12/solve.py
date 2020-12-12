from collections import defaultdict, namedtuple
import generic
from pprint import pprint as pp

Coordinate = namedtuple('Coordinate', ['x', 'y'])

class Solver(generic.AdventDay):
    def __init__(self, filename='input.txt'):
        super().__init__(filename)

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        line = super().process_line(line)
        return (line[0], int(line[1:]))

    def preprocess_input(self):
        """
        Turn the 2D chart into a dict of (x, y) -> character
        and make 2 graphs, one for immediate neighbors and one for visible neighbors
        """
        pass


    def solve_part_1(self):
        """
        Move the ship's coordinates as given by the actions
        """
        print('--- PART 2 ---')

        directions = [
            Coordinate(1, 0), # East
            Coordinate(0, -1), # South
            Coordinate(-1, 0), # West
            Coordinate(0, 1) # North
        ]
        # start facing east
        facing = 0

        commands = {
            'N': lambda coord, dist: Coordinate(coord.x, coord.y + dist),
            'S': lambda coord, dist: Coordinate(coord.x, coord.y - dist),
            'E': lambda coord, dist: Coordinate(coord.x + dist, coord.y),
            'W': lambda coord, dist: Coordinate(coord.x - dist, coord.y),
            'F': lambda coord, dist: Coordinate(coord.x + directions[facing].x * dist,
                                                coord.y + directions[facing].y * dist),
        }

        cur_coords = Coordinate(0, 0)
        path = [cur_coords]
        for action, value in self._raw_data:
            print(f'{action} -> {value}')
            if action in ('L', 'R'):
                sign = 1 if action == 'R' else -1
                value = int(value / 90)
                facing = (facing + sign * value) % 4
            else:
                cur_coords = commands[action](cur_coords, value)
            print(f'  FACING: {facing}  CUR: {cur_coords}')
            path.append(cur_coords)
        print(abs(cur_coords.x) + abs(cur_coords.y))

    def solve_part_2(self):
        """
        Move the ship or its waypoint
        """
        print('--- PART 2 ---')
        waypoint = Coordinate(10, 1)
        commands = {
            'N': lambda coord, dist: Coordinate(coord.x, coord.y + dist),
            'S': lambda coord, dist: Coordinate(coord.x, coord.y - dist),
            'E': lambda coord, dist: Coordinate(coord.x + dist, coord.y),
            'W': lambda coord, dist: Coordinate(coord.x - dist, coord.y),
            'F': lambda coord, dist: Coordinate(coord.x + waypoint.x * dist,
                                                coord.y + waypoint.y * dist),
        }
        rotations = {
            'R': lambda coord: Coordinate(coord.y, coord.x * -1),
            'L': lambda coord: Coordinate(coord.y * -1, coord.x)

        }

        cur_coords = Coordinate(0, 0)
        path = [cur_coords]
        for action, value in self._raw_data:
            print(f'{action} -> {value}')
            if action in ('L', 'R'):
                value = int(value / 90)
                for _ in range(value):
                    waypoint = rotations[action](waypoint)
            elif action == 'F':
                cur_coords = commands[action](cur_coords, value)
            else:
                # just moving the waypoint
                waypoint = commands[action](waypoint, value)
            print(f'  SHIP: {cur_coords} WAYPOINT: {waypoint}')
            path.append(cur_coords)
        print(abs(cur_coords.x) + abs(cur_coords.y))
