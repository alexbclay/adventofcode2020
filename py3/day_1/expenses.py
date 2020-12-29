lines = []
with open('input.txt', 'r') as inp_file:
    for line in inp_file:
        lines.append(int(line.strip()))

print('--- PART 1 ---')
for i in range(len(lines)):
    for j in range(i, len(lines)):
        if lines[i] + lines[j] == 2020:
            print(lines[i] + lines[j])
            print(lines[i] * lines[j])

print('--- PART 2 ---')
for i in range(len(lines)):
    for j in range(i, len(lines)):
        for k in range(j, len(lines)):
            if lines[i] + lines[j] + lines[k] == 2020:
                print(lines[i] + lines[j] + lines[k])
                print(lines[i] * lines[j] * lines[k])
