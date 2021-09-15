const formEl = document.getElementById("initGrid");
const formE2 = document.getElementById("step");
const neighborhood_size_input = document.getElementById("neighborhood_size");

const call_width = config.call_width;
const call_hight = config.call_hight;
let y = 0;
let grid = [];

let hight = 1000; 

function generate_array_from_number(number) {
  return Array.from(Array(Number(number)).keys());
}

function get_random_wolfram_number(neighborhood_size, color_count) {
  return Math.floor(Math.random() * 2 ** (color_count ** neighborhood_size));
}

function product(iterables, repeat) {
  var argv = Array.prototype.slice.call(arguments),
    argc = argv.length;
  if (argc === 2 && !isNaN(argv[argc - 1])) {
    var copies = [];
    for (var i = 0; i < argv[argc - 1]; i++) {
      copies.push(argv[0].slice());
    }
    argv = copies;
  }
  return argv.reduce(
    function tl(accumulator, value) {
      var tmp = [];
      accumulator.forEach(function (a0) {
        value.forEach(function (a1) {
          tmp.push(a0.concat(a1));
        });
      });
      return tmp;
    },
    [[]]
  );
}
const divmod = (x, y) => [Math.floor(x / y), x % y];

function n_nary(number, n) {
  if (number == 0) return "0";
  let nums = [];
  while (number) {
    [number, r] = divmod(number, n);
    nums.push(r.toString());
  }
  return nums.reverse().join("");
}

function wolfram_number_to_bin(wolfram_number, possible_states, colours_count) {
  let wolfram_number_s = n_nary(wolfram_number, colours_count);
  let temp = possible_states - wolfram_number_s.length;
  wolfram_number_s_final = "";
  for (let i = 0; i < temp; i++) {
    wolfram_number_s_final += "0";
  }
  wolfram_number_s_final += wolfram_number_s;
  return wolfram_number_s_final.split("").reverse();
}

arrays_equal = (a, b) => {
  return !!a && !!b && !(a < b || b < a);
};

function cellular_automata_step_1d(input_list, rules) {
  let output_list = [];
  const width = input_list.length;
  let rule_found = false;

  for (let i = 0; i < width; i++) {
    for (let rule of rules) {
      const neighborhood_size = rule.neighborhood.length;
      const temp = (neighborhood_size - 1) / 2;
      const current_neighborhood = [];
      for (
        let j = (((i - temp) % width) + width) % width;
        j < (i + temp + 1) % width;
        j++
      ) {
        current_neighborhood.push(input_list[j]);
      }
      if (arrays_equal(current_neighborhood, rule.neighborhood)) {
        output_list.push(rule.type);
        rule_found = true;
      }
    }
    if (!rule_found) {
      output_list.push(0);
    }
    rule_found = false;
  }
  return output_list;
}

function generate_grid_random(width, states = [0, 1]) {
  let row = [];
  for (let i = 0; i < width; i++) {
    let rand_int = Math.floor(Math.random() * states.length);
    row.push(rand_int);
  }
  return row;
}

function generate_grid_center(width, states = [0, 1]) {
  let row = [];
  for (let i = 0; i < width; i++) {
    row.push(0);
  }
  row[width / 2] = states[1];
  return row;
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
  let combinations = product(colours, neighborhood_size);
  for (let comb of combinations) {
    rule.push({ neighborhood: comb, type: parseInt(wolfram_number_a[i]) });
    i++;
  }
  return rule;
}

function initGrid(event) {
  y = 0;
  const params = new FormData(document.querySelector("#initGrid"));
  document.getElementById("canvas").width = params.get("width") * call_width;
  document.getElementById("canvas").height = hight;
  let states = generate_array_from_number(params.get("colors_count"));

  if (params.get("grid_type") == "random") {
    grid = generate_grid_random(params.get("width"), states);
  }
  if (params.get("grid_type") == "center") {
    grid = generate_grid_center(params.get("width"), states);
  }
  generateGrid(grid, y);
  event.preventDefault();
}

let rule = undefined;
const cash_rule = false

function step(event) {
  const params = new FormData(document.querySelector("#step"));
  const init_params = new FormData(document.querySelector("#initGrid"));
  const colours = generate_array_from_number(init_params.get("colors_count"));

  if (!rule || !cash_rule) {
    rule = generate_rule(
      parseInt(params.get("wolfram_number")),
      parseInt(params.get("neighborhood_size")),
      colours
    );
  }
  grid = cellular_automata_step_1d(grid, rule);
  y++;

  generateGrid(grid, y);
  if (event) {
    event.preventDefault();
  }
}

function on_neighborhood_size_change() {
  const params = new FormData(document.querySelector("#step"));
  const init_params = new FormData(document.querySelector("#initGrid"));

  const wolfram_number = get_random_wolfram_number(
    parseInt(params.get("neighborhood_size")),
    parseInt(init_params.get("colors_count"))
  );
  document.getElementById("wolfram_number").value = wolfram_number;
}

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

function generateGrid(grid, y) {
  let x_cor = 0;
  let y_cor = 0;
  for (var x = 0; x < grid.length; x++) {
    context.fillStyle = colors[grid[x]];
    y_cor = y * call_width;
    context.fillRect(x_cor, y_cor, call_width, call_hight);
    x_cor = x * call_width;
  }
}

function generate() {
  for (let i = 0; i < config.iterations; i++) {
    step();
    console.log(`step ${i}`);
  }
}
neighborhood_size_input.addEventListener("change", on_neighborhood_size_change);
formEl.addEventListener("submit", initGrid);
formE2.addEventListener("submit", step);
