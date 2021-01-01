from collections import defaultdict
from random import choice


def random_rule(min_to_born=1):
    rule_a = {(0, i): choice([0, 1]) for i in range(27)}
    rule_b = {(1, i): choice([0, 1]) for i in range(27)}

    rule = {**rule_a, **rule_b}

    rule[(0, min_to_born)] = 1

    return rule


rule_1 = {
    (0, 1): 1,
    (1, 3): 1,
    (1, 2): 1
}
rule_1 = defaultdict(lambda: 0, rule_1)

rule_2 = {
    (0, 1): 1,
    (1, 10): 1
}
rule_2 = defaultdict(lambda: 0, rule_2)

rule_3 = {
    (0, 4): 1,
    (0, 5): 1,
    (1, 5): 1
}
rule_3 = defaultdict(lambda: 0, rule_2)

standard_game_of_live_rules = {
    (0, 3): 1,
    (1, 3): 1,
    (1, 2): 1
}
standard_game_of_live_rules = defaultdict(lambda: 0, standard_game_of_live_rules)
