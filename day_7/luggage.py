from collections import defaultdict
import re
import generic


class Solver(generic.AdventDay):
    BAG_REGEX = re.compile('(\d*|no other) (.*)')

    @classmethod
    def process_line(self, line):
        """
        Parse the line into color -> [(color, number), (color, number)]
        """
        line = super().process_line(line)
        line = line.replace('bags', '')
        line = line.replace('bag', '')
        line = line.replace('.', '')
        tokens = line.split(' contain ')
        before_part = tokens[0].strip()
        after_part = tokens[1]
        bags = []
        for bag in after_part.split(','):
            bag = bag.strip()
            if 'no other' in bag:
                continue
            match = self.BAG_REGEX.match(bag)
            bags.append((match.group(2), int(match.group(1))))
        return (before_part, dict(bags))

    def preprocess_input(self):
        self._graph = dict(self._raw_data)

        # Reverse the graph
        self._reverse_graph = defaultdict(lambda: defaultdict(dict))
        for parent_color, values in self._graph.items():
            for child_color, weight in values.items():
                print(f'{parent_color} -> {weight} -> {child_color}')
                self._reverse_graph[child_color][parent_color] = weight

    def solve_part_1(self):
        print('--- PART 1 ---')
        # Find all parents of 'shiny gold'
        all_parents = set()
        todo = list(self._reverse_graph['shiny gold'].keys())

        while todo:
            cur_parent = todo.pop()
            parents = list(self._reverse_graph[cur_parent].keys())
            all_parents.add(cur_parent)
            todo.extend(parents)
        print(len(all_parents))

    def _nested_bags(self, color, depth=0):
        if not self._graph.get(color):
            return 0
        nested = 0
        for child_color, weight in self._graph[color].items():
            nested += weight
            nested += self._nested_bags(child_color, depth=depth + 1) * weight
        return nested

    def solve_part_2(self):
        print('--- PART 2 ---')
        print(self._nested_bags('shiny gold'))
