use sortedlist_rs::SortedList;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use rand::Rng;

const DEFAULT_ARRAY_SIZE: usize = 100_000;

fn get_random_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..DEFAULT_ARRAY_SIZE)
        .map(|_| {
            rng.gen::<i32>()
        })
        .collect()
}

fn get_random_sorted_list() -> SortedList<i32> {
    SortedList::from(get_random_array())
}

fn insert_element_benchmark(c: &mut Criterion) {
    c.bench_function(
        "insert element benchmark",
        |b| {
            b.iter(||{
                // data arrangement
                let arr = get_random_array();

                // action
                let mut sorted_list = SortedList::default();

                for x in &arr {
                    sorted_list.insert(*x);
                }
            })
        }
    );
}

fn remove_first_element_benchmark(c: &mut Criterion) {
    c.bench_function(
        "remove first element benchmark", 
        |b| {
            b.iter(|| {
                // data arrangement
                let mut sorted_list = get_random_sorted_list();

                for _ in 0..sorted_list.len() {
                    sorted_list.remove(0);
                }
            })
        }
    );
}

fn remove_last_element_benchmark(c: &mut Criterion) {
    c.bench_function(
        "remove last element benchmark", 
        |b| {
            b.iter(|| {
                // data arrangement
                let mut sorted_list = get_random_sorted_list();

                for i in (0..sorted_list.len()).rev() {
                    sorted_list.remove(i);
                }
            })
        }
    );
}

fn remove_middle_element_benchmark(c: &mut Criterion) {
    c.bench_function(
        "remove random element benchmark", 
        |b| {
            b.iter(|| {
                // data arrangement
                let mut sorted_list = get_random_sorted_list();

                for i in (0..sorted_list.len()).rev() {
                    sorted_list.remove(i/2);
                }
            })
        }
    );
}

fn remove_random_element_benchmark(c: &mut Criterion) {
    c.bench_function(
        "remove middle element benchmark", 
        |b| {
            b.iter(|| {
                // data arrangement
                let mut sorted_list = get_random_sorted_list();
                let mut rng = rand::thread_rng();

                for i in (0..sorted_list.len()).rev() {
                    // sorted list size is i+1
                    sorted_list.remove(rng.gen_range(0..i+1));
                }
            })
        }
    );
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