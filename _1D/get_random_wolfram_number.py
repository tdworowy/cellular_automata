from random import randrange


def get_wolfram_number(neighborhood_size: int = 3, color_count: int = 2) -> int:
    return randrange(0, 2 ** (color_count ** neighborhood_size))


if __name__ == "__main__":
    print(get_wolfram_number(neighborhood_size=3, color_count=6))
