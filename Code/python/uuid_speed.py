import pandas as pd
import numpy as np
import uuid
import time


class UUIDWrapper(int):
    """
    128비트의 데이터(16바이트)를 래핑해주는 래퍼
    """

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


ser = pd.Series([np.random.random()]*1000000)
s = time.time()
ser = ser.map(lambda x: str(uuid.uuid1()), na_action='ignore')
print(time.time()-s)

s = time.time()
ser.to_csv("uuidtest.csv", encoding='utf8')
print(time.time()-s)

ser = pd.Series([np.random.random()]*1000000)
s = time.time()
ser = ser.map(lambda x: UUIDWrapper(int(uuid.uuid1())), na_action='ignore')
print(time.time()-s)

s = time.time()
ser.to_csv("uuidtest.csv", encoding='utf8')
print(time.time()-s)
