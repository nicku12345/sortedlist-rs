use rand_chacha::ChaCha8Rng;
use sortedlist_rs::SortedList;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};
use rand::{Rng, SeedableRng};

const DEFAULT_TEST_SIZES: [usize; 4] = [100, 1_000, 10_000, 100_000];

fn get_random_number_generator() -> impl Rng {
    ChaCha8Rng::seed_from_u64(10)
}

fn get_random_array(test_size: usize) -> Vec<i32> {
    let mut rng = get_random_number_generator();
    (0..test_size)
        .map(|_| {
            rng.gen::<i32>()
        })
        .collect()
}

fn insert_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert element");
    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let mut sorted_list = SortedList::new();

                    for element in random_array {
                        sorted_list.insert(element);
                    }
                })
            });
        group.bench_with_input(
            BenchmarkId::new("Data setup", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let _random_array = get_random_array(*size);
                })
            });
    }
    group.finish();
}

fn remove_first_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove first element");
    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let mut sorted_list = SortedList::from(random_array);

                    for _ in 0..*size {
                        sorted_list.remove(0);
                    }
                })
            });
        group.bench_with_input(
            BenchmarkId::new("Data setup", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let _sorted_list = SortedList::from(random_array);
                })
            });
    }
    group.finish();
}

fn remove_last_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove last element");
    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let mut sorted_list = SortedList::from(random_array);

                    for i in (0..*size).rev() {
                        sorted_list.remove(i);
                    }
                })
            });
        group.bench_with_input(
            BenchmarkId::new("Data setup", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let _sorted_list = SortedList::from(random_array);
                })
            });
    }
    group.finish();
}

fn remove_middle_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove middle element");
    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let mut sorted_list = SortedList::from(random_array);

                    for i in (0..*size).rev() {
                        sorted_list.remove(i/2);
                    }
                })
            });
        group.bench_with_input(
            BenchmarkId::new("Data setup", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let _sorted_list = SortedList::from(random_array);
                })
            });
    }
    group.finish();
}

fn remove_random_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove random element");
    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let mut sorted_list = SortedList::from(random_array);
                    let mut rng = get_random_number_generator();

                    for i in (0..*size).rev() {
                        sorted_list.remove(rng.gen_range(0..i+1));
                    }
                })
            });
        group.bench_with_input(
            BenchmarkId::new("Data setup", test_size),
            test_size, 
            |b, size| {
                b.iter(|| {
                    // data arrangement
                    let random_array = get_random_array(*size);
                    let _sorted_list = SortedList::from(random_array);
                })
            });
    }
    group.finish();
}

criterion_group!(
    benches,
    insert_element_benchmark,
    remove_first_element_benchmark,
    remove_last_element_benchmark,
    remove_middle_element_benchmark,
    remove_random_element_benchmark,
);
criterion_main!(benches);