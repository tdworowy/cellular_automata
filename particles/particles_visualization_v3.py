from multiprocessing import Process, Queue

import pyglet
from pyglet import shapes
from pyglet import clock

from particles.particles_generator import ParticlesGenerator, random_rules

colours = {"blue": (0, 0, 255),
           "red": (255, 0, 0),
           "green": (0, 255, 0),
           "purple": (255, 0, 255)
           # "aquamarine": (102, 205, 212, 255),
           # "gold": (255, 215, 0, 255),
           }

WIDTH = 1280
HEIGHT = 720
# WIDTH = 800
# HEIGHT = 600

particles_queue = Queue()

window = pyglet.window.Window(WIDTH, HEIGHT)
batch = pyglet.graphics.Batch()
r = 2

rendered_particles = []


def generate_particles(particles: list):
    for particle in particles:
        rendered_particle = shapes.Circle(particle["x"], particle["y"], r, color=colours[particle["color"]],
                                          batch=batch)
        rendered_particle.opacity = 255
        rendered_particles.append(rendered_particle)


def update_particles(*args, **Kwargs):
    particles = particles_queue.get()
    if particles:
        rendered_particles.clear()
        generate_particles(particles)


start = [0]


@window.event
def on_draw():
    if start[0] == 0:
        particles_generator = ParticlesGenerator(width=WIDTH, height=HEIGHT)

        init_particles = particles_generator.generate_init_particles(150, "red")
        init_particles += particles_generator.generate_init_particles(150, "blue")
        init_particles += particles_generator.generate_init_particles(150, "green")
        init_particles += particles_generator.generate_init_particles(150, "purple")

        window.clear()
        generate_particles(init_particles)
        batch.draw()

        rules = random_rules(colours)
        with open("rules.txt", "a") as rule_file:
            rule_file.write(f"{str(rules)}\n")

        process = Process(target=particles_generator.update_particles,
                          args=(rules, init_particles, particles_queue))
        process.daemon = True
        process.start()

        clock.schedule(update_particles)
        start[0] = 1
    else:
        window.clear()
        batch.draw()


if __name__ == "__main__":
    pyglet.app.run()
