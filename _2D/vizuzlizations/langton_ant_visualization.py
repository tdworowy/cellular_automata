import tkinter
from collections import defaultdict
from doctest import master

import numpy as np

from _2D.langton_ant import generate_grid_ant, update_grid_ant


class LangtonAnt:
    def __init__(self, width: int = 1085, height: int = 1085, cell_size: int = 5):
        self.top = tkinter.Tk()
        self.top_frame = tkinter.Frame()
        self.button_frame = tkinter.Frame()

        self.width = width
        self.height = height

        self.canvas = tkinter.Canvas(master, width=self.width, height=self.height)
        self.button_play = tkinter.Button(master, text="Play", command=self.play_call_back)

        self.ants_count = tkinter.Entry(master)
        self.ants_count.insert(0, "3")

        self.random_init_turn = tkinter.Entry(master)
        self.random_init_turn.insert(0, "1")

        self.labelText = tkinter.StringVar(master)
        self.rules_count = tkinter.Label(master, textvariable=self.labelText)

        self.cell_size = cell_size

        self.prev_step = np.full((height // cell_size, width // cell_size), -1)

        self.cells = defaultdict(lambda: (-1, -1), {})
        self.step = 1

    def rectangle_coordinates(self, x: int, y: int) -> dict:
        dic = {'x': x, 'y': y, 'x1': self.cell_size + x, 'y1': self.cell_size + y}
        return dic

    def step_call_back(self):
        colours_rules = {
            (0, 0): "blue",
            (1, 0): "red",
        }
        colours_rules = defaultdict(lambda: "black", colours_rules)
        x = y = 0

        for row, row_prev in zip(self.grid, self.prev_step):
            for value, value_prev in zip(row, row_prev):
                coordinate = self.rectangle_coordinates(x, y)
                if value != value_prev:

                    if self.cells[(x, y)] != (-1, -1):
                        self.canvas.delete(self.cells[(x, y)])

                    colour = colours_rules[(list(value.keys())[0], list(value.values())[0])]
                    rectangle = self.canvas.create_rectangle(coordinate['x'],
                                                             coordinate['y'],
                                                             coordinate['x1'],
                                                             coordinate['y1'],
                                                             fill=colour)
                    self.cells[(x, y)] = rectangle

                y = coordinate['y1']
            x = coordinate['x1']
            y = 0
        self.prev_step = self.grid.copy()
        self.grid, self.turn = update_grid_ant(self.grid, self.turn)

    def play_call_back(self):
        self.grid, self.turn = generate_grid_ant(self.width // self.cell_size,
                                                 self.height // self.cell_size,
                                                 int(self.ants_count.get()),
                                                 bool(int(self.random_init_turn.get())))

        while 1:
            self.step_call_back()
            #  print(f"step: {self.step}")
            self.top.update()
            self.step += 1

    def main_loop(self):

        self.top_frame.pack(side="top", fill="both", expand=True)
        self.button_frame.pack(side="bottom", fill="both")

        self.button_play.pack(in_=self.top_frame, side="left")
        self.rules_count.pack(in_=self.top_frame, side="left")
        self.ants_count.pack(in_=self.top_frame, side="left")
        self.random_init_turn.pack(in_=self.top_frame, side="left")

        self.canvas.pack(in_=self.button_frame)

        self.top.mainloop()


def langton_ant():
    ui = LangtonAnt()
    ui.main_loop()


if __name__ == "__main__":
    langton_ant()
