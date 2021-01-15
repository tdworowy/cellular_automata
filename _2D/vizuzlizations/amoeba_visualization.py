from _2D.general_2d_automata import amoeba_rules
from _2D.vizuzlizations.general_visualization import CellularAutomata2DVisualization

if __name__ == "__main__":
    CellularAutomata2DVisualization(amoeba_rules, probability_of_one=0.3).main_loop()
