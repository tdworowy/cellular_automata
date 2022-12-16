const formEl = document.getElementById("initGrid");
const formE2 = document.getElementById("step");
const play_button = document.getElementById("play");

const call_width = config.call_width;
const call_height = config.call_height;
let grid;
let prev_grid;

game_of_live_rules = {
  "0_3": 1,
  "1_3": 1,
  "1_2": 1,
};
mazectric_rules = {
  "0_3": 1,
  "1_1": 1,
  "1_2": 1,
  "1_3": 1,
  "1_4": 1,
};
amoeba_rules = {
  "0_3": 1,
  "0_5": 1,
  "0_7": 1,
  "1_1": 1,
  "1_3": 1,
  "1_5": 1,
  "1_8": 1,
};
_2x2_rules = {
  "0_3": 1,
  "0_6": 1,
  "1_1": 1,
  "1_2": 1,
  "1_5": 1,
};
_34_live_rules = {
  "0_3": 1,
  "0_4": 1,
  "1_3": 1,
  "1_4": 1,
};
coagulations_rules = {
  "0_3": 1,
  "0_7": 1,
  "0_8": 1,
  "1_2": 1,
  "1_3": 1,
  "1_5": 1,
  "1_6": 1,
  "1_7": 1,
  "1_8": 1,
};
move_rules = {
  "0_3": 1,
  "0_6": 1,
  "0_8": 1,
  "1_2": 1,
  "1_4": 1,
  "1_5": 1,
};
walled_cities_rules = {
  "0_4": 1,
  "0_5": 1,
  "0_6": 1,
  "0_7": 1,
  "0_8": 1,
  "1_2": 1,
  "1_3": 1,
  "1_4": 1,
  "1_5": 1,
};
epileptic_rules = {
  "6_6": 1,
  "3_0": 1,
  "6_1": 1,
  "4_2": 1,
  "3_2": 1,
  "3_1": 1,
  "4_5": 1,
  "3_6": 1,
  "0_0": 1,
  "2_4": 1,
  "0_2": 1,
  "6_3": 1,
};

let requestId = undefined;

function generateSnowflakeRule(neighbours_numbers = [1]) {
  let snowflake_rules = {};

  for (let neighbours_number of neighbours_numbers) {
    snowflake_rules["0" + "_" + neighbours_number.toString()] = 1;
    snowflake_rules["1" + "_" + neighbours_number.toString()] = 1;
  }
  for (let i = 0; i < 9; i++)
    if (!neighbours_numbers.includes(i)) {
      snowflake_rules["0" + "_" + i.toString()] = 0;
      snowflake_rules["1" + "_" + i.toString()] = 1;
    }
  return snowflake_rules;
}

function generateRandomRule() {
  const minRuleSize = 5;
  const maxRuleSize = 20;
  const ruleSize =
    Math.floor(Math.random() * (maxRuleSize - minRuleSize + 1)) + minRuleSize;
  let rules = {};
  for (let i = 0; i < ruleSize; i++) {
    const first = Math.floor(Math.random() * 9);
    const second = Math.floor(Math.random() * 9);
    rules[`${first}_${second}`] = 1;
  }
  return rules;
}

rules = {
  game_of_life: game_of_live_rules,
  amoeba: amoeba_rules,
  twoXTwo: _2x2_rules,
  threeFourLive: _34_live_rules,
  coagulations: coagulations_rules,
  mazectric: mazectric_rules,
  move: move_rules,
  walled_cities: walled_cities_rules,
  snowflake_1: generateSnowflakeRule([1]),
  snowflake_1_5: generateSnowflakeRule([1, 5]),
  snowflake_1_3_5: generateSnowflakeRule([1, 3, 5]),
  snowflake_1_3: generateSnowflakeRule([1, 3]),
  epileptic: epileptic_rules,
  random: generateRandomRule(),
};

function generateGridRandom(height, width, prob_of_one) {
  let grid = [];
  for (let i = 0; i < height; i++) {
    let row = [];
    for (let j = 0; j < width; j++) {
      let rand_int = Math.floor(Math.random() * 10 + 1);
      if (rand_int <= prob_of_one * 10) row.push(1);
      else row.push(0);
    }
    grid.push(row);
  }
  return grid;
}

function generateGridCenter(height, width) {
  let grid = [];
  for (let i = 0; i < height; i++) {
    let row = [];
    for (let j = 0; j < width; j++) {
      if (i === height / 2 && j === width / 2) row.push(1);
      else row.push(0);
    }
    grid.push(row);
  }
  return grid;
}

function countColoredNeighbours(x, y, grid_x_axis, grid_y_axis, grid) {
  let colored_neighbours = 0;
  for (
    let i = (((x - 1) % grid_x_axis) + grid_x_axis) % grid_x_axis;
    i < (x + 2) % grid_x_axis;
    i++
  ) {
    for (
      let j = (((y - 1) % grid_y_axis) + grid_y_axis) % grid_y_axis;
      j < (y + 2) % grid_y_axis;
      j++
    ) {
      if (grid[i][j] === 1 && (i != x || j != y)) colored_neighbours += 1;
    }
  }
  return colored_neighbours;
}

function updateGrid(grid, grid_x_axis, grid_y_asix, rules) {
  let new_grid = JSON.parse(JSON.stringify(grid));
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      let state = grid[i][j];
      let live_neighbours = countColoredNeighbours(
        i,
        j,
        grid_x_axis,
        grid_y_asix,
        grid
      );
      let rule = rules[state.toString() + "_" + live_neighbours.toString()];
      if (rule) {
        new_grid[i][j] = rule;
      } else {
        new_grid[i][j] = 0;
      }
    }
  }
  return new_grid;
}

function initGrid(event) {
  const params = new FormData(document.querySelector("#initGrid"));
  if (params.get("grid_type") === "random") {
    grid = generateGridRandom(
      params.get("width"),
      params.get("height"),
      params.get("one_prob")
    );
  }
  if (params.get("grid_type") === "center") {
    params.set("cell_count", "1");
    grid = generateGridCenter(params.get("width"), params.get("height"));
  }
  event.preventDefault();
  document.getElementById("canvas").width = params.get("width") * call_width;
  document.getElementById("canvas").height = params.get("height") * call_height;
  generateGrid(grid);
}

function step() {
  const params = new FormData(document.querySelector("#step"));
  grid = JSON.parse(JSON.stringify(prev_grid));
  new_grid = updateGrid(
    grid,
    grid.length,
    grid[0].length,
    rules[params.get("rule")]
  );
  generateGrid(new_grid);
}
function step_event(event) {
  if (event && event.hasOwnProperty("preventDefault")) {
    event.preventDefault();
  }
  step();
}

function step_play() {
  step();
  requestId = window.requestAnimationFrame(step_play);
}

function generateGrid(grid) {
  const canvas = document.getElementById("canvas");
  const context = canvas.getContext("2d");

  let x_cor = 0;
  let y_cor = 0;

  for (var x = 0; x < grid.length; x++) {
    for (var y = 0; y < grid[x].length; y++) {
      context.fillStyle = colors[grid[x][y]];
      context.fillRect(x_cor, y_cor, call_width, call_height);
      x_cor = x * call_width;
      y_cor = y * call_width;
    }
  }
  prev_grid = JSON.parse(JSON.stringify(grid));
}

function play() {
  if (!requestId) {
    play_button.textContent = "Stop";
    window.requestAnimationFrame(step_play);
  }
}
function stop() {
  if (requestId) {
    play_button.textContent = "Play";
    window.cancelAnimationFrame(requestId);
    requestId = undefined;
  }
}

formEl.addEventListener("submit", initGrid);
formE2.addEventListener("submit", step_event);
play_button.addEventListener("click", play);
play_button.addEventListener("click", stop);
