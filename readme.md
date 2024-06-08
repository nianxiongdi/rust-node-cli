




##  运行

```js
npm run build
```


## 打包为node包

```js
wasm-pack build --target nodejs
```



**1. 使用**
 
打不为npm包的使用

```js

const { add } = require('xxx')

add(1, 2)

```


**2. 使用**

也可在package.json中配置使用


```json
  "dependencies": {
    "wasm-demo": "file:./pkg/rust_node_cli"
  },
```


```js

const  { add } = require('wasm-demo')
console.log(add(1, 2));

```

**3. 使用**

直接使用

```js

const fs = require('fs');
const path = require('path');

// 假设你已经有了一个编译好的 .wasm 文件，我们将其命名为 example.wasm
const wasmFilePath = path.join(__dirname, './pkg/index_bg.wasm');

// 读取 .wasm 文件
const wasmBuffer = fs.readFileSync(wasmFilePath);

(async () => {
  // 实例化 WebAssembly 模块
  const wasmModule = await WebAssembly.compile(wasmBuffer);
  const instance = await WebAssembly.instantiate(wasmModule);

  // 假设 WebAssembly 模块导出了一个名为 'add' 的函数
  // 这里我们调用这个导出的函数
  const result = instance.exports.add(1, 2);
  console.log(`Result: ${result}`); // 输出结果
})();

```


## 发布为npm包

```js
npm publish
```


## 测试


```js
wasm-pack test --chrome
```

