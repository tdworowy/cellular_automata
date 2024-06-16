from _2D.python.general_2d_automata import rules
from _2D.python.vizuzlizations.general_visualization import (
    CellularAutomata2DVisualization,
)
from _2D.python.vizuzlizations.langton_ant_visualization import LangtonAnt
from _2D.python.vizuzlizations.snowflake_visualization import SnowflakeVisualization

if __name__ == "__main__":
    visualizations = {i: rule for i, rule in enumerate(rules.keys())}
    visualizations[max(list(visualizations.keys())) + 1] = "snowflake"
    visualizations[max(list(visualizations.keys())) + 1] = "langton_ant"
    for ele in visualizations.items():
        print(ele)
    number = int(input("Enter visualization number: "))

    if visualizations[number] == "snowflake":
        SnowflakeVisualization(width=1085, height=1085, cell_size=5).main_loop()
    if visualizations[number] == "langton_ant":
        LangtonAnt(width=1085, height=1085, cell_size=5).main_loop()
    else:
        CellularAutomata2DVisualization(
            rules=rules[visualizations[number]], width=1085, height=1085, cell_size=5
        ).main_loop()
