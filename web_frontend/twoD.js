const generateInitGrid = async () => {
 
    var params = new FormData(document.querySelector('#initGrid'))
    var url = new URL('http://localhost:5000/grid/2d/random')
    
    url.search = new URLSearchParams(params).toString()
    //var response = await fetch(url)
    //var grid = await response.json()
    fetch(url) //fetch works (request is handler by api correctly)
    .then((resp) =>console.log("DEBUG"))// that never happens
    .then((resp) =>document.getElementById('debag').value = resp.json())
    //document.getElementById('debag').value = grid;
  }