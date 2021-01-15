from _2D.general_2d_automata import coagulations_rules
from _2D.vizuzlizations.general_visualization import CellularAutomata2DVisualization

if __name__ == "__main__":
    CellularAutomata2DVisualization(coagulations_rules, probability_of_one=0.1).main_loop()
