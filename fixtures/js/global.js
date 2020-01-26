const fs = require("fs");
const memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });
const global = new WebAssembly.Global({ value: "i32", mutable: true }, -2);

const importObj = {
  env: {
    table: new WebAssembly.Table({
      initial: 0,
      maximum: 0,
      element: "anyfunc"
    }),
    tableBase: 0,
    memory: memory,
    global,
    memoryBase: 1024,
    STACKTOP: 0,
    STACK_MAX: memory.buffer.byteLength
  }
};

const buf = fs.readFileSync(`./fixtures/wasm/global.wasm`);
WebAssembly.instantiate(buf, importObj)
  .then(mod => {
    console.log(mod.instance.exports.global(1, 2));
  })
  .catch(console.error);
