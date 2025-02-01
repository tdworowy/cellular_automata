import * as THREE from "three";
import { MapControls } from "three/addons/controls/MapControls.js";

function generateCubeRandom(height, width, depth, prob_of_one) {
  let cube = [];
  for (let x = 0; x < height; x++) {
    let grid = [];
    for (let y = 0; y < width; y++) {
      let row = [];
      for (let z = 0; z < depth; z++) {
        let rand_int = Math.floor(Math.random() * 10 + 1);
        if (rand_int <= prob_of_one * 10) row.push(1);
        else row.push(0);
      }

      grid.push(row);
    }
    cube.push(grid);
  }
  return cube;
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
  return scene
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
camera.position.set(0, 200, -400);

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

function render() {
   scene.clear()
   const cube = generateCubeRandom(20, 20, 20, 0.3);
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
