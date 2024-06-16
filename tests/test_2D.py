from collections import namedtuple

import numpy as np
import pytest

from _2D.python.general_2d_automata import (
    count_colored_neighbours,
    update_grid_two_d,
    rules,
)

count_colored_neighbours_test_tuple = namedtuple(
    "count_colored_neighbours_test", "x, y, grid, result"
)

count_colored_neighbours_tests = [
    count_colored_neighbours_test_tuple(
        1, 1, np.array([[1, 1, 0, 0], [0, 0, 1, 0], [0, 1, 0, 0]]), 4
    ),
    count_colored_neighbours_test_tuple(
        1, 1, np.array([[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]), 8
    ),
    count_colored_neighbours_test_tuple(
        0, 0, np.array([[1, 1, 0, 0], [1, 0, 0, 1], [0, 0, 1, 0]]), 2
    ),
    count_colored_neighbours_test_tuple(
        1, 2, np.array([[1, 1, 0, 0], [1, 0, 0, 1], [0, 0, 1, 0]]), 3
    ),
    count_colored_neighbours_test_tuple(
        1, 3, np.array([[1, 1, 0, 0], [1, 0, 0, 1], [0, 0, 1, 0]]), 1
    ),
    count_colored_neighbours_test_tuple(
        2, 3, np.array([[1, 1, 0, 0], [1, 0, 0, 1], [0, 0, 1, 0]]), 2
    ),
]


@pytest.mark.parametrize(
    "count_colored_neighbours_test", count_colored_neighbours_tests
)
def test_count_colored_neighbours(
    count_colored_neighbours_test: count_colored_neighbours_test_tuple,
):
    result = count_colored_neighbours(
        count_colored_neighbours_test.x,
        count_colored_neighbours_test.y,
        count_colored_neighbours_test.grid,
    )
    assert result == count_colored_neighbours_test.result


def test_update_grid_two_d():
    new_grid = update_grid_two_d(
        np.array([[1, 1, 0, 0], [1, 0, 0, 1], [0, 0, 1, 0]]), rules["game_of_life"]
    )
    expected_grid = np.array([[1, 1, 0, 0], [1, 0, 1, 0], [0, 0, 0, 0]])

    np.testing.assert_array_equal(new_grid, expected_grid)
