import tkinter
from doctest import master

from _2D.python.general_2d_automata import update_grid_two_d, \
    generate_snowflake_rule, generate_grid_central
from _2D.python.vizuzlizations.general_visualization import CellularAutomata2DVisualization


class SnowflakeVisualization(CellularAutomata2DVisualization):
    def __init__(self, width: int = 1085, height: int = 1085, cell_size: int = 4):

        super().__init__(None, width=width, height=height, cell_size=cell_size)
        self.neighbours_number = tkinter.Entry(master)
        self.neighbours_number.insert(0, "1")  # other "1,5","1,3,5", "1,3", "1"

        self.ini_cell_count = tkinter.Entry(master)
        self.ini_cell_count.insert(0, "1")

        self.cell_size = cell_size

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
                    colour = colours_rules[value]
                    self.canvas.create_rectangle(coordinate['x1'],
                                                 coordinate['y1'],
                                                 coordinate['x2'],
                                                 coordinate['y2'],
                                                 fill=colour)

                y = coordinate['y2']
            x = coordinate['x2']
            y = 0
        self.prev_step = self.grid.copy()
        neighbours_number = [int(number) for number in self.neighbours_number.get().split(",")]
        self.grid = update_grid_two_d(self.grid, rules=generate_snowflake_rule(neighbours_number))

    def play_call_back(self):
        self.grid = generate_grid_central(self.width // self.cell_size,
                                          self.height // self.cell_size,
                                          int(self.ini_cell_count.get()))

        while 1:
            self.step_call_back()
            self.top.update()
            # print(f"step:{self.step}")
            self.step += 1

    def main_loop(self):

        self.top_frame.pack(side="top", fill="both", expand=True)
        self.button_frame.pack(side="bottom", fill="both")

        self.button_play.pack(in_=self.top_frame, side="left")
        self.neighbours_number.pack(in_=self.top_frame, side="left")
        self.ini_cell_count.pack(in_=self.top_frame, side="left")

        self.canvas.pack(in_=self.button_frame)

        self.top.mainloop()


def snowflake():
    SnowflakeVisualization().main_loop()


if __name__ == "__main__":
    snowflake()
