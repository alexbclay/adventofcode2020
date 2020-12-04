import re

import generic

class Solver(generic.AdventDay):
    PATTERN = re.compile('(\S*):(\S*)')

    def preprocess_input(self):
        """
        Parse lines into passport dicts
        """
        self._data = []
        cur_chunk = {}
        for line in self._raw_data:
            if not line:
                self._data.append(cur_chunk)
                cur_chunk = {}
                continue
            # parse the line ('a:123 b:456')
            # into a list of tuples: [('a', '123'), ('b', '456')]
            line_values = self.PATTERN.findall(line)
            cur_chunk.update(dict(line_values))

        # Don't forget the last one!
        self._data.append(cur_chunk)


    def solve_part_1(self):
        """
        Check for correct passports
        """
        valid_keys = set([
            'byr',
            'iyr',
            'eyr',
            'hgt',
            'hcl',
            'ecl',
            'pid',
            'cid'
        ])
        valid = 0
        for info in self._data:
            missing = valid_keys - set(info.keys())
            print(missing)
            if not missing or missing == set(['cid']):
                valid += 1
        print('-- PART 1 --')
        print(valid)

    def solve_part_2(self):
        """
        Validate password fields!
        """
        def validate_height(height):
            pattern = re.compile('(\d*)(cm|in)')
            match = pattern.fullmatch(height)
            if not match:
                return False
            if match.group(2) == 'cm':
                return 150 <= int(match.group(1)) <= 193
            return 59 <= int(match.group(1)) <= 76
        key_validation = {
            'byr': lambda year: 1920 <= int(year) <= 2002,
            'iyr': lambda year: 2010 <= int(year) <= 2020,
            'eyr': lambda year: 2020 <= int(year) <= 2030,
            'hgt': validate_height,
            'hcl': lambda color: re.fullmatch(r'#[0-9a-f]{6}', color),
            'ecl': lambda color: color in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'),
            'pid': lambda pid: re.fullmatch(r'[0-9]{9}', pid)
        }

        valid_passports = 0
        for info in self._data:
            # CID doesn't matter at all
            cid = info.pop('cid', [])

            valid = True
            for key, func in key_validation.items():
                value = info.get(key, None)
                if value is None or not func(value):
                    # missing key: invalid!
                    # validation function is False: invalid!
                    valid = False
                    break
            if valid:
                # nothing was missing or invalid: this one is valid!
                valid_passports += 1

        print('--- PART 2 ---')
        print(valid_passports)
