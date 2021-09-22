use criterion::criterion_group;
use criterion::criterion_main;
use criterion::{black_box, Bencher, BenchmarkId, Criterion, Throughput};
use g60::{decode, decode_in_slice, decode_in_slice_unchecked, encode, encode_in_slice, verify};
use rand::{Rng, SeedableRng};

// ----------------------------------------------------------------------------
// BENCHES --------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn do_encode_bench(b: &mut Bencher, &size: &usize) {
    let mut input: Vec<u8> = Vec::with_capacity(size);
    fill(&mut input);

    b.iter(|| {
        let encoded = encode(&input);
        black_box(&encoded);
    });
}

fn do_encode_in_slice_bench(b: &mut Bencher, &size: &usize) {
    let mut input: Vec<u8> = Vec::with_capacity(size);
    fill(&mut input);

    let mut buffer = Vec::with_capacity(size << 2);
    // conservative estimate of encoded size
    buffer.resize(size * 4, 0);

    b.iter(|| {
        let result = encode_in_slice(&input, &mut buffer);
        black_box(&result);
    });
}

fn do_decode_bench(b: &mut Bencher, &size: &usize) {
    let mut input: Vec<u8> = Vec::with_capacity(size);
    fill(&mut input);

    let encoded = encode(&input);

    b.iter(|| {
        let original = decode(&encoded).unwrap();
        black_box(&original);
    });
}

fn do_decode_in_slice_bench(b: &mut Bencher, &size: &usize) {
    let mut input: Vec<u8> = Vec::with_capacity(size);
    fill(&mut input);

    let encoded = encode(&input);

    let mut buffer = Vec::new();
    buffer.resize(size, 0);

    b.iter(|| {
        decode_in_slice(&encoded, &mut buffer).unwrap();
        black_box(&buffer);
    });
}

fn do_decode_in_slice_unchecked_bench(b: &mut Bencher, &size: &usize) {
    let mut input: Vec<u8> = Vec::with_capacity(size);
    fill(&mut input);

    let encoded = encode(&input);

    let mut buffer = Vec::new();
    buffer.resize(size, 0);

    b.iter(|| {
        unsafe {
            decode_in_slice_unchecked(&encoded, &mut buffer).unwrap();
        }
        black_box(&buffer);
    });
}

fn do_verify_bench(b: &mut Bencher, &size: &usize) {
    let mut input: Vec<u8> = Vec::with_capacity(size);
    fill(&mut input);

    let encoded = encode(&input);

    b.iter(|| {
        let result = verify(&encoded);
        black_box(&result);
    });
}

// ----------------------------------------------------------------------------
// AUX METHODS ----------------------------------------------------------------
// ----------------------------------------------------------------------------

fn fill(vector: &mut Vec<u8>) {
    let capacity = vector.capacity();

    // weak randomness is plenty; we just want to not be completely friendly to the branch predictor
    let mut random = rand::rngs::StdRng::from_entropy();
    while vector.len() < capacity {
        vector.push(random.gen::<u8>());
    }
}

// ----------------------------------------------------------------------------
// CONFIGURATION --------------------------------------------------------------
// ----------------------------------------------------------------------------

const BYTE_SIZES: [usize; 5] = [3, 50, 100, 500, 3 * 1024];

// Benchmarks over these byte sizes take longer so we will run fewer samples to
// keep the benchmark runtime reasonable.
const LARGE_BYTE_SIZES: [usize; 3] = [3 * 1024 * 1024, 10 * 1024 * 1024, 30 * 1024 * 1024];

fn encode_benchmarks(c: &mut Criterion, label: &str, byte_sizes: &[usize]) {
    let mut group = c.benchmark_group(label);
    group
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(15));

    for size in byte_sizes {
        group
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::new("encode", size), size, do_encode_bench)
            .bench_with_input(
                BenchmarkId::new("encode_in_slice", size),
                size,
                do_encode_in_slice_bench,
            );
    }

    group.finish();
}

fn decode_benchmarks(c: &mut Criterion, label: &str, byte_sizes: &[usize]) {
    let mut group = c.benchmark_group(label);

    for size in byte_sizes {
        group
            .warm_up_time(std::time::Duration::from_millis(500))
            .measurement_time(std::time::Duration::from_secs(15))
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::new("decode", size), size, do_decode_bench)
            .bench_with_input(
                BenchmarkId::new("decode_in_slice", size),
                size,
                do_decode_in_slice_bench,
            )
            .bench_with_input(
                BenchmarkId::new("decode_in_slice_unchecked", size),
                size,
                do_decode_in_slice_unchecked_bench,
            );
    }

    group.finish();
}

fn verify_benchmarks(c: &mut Criterion, label: &str, byte_sizes: &[usize]) {
    let mut group = c.benchmark_group(label);

    for size in byte_sizes {
        group
            .warm_up_time(std::time::Duration::from_millis(500))
            .measurement_time(std::time::Duration::from_secs(3))
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::new("verify", size), size, do_verify_bench);
    }

    group.finish();
}

fn bench(c: &mut Criterion) {
    encode_benchmarks(c, "encode_small_input", &BYTE_SIZES[..]);
    encode_benchmarks(c, "encode_large_input", &LARGE_BYTE_SIZES[..]);
    decode_benchmarks(c, "decode_small_input", &BYTE_SIZES[..]);
    decode_benchmarks(c, "decode_large_input", &LARGE_BYTE_SIZES[..]);
    verify_benchmarks(c, "verify_small_input", &BYTE_SIZES[..]);
    verify_benchmarks(c, "verify_large_input", &LARGE_BYTE_SIZES[..]);
}

criterion_group!(benches, bench);
criterion_main!(benches);
