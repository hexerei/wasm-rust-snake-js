async function init() {

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
 
    const response = await fetch("test.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, importObject);

    const sum = wasm.instance.exports.addTwo;
    const result = sum(200, 100);
    console.log(result);
}
init();