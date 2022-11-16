from pandas import Series, DataFrame

raw_data = {'col0': [1, "asd", "a\"s", ",as", "\"sd\"\"\",f"]}

data = DataFrame(raw_data)
print(data)
data.to_csv("./out.csv")
