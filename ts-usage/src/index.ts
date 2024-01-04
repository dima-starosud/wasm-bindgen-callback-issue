import { TsCallback, rust_function } from 'rust-impl';

class A implements TsCallback {
    call(arg: string): number {
        console.log(["Here is what I've passed: ", [this, arg]]);
        return arg.length;
    }
}

const result = rust_function(new A());
console.log([result]);
