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


def generate_rule(wolfram_number: int, neighborhood_size: int = 3, colours: list = None) -> list:
    if not colours: colours = [0, 1]
    colours_count = len(colours)
    possible_states = colours_count ** neighborhood_size
    rule = []

    wolfram_number = wolfram_number_to_bin(wolfram_number, possible_states, colours_count)
    for i, comb in enumerate(product(colours, repeat=neighborhood_size)):
        rule.append(RuleSegment(comb, int(wolfram_number[i])))
    return rule

#TODO make it round (like in rust solution)
def get_current_neighborhood(input_list: np.ndarray, i: int, neighborhood_center: int):
    width = input_list.shape[0]
    return tuple(input_list[j] for j in range((i - neighborhood_center) % width, (i + neighborhood_center + 1) % width))


def cellular_automata_step_1d(input_list: np.ndarray, rules: list) -> np.ndarray:
    output_list = np.zeros(shape=input_list.shape)
    width = input_list.shape[0]
    neighborhood_size = len(rules[0].neighborhood)
    neighborhood_center = (neighborhood_size - 1) // 2
    for i in range(len(input_list)):
        for rule in rules:
            current_neighborhood = tuple(
                input_list[j] for j in range((i - neighborhood_center) % width, (i + neighborhood_center + 1) % width))
            if current_neighborhood == rule.neighborhood:
                output_list[i] = rule.type

    return output_list


def generate_random(input_list: tuple, length: int) -> np.ndarray:
    return np.random.choice(a=input_list, size=(length, 1))


if __name__ == "__main__":
    # input_list = generate_random((0, 1, 2), 100)
    # rule = generate_rule(110, 3)
    # for seg in rule:
    #     print(f"{seg.neighborhood} {seg.type} ")
    #
    # for i in range(20):
    #     input_list = cellular_automata_step_1d(input_list, rule)
    #     print("".join(["*" if i == 1 else " " for i in input_list]))
    # input_list1 = np.full((20, 1), 0)
    # input_list1[len(input_list1) // 2] = 1
    # print(input_list1)
    # print(wolfram_number_to_bin(110, 8, 2))
    # print(n_nary(110, 2))
    # print(n_nary(0, 2))
    # print(n_nary(10, 3))
    #
    # print(list(product([1, 2, 3], repeat=3)))
    # print(generate_rule(110, 3))
    print(get_current_neighborhood(np.array([0, 1, 0, 1, 0]), 2, 1))
    print(get_current_neighborhood(np.array([0, 1, 0, 1, 0]), 0, 1))
