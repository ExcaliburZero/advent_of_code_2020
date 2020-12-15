use std::io::{self, BufRead};

pub fn part_one() {
    let (current_time, buses) = read_input(io::stdin().lock());
    let answer = get_product_of_next_bus_id_and_wait_time(current_time, &buses).unwrap();

    println!("{}", answer);
}

pub fn part_two() {
    let (_, buses) = read_input(io::stdin().lock());
    let answer = get_earliest_perfect_bus_alignment_start_time(&buses);

    println!("{}", answer);
}

#[derive(Clone, Copy)]
struct Bus {
    id: u64,
}

impl Bus {
    fn is_valid_time(&self, t: u64) -> bool {
        (t % self.id) == 0
    }

    fn get_next_time_at_or_after(&self, t: u64) -> u64 {
        if t % self.id == 0 {
            t
        } else {
            (t / self.id) * self.id + self.id
        }
    }
}

fn read_input<R>(reader: R) -> (u64, Vec<Option<Bus>>)
where
    R: BufRead,
{
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let current_time = lines[0].parse::<u64>().unwrap();
    let buses = lines[1]
        .split(',')
        .map(|bus_id| match bus_id {
            "x" => None,
            _ => Some(Bus {
                id: bus_id.parse().unwrap(),
            }),
        })
        .collect();

    (current_time, buses)
}

fn get_product_of_next_bus_id_and_wait_time(
    current_time: u64,
    buses: &[Option<Bus>],
) -> Option<u64> {
    buses
        .iter()
        .filter(|bus| bus.is_some())
        .map(|bus| bus.unwrap())
        .map(|bus| (bus, bus.get_next_time_at_or_after(current_time)))
        .min_by(|(_, t_next_a), (_, t_next_b)| t_next_a.cmp(t_next_b))
        .map(|(bus, t_next)| bus.id * (t_next - current_time))
}

fn get_earliest_perfect_bus_alignment_start_time(buses: &[Option<Bus>]) -> u64 {
    // Filter out the out of service buses
    let mut actual_buses: Vec<(usize, Bus)> = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(i, b)| (i, b.unwrap()))
        .collect();

    // Start with the buses with high id's to make the search faster
    actual_buses.sort_by(|(_, bus_a), (_, bus_b)| (-(bus_a.id as i64)).cmp(&-(bus_b.id as i64)));

    // Given two buses with ids (n and m), once we find a time (t) that fits the pattern, then we
    // know that the next time (t) that will fit that pattern will be the previous t + (n * m).
    //
    // I figured this out by manually matching up some simple cases until I noticed this pattern.
    // To recreate this, try doing the pattern out manually for this case:
    //
    //     x,3,x,4,5
    //
    // 2	1	1
    // 5	5	6
    // 8	9	11
    // 11	13	16
    // 14	17	21
    // 17	21	26
    // 20	25	31
    // 23	29	36
    // 26	33	41
    // 29	37	46
    // 32	41	51
    // 35	45	56
    // 38	49	61
    // 41	53	66
    // 44	57	71
    // 47	61	76
    // 50	65	81
    // 53	69	86
    // 56	73	91
    // 59	77	96
    // 62	81	101
    // 65	85	106
    // 68	89	111
    // 71	93	116
    // 74	97	121
    // 77	101	126
    // 80	105	131
    // 83	109	136
    // 86	113	141
    // 89	117	146
    // 92	121	151
    // 95	125	156
    // 98	129	161
    // 101	133	166
    let first_bus_with_index = actual_buses.first().unwrap();
    let mut t = first_bus_with_index.1.id - first_bus_with_index.0 as u64;
    let mut increment = first_bus_with_index.1.id;
    for (i, bus) in actual_buses[1..].iter() {
        while !bus.is_valid_time(t + *i as u64) {
            t += increment;
        }

        increment *= bus.id;
    }

    t
}

// Attemped Mixed Integer Linear Programming formulation, works but way too slow
/*
fn get_earliest_perfect_bus_alignment_start_time(buses: &[Option<Bus>]) -> u64 {
    let actual_buses: Vec<(usize, Bus)> = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(i, b)| (i, b.unwrap()))
        .collect();

    let mut problem = LpProblem::new("Bus timing problem", LpObjective::Minimize);

    // Objective function
    let t = &LpInteger::new("t");
    problem += t;

    // Constraints
    problem += t.ge(0);
    problem += t.le(100_000_000_000_000.0);

    for (i, bus) in actual_buses.iter() {
        let variable = &LpInteger::new(&format!("mult_{}__{}", i, bus.id));

        problem += variable.ge(0);
        problem += (t - variable * bus.id as i32 + *i as i32).equal(0);
    }

    // Specify solver
    //let solver = CbcSolver::new();
    let solver = GlpkSolver::new();

    // Run optimization and process output hashmap
    match solver.run(&problem) {
        Ok(solution) => {
            println!("Status {:?}", solution.status);
            for (name, value) in solution.results.iter() {
                println!("value of {} = {}", name, value);
            }

            solution.results["t"] as u64
        }
        Err(msg) => {
            println!("{}", msg);
            panic!()
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bus_get_next_time_at_or_after_same() {
        let bus = Bus { id: 10 };

        let expected = 20;
        let actual = bus.get_next_time_at_or_after(20);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bus_get_next_time_at_or_after_diff() {
        let bus = Bus { id: 10 };

        let expected = 20;
        let actual = bus.get_next_time_at_or_after(15);

        assert_eq!(expected, actual);
    }
}
