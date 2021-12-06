use aoc2021::{day1, day2, day3, day4, day5};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn get_data_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data")
}

fn day1_benchmark(c: &mut Criterion) {
    let data_dir = get_data_dir();

    c.bench_function("day 1 setup", |b| {
        b.iter(|| day1::data(black_box(&data_dir)))
    });
    let day1_data = day1::data(&data_dir).expect("Programming error");

    c.bench_function("day 1 star 1", |b| {
        b.iter(|| day1::star1(black_box(&day1_data)))
    });

    c.bench_function("day 1 star 2", |b| {
        b.iter(|| day1::star2(black_box(&day1_data)))
    });
}

fn day2_benchmark(c: &mut Criterion) {
    let data_dir = get_data_dir();

    c.bench_function("day 2 setup", |b| {
        b.iter(|| day2::data(black_box(&data_dir)))
    });
    let day2_data = day2::data(&data_dir).expect("Programming error");

    c.bench_function("day 2 star 1", |b| {
        b.iter(|| day2::star1(black_box(&day2_data)))
    });

    c.bench_function("day 2 star 2", |b| {
        b.iter(|| day2::star2(black_box(&day2_data)))
    });
}

fn day3_benchmark(c: &mut Criterion) {
    let data_dir = get_data_dir();

    c.bench_function("day 3 setup", |b| {
        b.iter(|| day3::data(black_box(&data_dir)))
    });
    let day3_data = day3::data(&data_dir).expect("Programming error");

    c.bench_function("day 3 star 1", |b| {
        b.iter(|| day3::star1(black_box(&day3_data)))
    });

    c.bench_function("day 3 star 2", |b| {
        b.iter(|| day3::star2(black_box(&mut day3_data.clone())))
    });
}

fn day4_benchmark(c: &mut Criterion) {
    let data_dir = get_data_dir();

    c.bench_function("day 4 setup", |b| {
        b.iter(|| day4::data(black_box(&data_dir)))
    });
    let (called, boards) = day4::data(&data_dir).expect("Programming error");

    c.bench_function("day 4 star 1", |b| {
        b.iter(|| day4::star1(black_box(&called), black_box(&mut boards.clone())))
    });

    c.bench_function("day 4 star 2", |b| {
        b.iter(|| day4::star2(black_box(&called), black_box(&mut boards.clone())))
    });
}

fn day5_benchmark(c: &mut Criterion) {
    let data_dir = get_data_dir();

    c.bench_function("day 5 setup", |b| {
        b.iter(|| day5::data(black_box(&data_dir)))
    });
    let day5_data = day5::data(&data_dir).expect("Programming error");

    c.bench_function("day 5 star 1", |b| {
        b.iter(|| day5::star1(black_box(&day5_data)))
    });

    c.bench_function("day 5 star 2", |b| {
        b.iter(|| day5::star2(black_box(&mut day5_data.clone())))
    });
}

criterion_group!(
    benches,
    day1_benchmark,
    day2_benchmark,
    day3_benchmark,
    day4_benchmark,
    day5_benchmark,
);
criterion_main!(benches);
