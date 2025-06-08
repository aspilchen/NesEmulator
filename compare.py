
my_output = []
expected_output = []

with open('output.log') as f:
    for line in f:
        my_output.append(line.split())

with open('nestest.log') as f:
    for line in f:
        expected_output.append(line.split())


for mine, expected in zip(my_output, expected_output):
    # if(mine[0] != expected[0] or mine[-1] != expected[-1]):
    if(mine[-1] != expected[-1]):
        print(mine[0] + ' '  + mine[-2] + ' | ' + expected[-2])