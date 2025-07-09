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

const ant1 = { x: 150, y: 150, direction: Directions.Up };
const ant2 = { x: 50, y: 50, direction: Directions.Down };
const ant3 = { x: 25, y: 25, direction: Directions.Down };
const ant4 = { x: 10, y: 10, direction: Directions.Down };
let ants = [];

function addAnt(event) {
  event.preventDefault();

  const params = new FormData(document.querySelector("#addAnt"));
  const ant_x = Number(params.get("ant_x"));
  const ant_y = Number(params.get("ant_y"));
  const ant_direction = params.get("ant_direction");

  ants.push({ x: ant_x, y: ant_y, direction: ant_direction });
}

function checkRules(grid, ants) {
  ants.forEach((ant) => {
    if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Up) {
      grid[ant.x][ant.y] = 1;
      ant.direction = Directions.Right;
      ant.x = ant.x - 1 < 0 ? grid.length - 1 : ant.x - 1;
    }
    if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Down) {
      grid[ant.x][ant.y] = 1;
      ant.direction = Directions.Left;
      ant.x = (ant.x + 1) % grid.length;
    }
    if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Right) {
      grid[ant.x][ant.y] = 1;
      ant.direction = Directions.Down;
      ant.y = ant.y - 1 < 0 ? grid.length - 1 : ant.y - 1;
    }
    if (grid[ant.x][ant.y] === 0 && ant.direction === Directions.Left) {
      grid[ant.x][ant.y] = 1;
      ant.direction = Directions.Up;
      ant.y = (ant.y + 1) % grid.length;
    }
    if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Up) {
      grid[ant.x][ant.y] = 0;
      ant.direction = Directions.Left;
      ant.x = (ant.x + 1) % grid.length;
    }
    if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Down) {
      grid[ant.x][ant.y] = 0;
      ant.direction = Directions.Right;
      ant.x = ant.x - 1 < 0 ? grid.length - 1 : ant.x - 1;
    }
    if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Right) {
      grid[ant.x][ant.y] = 0;
      ant.direction = Directions.Up;
      ant.y = (ant.y + 1) % grid.length;
    }
    if (grid[ant.x][ant.y] === 1 && ant.direction === Directions.Left) {
      grid[ant.x][ant.y] = 0;
      ant.direction = Directions.Down;
      ant.y = ant.y - 1 < 0 ? grid.length - 1 : ant.y - 1;
    }
  });
  return [grid, ants];
}

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

  const params = new FormData(document.querySelector("#step"));
  grid = JSON.parse(JSON.stringify(prev_grid));
  const [new_grid, new_ants] = checkRules(grid, ants);
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
