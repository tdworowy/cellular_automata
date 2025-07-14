const formEl = document.getElementById("initGrid");
const formE2 = document.getElementById("step");
const play_button = document.getElementById("play");

const formAnt = document.getElementById("addAnt");

let grid;
let prev_grid;

let requestId = undefined;

const Directions = {
  Up: "Up",
  Down: "Down",
  Left: "Left",
  Right: "Right",
};

const standard_ant_rules = {
  "0_Up": {
    new_cell: 1,
    new_direction: Directions.Right,
  },
  "0_Down": {
    new_cell: 1,
    new_direction: Directions.Left,
  },
  "0_Right": {
    new_cell: 1,
    new_direction: Directions.Down,
  },
  "0_Left": {
    new_cell: 1,
    new_direction: Directions.Up,
  },
  "1_Up": {
    new_cell: 0,
    new_direction: Directions.Left,
  },
  "1_Down": {
    new_cell: 0,
    new_direction: Directions.Right,
  },
  "1_Right": {
    new_cell: 0,
    new_direction: Directions.Up,
  },
  "1_Left": {
    new_cell: 0,
    new_direction: Directions.Down,
  },
};

function get_random(list) {
  return list[Math.floor(Math.random() * list.length)];
}

function generate_array(n, except) {
  arr = [...Array(n).keys()];
  arr.filter((elem) => !elem in except);
  return arr;
}

function generate_random_rules(count) {
  let rules = {};
  const directions = ["Up", "Down", "Left", "Right"];
  let used_states = [];
  for (let i = 0; i <= count; i++) {
    new_cell = get_random(generate_array(count, [i, ...[used_states]]));
    used_states.push(new_cell);
  
    directions.forEach((direction) => {
      rules[`${i.toString()}_${direction}`] = {
        new_cell: new_cell,
        new_direction: get_random(directions),
      };
    });
  }
  return rules;
}

let ants = [];

function addAnt(event) {
  event.preventDefault();

  const params = new FormData(document.querySelector("#addAnt"));
  const ant_x = Number(params.get("ant_x"));
  const ant_y = Number(params.get("ant_y"));
  const ant_direction = params.get("ant_direction");

  ants.push({ x: ant_x, y: ant_y, direction: ant_direction });
}

function move(direction, curentValue, maxValue) {
  switch (direction) {
    case Directions.Up:
    case Directions.Left:
      return (curentValue + 1) % maxValue;
    case Directions.Right:
    case Directions.Down:
      return curentValue - 1 < 0 ? maxValue - 1 : curentValue - 1;
  }
}

function checkRules(grid, ants, rules) {
  ants.forEach((ant) => {
    result = rules[grid[ant.x][ant.y].toString() + "_" + ant.direction];
    grid[ant.x][ant.y] = result.new_cell;
    ant.direction = result.new_direction;
    switch (result.new_direction) {
      case Directions.Up:
      case Directions.Down:
        ant.y = move(ant.direction, ant.y, grid.length);
        break;
      case Directions.Left:
      case Directions.Right:
        ant.x = move(ant.direction, ant.x, grid.length);
        break;
    }
  });
  return [grid, ants];
}

// function checkRules_old(grid, ants) {
//   ants.forEach((ant) => {
//     if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Up) {
//       grid[ant.x][ant.y] = 1;
//       ant.direction = Directions.Right;
//       ant.x = move(ant.direction, ant.x, grid.length)
//     }
//     if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Down) {
//       grid[ant.x][ant.y] = 1;
//       ant.direction = Directions.Left;
//        ant.x = move(ant.direction, ant.x, grid.length)
//     }
//     if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Right) {
//       grid[ant.x][ant.y] = 1;
//       ant.direction = Directions.Down;
//       ant.y = move(ant.direction, ant.y, grid.length)
//     }
//     if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Left) {
//       grid[ant.x][ant.y] = 1;
//       ant.direction = Directions.Up;
//       ant.y = move(ant.direction, ant.y, grid.length)
//     }
//     if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Up) {
//       grid[ant.x][ant.y] = 0;
//       ant.direction = Directions.Left;
//       ant.x = move(ant.direction, ant.x, grid.length)
//     }
//     if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Down) {
//       grid[ant.x][ant.y] = 0;
//       ant.direction = Directions.Right;
//       ant.x = move(ant.direction, ant.x, grid.length)
//     }
//     if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Right) {
//       grid[ant.x][ant.y] = 0;
//       ant.direction = Directions.Up;
//       ant.y = move(ant.direction, ant.y, grid.length)
//     }
//     if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Left) {
//       grid[ant.x][ant.y] = 0;
//       ant.direction = Directions.Down;
//       ant.y = move(ant.direction, ant.y, grid.length)
//     }
//   });
//   return [grid, ants];
// }

function generateGrid(height, width) {
  let grid = [];
  for (let i = 0; i < height; i++) {
    let row = [];
    for (let j = 0; j < width; j++) {
      row.push(0);
    }
    grid.push(row);
  }
  return grid;
}

function initGrid(event) {
  event.preventDefault();

  const params = new FormData(document.querySelector("#initGrid"));
  const cell_width = Number(params.get("cell_width"));
  const cell_height = Number(params.get("cell_height"));

  grid = generateGrid(params.get("width"), params.get("height"));

  document.getElementById("canvas").width = params.get("width") * cell_width;
  document.getElementById("canvas").height = params.get("height") * cell_height;
  renderGrid(grid, null, cell_width, cell_height);
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
  const color_count = Number(params_init_grid.get("colors_count"));

  const params = new FormData(document.querySelector("#step"));
  grid = JSON.parse(JSON.stringify(prev_grid));
  rules =
    color_count == 2 ? standard_ant_rules : generate_random_rules(color_count);
  const [new_grid, new_ants] = checkRules(grid, ants, rules);
  ants = new_ants;
  renderGrid(new_grid, prev_grid, cell_width, cell_height);
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

function renderGrid(grid, old_grid, cell_width, cell_height) {
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

formAnt.addEventListener("submit", addAnt);
