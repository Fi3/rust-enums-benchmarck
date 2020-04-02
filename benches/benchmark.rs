extern crate bensc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bensc::{Wrapper1, Wrapper2, Wrapper3, Wrapper4, Wrapper5};

pub fn get_inner_wrapper1(c: &mut Criterion) {
    let wrapper = Wrapper1::A(214748);
    c.bench_function("get inner 1",
                     |b| b.iter(|| black_box(wrapper).get_inner()));
}
pub fn get_inner_wrapper2(c: &mut Criterion) {
    let wrapper = Wrapper2::A([9;4]);
    c.bench_function("get inner 2",
                     |b| b.iter(|| black_box(wrapper).get_inner()));
}
pub fn get_inner_wrapper3(c: &mut Criterion) {
    let wrapper = Wrapper3::A([9;4]);
    c.bench_function("get inner 3",
                     |b| b.iter(|| black_box(wrapper).get_inner()));
}
pub fn get_inner_wrapper4(c: &mut Criterion) {
    let wrapper = Wrapper4::A([9;4]);
    c.bench_function("get inner 4",
                     |b| b.iter(|| black_box(wrapper).get_inner()));
}

pub fn get_inner_wrapper5(c: &mut Criterion) {
    let wrapper = Wrapper5::A(214748);
    c.bench_function("get inner 5",
                     |b| b.iter(|| black_box(wrapper).get_inner()));
}

criterion_group!(benches,
                 get_inner_wrapper1,
                 get_inner_wrapper2,
                 get_inner_wrapper3,
                 get_inner_wrapper4,
                 get_inner_wrapper5,
                 );
criterion_main!(benches);
