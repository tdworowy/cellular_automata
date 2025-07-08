const formEl = document.getElementById("initGrid");
const formE2 = document.getElementById("step");
const formE3 = document.getElementById("add");
const play_button = document.getElementById("play");

let grid;
let prev_grid;

const game_of_live_rules = {
  "0_3": 1,
  "1_3": 1,
  "1_2": 1,
};
const mazectric_rules = {
  "0_3": 1,
  "1_1": 1,
  "1_2": 1,
  "1_3": 1,
  "1_4": 1,
};
const amoeba_rules = {
  "0_3": 1,
  "0_5": 1,
  "0_7": 1,
  "1_1": 1,
  "1_3": 1,
  "1_5": 1,
  "1_8": 1,
};
const _2x2_rules = {
  "0_3": 1,
  "0_6": 1,
  "1_1": 1,
  "1_2": 1,
  "1_5": 1,
};
const _34_live_rules = {
  "0_3": 1,
  "0_4": 1,
  "1_3": 1,
  "1_4": 1,
};
const coagulations_rules = {
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
const move_rules = {
  "0_3": 1,
  "0_6": 1,
  "0_8": 1,
  "1_2": 1,
  "1_4": 1,
  "1_5": 1,
};
const walled_cities_rules = {
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
const epileptic_rules = {
  "0_0": 1,
  "0_2": 1,
};

const pattentrs = {
  glider: [
    [0, 0, 1],
    [1, 0, 1],
    [0, 1, 1],
  ],
  eater1: [
    [1, 1, 0, 0],
    [1, 0, 1, 0],
    [0, 0, 1, 0],
    [0, 0, 1, 1],
  ],
  glider_generator: [
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
  ],

  glider_gun: [
    [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
  ],
  pentadecathlon: [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ],
  queen_bee: [
    [1, 0, 0, 0, 0],
    [1, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 0],
    [1, 0, 0, 0, 0],
  ],
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
    const first = Math.floor(Math.random() * 2);
    const second = Math.floor(Math.random() * 9);
    rules[`${first}_${second}`] = 1;
  }
  return rules;
}

const rules = {
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
  const cell_width = Number(params.get("cell_width"));
  const cell_height = Number(params.get("cell_height"));

  if (params.get("grid_type") === "random") {
    grid = generateGridRandom(
      params.get("width"),
      params.get("height"),
      params.get("one_prob")
    );
  }
  if (params.get("grid_type") === "center") {
    params.set("cell_count", "1");
    grid = generateGridCenter(
      params.get("width"),
      params.get("height"),
      cell_width,
      cell_height
    );
  }
  event.preventDefault();
  document.getElementById("canvas").width = params.get("width") * cell_width;
  document.getElementById("canvas").height = params.get("height") * cell_height;
  generateGrid(grid, null, cell_width, cell_height);
}

function add(event) {
  event.preventDefault();
  const params = new FormData(document.querySelector("#add"));
  addPattern(params.get("pattern"));
}

function step() {
  const params_init_grid = new FormData(document.querySelector("#initGrid"));
  const cell_width = Number(params_init_grid.get("cell_width"));
  const cell_height = Number(params_init_grid.get("cell_height"));

  const params = new FormData(document.querySelector("#step"));
  //console.log( rules[params.get("rule")])
  grid = JSON.parse(JSON.stringify(prev_grid));
  const new_grid = updateGrid(
    grid,
    grid.length,
    grid[0].length,
    rules[params.get("rule")]
  );
  generateGrid(new_grid, prev_grid, cell_width, cell_height);
}
function step_event(event) {
  event.preventDefault();
  step();
}

function step_play() {
  step();
  requestId = window.requestAnimationFrame(step_play);
}

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

function generateGrid(grid, old_grid, cell_width, cell_height) {
  let x_cor = 0;
  let y_cor = 0;

  for (var x = 0; x < grid.length; x++) {
    for (var y = 0; y < grid[x].length; y++) {
      if (!old_grid || old_grid[x][y] !== grid[x][y]) {
        context.fillStyle = colors[grid[x][y]];
        context.fillRect(x_cor, y_cor, cell_width, cell_height);
      } 
      x_cor = x * cell_width;
      y_cor = y * cell_width;
    }
  }
  prev_grid = JSON.parse(JSON.stringify(grid));
}

// canvas.onclick = (event) => {
//   const x = Math.round(event.offsetX / call_width);
//   const y = Math.round(event.offsetY / call_height);

//   const x_cord = event.offsetX;
//   const y_cord = event.offsetY;

//   context.fillStyle = colors[1];
//   // TODO make it align to grid
//   // TODO is seems to not update grid
//   grid[x][y] = 1;
//   context.fillRect(x_cord, y_cord, call_width, call_height);
// };

function addPattern(pattern_key) {
  const pattern = pattentrs[pattern_key];
  const pattern_height = pattern.length;
  const pattern_width = pattern[0].length;

  let grid = JSON.parse(JSON.stringify(prev_grid));

  const start_y = Math.floor(
    Math.random() * grid[0].length - pattern_width + 1
  );
  const start_x = Math.floor(Math.random() * grid.length - pattern_height + 1);

  const end_x = start_x + pattern_height;
  const end_y = start_y + pattern_width;

  let i = 0;
  let j = 0;

  for (let x = start_x; x < end_x; x++) {
    j = 0;
    for (let y = start_y; y < end_y; y++) {
      grid[y][x] = pattern[i][j];
      j++;
    }
    i++;
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
formE3.addEventListener("submit", add);
play_button.addEventListener("click", play);
play_button.addEventListener("click", stop);
