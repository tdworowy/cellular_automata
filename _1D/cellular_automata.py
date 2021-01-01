import random
from itertools import product

from utils.utils import RoundList


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


def cellular_automata_step_1d(input_list: RoundList, rules: list) -> RoundList:
    output_list = []
    for i in range(len(input_list)):
        for rule in rules:
            neighborhood_size = len(rule.neighborhood)
            temp = (neighborhood_size - 1) // 2
            current_neighborhood = tuple(input_list[j] for j in range(i - temp, i + temp + 1))

            if current_neighborhood == rule.neighborhood:
                output_list.append(rule.type)

    return RoundList(output_list)


def generate_random(input_list: list, length: int):
    temp = [random.choices(input_list) for i in range(length)]
    return [ele[0] for ele in temp]


if __name__ == "__main__":
    input_list = [0] * random.randrange(50, 100) + [1] * random.randrange(50, 100)
    random.shuffle(input_list)
    input_list = RoundList(input_list)
    rule = generate_rule(110, 3)
    for seg in rule:
        print(f"{seg.neighborhood} {seg.type} ")

    for i in range(20):
        input_list = cellular_automata_step_1d(input_list, rule)
        print("".join(["*" if i == 1 else " " for i in input_list]))
