import pickle

filename = input()
data: dict = pickle.load(open(filename, 'rb'))

for item in data.items():
    print(item)
