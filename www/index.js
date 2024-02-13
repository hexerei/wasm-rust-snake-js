async function init() {

    // js functions exported to wasm
    const importObject = {
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
    const wasmMemory = wasm.instance.exports.mem;
    const uint8Array = new Uint8Array(wasmMemory.buffer, 0, 2);
    const hiText = new TextDecoder().decode(uint8Array);
    console.log(hiText);
}
init();