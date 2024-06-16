import os
import tkinter
from doctest import master
from os import path, mkdir
from random import randrange
import numpy as np
from typing import Iterable

from PIL import Image

from _1D.python.cellular_automata import (
    generate_rule,
    cellular_automata_step_1d,
    generate_random,
)


def count_rules(neighborhood_size: int, color_count: int) -> int:
    return 2 ** (color_count**neighborhood_size)


def take_screenshot(folder: str, file_name: str, canvas: tkinter.Canvas):
    if not path.isdir(folder):
        mkdir(folder)
    if not path.isfile(path.join(folder, f"{file_name}.png")):
        canvas.postscript(file=f"{file_name}.eps")
        img = Image.open(f"{file_name}.eps")
        img.save(path.join(folder, f"{file_name}.png"), "png")


class GUI:
    def __init__(self, width: int = 1920, height: int = 1080, cell_size: int = 5):
        self.top = tkinter.Tk()
        self.top_frame = tkinter.Frame()
        self.button_frame = tkinter.Frame()

        self.width = width
        self.height = height

        self.canvas = tkinter.Canvas(master, width=self.width, height=self.height)

        self.button_step = tkinter.Button(
            master, text="Step", command=self.step_call_back
        )
        self.button_play = tkinter.Button(
            master, text="Play", command=self.play_call_back
        )

        self.button_init_random = tkinter.Button(
            master, text="Init random", command=self.init_random_call_back
        )
        self.button_init_one = tkinter.Button(
            master, text="Init one", command=self.init_one_call_back
        )

        self.button_clear = tkinter.Button(
            master, text="Clear", command=self.clear_call_back
        )

        self.button_play_all = tkinter.Button(
            master, text="Play all rules", command=self.play_all_rules_call_back
        )
        self.button_play_all_random = tkinter.Button(
            master,
            text="Play all rules random",
            command=self.play_all_rules_random_call_back,
        )

        self.wolfram_rule_number = tkinter.Entry(master)
        self.wolfram_rule_number.insert(0, "90")

        self.neighborhood_size = tkinter.Entry(master)
        self.neighborhood_size.insert(0, "3")

        self.color_count = tkinter.Entry(master)
        self.color_count.insert(0, "2")

        self.labelText = tkinter.StringVar(master)
        self.rules_count = tkinter.Label(master, textvariable=self.labelText)

        self.cell_size = cell_size

        self.x = 0
        self.y = 0

        self.cells = []
        self.init_way = "random"
        self.silent = False
        self.weights = [0.7, 0.3]
        self.replay = False

    def rectangle_coordinates(self, x: int, y: int) -> dict:
        dic = {"x1": x, "y1": y, "x2": self.cell_size + x, "y2": self.cell_size + y}
        return dic

    def init(self):
        self.rule = generate_rule(
            int(self.wolfram_rule_number.get()),
            int(self.neighborhood_size.get()),
            [i for i in range(int(self.color_count.get()))],
        )
        self.labelText.set(
            f"Possible Rules: {str(count_rules(int(self.neighborhood_size.get()), int(self.color_count.get())))}"
        )

    def random_init_list(self):
        return generate_random(
            input_list=tuple(i for i in range(int(self.color_count.get()))),
            length=self.width // self.cell_size,
        )

    def one_cell_start(self) -> np.ndarray:
        input_list = np.full((self.width // self.cell_size, 1), 0)
        input_list[len(input_list) // 2] = 1
        return input_list

    def init_random_call_back(self):
        self.init()
        self.input_list = self.random_init_list()
        self.init_way = "random_start"

    def init_one_call_back(self):
        self.init()
        self.input_list = self.one_cell_start()
        self.init_way = "one_cell_start"

    def step_call_back(self):
        self.x = 0
        colours = {
            0: "blue",
            1: "red",
            2: "green",
            3: "black",
            4: "white",
            5: "gold",
            6: "purple",
        }

        for i in self.input_list:
            coordinate = self.rectangle_coordinates(self.x, self.y)
            colour = colours[int(i)]
            rectangle = self.canvas.create_rectangle(
                coordinate["x1"],
                coordinate["y1"],
                coordinate["x2"],
                coordinate["y2"],
                fill=colour,
            )

            self.cells.append(rectangle)
            self.x += self.cell_size
        self.input_list = cellular_automata_step_1d(self.input_list, self.rule)
        self.y += self.cell_size

    def play_call_back(self):
        for i in range(self.height // self.cell_size):
            self.step_call_back()
            if not self.silent:
                self.top.update()

    def clear_call_back(self):
        self.x = 0
        self.y = 0
        for rectangle in self.cells:
            self.canvas.delete(rectangle)

    def play_all_rules_call_back(self):
        ordered = range(
            count_rules(int(self.neighborhood_size.get()), int(self.color_count.get()))
        )
        self.play_all(ordered)

    def play_all_rules_random_call_back(self):
        count = count_rules(
            int(self.neighborhood_size.get()), int(self.color_count.get())
        )

        def generator():
            for i in range(count):
                yield randrange(0, count)

        self.play_all(generator())

    def play_all(self, rules_iter: Iterable):
        self.silent = False

        # folder = f"1d_neighborhood_size_{self.neighborhood_size.get()}_colours_{self.color_count.get()}"
        # if not path.isdir(folder):
        #     mkdir(folder)
        # files_list = os.listdir(folder)

        for rule in rules_iter:
            rule_file = f"rule_{rule}_{self.init_way}"

            # if not self.replay and rule_file + ".png" in files_list:
            #     continue

            self.rule = generate_rule(
                rule,
                int(self.neighborhood_size.get()),
                [i for i in range(int(self.color_count.get()))],
            )

            self.wolfram_rule_number.delete(0, tkinter.END)
            self.wolfram_rule_number.insert(0, str(rule))

            self.play_call_back()
            self.top.update()

            # take_screenshot(folder,
            #                 rule_file,
            #                 self.canvas)

            self.clear_call_back()

            self.input_list = (
                self.random_init_list()
                if self.init_way == "random_start"
                else self.one_cell_start()
            )

    def main_loop(self):

        self.top_frame.pack(side="top", fill="both", expand=True)
        self.button_frame.pack(side="bottom", fill="both")

        self.button_step.pack(in_=self.top_frame, side="left")
        self.button_play.pack(in_=self.top_frame, side="left")
        self.button_init_random.pack(in_=self.top_frame, side="left")
        self.button_init_one.pack(in_=self.top_frame, side="left")
        self.button_clear.pack(in_=self.top_frame, side="left")
        self.button_play_all.pack(in_=self.top_frame, side="left")
        self.button_play_all_random.pack(in_=self.top_frame, side="left")

        self.wolfram_rule_number.pack(in_=self.top_frame, side="left")
        self.neighborhood_size.pack(in_=self.top_frame, side="left")
        self.color_count.pack(in_=self.top_frame, side="left")
        self.rules_count.pack(in_=self.top_frame, side="left")

        self.canvas.pack(in_=self.button_frame)

        self.top.mainloop()


def main():
    ui = GUI(width=1920, height=1080, cell_size=3)
    ui.main_loop()


if __name__ == "__main__":
    main()
