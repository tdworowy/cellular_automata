const FetchTest = async () => {
 
    //var url = new URL('http://localhost:5000/test')
    var url = "http://localhost:5000/grid/2d/random?width=10&height=10&one_prob=0.2"
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
      var response_json = await response.json()
      console.log(`response_json: ${JSON.stringify(response_json)}`)
      
    }
    catch (e) {
      console.log(e)
      console.log(e.message)
  }
    
  }

