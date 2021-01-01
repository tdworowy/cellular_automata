from Playground.Complexity.cellular_automata._2D.general_2d_automata import _2x2_rules
from Playground.Complexity.cellular_automata._2D.general_visualization import CellularAutomata2DVisualization

if __name__ == "__main__":
    CellularAutomata2DVisualization(_2x2_rules, probability_of_one=0.1).main_loop()
