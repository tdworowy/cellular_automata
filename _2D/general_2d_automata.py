import numpy as np
from collections import defaultdict
from utils.utils import default_dict
import time

game_of_live_rules = {
    (0, 3): 1,
    (1, 3): 1,
    (1, 2): 1
}
game_of_live_rules = defaultdict(lambda: 0, game_of_live_rules)

amoeba_rules = {
    (0, 3): 1,
    (0, 5): 1,
    (0, 7): 1,
    (1, 1): 1,
    (1, 3): 1,
    (1, 5): 1,
    (1, 8): 1
}
amoeba_rules = defaultdict(lambda: 0, amoeba_rules)

_2x2_rules = {
    (0, 3): 1,
    (0, 6): 1,
    (1, 1): 1,
    (1, 2): 1,
    (1, 5): 1,
}
_2x2_rules = defaultdict(lambda: 0, _2x2_rules)

_34_live_rules = {
    (0, 3): 1,
    (0, 4): 1,
    (1, 3): 1,
    (1, 4): 1,
}
_34_live_rules = defaultdict(lambda: 0, _34_live_rules)

coagulations_rules = {
    (0, 3): 1,
    (0, 7): 1,
    (0, 8): 1,
    (1, 2): 1,
    (1, 3): 1,
    (1, 5): 1,
    (1, 6): 1,
    (1, 7): 1,
    (1, 8): 1,
}
coagulations_rules = defaultdict(lambda: 0, coagulations_rules)

mazectric_rules = {
    (0, 3): 1,
    (1, 1): 1,
    (1, 2): 1,
    (1, 3): 1,
    (1, 4): 1,
}
mazectric_rules = defaultdict(lambda: 0, mazectric_rules)

move_rules = {
    (0, 3): 1,
    (0, 6): 1,
    (0, 8): 1,
    (1, 2): 1,
    (1, 4): 1,
    (1, 5): 1,
}
move_rules = defaultdict(lambda: 0, move_rules)

walled_cities_rules = {
    (0, 4): 1,
    (0, 5): 1,
    (0, 6): 1,
    (0, 7): 1,
    (0, 8): 1,

    (1, 2): 1,
    (1, 3): 1,
    (1, 4): 1,
    (1, 5): 1,
}
walled_cities_rules = defaultdict(lambda: 0, walled_cities_rules)


def generate_snowflake_rule(neighbours_numbers: list = [1]):
    snowflake_rules = default_dict(lambda self, key: key[0])

    for neighbours_number in neighbours_numbers:
        snowflake_rules[(0, neighbours_number)] = 1
        snowflake_rules[(1, neighbours_number)] = 1

    return snowflake_rules


rules = {
    'game_of_life': game_of_live_rules,
    'amoeba': amoeba_rules,
    'twoXTwo': _2x2_rules,
    'threeFourLive': _34_live_rules,
    'coagulations': coagulations_rules,
    'mazectric_rules': mazectric_rules,
    'move': move_rules,
    'walled_cities': walled_cities_rules,
    'snowflake_1': generate_snowflake_rule(neighbours_numbers=[1]),
    'snowflake_1_5': generate_snowflake_rule(neighbours_numbers=[1, 5]),
    'snowflake_1_3_5': generate_snowflake_rule(neighbours_numbers=[1, 3, 5]),
    'snowflake_1_3': generate_snowflake_rule(neighbours_numbers=[1, 3]),

}


def generate_grid_random_cells(width: int, height: int, probability_of_one: float) -> np.ndarray:
    probability_of_zero = 1 - probability_of_one
    return np.random.choice(a=(0, 1), size=(width, height), p=(probability_of_zero, probability_of_one))


def generate_grid_one_cell(width: int, height: int) -> np.ndarray:
    grid = np.full((width,height ), 0)
    grid[width // 2][height // 2] = 1
    return grid


def generate_grid_central(width: int, height: int, cell_count: int = 1) -> np.ndarray:
    if cell_count == 1: return generate_grid_one_cell(width, height)
    grid = np.full((height, width), 0)
    x = width // 2
    y = height // 2
    for i in range(cell_count // 2):
        for j in range(cell_count // 2):
            grid[x + i][y + j] = 1
    return grid


def count_colored_neighbours(x: int, y: int, grid: np.ndarray):
    colored_neighbours = 0
    for i in range((x - 1) % grid.shape[0], (x + 2) % grid.shape[0]):
        for j in range((y - 1) % grid.shape[1], (y + 2) % grid.shape[1]):
            if grid[i][j] == 1 and (i, j) != (x, y): colored_neighbours += 1
    return colored_neighbours


def update_grid_two_d(grid: np.ndarray, rules: defaultdict):
    new_grid = grid.copy()
    for i, row in enumerate(grid):
        for j, cell in enumerate(row):
            state = cell
            live_neighbours = count_colored_neighbours(i, j, grid)
            new_grid[i][j] = rules[(state, live_neighbours)]
    return new_grid


if __name__ == "__main__":
    pass
   # print(generate_grid_central(10, 10, 4))
    # grid = generate_grid_random_cells(500, 500, 0.7)
    # print(list(grid.tolist()))
    # for i in range(10):
    #      start = time.time()
    #      grid = update_grid_two_d(grid, game_of_live_rules)
    #      end = time.time()
    #      print(f"Step time: {end - start}")
