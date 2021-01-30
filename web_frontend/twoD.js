const formEl = document.getElementById('initGrid');
const formE2 = document.getElementById('step');


 function initGrid (event) {
  const headers = new Headers();
  headers.set('Accept', 'application/json');

  const params = new FormData(document.querySelector('#initGrid'))

  document.getElementById("canvas").width = params.get('width') 
  document.getElementById("canvas").height = params.get('height') 
 
  const url = new URL('http://localhost:5000/grid/2d/random')
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
}



function step (event) {
  const headers = new Headers();
  headers.set("Content-Type", "application/json");

  const params = new FormData(document.querySelector('#step'))
  const grid = JSON.parse(localStorage.getItem("prevGrid"))
 
  let body = {
    'rule':params.get('rule'),
    'grid':grid
  } 
  body = JSON.stringify({...body})
  const url = new URL('http://localhost:5000/CellularAutomata/2d/step')
 
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
return responsePromise

}

const blue = "rgba("+0+","+0+","+255+","+255+")";
const red = "rgba("+255+","+0+","+0+","+255+")";

function generateGrid(grid){
 const canvas = document.getElementById("canvas");
 const context = canvas.getContext("2d");
 const prevGrid = JSON.parse(localStorage.getItem("prevGrid"));
if(prevGrid != grid) {
  for(var x=0; x<grid.length;x++) {
    for(var y=0; y<grid[x].length;y++) {
      if(grid[x][y] == 1) {
        var color = red
      } else {
        var color = blue
      }
    context.fillStyle = color
    context.fillRect( x, y, 1, 1 );
  }
  }
  localStorage.setItem("prevGrid", JSON.stringify(grid));
}    
}

async function play(){
  let count = 1
  while(true) {
    await step()
    console.log(`step ${count}`)
    count ++
    
  }
}


formEl.addEventListener('submit',initGrid);
formE2.addEventListener('submit',step);