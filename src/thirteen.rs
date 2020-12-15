use std::io::{self, BufRead};

pub fn part_one() {
    let (current_time, buses) = read_input(io::stdin().lock());
    let answer = get_product_of_next_bus_id_and_wait_time(current_time, &buses).unwrap();

    println!("{}", answer);
}

pub fn part_two() {}

#[derive(Clone, Copy)]
struct Bus {
    id: u32,
}

impl Bus {
    fn get_next_time_after(&self, t: u32) -> u32 {
        if t % self.id == 0 {
            t
        } else {
            (t / self.id) * self.id + self.id
        }
    }
}

fn read_input<R>(reader: R) -> (u32, Vec<Option<Bus>>)
where
    R: BufRead,
{
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let current_time = lines[0].parse::<u32>().unwrap();
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
    current_time: u32,
    buses: &[Option<Bus>],
) -> Option<u32> {
    buses
        .iter()
        .filter(|bus| bus.is_some())
        .map(|bus| bus.unwrap())
        .map(|bus| (bus, bus.get_next_time_after(current_time)))
        .min_by(|(_, t_next_a), (_, t_next_b)| t_next_a.cmp(t_next_b))
        .map(|(bus, t_next)| bus.id * (t_next - current_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bus_get_next_time_after_same() {
        let bus = Bus { id: 10 };

        let expected = 20;
        let actual = bus.get_next_time_after(20);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bus_get_next_time_after_diff() {
        let bus = Bus { id: 10 };

        let expected = 20;
        let actual = bus.get_next_time_after(15);

        assert_eq!(expected, actual);
    }
}
