import * as wasm from "my-wasm-project";

wasm.greet();

const data = wasm.sum_factorial(10n);
console.info('data?', data);