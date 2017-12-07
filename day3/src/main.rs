use std::collections::BTreeMap;
use std::iter::Iterator;

fn side_len(loc: u64) -> u64 {
    let side_len = (loc as f64).sqrt().ceil() as u64;
    if side_len % 2 != 0 {
        side_len
    } else {
        side_len + 1
    }
}

fn spiral_access(location: u64) -> u64 {
    if location == 1 {
        return 0;
    }


    let side_len = side_len(location);
    let side = location as i64 - (side_len as i64 - 2).pow(2);
    let side_offset = side % (side_len as i64 - 1);
    let steps_to_center = (side_len - 1) / 2;
    steps_to_center as u64 + ((side_offset as i64) - (steps_to_center as i64)).abs() as u64
}

#[test]
fn test_spiral_access() {
    assert_eq!(0, spiral_access(1));
    assert_eq!(3, spiral_access(12));
    assert_eq!(2, spiral_access(23));
    assert_eq!(31, spiral_access(1024));
}

const NEIGHBOR_OFFSETS: [(i64, i64); 8] = [
    (-1, 1), (0, 1), (1, 1),
    (-1, 0)        , (1, 0),
    (-1, -1), (0, -1), (1, -1)
];

#[derive(Debug, Eq, PartialEq)]
struct Spiral {
    cache: BTreeMap<(i64, i64), u64>,
    last: (i64, i64),
    direction: (i64, i64),
}

impl Spiral {
    fn new() -> Spiral {
        Spiral {
            cache: BTreeMap::new(),
            last: (0, 0),
            direction: (1, 0),
        }
    }

    fn sum_to(&mut self, target: u64) -> usize {
        self.find(|&i| i >= target).expect("infinite iterator") as usize
    }
}

impl Iterator for Spiral {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_empty() {
            self.cache.insert((0, 0), 1);
            return Some(1)
        }

        self.last = {
            let (i, j) = self.last;
            let quadrant4 = i >= 0 && j <= 0;

            if !quadrant4 && i.abs() == j.abs()
                || quadrant4 && i - 1 == -j
            {
                self.direction = (-self.direction.1, self.direction.0);
            }

            (i + self.direction.0, j + self.direction.1)
        };

        let sum = NEIGHBOR_OFFSETS.iter()
            .map(|&(x_off, y_off)| {
                let (x, y) = self.last;
                self.cache.get(&(x + x_off, y + y_off)).unwrap_or(&0)
            }).sum();

        self.cache.insert(self.last, sum);
        Some(sum)
    }
}


#[test]
fn test_spiral_sum() {
    let mut spiral = Spiral::new();
    assert_eq!(1, spiral.next().unwrap());
    assert_eq!(1, spiral.next().unwrap());
    assert_eq!(2, spiral.next().unwrap());
    assert_eq!(4, spiral.next().unwrap());
    assert_eq!(5, spiral.next().unwrap());
}

fn main() {
    let input = 361527;
    println!("Spiral access: {}", spiral_access(input));
    println!("Spiral sum: {}", Spiral::new().sum_to(input));
}
