from _2D.general_visualization import CellularAutomata2DVisualization
from _2D.general_2d_automata import _34_live_rules
if __name__ == "__main__":
    CellularAutomata2DVisualization(_34_live_rules, probability_of_one=0.1).main_loop()
