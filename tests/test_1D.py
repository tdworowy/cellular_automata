from collections import namedtuple

from _1D.python.cellular_automata import n_nary, wolfram_number_to_bin
import pytest

n_ary_test_tuple = namedtuple("n_ary_test", "n, number, result")
wolfram_number_to_bin_tuple = namedtuple("wolfram_number_to_bin_test", "wolfram_number, possible_states, "
                                                                       "colours_count, result")

n_ary_tests = [n_ary_test_tuple(10, 10, "10"),
               n_ary_test_tuple(110, 2, "1101110"),
               n_ary_test_tuple(0, 2, "0"),
               n_ary_test_tuple(10, 3, "101")
               ]

wolfram_number_to_bin_tests = [
    wolfram_number_to_bin_tuple(110, 2 ** 3, 2, ['0', '1', '1', '1', '0', '1', '1', '0']),
    wolfram_number_to_bin_tuple(215, 2 ** 3, 2, ['1', '1', '1', '0', '1', '0', '1', '1'])
]


# wolfram_number_to_bin(110, 2 ** 3, 2)

@pytest.mark.parametrize("n_ary_test", n_ary_tests)
def test_n_nary(n_ary_test: n_ary_test_tuple):
    assert n_nary(n_ary_test.n, n_ary_test.number) == n_ary_test.result


@pytest.mark.parametrize("wolfram_number_to_bin_test", wolfram_number_to_bin_tests)
def test_wolfram_number_to_bin(wolfram_number_to_bin_test: wolfram_number_to_bin_tuple):
    assert wolfram_number_to_bin(wolfram_number_to_bin_test.wolfram_number, wolfram_number_to_bin_test.possible_states,
                                 wolfram_number_to_bin_test.colours_count) == wolfram_number_to_bin_test.result
