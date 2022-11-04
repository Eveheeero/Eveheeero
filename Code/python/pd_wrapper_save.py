import time
import uuid
import pandas as pd


class Wrapper(bytes):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls, *args, **kwargs)

    def __str__(self) -> str:
        return super().decode("utf-8")


class UUIDWrapper(int):
    """
    128비트의 데이터(16바이트)를 래핑해주는 래퍼
    """

    def __new__(cls, *args, **kwargs):
        """
        기본 생성자

        Returns:
            _type_: 추가적인 처리가 필요하지 않아 args와 kwargs를 그대로 넘겨줌
        """
        return super().__new__(cls, *args, **kwargs)

    def __str__(self) -> str:
        """
        str(data)를 했을때 나타나는 데이터

        Returns:
            str: UUID형식에 맞는 데이터
        """
        data = '%032x' % self
        return '%s-%s-%s-%s-%s' % (
            data[:8], data[8:12], data[12:16], data[16:20], data[20:])

    def __repr__(self) -> str:
        """
        data 를 실행했을때 출력되는 데이터

        Returns:
            str: UUID 형식에 맞는 데이터
        """
        data = '%032x' % self
        return f"UUID Wrapper ({ data })"


uuids = []
numbers = []
for now in range(10000000):
    uuids.append(uuid.uuid4())
    numbers.append(str(now))

print("UUID")

start_time = time.time()
data = []
for now in uuids:
    data.append(UUIDWrapper(now.int))
data = pd.Series(data)
end_time = time.time()
print(f"UUID Wrapper List Save Time is {end_time - start_time} seconds")
start_time = time.time()
data.to_csv("test.csv", index=False)
end_time = time.time()
print(f"UUID Wrapper File Save Time is {end_time - start_time} seconds")


start_time = time.time()
data = []
for now in uuids:
    data.append(now)
data = pd.Series(data)
end_time = time.time()
print(f"List Save Time is {end_time - start_time} seconds")
start_time = time.time()
data.to_csv("test.csv", index=False)
end_time = time.time()
print(f"File Save Time is {end_time - start_time} seconds")

print("Number")

start_time = time.time()
data = []
for now in numbers:
    data.append(Wrapper(now.encode("utf-8")))
data = pd.Series(data)
end_time = time.time()
print(f"Wrapper List Save Time is {end_time - start_time} seconds")
start_time = time.time()
data.to_csv("test.csv", index=False)
end_time = time.time()
print(f"Wrapper File Save Time is {end_time - start_time} seconds")


start_time = time.time()
data = []
for now in numbers:
    data.append(now)
data = pd.Series(data)
end_time = time.time()
print(f"List Save Time is {end_time - start_time} seconds")
start_time = time.time()
data.to_csv("test.csv", index=False)
end_time = time.time()
print(f"File Save Time is {end_time - start_time} seconds")
