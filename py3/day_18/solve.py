from functools import reduce
from pprint import pprint as pp
import generic


class Solver(generic.AdventDay):
    def __init__(self, filename="input.txt"):
        super().__init__(filename)

    @classmethod
    def process_line(cls, line):
        line = super().process_line(line)

        line = line.split(" ")
        tokens = []
        for t in line:
            tokens.extend(list(t))
        return tokens

    def preprocess_input(self):
        pass

    def evaluate_part_1(self, line):
        opers = {"*": lambda x, y: x * y, "+": lambda x, y: x + y}

        stack = []
        oper_stack = []
        parens = []
        for token in line:
            # print(f"---  {token} ---")
            if token in opers.keys():
                oper_stack.append(opers[token])
            elif token == "(":
                parens.append(len(stack))
            elif token == ")":
                parens.pop()
            else:
                stack.append(int(token))
            if len(stack) == 2 + (parens[-1] if parens else 0):
                x = stack.pop()
                y = stack.pop()
                oper = oper_stack.pop()
                stack.append(oper(x, y))
            # from pprint import pprint as pp

            # pp(stack)
            # pp(oper_stack)
            # pp(parens)
        return stack.pop()

    def evaluate_part_2(self, line):

        add_stack = []
        mult_stack = []
        parens_stack = []
        for token in line:
            if token == "(":
                parens_stack.append((add_stack, mult_stack))
                add_stack = []
                mult_stack = []
                continue
            if token == ")":
                x = add_stack.pop() if add_stack else 1
                y = mult_stack.pop() if mult_stack else 1
                add_stack, mult_stack = parens_stack.pop()
                if add_stack:
                    add_stack.append(add_stack.pop() + (x * y))
                else:
                    add_stack.append(x * y)
                continue
            if token == "+":
                continue
            if token == "*":
                arg = add_stack.pop()
                if mult_stack:
                    arg *= mult_stack.pop()
                mult_stack.append(arg)
                continue
            token = int(token)
            if add_stack:
                arg = add_stack.pop()
                add_stack.append(token + arg)
            else:
                add_stack.append(token)

        print("---- END ----")
        mult_stack.append(add_stack.pop())
        return reduce(lambda x, y: x * y, mult_stack, 1)

    def solve_part_1(self):
        print("--- PART 1 ---")
        total = 0
        for line in self._raw_data:
            value = self.evaluate_part_1(line)
            print(f"    {value}")
            total += value
        print(f"SOLUTION: {total}")

    def solve_part_2(self):
        print("--- PART 2 ---")

        total = 0
        for line in self._raw_data:
            value = self.evaluate_part_2(line)
            print(f"    {value}")
            total += value
        print(f"SOLUTION: {total}")
