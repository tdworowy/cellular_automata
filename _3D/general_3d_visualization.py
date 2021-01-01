from collections import defaultdict
from vpython import box, vector, color

from time import sleep

from _3D import _3d_rules
from _3D.general_3d_automata import generate_grid_center_cell_start, update_grid


class CellularAutomataCDVisualization:
    def __init__(self, side=40, thk=0.1):
        self.side = side
        self.thk = thk
        self.s2 = 2 * self.side - self.thk
        self.s3 = 2 * self.side + self.thk
        self.cells = defaultdict(lambda: -1, {})

    def generate_stage(self):

        wall_r = box(pos=vector(self.side, 0, 0), size=vector(self.thk, self.s2, self.s3), color=color.blue)
        wall_l = box(pos=vector(-self.side, 0, 0), size=vector(self.thk, self.s2, self.s3), color=color.blue)
        wall_g = box(pos=vector(0, -self.side, 0), size=vector(self.s3, self.thk, self.s3), color=color.blue)
        wall_t = box(pos=vector(0, self.side, 0), size=vector(self.s3, self.thk, self.s3), color=color.blue)
        wall_bk = box(pos=vector(0, 0, -self.side), size=vector(self.s2, self.s2, self.thk), color=color.blue)

    def generate_cells(self, grid: dict):
        for key in grid.keys():
            if grid[key]:
                cell = box(pos=vector(*key), size=vector(1, 1, 1), color=color.red)
                self.cells[key] = cell
            elif self.cells[key] != -1:
                self.cells[key].visible = False
                del self.cells[key]


if __name__ == "__main__":

    grid = generate_grid_center_cell_start()
    rules = _3d_rules.rule_3#_3d_rules.rule_1  # random_rule()
    print(rules)
    visualization_3d = CellularAutomataCDVisualization()
    visualization_3d.generate_stage()
    while 1:
        visualization_3d.generate_cells(grid)
        grid = update_grid(grid, rules)
        sleep(0.5)
