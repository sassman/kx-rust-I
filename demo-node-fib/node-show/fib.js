const ffi = require('ffi');
const path = require('path');

let lib = ffi.Library(path.join(__dirname, '../target/release/libfibonacci'), {
    fibonacci: ['int', ['int']]
});

console.log(lib.fibonacci(40));
