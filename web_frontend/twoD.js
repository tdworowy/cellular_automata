const generateInitGrid = async () => {
 
    var params = new FormData(document.querySelector('#initGrid'))
    var url = new URL('http://localhost:5000/grid/2d/random')
    
    url.search = new URLSearchParams(params).toString()
    console.log(`url: ${url}`)
    try {
      var response = await fetch(url,
        {method: "GET",
        //  headers:{
        //    "Accept": "*/*",
        //    "Accept-Encoding": "gzip, deflate, br",
        //    "Connection": "keep-alive",
        //    "Cache-Control":"no-cache",
        //    "Host":"localhost:5000",
        //    "Origin":"localhost:5000"
        //  }
      })
      console.log(`response: ${response}`)
      var grid = await response.json()
      console.log(`grid: ${grid}`)
      document.getElementById('debag').value = grid

    }
    catch (e) {
      console.log(e.message)
  }
    
  }

// works fine from swagger, postman and just as url in browser, but not from fucking javascript 