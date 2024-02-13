async function init() {

    // create wasm memory page
    const memory = new WebAssembly.Memory({initial: 1})

    // js objects exported to wasm
    const importObject = {
        js: {
            mem: memory
        },
        console: {
          log: () => {
            console.log("Just logging something!");
          },
          error: () => {
            console.log("I am just error");
          }
        }
    }
 
    // instantiate wasm
    const response = await fetch("test.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, importObject);

    // test sum function
    const sum = wasm.instance.exports.addTwo;
    const result = sum(200, 100);
    console.log(result);

    // test wasm memory
    const uint8Array = new Uint8Array(memory.buffer, 0, 2);
    const hiText = new TextDecoder().decode(uint8Array);
    console.log(hiText);
}
init();