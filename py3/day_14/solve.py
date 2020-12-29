import re
import generic


class Mask:
    def __init__(self, mask):
        self._source_mask = mask
        self.single_mask = self.split_mask(mask)

        # The first part of part_2 requires an | mask (except for X characters)
        part_2_str = "".join(["0" if c == "X" else c for c in mask])
        self.mask_part_2 = int(part_2_str, 2)

        self.all_masks = ["".join(m) for m in self.get_all_masks(self._source_mask)]
        self.all_masks_int = [self.split_mask(m) for m in self.all_masks]

    def split_mask(self, mask):
        """
        Turn the mask into 2 masks:
        [0]: replaces input with 0 if mask is 0, otherwise leaves alone (&)
        [1]: replaces input with 1 if mask is 1, otherwise leaves alone (|)
        """
        return (
            int("".join(["0" if c == "0" else "1" for c in mask]), 2),
            int("".join(["1" if c == "1" else "0" for c in mask]), 2),
        )

    def get_all_masks(self, string):
        """
        Recursively generate all possible masks where X can be replaced by either 1 or 0
        0,1 are replaced with X, so that we can generate the appropriate masks for part 2
        """
        if len(string) == 0:
            return [[]]
        char = string[0]
        options = ["X"]
        if char == "X":
            options = ["0", "1"]
        res = []
        for opt in options:
            for mask in self.get_all_masks(string[1:]):
                res.append([opt] + mask)
        return res

    def apply_mask(self, mask, value):
        return (value & mask[0]) | mask[1]

    def get_addresses(self, value):
        base_value = value | self.mask_part_2
        return [self.apply_mask(mask, base_value) for mask in self.all_masks_int]


class Solver(generic.AdventDay):
    MASK_RE = re.compile(r"mask = ([X10]*)")
    MEM_RE = re.compile(r"mem\[(\d*)\] = (\d*)")

    @classmethod
    def process_line(cls, line):
        """
        Process the given raw input line
        """
        line = super().process_line(line)
        mask_match = cls.MASK_RE.match(line)
        if mask_match:
            mask = mask_match.group(1)

            return ("mask", Mask(mask))

        mem_match = cls.MEM_RE.match(line)
        if mem_match:
            return (int(mem_match.group(1)), int(mem_match.group(2)))

    def solve_part_1(self):
        """
        Move the ship's coordinates as given by the actions
        """
        print("--- PART 1 ---")
        mem = dict()
        cur_mask = self._raw_data[0][1]
        for line in self._raw_data[1:]:
            if line[0] == "mask":
                cur_mask = line[1]
                continue
            address = line[0]
            mem[address] = cur_mask.apply_mask(cur_mask.single_mask, line[1])
        print(sum(mem.values()))

    def solve_part_2(self):
        """
        Move the ship or its waypoint
        """
        print("--- PART 2 ---")
        mem = {}
        cur_mask = self._raw_data[0][1]
        for line in self._raw_data[1:]:
            if line[0] == "mask":
                cur_mask = line[1]
                continue
            addresses = cur_mask.get_addresses(int(line[0]))
            for addr in addresses:
                mem[addr] = line[1]
        print(sum(mem.values()))
