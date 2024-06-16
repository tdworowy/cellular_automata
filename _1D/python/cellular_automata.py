from itertools import product
import numpy as np


class RuleSegment:
    def __init__(self, neighborhood: tuple, type: int):
        self.neighborhood = neighborhood
        self.type = type


def n_nary(number: int, n: int) -> str:
    if number == 0:
        return "0"
    nums = []
    while number:
        number, r = divmod(number, n)
        nums.append(str(r))
    return "".join(reversed(nums))


def wolfram_number_to_bin(
    wolfram_number: int, possible_states: int, colours_count: int
) -> list:
    wolfram_number = n_nary(wolfram_number, colours_count)
    temp = possible_states - len(wolfram_number)
    wolfram_number = "0" * temp + wolfram_number
    return list(wolfram_number)[::-1]


def generate_rule(
    wolfram_number: int, neighborhood_size: int = 3, colours: list = None
) -> list:
    if not colours:
        colours = [0, 1]
    colours_count = len(colours)
    possible_states = colours_count**neighborhood_size
    rule = []

    wolfram_number = wolfram_number_to_bin(
        wolfram_number, possible_states, colours_count
    )
    for i, comb in enumerate(product(colours, repeat=neighborhood_size)):
        rule.append(RuleSegment(comb, int(wolfram_number[i])))
    return rule


def match_index(index: int, width: int) -> int:
    match index:
        case x if x < 0:
            return index + width
        case x if x >= width:
            return index - width
        case _:
            return index


def get_current_neighborhood(
    input_list: np.ndarray, i: int, neighborhood_center: int
) -> tuple:
    width = input_list.shape[0]

    return tuple(
        input_list[match_index(j, width)]
        for j in range((i - neighborhood_center), (i + neighborhood_center + 1))
    )


def cellular_automata_step_1d(input_list: np.ndarray, rules: list) -> np.ndarray:
    output_list = np.zeros(shape=input_list.shape)
    neighborhood_size = len(rules[0].neighborhood)
    neighborhood_center = (neighborhood_size - 1) // 2
    for i in range(len(input_list)):
        current_neighborhood = get_current_neighborhood(
            input_list, i, neighborhood_center
        )
        for rule in rules:
            if current_neighborhood == rule.neighborhood:
                output_list[i] = rule.type

    return output_list


def generate_random(input_list: tuple, length: int) -> np.ndarray:
    return np.random.choice(a=input_list, size=(length, 1))
