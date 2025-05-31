
my_output = []
expected_output = []

with open('output.txt') as f:
    for line in f:
        my_output.append(line.split())

with open('nestest.log') as f:
    for line in f:
        expected_output.append([line.split()[0], line[48:52]])


for mine, expected in zip(my_output, expected_output):
    if(mine[0] != expected[0] or mine[3] != expected[1]):
        print(mine[0] + ' ' + mine[2] + ' ' + mine[3] + ' | ' + expected[0] + ' ' + expected[1])