import itertools
import math
from random import randint, uniform

from kivy.app import App
from kivy.clock import Clock
from kivy.uix.widget import Widget
from kivy.graphics import Color, Ellipse
from kivy.config import Config

colours = {"blue": (0, 0, 255, 255),
           "red": (255, 0, 0, 255),
           "green": (0, 255, 0, 255),
           # "aquamarine": (102, 205, 212, 255),
           # "gold": (255, 215, 0, 255),
           # "purple": (255, 0, 255, 255)
           }

# WIDTH = 1280
# HEIGHT = 720
WIDTH = 800
HEIGHT = 600


def ParticleInfo(color: str, x: int, y: int, vx: int, vy: int):
    return {"color": color, "x": x, "y": y, "vx": vx, "vy": vy}


def random_rules():
    rules = {}
    colours_pairs = itertools.product(colours.keys(), colours.keys())
    for pair in colours_pairs:
        rules[pair] = uniform(-2, 2)  # -1, 1

    return rules


class CanvasWidget(Widget):

    def __init__(self, **kwargs):
        super(CanvasWidget, self).__init__(**kwargs)
        self.r = 1
        X = range(WIDTH)
        Y = range(HEIGHT)
        self.coordinates = list(itertools.product(X, Y))

        self.particles = []

        self.time_scale = 1
        self.viscosity = 0.7
        self.pulse_duration = 10

        self.cutOff = 1000  # 6400 # interaction distance

        self.rules = random_rules()

    def generate_particle(self, color: str, x: int, y: int, vx: int, vy: int):
        with self.canvas:
            Color(*colours[color], mode='rgba')
            particle = Ellipse(pos=(x, y), size=(2 * self.r, 2 * self.r))
            self.particles.append(ParticleInfo(color, x, y, vx, vy))
            print(particle)

    def generate_init_particles(self, count: int, color: str):
        for i in range(count):
            x, y = self.coordinates.pop(randint(0, len(self.coordinates) - 1))
            self.generate_particle(color, x, y, 0, 0)

    def update_particles(self):
        self.canvas.clear()
        particles = self.particles.copy()
        self.particles = []
        for particle in particles:
            self.generate_particle(**particle)

    def apply_rules(self, rules):
        for particle_1 in self.particles:
            fx = 0
            fy = 0
            for particle_2 in self.particles:
                if id(particle_1) != id(particle_2):
                    g = rules[(particle_1["color"], particle_2["color"])]

                    dx = particle_1["x"] - particle_2["x"]
                    dy = particle_1["y"] - particle_2["y"]
                    if dx != 0 or dy != 0:
                        distance = dx * dx + dy * dy
                        if distance < self.cutOff:
                            F = g / math.sqrt(distance)
                            fx += F * dx
                            fy += F * dy

            vmix = (1. - self.viscosity)
            particle_1["vx"] = particle_1["vx"] * vmix + fx * self.time_scale
            particle_1["vy"] = particle_1["vy"] * vmix + fy * self.time_scale

        for particle in self.particles:
            particle["x"] += particle["vx"]
            particle["y"] += particle["vy"]

            if particle["x"] < 0 or particle["x"] >= WIDTH:
                particle["vx"] *= -1
                particle["x"] = 0 if particle["x"] < 0 else WIDTH - 1

            if particle["y"] < 0 or particle["y"] >= HEIGHT:
                particle["vy"] *= -1
                particle["y"] = 0 if particle["y"] < 0 else HEIGHT - 1


class CanvasApp(App):
    def build(self):
        Config.set('graphics', 'width', f'{WIDTH}')
        Config.set('graphics', 'height', f'{HEIGHT}')

        self.canvasWidget = CanvasWidget()
        return self.canvasWidget

    def update(self, args):
        self.canvasWidget.apply_rules(self.canvasWidget.rules)
        self.canvasWidget.update_particles()

    def on_start(self, **kwargs):
        self.canvasWidget.generate_init_particles(130, "red")
        self.canvasWidget.generate_init_particles(130, "blue")
        self.canvasWidget.generate_init_particles(130, "green")

        Clock.schedule_interval(callback=self.update, timeout=0.0)
        with open("rules.txt", "a") as rule_file:
            rule_file.write(f"{str(self.canvasWidget.rules)}\n")


if __name__ == "__main__":
    CanvasApp().run()
