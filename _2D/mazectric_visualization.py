from _2D.general_2d_automata import mazectric_rules
from _2D.general_visualization import CellularAutomata2DVisualization

if __name__ == "__main__":
    CellularAutomata2DVisualization(mazectric_rules, probability_of_one=0.1).main_loop()
