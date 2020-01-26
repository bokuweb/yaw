#![allow(clippy::all)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use std::fs::File;
use std::io::Read;

use wasmi::{ImportsBuilder, Module, ModuleInstance, NopExternals, RuntimeValue};

fn load_from_file(filename: &str) -> Module {
    use std::io::prelude::*;
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    Module::from_buffer(buf).unwrap()
}

fn bench_fibs(c: &mut Criterion) {
    let mut file = File::open("./fixtures/wasm/fib.wasm").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let mut group = c.benchmark_group("Fibonacci");
    for i in [10, 15].iter() {
        group.bench_with_input(BenchmarkId::new("yaw", i), i, |b, i| {
            let mut file = File::open("./fixtures/wasm/fib.wasm").unwrap();
            let mut buf = vec![];
            file.read_to_end(&mut buf).unwrap();
            let ins = yaw::instantiate(&buf, None).unwrap();
            b.iter(|| {
                ins.invoke("fib", &[yaw::RuntimeValue::I32(*i as i32)])
                    .unwrap()
            })
        });
        group.bench_with_input(BenchmarkId::new("wasmi", i), i, |b, i| {
            let kernel = load_from_file("./fixtures/wasm/fib.wasm");
            let instance = ModuleInstance::new(&kernel, &ImportsBuilder::default())
                .unwrap()
                .assert_no_start();
            b.iter(|| {
                instance
                    .invoke_export("fib", &[RuntimeValue::I32(*i as i32)], &mut NopExternals)
                    .unwrap();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
