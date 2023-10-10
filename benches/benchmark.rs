use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use sortedlist_rs::SortedList;

const DEFAULT_TEST_SIZES: [usize; 5] = [100, 1_000, 10_000, 100_000, 1_000_000];

fn get_random_number_generator() -> impl Rng {
    ChaCha8Rng::seed_from_u64(10)
}

fn get_random_array(test_size: usize) -> Vec<i32> {
    let mut rng = get_random_number_generator();
    (0..test_size).map(|_| rng.gen::<i32>()).collect()
}

fn insert_random_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert random element");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size,
            |b, size| {
                b.iter_batched_ref(
                    || (get_random_array(*size), SortedList::new()),
                    |(randdom_array, sorted_list)| {
                        for x in randdom_array {
                            sorted_list.insert(*x);
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn remove_first_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove first element");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size,
            |b, size| {
                b.iter_batched_ref(
                    || SortedList::from(get_random_array(*size)),
                    |sorted_list| {
                        for _ in 0..*size {
                            sorted_list.remove(0);
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn remove_last_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove last element");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size,
            |b, size| {
                b.iter_batched_ref(
                    || SortedList::from(get_random_array(*size)),
                    |sorted_list| {
                        for i in (0..*size).rev() {
                            sorted_list.remove(i);
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn remove_middle_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove middle element");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size,
            |b, size| {
                b.iter_batched_ref(
                    || SortedList::from(get_random_array(*size)),
                    |sorted_list| {
                        for i in (0..*size).rev() {
                            sorted_list.remove(i / 2);
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn remove_random_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove random element");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size,
            |b, size| {
                b.iter_batched_ref(
                    || {
                        let mut rng = get_random_number_generator();
                        (
                            SortedList::from(get_random_array(*size)),
                            (0..*size)
                                .rev()
                                .map(|i| rng.gen_range(0..i + 1))
                                .collect::<Vec<usize>>(),
                        )
                    },
                    |(sorted_list, iter)| {
                        for i in iter {
                            sorted_list.remove(*i);
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn get_random_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get random element");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size,
            |b, size| {
                b.iter_batched_ref(
                    || {
                        (
                            SortedList::from(get_random_array(*size)),
                            get_random_number_generator(),
                        )
                    },
                    |(sorted_list, rng)| {
                        sorted_list.get(rng.gen_range(0..*size + 1));
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn binary_search_random_element_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Binary search random element");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("SortedList", test_size),
            test_size,
            |b, size| {
                b.iter_batched_ref(
                    || {
                        (
                            SortedList::from(get_random_array(*size)),
                            get_random_number_generator(),
                        )
                    },
                    |(sorted_list, rng)| {
                        let _result = sorted_list.binary_search(&rng.gen::<i32>());
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    insert_random_element_benchmark,
    remove_first_element_benchmark,
    remove_last_element_benchmark,
    remove_middle_element_benchmark,
    remove_random_element_benchmark,
    get_random_element_benchmark,
    binary_search_random_element_benchmark,
);
criterion_main!(benches);
