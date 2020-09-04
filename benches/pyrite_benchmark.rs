use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pyrite::math::{prelude::*, *};

pub fn bench_mul_vec3_scalar(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("mul vector3 and scalar", |b| b.iter(|| v * black_box(3.2)));
}

pub fn bench_mul_scalar_vec3(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("mul scalar and vector3", |b| b.iter(|| black_box(3.2) * v));
}

pub fn index_access_0(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("index access get 0", |b| {
        b.iter(|| {
            let _ = v[black_box(0)];
        })
    });
}

pub fn index_access_1(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("index access get 1", |b| {
        b.iter(|| {
            let _ = v[black_box(1)];
        })
    });
}

pub fn index_access_2(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("index access get 2", |b| {
        b.iter(|| {
            let _ = v[black_box(2)];
        })
    });
}

pub fn length_squared(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("length_squared", |b| {
        b.iter(|| {
            let l = v.length_squared();
            black_box(l);
        })
    });
}

pub fn length(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("length", |b| {
        b.iter(|| {
            let l = v.length();
            black_box(l);
        })
    });
}

pub fn normalized(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    c.bench_function("normalize", |b| {
        b.iter(|| {
            let n = v.normalized();
            black_box(n);
        })
    });
}

criterion_group!(
    benches,
    bench_mul_vec3_scalar,
    bench_mul_scalar_vec3,
    index_access_0,
    index_access_1,
    index_access_2,
    length_squared,
    length,
    normalized,
);
criterion_main!(benches);
