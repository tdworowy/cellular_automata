const host = 'http://localhost:5000'
const config = {
    "call_width" : 5,
    "call_hight" : 5,
    "oneDRandom":`${host}/grid/1d/random`,
    "oneDCenter" : `${host}/grid/1d/center`,
    "oneDStep":`${host}/CellularAutomata/1d/step`,
    "twoDRandom": `${host}/grid/2d/random`,
    "twoDCenter":  `${host}/grid/2d/center`,
    "twoDStep": `${host}/CellularAutomata/2d/step`
    }


 const blue = "rgba("+0+","+0+","+255+","+255+")";
 const red = "rgba("+255+","+0+","+0+","+255+")";
 const green = "rgba("+0+","+255+","+0+","+255+")";
 const black = "rgba("+0+","+0+","+0+","+255+")";
 const aquamarine = "rgba("+102+","+205+","+212+","+255+")";
 const gold = "rgba("+255+","+215+","+0+","+255+")";
 const purple = "rgba("+255+","+0+","+255+","+255+")";
    
 const colors = {
      0:blue,
      1:red,
      2:green,
      3:black,
      4:aquamarine,
      5:gold,
      6:purple
    }