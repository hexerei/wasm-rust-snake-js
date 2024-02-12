async function init() {

    const response = await fetch("test.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer);

    const sum = wasm.instance.exports.addTwo;
    const result = sum(200, 100);
    console.log(result);
}
init();