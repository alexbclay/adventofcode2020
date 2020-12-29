import re

LINE_RE = re.compile('(\d+)-(\d+) ([a-z]): ([a-z]*)')


def parse_line(string):
    """
    Parse out the password requirement and the password
    """
    match = LINE_RE.match(string)

    return {
        'first': int(match.group(1)),
        'second': int(match.group(2)),
        'letter': match.group(3),
        'password': match.group(4),
    }


lines = []
with open('input.txt', 'r') as inp:
    for line in inp:
        lines.append(parse_line(line))

count = 0
for info in lines:
    only_letters = re.sub(f'[^{info["letter"]}]', '', info['password'])
    if info['first'] <= len(only_letters) <= info['second']:
        count += 1

print('--- PART 1 ---')
print(count)

count = 0
for info in lines:
    in_first = info['password'][info['first'] - 1] == info['letter']

    in_second = info['password'][info['second'] - 1] == info['letter']

    if (in_first and not in_second) or (in_second and not in_first):
        count += 1
        from pprint import pprint as pp

        pp(info)

print('--- PART 2 ---')
print(count)
