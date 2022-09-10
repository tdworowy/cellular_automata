# from threading import Thread
# from queue import Queue

from multiprocessing import Process
from multiprocessing import Queue

test_queue = Queue()


def f1(test_queue):
    for i in range(100):
        test_queue.put(i)


def f2():
    while 1:
        print(test_queue.get())


if __name__ == "__main__":
    t = Process(target=f1, args=(test_queue,))
    t.daemon = True
    t.start()
    f2()
