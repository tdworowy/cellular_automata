from multiprocessing import Process, Queue
from kivy.app import App
from kivy.clock import Clock
from kivy.uix.widget import Widget
from kivy.graphics import Color, Ellipse
from kivy.config import Config

from particles.particles_generator import ParticlesGenerator, random_rules

colours = {"blue": (0, 0, 255, 255),
           "red": (255, 0, 0, 255),
           "green": (0, 255, 0, 255),
           "purple": (255, 0, 255, 255)
           # "aquamarine": (102, 205, 212, 255),
           # "gold": (255, 215, 0, 255),
           }

WIDTH = 1280
HEIGHT = 720
# WIDTH = 800
# HEIGHT = 600

particles_queue = Queue()


# TODO maybe use pyglet or  wxPython to render graphics


class CanvasWidget(Widget):

    def __init__(self, **kwargs):
        super(CanvasWidget, self).__init__(**kwargs)
        r = 1
        self.particle_size = (2 * r, 2 * r)

    def generate_particle(self, particles: list):
        with self.canvas:
            for particle in particles:
                Color(*colours[particle["color"]], mode='rgba')
                Ellipse(pos=(particle["x"], particle["y"]), size=self.particle_size)

    def update_particles(self):
        self.canvas.clear()
        particles = particles_queue.get()
        if particles:
            self.generate_particle(particles)


class CanvasApp(App):
    def build(self):
        Config.set('graphics', 'width', f'{WIDTH}')
        Config.set('graphics', 'height', f'{HEIGHT}')

        self.canvasWidget = CanvasWidget()
        return self.canvasWidget

    def update(self, args):
        self.canvasWidget.update_particles()

    def on_start(self, **kwargs):
        particles_generator = ParticlesGenerator(width=WIDTH, height=HEIGHT)
        init_particles = particles_generator.generate_init_particles(150, "red")
        init_particles += particles_generator.generate_init_particles(150, "blue")
        init_particles += particles_generator.generate_init_particles(150, "green")
        init_particles += particles_generator.generate_init_particles(150, "purple")

        self.canvasWidget.generate_particle(init_particles)
        #rules = random_rules(colours)
        rules = {('blue', 'blue'): -1.619456250134184, ('blue', 'red'): -0.3459078130538211, ('blue', 'green'): 1.9235154087769644, ('blue', 'purple'): 0.7328078105108142, ('red', 'blue'): -1.9393750396825413, ('red', 'red'): -0.6213147956593676, ('red', 'green'): 1.4581702264244472, ('red', 'purple'): 1.0189031499122243, ('green', 'blue'): -0.7719620051953995, ('green', 'red'): -0.07758659422276581, ('green', 'green'): 0.7873139469760457, ('green', 'purple'): -0.44476962397851905, ('purple', 'blue'): -0.5279724795400558, ('purple', 'red'): 1.7644963352292482, ('purple', 'green'): 1.8229069818418595, ('purple', 'purple'): 1.8826645344706283}

        process = Process(target=particles_generator.update_particles,
                          args=(rules, init_particles, particles_queue))
        process.daemon = True
        process.start()

        Clock.schedule_interval(callback=self.update, timeout=0.0)

        with open("rules.txt", "a") as rule_file:
            rule_file.write(f"{str(rules)}\n")


if __name__ == "__main__":
    CanvasApp().run()
