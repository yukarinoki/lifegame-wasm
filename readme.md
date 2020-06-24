## Lifegame by (Rust to WASM)

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/yukarinoki/lifegame-wasm)  
https://rustwasm.github.io/docs/book/  

![demo](https://github.com/yukarinoki/lifegame-wasm/blob/master/lifegame.gif)

## How to build

```
git clone {this repository}  
cd lifegame-wasm/wasm-game-of-life  
wasm-pack build  
cd www  
npm install  
npm run start  
// connect to localhost:8080   
```


# Note
In this gitpod image, headless chrome and firefox are not available, so when you test by wasm-pack, you do "wasm-pack test" without "--headless option" and open it by using your own blowser. 
![test_result](https://github.com/yukarinoki/lifegame-wasm/blob/master/image.png)
