const formEl = document.getElementById('initGrid');
const formE2 = document.getElementById('step');

const call_width = config.call_width
const call_hight =  config.call_hight

function initGrid (event) {
  const params = new FormData(document.querySelector('#initGrid')) 
  if(params.get('grid_type')== 'random'){
    getGrid(config.twoDRandom,params)
  }
  if(params.get('grid_type')== 'center'){
    params.set('cell_count', '1') 
    getGrid(config.twoDCenter, params)
  }
  event.preventDefault();
}

function getGrid(url_str, params) {
  const headers = new Headers();
  headers.set('Accept', 'application/json');

  document.getElementById("canvas").width = params.get('width') * call_width
  document.getElementById("canvas").height = params.get('height') * call_hight
 
  const url = new URL(url_str)
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
  const url = new URL(config.twoDStep)
 
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
 
  let color;
  let x_cor=0;
  let y_cor=0;
  for(var x=0; x < grid.length; x++) {
     for(var y=0; y < grid[x].length; y++) {
       if(grid[x][y] == 1) {
         color = red
       } else {
         color = blue
       }
        
        context.fillStyle = color
        context.fillRect( x_cor, y_cor, call_width, call_hight );
        x_cor = x * call_width
        y_cor = y * call_width
      }
    }
    localStorage.setItem("prevGrid", JSON.stringify(grid));
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