use std::{collections::HashSet, fs::write, io::{stdout, Write}};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point(u32, u32);

#[derive(Debug)]
struct Attractors {
    types: Vec<Vec<Point>>,
    elements: Vec<HashSet<Point>>
}

impl Attractors {

    fn new() -> Self {
        let mut instance: Self = Self { types: Vec::new(), elements: Vec::new() };

        // Create "0th loop type" for point chains that don't contain a loopback.
        instance.types.push(Vec::new());
        instance.elements.push(HashSet::new());

        instance
    }

    fn add_converged_chain(&mut self, point_chain: Vec<Point>) -> usize {

        // Check whether point_chain contains a loopback to an already existing item.
        let new_last_point: Point = collatz_step(&point_chain[point_chain.len() - 1]);
        let point_idx_opt: Option<usize> = point_chain.iter().position(|x| *x == new_last_point);

        match point_idx_opt {

            // No loopback found. This defaults to the "0th loop type".
            Option::None => {
                for point in point_chain {
                    self.elements[0].insert(point);
                }
                return 0;
            },

            // Loopback found at index point_idx.
            Option::Some(point_idx) => {

                // Check whether we already know the loop type.
                let attr_type: Vec<Point> = Vec::from(&point_chain[point_idx..]);
                let attr_idx_opt: Option<usize> = self.types.iter().position(|x| cyclic_eq(&attr_type, x));

                match attr_idx_opt {

                    // We don't know the loop type yet, so we add it to the types.
                    Option::None => {

                        self.types.push(attr_type);  // Add new loop type...
                        self.elements.push(HashSet::new());  // Add container for new loop type...

                        let last_idx: usize = self.elements.len() - 1;
                        for point in point_chain { self.elements[last_idx].insert(point); }
                        return last_idx;
                    },

                    // We already know the loop type at index attr_idx.
                    Option::Some(attr_idx) => {
                        for point in point_chain { self.elements[attr_idx].insert(point); }
                        return attr_idx;
                    }
                }

            }
        }
    }

    fn add_chain_to_attr(&mut self, point_chain: Vec<Point>, type_idx: usize) -> () {
        for point in point_chain {
            self.elements[type_idx].insert(point);
        }
    }

    fn find(&self, point: &Point) -> Option<usize> {
        for (idx, hset) in self.elements.iter().enumerate() {
            if (*hset).contains(point) {
                return Option::Some(idx);
            }
        }
        Option::None
    }
}

fn collatz_step(start_point: &Point) -> Point {

    let parity: (bool, bool) = (start_point.0 % 2 == 0, start_point.1 % 2 == 0);

    if parity.0 && parity.1 {
        Point(start_point.0 / 2, start_point.1 / 2)
    } else if ! parity.0 && ! parity.1 {
        Point(start_point.0 * 3 + 1, start_point.1 * 3 + 1)
    } else {
        if start_point.0 > start_point.1 {
            Point(start_point.0 + 1, start_point.1)
        } else {
            Point(start_point.0, start_point.1 + 1)
        }
    }
}

fn cyclic_eq<T: Eq>(a: &Vec<T>, b: &Vec<T>) -> bool {

    if a.len() != b.len() { return false; }

    let b_offset_opt: Option<usize> = b.iter().position(|x| *x == a[0]);
    match b_offset_opt {
        Option::None => false,
        Option::Some(b_offset) => {
            for idx_a in 0..a.len() {
                let idx_b = (idx_a + b_offset) % b.len();
                if a[idx_a] != b[idx_b] { return false; }
            }
            true
        }
    }
}

fn main() {

    const RECTANGLE_SIZE: u32 = 2000;
    const MAX_ITER: usize = 10000;
    
    let mut attrs: Attractors = Attractors::new();

    for idx1 in 0..RECTANGLE_SIZE {
        for idx2 in idx1..RECTANGLE_SIZE {

            let mut points_traversed: Vec<Point> = Vec::new();
            points_traversed.push(Point(idx1, idx2));

            for iteration in 0..MAX_ITER {

                let new_point: Point = collatz_step(&points_traversed[iteration]);

                // Chain converged.
                if points_traversed.contains(&new_point) {
                    attrs.add_converged_chain(points_traversed);
                    break;
                }

                // Chain hasn't converged yet. Check whether we know the new_point already.
                match attrs.find(&new_point) {

                    // We haven't encountered this point yet. Add it to the point chain.
                    Option::None => {
                        points_traversed.push(new_point);
                    },

                    // We have already encountered this point with an attractor type attr_idx.
                    Option::Some(attr_idx) => {
                        attrs.add_chain_to_attr(points_traversed, attr_idx);
                        break;
                    }
                }

                // The point_chain didn't converge. Add it to the 0th (unconverged) loopback type.
                if iteration == MAX_ITER - 1 {
                    attrs.add_chain_to_attr(points_traversed, 0);
                    break;
                }
            }

        }
    }

    // CLI prints
    for attr_idx in 0..attrs.types.len() {
        println!("Attractor type {} has {} elements.", attr_idx, attrs.elements[attr_idx].len());
        print!("\tCycle: ");
        for point in attrs.types[attr_idx].iter() {
            print!("{:?} -> ", *point);
        }
        stdout().flush().unwrap();
        println!("...");
    }

    // Saving into file
    let mut str_out: String = String::new();
    for idx1 in 0..RECTANGLE_SIZE {
        str_out.push_str(&"\t".repeat(idx1 as usize));
        for idx2 in idx1..RECTANGLE_SIZE {
            let current_value = format!("{}", attrs.find(&Point(idx1, idx2)).unwrap());
            str_out.push_str(&current_value);
            str_out.push_str("\t");
        }
        str_out.pop();
        str_out.push_str("\n");
    }
    write("./results.txt", str_out).expect("Something went wrong during writing!");
}
