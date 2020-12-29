from collections import defaultdict, namedtuple
import generic


class Coord3d:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __repr__(self):
        return f"c3d({self.x}, {self.y}, {self.z})"

    def __add__(self, other):
        return Coord3d(self.x + other.x, self.y + other.y, self.z + other.z)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __hash__(self):
        return (self.x, self.y, self.z).__hash__()

    def get_neighbors(self):
        neighbors = []
        for row in range(-1, 2):
            for col in range(-1, 2):
                for z in range(-1, 2):
                    if row == 0 and col == 0 and z == 0:
                        continue
                    neighbors.append(self + Coord3d(col, row, z))
        return neighbors


class Coord4d:
    def __init__(self, x, y, z, w):
        self.x = x
        self.y = y
        self.z = z
        self.w = w

    def __repr__(self):
        return f"c4d({self.x}, {self.y}, {self.z}, {self.w})"

    def __add__(self, other):
        return Coord4d(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z and self.w == other.w

    def __hash__(self):
        return (self.x, self.y, self.z, self.w).__hash__()

    def get_neighbors(self):
        neighbors = []
        for row in range(-1, 2):
            for col in range(-1, 2):
                for z in range(-1, 2):
                    for w in range(-1, 2):
                        if row == 0 and col == 0 and z == 0 and w == 0:
                            continue
                        neighbors.append(self + Coord4d(col, row, z, w))
        return neighbors


class Solver(generic.AdventDay):
    def __init__(self, filename="input.txt"):
        super().__init__(filename)

        self._on_cubes = set()
        self._on_cubes_4d = set()

    def preprocess_input(self):
        for row, line in enumerate(self._raw_data):
            for col, val in enumerate(line):
                if val == "#":
                    self._on_cubes.add(Coord3d(col, row, 0))
                    self._on_cubes_4d.add(Coord4d(col, row, 0, 0))

    def do_step(self, cubeset):
        maybe_ons = defaultdict(int)
        offs = set()
        for coord in cubeset:
            active_neighbors = 0
            for neighbor in coord.get_neighbors():
                if neighbor in cubeset:
                    active_neighbors += 1
                else:
                    maybe_ons[neighbor] += 1
            if active_neighbors not in (2, 3):
                offs.add(coord)
        ons = [coord for coord, size in maybe_ons.items() if size == 3]
        cubeset -= offs
        cubeset |= set(ons)

    def solve_part_1(self):
        print("--- PART 1 ---")

        for _ in range(6):
            self.do_step(self._on_cubes)

        print(len(self._on_cubes))

    def solve_part_2(self):
        print("--- PART 2 ---")

        for _ in range(6):
            self.do_step(self._on_cubes_4d)
        print(len(self._on_cubes_4d))
