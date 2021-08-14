const formEl = document.getElementById("initGrid");
const formE2 = document.getElementById("step");

const call_width = config.call_width;
const call_hight = config.call_hight;
let y = 0;

let hight = 10000; //TODO make canvas with dynamic height

function product(iterable, repeat = 1) {
  let pools;
  let result;
  for (let i = 0; i < repeat; i++) {
    pools.push(iterable);
  }
  for (let pool of pools) {
    for (let x of result) {
      for (let y of pool) {
        if (![x + [y]] in result && (x + [y]).length <= repeat) {
          result.push(x + [y]);
        }
      }
    }
  }
  let final_result = [];
  for (let res of result) {
    if (res.length == repeat) {
      final_result.push(res);
    }
  }
  return final_result;
}

function n_nary(number, n) {
  if (number == 0) return "0";
  let nums = [];
  while (number) nums.push((number % n).toString());
  return nums.reverse().join("");
}

function wolfram_number_to_bin(wolfram_number, possible_states, colours_count) {
  let wolfram_number_s = n_nary(wolfram_number, colours_count);
  let temp = possible_states - wolfram_number_s.length;
  wolfram_number_s = "0" * temp + wolfram_number_s;
  return wolfram_number_s.split("", true).reverse();
}

function cellular_automata_step_1d(input_list, rules) {
  let output_list = [];
  let width = input_list[0].length;

  for (let i = 0; i < input_list.length; i++) {
    for (let rule of rules) {
      let neighborhood_size = rule.neighborhood.length;
      let temp = (neighborhood_size - 1) / 2;
      let current_neighborhood = [];

      for (
        let j = (((i - temp) % width) + width) % width;
        j < (i + temp + 1) % width;
        j++
      ) {
        current_neighborhood.push(input_list[j]);

        if (current_neighborhood == rule.neighborhood)
          output_list.push(rule.type);
        else output_list.push(0);
      }
    }
  }

  return output_list;
}

function generate_grid_random(hight, width, states = [0, 1]) {
  let grid = [];
  let first_row = [];
  for (let i = 0; i < width; i++) {
    var rand_int = Math.random() * states.length + 1;
    first_row.push(rand_int);
  }
  grid.push(first_row);

  for (let j = 0; i < hight - 1; i++) {
    let row = [];
    for (let i = 0; i < width; i++) {
      row.push(0);
    }
    grid.push(row);
  }
}

function generate_grid_center(hight, width, states = [0, 1]) {
  let grid = [];
  let first_row = [];
  for (let i = 0; i < width; i++) {
    first_row.push(0);
  }
  first_row[width / 2] = states[1];
  grid.push(first_row);

  for (let j = 0; j < hight - 1; j++) {
    let row = [];
    for (let i = 0; i < width; i++) {
      row.push(0);
    }
    grid.push(row);
  }
  return grid;
}

function generate_rule(
  wolfram_number,
  neighborhood_size = 3,
  colours = [0, 1]
) {
  let colours_count = colours.length;
  let possible_states = colours_count ** neighborhood_size;
  let rule = [];

  let wolfram_number_a = wolfram_number_to_bin(
    wolfram_number,
    possible_states,
    colours_count
  );
  let i = 0;
  for (let comb of product(colours, neighborhood_size)) {
    rule.append(self.RuleSegment.new(comb, int(wolfram_number_a[i])));
    i += 1;
  }
  return rule;
}

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
