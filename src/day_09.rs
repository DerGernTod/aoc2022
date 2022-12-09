use std::{fs, collections::HashSet, ops::Add, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coords(i32, i32);

impl Coords {
    pub fn calc_move_towards(&self, coord: &Coords) -> Coords {
        let hori_dist = coord.0 - self.0;
        let vert_dist = coord.1 - self.1;
        if hori_dist.abs().max(vert_dist.abs()) > 1 && hori_dist.abs().min(vert_dist.abs()) >= 1 {
            self.add(Coords(
                hori_dist.clamp(-1, 1),
                vert_dist.clamp(-1, 1)))
        } else if hori_dist.abs() > 1 {
            self.add(Coords(hori_dist.clamp(-1, 1), 0))
        } else if vert_dist.abs() > 1 {
            self.add(Coords(0, vert_dist.clamp(-1, 1)))
        } else {
            *self
        }
    }
}
impl Add<Coords> for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Coords({},{})", self.0, self.1))
    }
}
pub fn day_09() {
    let head_coords = read_into_head_coords("./input/day_09.txt");
    let tail_coords = calc_tail_coords(&head_coords);
    let multi_tail_coords = calc_multi_knot_coords(&head_coords);
    println!("Num tail positions: {}\n num multi positions: {}", tail_coords.len(), multi_tail_coords.len());
}

fn print_map(min: &Coords, max: &Coords, head: &Coords, knots: &[Coords]) {
    println!();
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            let cur_coords = Coords(x, y);
            if head == &cur_coords {
                print!("H");
            } else if let Some(knot_id) = knots.iter().position(|knot| knot == &cur_coords) {
                print!("{}", knot_id);
            } else if cur_coords == Coords(0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn calc_multi_knot_coords(coords: &[Coords]) -> HashSet<Coords> {
    let mut min = Coords(0, 0);
    let mut max = Coords(0, 0);

    let (mut tail_coords, _) = coords.iter()
        .fold((HashSet::new(), vec![Coords(0, 0);9]), |(mut tail_coords, knots), head| {
            let mut cur_head = *head;
            let mut new_knots = vec![];
            for cur_tail in knots {
                let new_tail = cur_tail.calc_move_towards(&cur_head);
                min = Coords(min.0.min(head.0 - 1).min(new_tail.0 - 1), min.1.min(head.1 - 1).min(new_tail.1 - 1));
                max = Coords(max.0.max(head.0 + 1).max(new_tail.0 + 1), max.1.max(head.1 + 1).max(new_tail.1 + 1));
                cur_head = new_tail;
                new_knots.push(new_tail);
            }
            // print_map(&min, &max, head, &new_knots);
            
            tail_coords.insert(*new_knots.last().unwrap());
            (tail_coords, new_knots)
        });
    tail_coords.insert(Coords(0, 0));
    tail_coords
}

fn calc_tail_coords(coords: &[Coords]) -> HashSet<Coords> {
    let mut min = Coords(0, 0);
    let mut max = Coords(0, 0);
    let (mut tail_coords, _) = coords.iter()
        .fold((HashSet::new(), Coords(0, 0)), |(mut tail_coords, tail), head| {
            let new_tail = tail.calc_move_towards(head);
            min = Coords(min.0.min(head.0 - 1).min(new_tail.0 - 1), min.1.min(head.1 - 1).min(new_tail.1 - 1));
            max = Coords(max.0.max(head.0 + 1).max(new_tail.0 + 1), max.1.max(head.1 + 1).max(new_tail.1 + 1));
            // print_map(&min, &max, head, &new_tail);
            
            tail_coords.insert(new_tail);
            (tail_coords, new_tail)
        });
    tail_coords.insert(Coords(0, 0));
    tail_coords
}

fn read_into_head_coords(path: &str) -> Vec<Coords> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .flat_map(|line| {
            let mut iter = line.split(' ');
            let ch = iter.next().unwrap().chars().next().unwrap();
            let times = iter.next().unwrap().parse::<usize>().unwrap();
            vec![ch; times]
        })
        .fold(vec![Coords(0, 0)], |mut positions, ch| {
            let last_pos: &Coords = positions.last().unwrap();
            let mut new_pos: Coords = *last_pos;
            match ch {
                'U' => new_pos.1 -= 1,
                'R' => new_pos.0 += 1,
                'L' => new_pos.0 -= 1,
                'D' => new_pos.1 += 1,
                _ => panic!("Invalid character to move: {ch}"),
            }
            positions.push(new_pos);
            positions
        })

}

#[cfg(test)]
mod tests {
    use crate::day_09::Coords;

    use super::{read_into_head_coords, calc_tail_coords, calc_multi_knot_coords};

    #[test]
    fn test_calc_move_towards() {
        let tail = Coords(3, 0);
        let new_tail = tail.calc_move_towards(&Coords(4, -1));
        assert_eq!(tail, new_tail);
        let new_tail = new_tail.calc_move_towards(&Coords(4, -2));
        assert_eq!(new_tail, Coords(4, -1));
    }
    #[test]
    fn test_part_1() {
        let head_coords = read_into_head_coords("./input/day_09.test.txt");
        let tail_coords = calc_tail_coords(&head_coords);
        assert!(tail_coords.contains(&Coords(0, 0)));
        assert!(tail_coords.contains(&Coords(1, 0)));
        assert!(tail_coords.contains(&Coords(2, 0)));
        assert!(tail_coords.contains(&Coords(3, 0)));
        assert!(tail_coords.contains(&Coords(4, -1)));
        assert!(tail_coords.contains(&Coords(4, -2)));
        assert!(tail_coords.contains(&Coords(4, -3)));
        assert!(tail_coords.contains(&Coords(3, -4)));
        assert!(tail_coords.contains(&Coords(2, -4)));
        assert!(tail_coords.contains(&Coords(3, -3)));
        assert!(tail_coords.contains(&Coords(3, -2)));
        assert!(tail_coords.contains(&Coords(2, -2)));
        assert!(tail_coords.contains(&Coords(1, -2)));
        assert_eq!(tail_coords.len(), 13);
    }

    
    #[test]
    fn test_part_2() {
        let head_coords = read_into_head_coords("./input/day_09.2.test.txt");
        let tail_coords = calc_multi_knot_coords(&head_coords);
        assert_eq!(tail_coords.len(), 36);
    }
}