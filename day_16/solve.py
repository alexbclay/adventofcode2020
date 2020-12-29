import re
import generic


class Solver(generic.AdventDay):
    RULE_REGEX = re.compile(r"([a-z ]*): (\d*)-(\d*) or (\d*)-(\d*)")

    def __init__(self, filename="input.txt"):
        super().__init__(filename)

        self._rules = {}

        # make a set of all the valid numbers and check each nearby ticket to see if all its numbers
        # *could* be valid
        self._total_valid_set = set()

        self._my_ticket = []
        self._nearby_tickets = []

        self._current_input = "rules"
        self._wrong_count = 0

    def preprocess_input(self):
        for line in self._raw_data:
            # ignore blank lines, and switch the input mode based on the labels in the input
            if not line:
                continue
            if line == "your ticket:":
                self._current_input = "mine"
                continue
            if line == "nearby tickets:":
                self._current_input = "nearby"
                continue

            if self._current_input == "rules":
                match = self.RULE_REGEX.match(line)
                cur_rule_set = set(range(int(match.group(2)), int(match.group(3)) + 1)) | set(
                    range(int(match.group(4)), int(match.group(5)) + 1)
                )

                self._rules[match.group(1)] = cur_rule_set
                self._total_valid_set |= cur_rule_set
            elif self._current_input == "mine":
                self._my_ticket = [int(t) for t in line.split(",")]
                continue
            elif self._current_input == "nearby":
                cur_ticket = [int(t) for t in line.split(",")]
                wrong_numbers = set(cur_ticket) - self._total_valid_set
                if wrong_numbers:
                    self._wrong_count += sum(wrong_numbers)
                else:
                    self._nearby_tickets.append(cur_ticket)

    def solve_part_1(self):
        print("--- PART 1 ---")
        print(self._wrong_count)

    def solve_part_2(self):
        print("--- PART 2 ---")

        # keep track of which fields could be which rules
        possible_fields = [set(self._rules.keys()) for _ in range(len(self._my_ticket))]

        def remove_from_possibles(rule_list):
            print(f"Removing {rule_list}")
            if not rule_list:
                return
            to_update = []
            for rule_set in rule_list:
                for fields in possible_fields:
                    if len(fields) == 1:
                        continue
                    fields -= rule_set
                    if len(fields) == 1:
                        to_update.append(fields)
            remove_from_possibles(to_update)

        for ticket in self._nearby_tickets + [self._my_ticket]:
            # check each position against rules to see if it fits
            print(f"--- {ticket} ---")
            for idx, val in enumerate(ticket):
                if len(possible_fields[idx]) == 1:
                    # already found this field!
                    continue
                for rule, valid_set in self._rules.items():
                    if rule not in possible_fields[idx]:
                        continue
                    if val not in valid_set:
                        print(f"  [{idx}] {val} does not match rule: {rule}")
                        possible_fields[idx].remove(rule)
                        if len(possible_fields[idx]) == 1:
                            # found the right rule!  no other fields can fit this rule now
                            remove_from_possibles([possible_fields[idx]])
                            # correct_rule = possible_fields[idx]
                            # print(f"  [{idx}] must match {correct_rule}")
                            # for fields in (
                            #     possible_fields[:idx] + possible_fields[idx + 1 :]
                            # ):
                            #     fields -= correct_rule
                            #     if len(fields) == 1:
                            #         import ipdb

                            #         ipdb.set_trace()
        from pprint import pprint as pp

        possible_fields = [f.pop() for f in possible_fields]
        pp(possible_fields)
        final_answer = 1
        for idx, val in enumerate(self._my_ticket):
            if "departure" in possible_fields[idx]:
                final_answer *= val
        print(final_answer)
