import generic


class Solver(generic.AdventDay):
    def __init__(self, filename="input.txt"):
        super().__init__(filename)
        self.timestamp = 0
        self.busses = []
        self.tests = []

    def preprocess_input(self):
        """
        Turn the 2D chart into a dict of (x, y) -> character
        and make 2 graphs, one for immediate neighbors and one for visible neighbors
        """
        self.timestamp = int(self._raw_data[0])
        self.busses = list(
            map(
                int,
                filter(lambda p: p != "x", self._raw_data[1].split(",")),
            )
        )
        self.busses_2 = []
        for idx, num in enumerate(self._raw_data[1].split(",")):
            if num == "x":
                continue
            self.busses_2.append((int(num), idx))

        if len(self._raw_data) > 2:
            for test_line in self._raw_data[2:]:
                result, nums = test_line.split(":")
                result = int(result)
                cur_input = []
                for idx, num in enumerate(nums.split(",")):
                    if num == "x":
                        continue
                    cur_input.append((int(num), idx))
                self.tests.append((cur_input, result))

    def solve_part_1(self):
        """
        Find the next bus
        """
        print("--- PART 1 ---")

        min_wait = 1000000
        solution = 0
        for bus_num in self.busses:
            wait = bus_num - (self.timestamp % bus_num)
            if wait < min_wait:
                min_wait = wait
                solution = bus_num * min_wait
        print(solution)

    def find_timestamp(self, busses):
        first = busses[0]
        cur_ts = first[1]
        interval = first[0]
        for next_bus in busses[1:]:
            bus = next_bus[0]
            bus_diff = next_bus[1]
            for _ in range(bus):
                if (cur_ts + bus_diff) % bus == 0:
                    break
                cur_ts += interval
            interval *= bus
        return cur_ts

    def solve_part_2(self):
        """
        Find the timestamp that works
        """
        print("--- PART 2 ---")
        if self.tests:
            print(" === TEST CASES ===")
            for test_case in self.tests:
                test, expected = test_case
                print(f"TEST SET: {test}")
                solution = self.find_timestamp(test)
                print(f"CALCULATED: {solution}")
                print(f"EXPECTED  : {expected}")
            print(" === DONE TESTS ===")
        print(self.busses_2)
        solution = self.find_timestamp(self.busses_2)
        print(solution)
