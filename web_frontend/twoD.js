var formEl = document.getElementById('initGrid');
var formE2 = document.getElementById('step');

formEl.addEventListener('submit', function(event) {
  var headers = new Headers();
  headers.set('Accept', 'application/json');

  var params = new FormData(document.querySelector('#initGrid'))

  document.getElementById("canvas").width = params.get('width') 
  document.getElementById("canvas").height = params.get('height') 
 
  var url = new URL('http://localhost:5000/grid/2d/random')
  url.search = new URLSearchParams(params).toString()
 
  var responsePromise = fetch(url,  {method: "GET",
    headers:headers
});
  
  responsePromise
    .then(function(response) {
      return response.json();
    })
    .then(function(jsonData) {
      generateGrid(jsonData.grid)
    });

  event.preventDefault();
});

function step (event) {
  var headers = new Headers();
  headers.set("Content-Type", "application/json");

  var params = new FormData(document.querySelector('#step'))
  var grid = JSON.parse(localStorage.getItem("prevGrid"))
 
  var body = {
    'rule':params.get('rule'),
    'grid':grid
  } 
  body = JSON.stringify({...body})
  var url = new URL('http://localhost:5000/CellularAutomata/2d/step')
 
  var responsePromise = fetch(url,  {
    method: "POST",
    headers:headers,
    body:body 
})
responsePromise
.then(function(response) {
  return response.json();
})
.then(function(jsonData) {
  generateGrid(jsonData.grid)
});
if(event) {
event.preventDefault();
}
}

formE2.addEventListener('submit',step);

function generateGrid(grid){

var canvas = document.getElementById("canvas");
var context = canvas.getContext("2d");
var prevGrid = JSON.parse(localStorage.getItem("prevGrid"));
if(prevGrid != grid) {
  for(var x=0; x<grid.length;x++) {
    for(var y=0; y<grid[x].length;y++) {
      if(grid[x][y] == 1) {
        r = 255
        g = 255
        b = 255
      } else {
        r = 0
        g = 0
        b = 0
      }
    context.fillStyle = "rgba("+r+","+g+","+b+","+255+")";
    context.fillRect( x, y, 1, 1 );
  }
  }
  localStorage.setItem("prevGrid", JSON.stringify(grid));
}    
}

function play(){
  while(true) {
    step()
  }
}