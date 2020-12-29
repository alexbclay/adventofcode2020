from copy import deepcopy
import re
import generic


class Solver(generic.AdventDay):
    LINE_REGEX = re.compile('(jmp|acc|nop) ([-+]\d*)')

    @classmethod
    def process_line(self, line):
        """
        Parse the line into instruction -> argument
        """
        line = super().process_line(line)
        match = self.LINE_REGEX.match(line)

        return {'instruction': match.group(1), 'argument': int(match.group(2))}

    def preprocess_input(self):
        pass

    def execute_program(self, program):
        """
        Execute the given program
        """
        accumulator = 0
        index = 0
        visited = set({})

        while index < len(program):
            # Found a loop
            if index in visited:
                return {
                    'state': 'loop',
                    'accumulator': accumulator,
                }
            cur_instruction = program[index]
            visited.add(index)
            if cur_instruction['instruction'] == 'nop':
                index += 1
                continue
            if cur_instruction['instruction'] == 'acc':
                index += 1
                accumulator += cur_instruction['argument']
                continue
            if cur_instruction['instruction'] == 'jmp':
                index += cur_instruction['argument']
                continue
            print(f'!! Unknown instruction! {cur_instruction} !!')
        # Exit the loop normally
        return {
            'state': 'done',
            'accumulator': accumulator,
        }

    def solve_part_1(self):
        print('--- PART 1 ---')
        res = self.execute_program(self._raw_data)
        print(res['accumulator'])

    def solve_part_2(self):
        print('--- PART 2 ---')

        for index, line in enumerate(self._raw_data):
            if line['instruction'] == 'nop':
                # try switching this nop to a jmp
                program_copy = deepcopy(self._raw_data)
                program_copy[index]['instruction'] = 'jmp'
                res = self.execute_program(program_copy)
                if res['state'] == 'done':
                    print(f'-- swap nop -> jmp: {index}')
                    print(f'Finished loop: {res["accumulator"]}')
            if line['instruction'] == 'jmp':
                # try switching this nop to a jmp
                program_copy = deepcopy(self._raw_data)
                program_copy[index]['instruction'] = 'nop'
                res = self.execute_program(program_copy)
                if res['state'] == 'done':
                    print(f'-- swap jmp -> nop: {index}')
                    print(f'Finished loop: {res["accumulator"]}')
