from utils.utils import RoundList
from random import randint

ant_symbols = [2]


def if_zero(grid: list, x: int, y: int, turn: int) -> tuple:
    grid = RoundList([RoundList([[value[0], value[1]] for value in row]) for row in grid])
    grid[x][y] = [1, 0]
    turn = turn + 1
    return grid, turn


def if_one(grid: list, x: int, y: int, turn: int) -> tuple:
    grid = RoundList([RoundList([[value[0], value[1]] for value in row]) for row in grid])
    grid[x][y] = [0, 0]
    turn = turn - 1
    return grid, turn


rules = {
    0: if_zero,
    1: if_one
}


def generate_grid(width: int, height: int, ant_count: int = 1, random_init_turn=False) -> tuple:
    array = RoundList([RoundList([[0, 0] for _ in range(width)]) for _ in range(height)])
    if ant_count == 1:
        array[width // 2][height // 2] = [0, ant_symbols[0]]
    else:
        for i in range(2, ant_count + 2):
            x = randint(0, width)
            y = randint(0, height)
            ant_symbols.append(i)
            array[x][y] = [0, i]

    turns = {ant_symbol: randint(1, 4) if random_init_turn else 1 for ant_symbol in ant_symbols}
    return array, turns


def update_grid(grid: list, turns: dict, rules: dict = rules) -> tuple:
    grid = RoundList([RoundList([[value[0], value[1]] for value in row]) for row in grid])
    positions = []

    for i, row in enumerate(grid):
        for j, value in enumerate(row):
            if value[1] in ant_symbols:
                positions.append((i, j, value[1]))

    for position in positions:
        x, y, ant_symbol = position
        grid, turn = rules[grid[x][y][0]](grid, x, y, turns[ant_symbol])

        if turn == 5:
            turn = 1
        if turn == 0:
            turn = 4

        turns[ant_symbol] = turn

        if turn == 1:
            grid[x + 1][y][1] = ant_symbol
        if turn == 2:
            grid[x][y - 1][1] = ant_symbol
        if turn == 3:
            grid[x - 1][y][1] = ant_symbol
        if turn == 4:
            grid[x][y + 1][1] = ant_symbol

    return grid, turns


if __name__ == "__main__":
    grid, turns = generate_grid(5, 5, 3)
    for line in grid:
        print(line)
    for i in range(10):
        grid, turns = update_grid(grid, turns)
        print("*" * 10)
        for line in grid:
            print(line)
