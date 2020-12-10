use std::collections::btree_map;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufRead};

pub fn part_one() {
    let adapters = read_input(io::stdin().lock());
    let answer = get_product_of_num_1_diffs_and_num_3_diffs(&adapters);

    println!("{}", answer)
}

pub fn part_two() {
    let adapters = read_input(io::stdin().lock());
    let answer = get_total_num_valid_adapter_configurations(&adapters);

    println!("{}", answer)
}

fn read_input<R>(reader: R) -> BTreeSet<i32>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn get_product_of_num_1_diffs_and_num_3_diffs(adapters: &BTreeSet<i32>) -> i32 {
    let mut adapters_wall_and_device = adapters.clone();
    adapters_wall_and_device.insert(0);
    adapters_wall_and_device.insert(adapters_wall_and_device.iter().max().unwrap() + 3);

    let diffs: Vec<i32> = adapters_wall_and_device
        .iter()
        .zip(adapters_wall_and_device.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    let num_1_diffs = diffs.iter().filter(|a| **a == 1).count() as i32;
    let num_3_diffs = diffs.iter().filter(|a| **a == 3).count() as i32;

    num_1_diffs * num_3_diffs
}

fn get_total_num_valid_adapter_configurations(adapters: &BTreeSet<i32>) -> u64 {
    let device = adapters.iter().max().unwrap() + 3;

    let mut adapters_wall_and_device = adapters.clone();
    adapters_wall_and_device.insert(0);
    adapters_wall_and_device.insert(device);

    calc_num_valid_paths(&adapters_wall_and_device, 0, device, &mut BTreeMap::new())
}

fn get_next_nodes(nodes: &BTreeSet<i32>, node: i32) -> Option<Vec<i32>> {
    let max = *nodes.iter().max().unwrap();

    if node == max {
        None
    } else {
        Some(
            (1..4)
                .map(|i| i + node)
                .filter(|n| nodes.contains(n))
                .collect(),
        )
    }
}

fn calc_num_valid_paths(
    nodes: &BTreeSet<i32>,
    source: i32,
    destination: i32,
    cache: &mut BTreeMap<i32, u64>,
) -> u64 {
    if source == destination {
        1
    } else if let btree_map::Entry::Occupied(entry) = cache.entry(source) {
        *entry.get()
    } else {
        match get_next_nodes(nodes, source) {
            None => 0,
            Some(next_nodes) => {
                let num_paths = next_nodes
                    .iter()
                    .map(|n| calc_num_valid_paths(nodes, *n, destination, cache))
                    .sum();

                cache.insert(source, num_paths);

                num_paths
            }
        }
    }
}
