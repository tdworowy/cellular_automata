import itertools
import math
import uuid
from multiprocessing import Queue as MultiprocessingQueue
from threading import Thread
from random import uniform, randint
from time import sleep

# TODO fix it

thread_results = []


def particle_info(color: str, x: int, y: int, vx: int, vy: int) -> dict:
    return {"id": uuid.uuid4(), "color": color, "x": x, "y": y, "vx": vx, "vy": vy}


def random_rules(colours: dict, rule_range: tuple = (-2, 2)) -> dict:
    rules = {}
    colours_pairs = itertools.product(colours.keys(), colours.keys())
    for pair in colours_pairs:
        rules[pair] = uniform(*rule_range)  # -1, 1

    return rules


def split(lst: list, n: int):
    sub_list_length = len(lst) // n
    for i in range(0, len(lst), sub_list_length):
        yield lst[i:i + sub_list_length]


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

    def apply_rules_thread(self, rules: dict, particles_sub_list: list, particles_all_list: list):
        for particle_1 in particles_sub_list:
            fx = 0
            fy = 0
            for particle_2 in particles_all_list:
                if particle_1["id"] != particle_2["id"]:
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

        thread_results.append(particles_sub_list)

    def apply_rules(self, rules: dict, particles: list, number_of_threads: int) -> list:
        threads = []
        particles_chunks = list(split(particles, number_of_threads))
        print(len(particles_chunks))

        for i in range(number_of_threads):
            thread = Thread(target=self.apply_rules_thread, args=(rules, particles_chunks[i], particles))
            thread.start()
            threads.append(thread)

        for thread in threads:
            thread.join()

        particles_results = [particle for particle in thread_results.pop() for _ in range(number_of_threads)]

        for particle in particles_results:
            particle["x"] += particle["vx"]
            particle["y"] += particle["vy"]

            if particle["x"] < 0 or particle["x"] >= self.width:
                particle["vx"] *= -1
                particle["x"] = 0 if particle["x"] < 0 else self.width - 1

            if particle["y"] < 0 or particle["y"] >= self.height:
                particle["vy"] *= -1
                particle["y"] = 0 if particle["y"] < 0 else self.height - 1

        return particles_results

    def update_particles(self, rule: dict, init_particles: list, particles_queue: MultiprocessingQueue,
                         ):
        particles = init_particles
        number_of_threads = len(particles) // 10
        while 1:
            particles = self.apply_rules(rule, particles, number_of_threads)
            particles_queue.put(particles)


if __name__ == "__main__":
    from multiprocessing import Process

    particles_queue = MultiprocessingQueue()

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
    init_particles = particles_generator.generate_init_particles(5, "red")
    init_particles += particles_generator.generate_init_particles(5, "blue")
    init_particles += particles_generator.generate_init_particles(5, "green")
    init_particles += particles_generator.generate_init_particles(5, "purple")

    rules = random_rules(colours)

    process = Process(target=particles_generator.update_particles,
                      args=(rules, init_particles, particles_queue))
    process.daemon = True
    process.start()

    while 1:
        sleep(20)
        break
