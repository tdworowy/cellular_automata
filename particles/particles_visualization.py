import itertools
from random import randint
from kivy.app import App
from kivy.uix.widget import Widget
from kivy.graphics import Color, Ellipse
from kivy.config import Config

colours = {"blue": (0, 0, 255, 255),
           "red": (255, 0, 0, 255),
           "green": (0, 255, 0, 255),
           "aquamarine": (102, 205, 212, 255),
           "gold": (255, 215, 0, 255),
           "purple": (255, 0, 255, 255)
           }

WIDTH = 1280
HEIGHT = 720


class CanvasWidget(Widget):

    def __init__(self, **kwargs):
        super(CanvasWidget, self).__init__(**kwargs)
        self.r = 1
        X = range(WIDTH)
        Y = range(HEIGHT)
        self.coordinates = list(itertools.product(X, Y))

    def generate_particle(self, x: int = 100, y: int = 100, color: tuple = (255, 255, 255, 8)):
        with self.canvas:
            Color(*color, mode='rgba')
            self.particle = Ellipse(pos=(x - self.r, y - self.r), size=(2 * self.r, 2 * self.r))
            print(self.particle)

    def generate_particles(self, count: int, color: tuple):
        for i in range(count):
            x, y = self.coordinates.pop(randint(0, len(self.coordinates) - 1))
            self.generate_particle(x, y, color)


class CanvasApp(App):
    def build(self):
        Config.set('graphics', 'width', f'{WIDTH}')
        Config.set('graphics', 'height', f'{HEIGHT}')

        self.canvasWidget = CanvasWidget()
        return self.canvasWidget

    def on_start(self, **kwargs):
        self.canvasWidget.generate_particles(1000, colours["red"])
        self.canvasWidget.generate_particles(1000, colours["blue"])
        self.canvasWidget.generate_particles(1000, colours["green"])


if __name__ == "__main__":
    CanvasApp().run()
