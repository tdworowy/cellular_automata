import * as THREE from "three";
import { MapControls } from "three/addons/controls/MapControls.js";

const game_of_live_rules = {
  "0_3": 1,
  "1_1": 0,
  "1_4": 0,
};

const three_d_game_of_live_rules1 = {
  //https://cs.brown.edu/courses/cs195v/projects/life/edwallac/index.html
  "0_5": 1,
  "1_4": 0,
  "1_3": 0,
  "1_2": 0,
  "1_1": 0,
  "1_0": 0,
  "1_8": 0,
};

const three_d_game_of_live_rules2 = {
  //https://cs.brown.edu/courses/cs195v/projects/life/edwallac/index.html
  "0_14": 1,
  "0_15": 1,
  "0_16": 1,
  "0_17": 1,
  "0_18": 1,
  "0_19": 1,
  "1_12": 0,
  "1_11": 0,
  "1_10": 0,
  "1_9": 0,
  "1_7": 0,
  "1_6": 0,
  "1_5": 0,
  "1_4": 0,
  "1_3": 0,
  "1_2": 0,
  "1_1": 0,
  "1_0": 0,
};
//TODO find more rules

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


function generateCubeRandom(height, width, depth, prob_of_one) {
  let cube = [];
  for (let x = 0; x < height; x++) {
    let grid = [];
    for (let y = 0; y < width; y++) {
      let row = [];
      for (let z = 0; z < depth; z++) {
        let rand_int = Math.floor(Math.random() * 100 + 1);
        if (rand_int <= prob_of_one * 100) {
          row.push(1);
        } else row.push(0);
      }
      grid.push(row);
    }
    cube.push(grid);
  }
  return cube;
}

function generateCubeCenter(height, width, depth) {
  let cube = [];
  for (let x = 0; x < height; x++) {
    let grid = [];
    for (let y = 0; y < width; y++) {
      let row = [];
      for (let z = 0; z < depth; z++) {
        if (x === height / 2 && y === width / 2 && z === depth /2) row.push(1)
          else row.push(0);
      }
      grid.push(row);
    }
    cube.push(grid);
  }
  return cube;
}


//TODO something seems to be wrong
function countColoredNeighbours(
  x,
  y,
  z,
  cube_x_axis,
  cube_y_axis,
  cube_z_axis,
  cube
) {
  let colored_neighbours = 0;
  for (let i = 0; i < 3; i++) {
    let ni = (x + i - 1 + cube_x_axis) % cube_x_axis;
    for (let j = 0; j < 3; j++) {
      let nj = (y + j - 1 + cube_y_axis) % cube_y_axis;
      for (let k = 0; k < 3; k++) {
        let nk = (z + k - 1 + cube_z_axis) % cube_z_axis;
        if (cube[ni][nj][nk] === 1 && !(i === 1 && j === 1 && k === 1)) {
          colored_neighbours += 1;
        }
      }
    }
  }
  //console.log(colored_neighbours);
  return colored_neighbours;
}

function updateCube(cube, rules) {
  let new_cube = JSON.parse(JSON.stringify(cube));
  for (let i = 0; i < cube.length; i++) {
    for (let j = 0; j < cube[i].length; j++) {
      for (let k = 0; k < cube[i][j].length; k++) {
        let state = cube[i][j][k];
        let live_neighbours = countColoredNeighbours(
          i,
          j,
          k,
          cube.length,
          cube[0].length,
          cube[0][0].length,
          cube
        );
        let rule = rules[state.toString() + "_" + live_neighbours.toString()];
        if (rule) {
          new_cube[i][j][k] = rule;
        } else {
          new_cube[i][j][k] = cube[i][j][k];
        }
      }
    }
  }
  return new_cube;
}

function renderCubes(cube, scene, geometry, material) {
  for (let x = 0; x < cube.length; x++) {
    for (let y = 0; y < cube[x].length; y++) {
      for (let z = 0; z < cube[x][y].length; z++) {
        const mesh = new THREE.Mesh(geometry, material);
        if (cube[x][y][z] === 1) {
          mesh.position.set(x, y, z);
          scene.add(mesh);
        }
      }
    }
  }
  return scene;
}

const width = window.innerWidth;
const height = window.innerHeight;

const renderer = new THREE.WebGLRenderer({ antialias: true });
const camera = new THREE.PerspectiveCamera(
  60,
  window.innerWidth / window.innerHeight,
  1,
  1000
);
camera.position.set(-50, 50, -50);

let controls = new MapControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.05;
controls.screenSpacePanning = false;
controls.minDistance = 0.1;
controls.maxDistance = 500;
controls.maxPolarAngle = Math.PI / 2;

let scene = new THREE.Scene();

const x_size = 0.9;
const y_size = 0.9;
const z_size = 0.9;

const material = new THREE.MeshNormalMaterial();
const geometry = new THREE.BoxGeometry(x_size, y_size, z_size);

//let cube = generateCubeRandom(30, 30, 30, 0.3);
let cube = generateCubeCenter(30, 30, 30);

function render() {
 // cube = updateCube(cube, three_d_game_of_live_rules2);
  cube = updateCube(cube, generateSnowflakeRule([1,15]))
  scene.clear();
  scene = renderCubes(cube, scene, geometry, material);
  renderer.render(scene, camera);  
}

renderer.setSize(width, height);
renderer.setAnimationLoop(animate);
document.body.appendChild(renderer.domElement);

function onWindowResize() {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();

  renderer.setSize(window.innerWidth, window.innerHeight);
}

function animate(time) {
  controls.update();
  render();
}
