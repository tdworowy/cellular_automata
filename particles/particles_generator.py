import itertools
import math
from multiprocessing import Queue
from random import uniform, randint


# TODO rewrite using numpy ?
def particle_info(color: str, x: int, y: int, vx: int, vy: int) -> dict:
    return {"color": color, "x": x, "y": y, "vx": vx, "vy": vy}


def random_rules(colours: dict, rule_range: tuple = (-2, 2)) -> dict:
    rules = {}
    colours_pairs = itertools.product(colours.keys(), colours.keys())
    for pair in colours_pairs:
        rules[pair] = uniform(*rule_range)  # -1, 1

    return rules


class ParticlesGenerator:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

        X = range(width)
        Y = range(height)
        self.coordinates = list(itertools.product(X, Y))

        self.time_scale = 1
        self.viscosity = 0.7

        self.interaction_distance = 2000  # 6400

    def generate_init_particles(self, count: int, color: str) -> list:
        particles = []
        for i in range(count):
            x, y = self.coordinates.pop(randint(0, len(self.coordinates) - 1))
            particles.append(particle_info(color, x, y, 0, 0))
        return particles

    # TODO make it multithread ?
    def apply_rules(self, rules: dict, particles: list) -> list:
        for particle_1 in particles:
            fx = 0
            fy = 0
            for particle_2 in particles:
                if id(particle_1) != id(particle_2):
                    g = rules[(particle_1["color"], particle_2["color"])]
                    dx = particle_1["x"] - particle_2["x"]
                    dy = particle_1["y"] - particle_2["y"]
                    if dx != 0 or dy != 0:
                        distance = dx * dx + dy * dy
                        if distance < self.interaction_distance:
                            F = g / math.sqrt(distance)
                            fx += F * dx
                            fy += F * dy

            vmix = (1. - self.viscosity)
            particle_1["vx"] = particle_1["vx"] * vmix + fx * self.time_scale
            particle_1["vy"] = particle_1["vy"] * vmix + fy * self.time_scale

        for particle in particles:
            particle["x"] += particle["vx"]
            particle["y"] += particle["vy"]

            if particle["x"] < 0 or particle["x"] >= self.width:
                particle["vx"] *= -1
                particle["x"] = 0 if particle["x"] < 0 else self.width - 1

            if particle["y"] < 0 or particle["y"] >= self.height:
                particle["vy"] *= -1
                particle["y"] = 0 if particle["y"] < 0 else self.height - 1

        return particles

    def update_particles(self, rule: dict, init_particles: list, particles_queue: Queue):
        particles = init_particles
        while 1:
            particles = self.apply_rules(rule, particles)
            particles_queue.put(particles)


if __name__ == "__main__":
    from multiprocessing import Process

    particles_queue = Queue()

    WIDTH = 1280
    HEIGHT = 720

    colours = {"blue": (0, 0, 255, 255),
               "red": (255, 0, 0, 255),
               "green": (0, 255, 0, 255),
               "purple": (255, 0, 255, 255)
               # "aquamarine": (102, 205, 212, 255),
               # "gold": (255, 215, 0, 255),
               }

    particles_generator = ParticlesGenerator(width=WIDTH, height=HEIGHT)
    init_particles = particles_generator.generate_init_particles(300, "red")
    init_particles += particles_generator.generate_init_particles(300, "blue")
    init_particles += particles_generator.generate_init_particles(300, "green")
    init_particles += particles_generator.generate_init_particles(300, "purple")

    rules = random_rules(colours)

    process = Process(target=particles_generator.update_particles,
                      args=(rules, init_particles, particles_queue))
    process.daemon = True
    process.start()

    while 1:
        pass
