const formEl = document.getElementById("initGrid");
const formE2 = document.getElementById("step");

const call_width = config.call_width;
const call_hight = config.call_hight;
let y = 0;

let hight = 10000; //TODO make canvas with dynamic height

function initGrid(event) {
  const params = new FormData(document.querySelector("#initGrid"));
  if (params.get("grid_type") == "random") {
    getGrid(config.oneDRandom, params);
  }
  if (params.get("grid_type") == "center") {
    getGrid(config.oneDCenter, params);
  }
  event.preventDefault();
}

function getGrid(url_str, params) {
  document.getElementById("canvas").width = params.get("width") * call_width;
  document.getElementById("canvas").height = hight;
  // TODO
}

function step(event) {
  const params = new FormData(document.querySelector("#step"));
  const init_params = new FormData(document.querySelector("#initGrid"));
  const grid = JSON.parse(localStorage.getItem("prevGrid"));
  let colours = Array.from(
    Array(Number(init_params.get("colors_count"))).keys()
  );

  let body = {
    wolfram_number: params.get("wolfram_number"),
    neighborhood_size: params.get("neighborhood_size"),
    colours: colours.toString(),
    grid: grid,
  };
  //TODO
}

function generateGrid(grid, y) {
  const canvas = document.getElementById("canvas");
  const context = canvas.getContext("2d");

  let x_cor = 0;
  let y_cor = 0;
  for (var x = 0; x < grid.length; x++) {
    context.fillStyle = colors[grid[x][0]];
    y_cor = y * call_width;
    context.fillRect(x_cor, y_cor, call_width, call_hight);
    x_cor = x * call_width;
  }

  localStorage.setItem("prevGrid", JSON.stringify(grid));
}

async function play() {
  let count = 1;
  while (true) {
    await step();
    console.log(`step ${count}`);
    count++;
  }
}

formEl.addEventListener("submit", initGrid);
formE2.addEventListener("submit", step);
