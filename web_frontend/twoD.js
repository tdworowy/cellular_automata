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
      var response = await response.json()
      var grid = JSON.stringify(response)
      console.log(`grid: ${grid}`)
      document.getElementById('debag').value = grid

    }
    catch (e) {
      console.log(e)
      console.log(e.message)
  }
    
  }

// form is the root of problems