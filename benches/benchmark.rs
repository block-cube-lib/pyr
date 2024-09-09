use criterion::{black_box, criterion_group, criterion_main, Criterion};

type Vec3 = pyr::math::Vec3<f64>;

pub fn mul_vec3_scalar(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("mul vec3 and scalar", |b| b.iter(|| v * black_box(3.2)));
}

pub fn mul_scalar_vec3(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("mul scalar and vec3", |b| b.iter(|| black_box(3.2) * v));
}

pub fn vec3_index_access_0(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("index access get 0", |b| {
        b.iter(|| {
            let v = v[black_box(0)];
            black_box(v);
        })
    });
}

pub fn vec3_index_access_1(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("index access get 1", |b| {
        b.iter(|| {
            let v = v[black_box(1)];
            black_box(v);
        })
    });
}

pub fn vec3_index_access_2(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("index access get 2", |b| {
        b.iter(|| {
            let v = v[black_box(2)];
            black_box(v);
        })
    });
}

pub fn vec3_length_squared(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("vec3 length_squared", |b| {
        b.iter(|| {
            let l = v.length_squared();
            black_box(l);
        })
    });
}

pub fn vec3_length(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("vec3 length", |b| {
        b.iter(|| {
            let l = v.length();
            black_box(l);
        })
    });
}

pub fn vec3_normalized(c: &mut Criterion) {
    let v = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("vec3 normalized", |b| {
        b.iter(|| {
            let n = v.normalized();
            black_box(n);
        })
    });
}

pub fn vec3_dot(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("vec3 dot", |b| {
        b.iter(|| {
            let n = v1.dot(v2);
            black_box(n);
        })
    });
}

pub fn vec3_distance(c: &mut Criterion) {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    c.bench_function("vec3 distance", |b| {
        b.iter(|| {
            let n = v1.distance(v2);
            black_box(n);
        })
    });
}

pub fn vec_ops_dot_3dim(c: &mut Criterion) {
    use pyr::math::ops;
    let v1 = (1.0, 2.0, 3.0);
    let v2 = [4.0, 8.0, 9.0];
    c.bench_function("vec_ops_dot_3dim", |b| {
        b.iter(|| {
            let n = ops::dot(v1, v2);
            black_box(n);
        })
    });
}

pub fn vec_ops_length_squared_3dim(c: &mut Criterion) {
    use pyr::math::ops;
    let v1 = (1.0, 2.0, 3.0);
    c.bench_function("vec_ops_length_squared_3dim", |b| {
        b.iter(|| {
            let n = ops::length_squared(v1);
            black_box(n);
        })
    });
}

pub fn vec_ops_length_3dim(c: &mut Criterion) {
    use pyr::math::ops;
    let v1 = (1.0, 2.0, 3.0);
    c.bench_function("vec_ops_length_3dim", |b| {
        b.iter(|| {
            let n = ops::length(v1);
            black_box(n);
        })
    });
}

criterion_group!(
    benches,
    mul_vec3_scalar,
    mul_scalar_vec3,
    vec3_index_access_0,
    vec3_index_access_1,
    vec3_index_access_2,
    vec3_length_squared,
    vec3_length,
    vec3_normalized,
    vec3_dot,
    vec3_distance,
    vec_ops_dot_3dim,
    vec_ops_length_squared_3dim,
    vec_ops_length_3dim,
);
criterion_main!(benches);
