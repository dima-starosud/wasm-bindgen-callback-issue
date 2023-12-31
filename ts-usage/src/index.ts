import { rust_function } from 'rust-impl';

const result = rust_function(function (this: any, arg: string): number {
    console.log(["Here is what I've passed: ", [this, arg]]);
    return arg.length;
});
console.log([result]);
