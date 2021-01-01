from _2D.general_2d_automata import \
    game_of_live_rules

from _2D.general_visualization import CellularAutomata2DVisualization

if __name__ == "__main__":
    CellularAutomata2DVisualization(game_of_live_rules, width=1085, height=1085, cell_size=10).main_loop()
