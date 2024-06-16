from random import randint
import numpy as np

ant_symbols = ["2"]


def if_zero(grid: np.arange, x: int, y: int, turn: int) -> tuple:
    grid = grid.copy()
    grid[x][y] = {1: 0}
    turn = turn + 1
    return grid, turn


def if_one(grid: np.arange, x: int, y: int, turn: int) -> tuple:
    grid = grid.copy()
    grid[x][y] = {0: 0}
    turn = turn - 1
    return grid, turn


rules = {0: if_zero, 1: if_one}


def generate_grid_ant(
    width: int, height: int, ant_count: int = 1, random_init_turn=False
) -> tuple:
    array = np.full((height, width), {0: 0})
    if ant_count == 1:
        array[width // 2][height // 2] = {0: ant_symbols[0]}
    else:
        for i in range(2, ant_count + 2):
            x = randint(0, width - 1)
            y = randint(0, height - 1)
            ant_symbols.append(str(i))
            array[x][y] = {0: str(i)}

    turns = {
        ant_symbol: randint(1, 4) if random_init_turn else 1
        for ant_symbol in ant_symbols
    }
    return array, turns


def update_grid_ant(grid: np.ndarray, turns: dict, rules: dict = rules) -> tuple:
    positions = []

    for i, row in enumerate(grid):
        for j, value in enumerate(row):
            if list(value.values())[0] in ant_symbols:
                positions.append((i, j, list(value.values())[0]))

    for position in positions:
        x, y, ant_symbol = position
        turn = turns[ant_symbol]
        rule_key = int(list(grid[x][y].keys())[0])
        grid, turn = rules[rule_key](grid, x, y, turn)

        if turn == 5:
            turn = 1
        if turn == 0:
            turn = 4

        turns[ant_symbol] = turn
        width = grid.shape[0]
        height = grid.shape[1]
        if turn == 1:
            grid[(x + 1) % width][y] = {
                list(grid[(x + 1) % width][y].keys())[0]: ant_symbol
            }
        if turn == 2:
            grid[x][(y - 1) % height] = {
                list(grid[x][(y - 1) % height].keys())[0]: ant_symbol
            }
        if turn == 3:
            grid[(x - 1) % width][y] = {
                list(grid[(x - 1) % width][y].keys())[0]: ant_symbol
            }
        if turn == 4:
            grid[x][(y + 1) % height] = {
                list(grid[x][(y + 1) % height].keys())[0]: ant_symbol
            }

    return grid, turns


if __name__ == "__main__":
    grid, turns = generate_grid_ant(5, 5, 3)
    for line in grid:
        print(line)
    for i in range(10):
        grid, turns = update_grid_ant(grid, turns)
        print("*" * 10)
        for line in grid:
            print(line)
