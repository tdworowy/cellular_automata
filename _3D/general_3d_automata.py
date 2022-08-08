import itertools
from random import shuffle, choice


def count_neighbours(cords: tuple, grid: dict) -> int:
    neighbours = 0
    x, y, z = cords[0], cords[1], cords[2]
    try:
        for i in range(x - 1, x + 2):
            for j in range(y - 1, y + 2):
                for k in range(z - 1, z + 2):
                    if grid[(i, j, k)] == 1 and (i, j, k) != (x, y, z): neighbours += 1
    except KeyError:
        pass
    return neighbours


def generate_random_grid(cell_range: tuple = (-20, 20)) -> dict:
    grid = [x for x in range(*cell_range)]
    grid = list(itertools.product(grid, repeat=3))
    shuffle(grid)
    return {cords: choice([0, 1]) for cords in grid}


def generate_grid_center_cell_start(cell_range: tuple = (-20, 20)) -> dict:
    grid = [x for x in range(*cell_range)]
    grid = list(itertools.product(grid, repeat=3))
    return {cords: 1 if cords == (0, 0, 0) else 0 for cords in grid}


def update_grid(grid: dict, rules: dict) -> dict:
    new_grid = {}
    for key in grid:
        neighbours = count_neighbours(key, grid)
        state = grid[key]
        new_grid[key] = rules[(state, neighbours)]
    return new_grid
