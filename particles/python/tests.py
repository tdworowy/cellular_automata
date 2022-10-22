# from threading import Thread
# from queue import Queue

# from multiprocessing import Process
# from multiprocessing import Queue
#
# test_queue = Queue()
#
#
# def f1(test_queue):
#     for i in range(100):
#         test_queue.put(i)
#
#
# def f2():
#     while 1:
#         print(test_queue.get())
#
#
# if __name__ == "__main__":
#     t = Process(target=f1, args=(test_queue,))
#     t.daemon = True
#     t.start()
#     f2()

import pyglet
from pyglet import shapes
from pyglet import clock

window = pyglet.window.Window(960, 540)
batch = pyglet.graphics.Batch()
circles = []


def draw_circles(*args,**kwargs):
    for i in range(10):
        circle = shapes.Circle(i * 5, i * 5, 5, color=(50, 225, 30), batch=batch)
        circles.append(circle)


@window.event
def on_draw():
    draw_circles()
    window.clear()
    batch.draw()

    clock.schedule(draw_circles)


if __name__ == "__main__":
    pyglet.app.run()
