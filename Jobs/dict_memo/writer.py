import pickle

filename = input()

try:
    data = pickle.load(open(filename, 'rb'))
except FileNotFoundError:
    data = {}


def write_line(line: str):
    if line.count(' ') == 0:
        data[line] = ('', '')

    elif line.count(' ') == 1:
        idx = line.find(' ')
        data[line[0:idx]] = (line[idx+1:], '')

    else:
        idx1 = line.find(' ')
        idx2 = line[idx1+1:].find(' ')
        data[line[0:idx1]] = (line[idx1+1:idx1+idx2+1], line[idx1+idx2+2:])


try:
    print("Serial - Name - Description (띄어쓰기 구분)")
    while True:
        line = input()
        write_line(line)
except KeyboardInterrupt:
    pickle.dump(data, open(filename, 'wb'))
