import psutil
import gc
import uuid


process = psutil.Process()

data = []
for now in range(10000000):
    data.append(uuid.uuid4().bytes)  # 물리메모리 - 710MB
    if now % 1000000 == 0:
        print(f"Now Usage is { process.memory_info().rss / 1024 / 1024 } MB")
gc.collect()
print(f"Now Usage is { process.memory_info().rss / 1024 / 1024 } MB")
input()
