import tkinter
from collections import defaultdict
from doctest import master
import numpy as np
from _2D.python.general_2d_automata import generate_grid_random_cells, update_grid_two_d


# TODO use pyglet to increase performance
class CellularAutomata2DVisualization:
    def __init__(self, rules: defaultdict, width: int = 1085, height: int = 1085, cell_size: int = 10,
                 probability_of_one: float = 0.7, init_grid=None):
        self.top = tkinter.Tk()
        self.top_frame = tkinter.Frame()
        self.button_frame = tkinter.Frame()

        self.width = width
        self.height = height

        self.canvas = tkinter.Canvas(master, width=self.width, height=self.height)
        self.button_play = tkinter.Button(master, text="Play", command=self.play_call_back)

        self.cell_size = cell_size

        self.prev_step = np.full((height // cell_size, width // cell_size), -1)

        self.probability_of_one = probability_of_one

        self.cells = defaultdict(lambda: (-1, -1), {})
        self.step = 1
        self.rules = rules
        if not init_grid:
            self.grid = generate_grid_random_cells(self.width // self.cell_size, self.height // self.cell_size,
                                                   self.probability_of_one)
        else:
            self.grid = init_grid

    def rectangle_coordinates(self, x: int, y: int) -> dict:
        dic = {'x1': x, 'y1': y, 'x2': self.cell_size + x, 'y2': self.cell_size + y}
        return dic

    def step_call_back(self):
        colours_rules = {
            0: "blue",
            1: "red",
        }
        x = y = 0

        for row, row_prev in zip(self.grid, self.prev_step):
            for value, value_prev in zip(row, row_prev):
                coordinate = self.rectangle_coordinates(x, y)
                if value != value_prev:

                    if self.cells[(x, y)] != (-1, -1):
                        self.canvas.delete(self.cells[(x, y)])

                    colour = colours_rules[value]
                    rectangle = self.canvas.create_rectangle(coordinate['x1'],
                                                             coordinate['y1'],
                                                             coordinate['x2'],
                                                             coordinate['y2'],
                                                             fill=colour)
                    self.cells[(x, y)] = rectangle
                y = coordinate['y2']
            x = coordinate['x2']
            y = 0
        self.prev_step = [[value for value in row] for row in self.grid]
        self.grid = update_grid_two_d(self.grid, rules=self.rules)

    def play_call_back(self):
        while 1:
            self.step_call_back()
            self.top.update()
            # print(f"step: {self.step}")
            self.step += 1

    def main_loop(self):

        self.top_frame.pack(side="top", fill="both", expand=True)
        self.button_frame.pack(side="bottom", fill="both")

        self.button_play.pack(in_=self.top_frame, side="left")
        self.canvas.pack(in_=self.button_frame)

        self.top.mainloop()
