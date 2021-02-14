from itertools import product
import numpy as np


class RuleSegment:
    def __init__(self, neighborhood: tuple, type: int):
        self.neighborhood = neighborhood
        self.type = type


def n_nary(number: int, n: int) -> str:
    if number == 0:
        return '0'
    nums = []
    while number:
        number, r = divmod(number, n)
        nums.append(str(r))
    return ''.join(reversed(nums))


def wolfram_number_to_bin(wolfram_number: int, possible_states: int, colours_count: int) -> list:
    wolfram_number = n_nary(wolfram_number, colours_count)
    temp = possible_states - len(wolfram_number)
    wolfram_number = "0" * temp + wolfram_number
    return list(wolfram_number)[::-1]


def generate_rule(wolfram_number: int, neighborhood_size: int = 3, colours: list = None):
    if not colours: colours = [0, 1]
    colours_count = len(colours)
    possible_states = colours_count ** neighborhood_size
    rule = []

    wolfram_number = wolfram_number_to_bin(wolfram_number, possible_states, colours_count)
    for i, comb in enumerate(product(colours, repeat=neighborhood_size)):
        rule.append(RuleSegment(comb, int(wolfram_number[i])))
    return rule


def cellular_automata_step_1d(input_list: np.ndarray, rules: list) -> np.ndarray:
    output_list = np.zeros(shape=input_list.shape)
    width = input_list.shape[0]
    for i in range(len(input_list)):
        for rule in rules:
            neighborhood_size = len(rule.neighborhood)
            temp = (neighborhood_size - 1) // 2
            current_neighborhood = tuple(input_list[j] for j in range((i - temp) % width, (i + temp + 1) % width))

            if current_neighborhood == rule.neighborhood:
                output_list[i] = (rule.type)

    return output_list


def generate_random(input_list: tuple, length: int):
    return np.random.choice(a=input_list, size=(length, 1))


if __name__ == "__main__":
    input_list = generate_random((0, 1, 2), 100)
    rule = generate_rule(110, 3)
    for seg in rule:
        print(f"{seg.neighborhood} {seg.type} ")

    for i in range(20):
        input_list = cellular_automata_step_1d(input_list, rule)
        print("".join(["*" if i == 1 else " " for i in input_list]))
