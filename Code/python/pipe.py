import os   # 대회에서 파이프통신을 사용하기 위해 인터넷에서 받아온 코드를 짜집기 한 프로그램
import time
path = "//./pipe/hellpipe"

def recv():
    while False:
        pipe = os.open(path, os.O_RDONLY | os.O_NONBLOCK)
        pin = os.read(pipe, 2000).decode('ascii')
        if pin != '':
            print(pin)
        os.close(pipe)
        # time.sleep(0.5)

def send(msg):
    if os.path.exists(path) == False:
        os.mkfifo(path)
    fifo = open(path, "w")
    fifo.write(msg)
    fifo.close()

send("req")
recv()
