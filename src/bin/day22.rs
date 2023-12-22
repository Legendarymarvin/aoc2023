use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("./inputs/input22");
    let (part1, part2, part2_why) = solve(input, true);
    println!("Part1: {} , Part2: {} , Why is this not part2: {}", part1, part2, part2_why);
}


fn solve(p0: &str, _part2: bool) -> (usize, usize, usize) {
    let mut resting = vec![];
    let mut falling: Vec<Brick> = p0.lines().filter_map(|line| {
        if line.is_empty() { return None };
        let (start, end) = line.split_once("~").unwrap();
        Some(Brick {start: parse_coords(start), end:parse_coords(end)})
    }).collect();
    loop {
        let mut counter = 0;
        loop {
            let still_falling = falling.len();
            falling = falling.into_iter().filter(|&brick| {
                return if is_touching_resting(brick, &resting) {
                    resting.push(brick.clone());
                    false
                } else {
                    true
                }
            }).collect();
            if falling.len() == still_falling {
                counter += 1;
            }
            // pointless, making sure we don't miss anything after a falling step.
            if counter == 2 {
                break;
            }
        }
        if falling.len() == 0 {
            break;
        }
        falling = falling.into_iter().map(|brick| {
            Brick {
                start: Coord {z: brick.start.z - 1, ..brick.start},
                end: Coord{z: brick.end.z - 1, ..brick.end}
            }
        }).collect();
    }

    let (can_go, cannot_go): (Vec<Brick>, Vec<Brick>) = resting.iter().partition(|&brick| {
        can_be_disintegrated(&brick, &resting)
    });
    let old_part2 = cannot_go.clone().into_iter().map(|brick| {
        get_falling(&brick, &resting, &Vec::new()).iter().sorted().dedup().count()
    }).sum::<usize>();
    let part2 = cannot_go.into_iter().map(|brick| {
        let mut falling = HashSet::new();
        find_falling(&brick, &resting, &mut falling);
        falling.len()
    }).sum::<usize>();
    (can_go.len(), part2, old_part2)
}

fn find_falling(brick: &Brick, resting: &Vec<Brick>, falling: &mut HashSet<Brick>) {
    let no_other_support: Vec<Brick> = get_lying_on_brick(brick, resting).into_iter().filter(|brick_on_top| {
        brick_on_top != brick && !falling.contains(brick_on_top) && resting.iter().all(|potential_support|{
            potential_support == brick || potential_support == brick_on_top || falling.contains(potential_support) || !lies_on_top(brick_on_top, potential_support)
        })
    }).collect();
    if no_other_support.is_empty() {
        return;
    }

    falling.extend(no_other_support.clone());

    no_other_support.into_iter()
        .for_each(|brick| find_falling(&brick, resting, falling));
}

fn get_falling(p0: &Brick, p1: &Vec<Brick>, other_falling: &Vec<Brick>) -> Vec<Brick> {
    let lie_on_brick = get_lying_on_brick(p0, p1);
    let mut would_fall_if_removed: Vec<Brick> = lie_on_brick.into_iter().filter(|cand| {
        p1.iter().all(|support| {
            support == cand || support == p0 || other_falling.contains(support) || !lies_on_top(cand, support)
        })
    }).collect();

    if would_fall_if_removed.is_empty() {
        return vec![];
    }

    let mut already_falling = other_falling.to_vec();
    already_falling.append(&mut would_fall_if_removed.clone());

    would_fall_if_removed.clone().iter()
        .for_each(|brick| would_fall_if_removed.append(&mut get_falling(brick, p1, &already_falling)));

    would_fall_if_removed.append(&mut already_falling);
    would_fall_if_removed
}

fn can_be_disintegrated(p0: &Brick, p1: &Vec<Brick>) -> bool {
    let lie_on_me = get_lying_on_brick(p0, p1);
    if lie_on_me.len() == 0 {
        return true;
    }
    let have_other_support: Vec<Brick> = lie_on_me.clone().into_iter().filter(|&cand| {
        p1.clone().into_iter().any(|support| {
            support != *p0 && support != cand && lies_on_top(&cand, &support)
        })
    }).collect();
    have_other_support.len() == lie_on_me.len()
}

fn get_lying_on_brick(p0: &Brick, p1: &Vec<Brick>) -> Vec<Brick> {
    let lie_on_me: Vec<Brick> = p1.clone().into_iter().filter(|cand| {
        p0 != cand && lies_on_top(cand, p0)
    }).collect();
    lie_on_me
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Brick {
    start: Coord,
    end: Coord
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}

fn is_touching_resting(brick: Brick, resting: &Vec<Brick>) -> bool {
    if brick.start.z == 1 || brick.end.z == 1 {
        return true;
    }
    for rests in resting.iter() {
        //println!("brick {:?}, rests {:?}, touches {}", &brick, &rests, is_touching(&brick, rests));
        if *rests != brick && lies_on_top(&brick, rests) {
            return true;
        }
    }
    false
}

fn lies_on_top(falling: &Brick, resting: &Brick) -> bool {
    let min_falling_z = falling.start.z.min(falling.end.z);
    let max_resting_z = resting.start.z.max(resting.end.z);
    if min_falling_z == (max_resting_z + 1) {
        do_vectors_intersect(&falling.start, &falling.end, &resting.start, &resting.end)
    } else {
        false
    }
}

// second attempt at checking vector intersection, because I thought it might be wrong.
// fn do_vectors_intersect(a: &Coord, b: &Coord, c: &Coord, d: &Coord) -> bool {
//     let a_min_x = a.x.min(b.x);
//     let a_max_x = a.x.max(b.x);
//     let a_min_y = a.y.min(b.y);
//     let a_max_y = a.y.max(b.y);
//
//     let b_min_x = c.x.min(d.x);
//     let b_max_x = c.x.max(d.x);
//     let b_min_y = c.y.min(d.y);
//     let b_max_y = c.y.max(d.y);
//
//     // Check if the bounding boxes intersect
//     let x_overlap = (a_min_x <= b_max_x) && (b_min_x <= a_max_x);
//     let y_overlap = (a_min_y <= b_max_y) && (b_min_y <= a_max_y);
//
//     x_overlap && y_overlap
// }
//

fn direction(a: &Coord, b: &Coord, c: &Coord) -> i32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn on_segment(a: &Coord, b: &Coord, c: &Coord) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    c.x >= min_x && c.x <= max_x && c.y >= min_y && c.y <= max_y
}

fn do_vectors_intersect(a: &Coord, b: &Coord, c: &Coord, d: &Coord) -> bool {
    let dir1 = direction(a, b, c);
    let dir2 = direction(a, b, d);
    let dir3 = direction(c, d, a);
    let dir4 = direction(c, d, b);

    if (dir1.signum() != dir2.signum()) && (dir3.signum() != dir4.signum()) {
        return true;
    }

    if dir1 == 0 && on_segment(a, b, c)
        || dir2 == 0 && on_segment(a, b, d)
        || dir3 == 0 && on_segment(c, d, a)
        || dir4 == 0 && on_segment(c, d, b)
    {
        return true;
    }

    false
}

fn parse_coords(p0: &str) -> Coord {
    let (x, y , z) = p0.split(",").collect_tuple().unwrap();
    Coord {x: x.parse::<i32>().unwrap(), y: y.parse::<i32>().unwrap(), z: z.parse::<i32>().unwrap()}
}

#[cfg(test)]
mod tests {
    use crate::{Brick, Coord, do_vectors_intersect, lies_on_top, solve};

    use indoc::indoc;

    #[test]
    fn desperation() {
        assert_eq!(true, do_vectors_intersect(&Coord{x: 0, y: 0, z: 2}, &Coord{x:2, y: 0, z: 2}, &Coord{x:0, y:0, z:0}, &Coord {x:0, y:0, z:1}));
        assert_eq!(true, do_vectors_intersect(&Coord{x: 0, y: 0, z: 2}, &Coord{x:0, y: 2, z: 2}, &Coord{x:0, y:0, z:0}, &Coord {x:0, y:0, z:1}));
        assert_eq!(true, lies_on_top(&Brick {start: Coord{x: 0, y: 0, z: 2}, end: Coord{x:0, y: 0, z: 3} }, &Brick {start: Coord{x:0, y:0, z:0}, end: Coord {x:0, y:0, z:1}}));
        assert_eq!(false, do_vectors_intersect(&Coord{x: 1, y: 1, z: 2}, &Coord{x:2, y: 1, z: 2}, &Coord{x:0, y:2, z:1}, &Coord {x:1, y:2, z:1}));
        assert_eq!(true, do_vectors_intersect(&Coord{x: 0, y: 1, z: 2}, &Coord{x:2, y: 1, z: 2}, &Coord{x:0, y:1, z:1}, &Coord {x:0, y:3, z:1}));
        assert_eq!(true, do_vectors_intersect(&Coord{x: 0, y: 1, z: 2}, &Coord{x:2, y: 1, z: 2}, &Coord{x:1, y:0, z:1}, &Coord {x:1, y:2, z:1}));
        assert_eq!(true, lies_on_top(&Brick {start: Coord{x: 0, y: 0, z: 2}, end: Coord{x:2, y: 0, z: 2} }, &Brick {start: Coord{x:0, y:2, z:1}, end: Coord {x:0, y:0, z:1}}));
        assert_eq!(false, lies_on_top(&Brick {start: Coord{x: 0, y: 0, z: 3}, end: Coord{x:2, y: 0, z: 3} }, &Brick {start: Coord{x:0, y:2, z:1}, end: Coord {x:0, y:0, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 0, y: 0, z: 2}, end: Coord{x:2, y: 0, z: 2}}, &Brick {start: Coord{x:0, y:0, z:0}, end: Coord {x:0, y:0, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 0, y: 0, z: 2}, end: Coord{x:0, y: 2, z: 2}}, &Brick {start: Coord{x:0, y:2, z:0}, end: Coord {x:0, y:2, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 0, y: 0, z: 2}, end: Coord{x:0, y: 0, z: 4}}, &Brick {start: Coord{x:0, y:2, z:1}, end: Coord {x:0, y:0, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 0, y: 2, z: 2}, end: Coord{x:0, y: 2, z: 4}}, &Brick {start: Coord{x:0, y:2, z:1}, end: Coord {x:0, y:0, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 0, y: 2, z: 2}, end: Coord{x:0, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:2, z:1}, end: Coord {x:0, y:0, z:1}}));
        assert_eq!(false, lies_on_top(&Brick{start: Coord{x: 0, y: 2, z: 2}, end: Coord{x:0, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:2, z:3}, end: Coord {x:0, y:0, z:3}}));
        assert_eq!(false, lies_on_top(&Brick{start: Coord{x: 0, y: 2, z: 2}, end: Coord{x:0, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:2, z:2}, end: Coord {x:0, y:0, z:2}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 1, y: 2, z: 2}, end: Coord{x:1, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:2, z:1}, end: Coord {x:2, y:2, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 1, y: 2, z: 2}, end: Coord{x:1, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:3, z:1}, end: Coord {x:3, y:2, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{x: 1, y: 2, z: 2}, end: Coord{x:1, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:4, z:1}, end: Coord {x:3, y:4, z:1}}));
        assert_eq!(false, lies_on_top(&Brick{start: Coord{x: 1, y: 2, z: 2}, end: Coord{x:1, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:5, z:1}, end: Coord {x:3, y:5, z:1}}));
        assert_eq!(false, lies_on_top(&Brick{start: Coord{x: 1, y: 2, z: 2}, end: Coord{x:1, y: 4, z: 2}}, &Brick {start: Coord{x:0, y:1, z:1}, end: Coord {x:3, y:1, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{y: 1, x: 2, z: 2}, end: Coord{y:1, x: 4, z: 2}}, &Brick {start: Coord{y:0, x:2, z:1}, end: Coord {y:2, x:2, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{y: 1, x: 2, z: 2}, end: Coord{y:1, x: 4, z: 2}}, &Brick {start: Coord{y:0, x:3, z:1}, end: Coord {y:3, x:2, z:1}}));
        assert_eq!(true, lies_on_top(&Brick{start: Coord{y: 1, x: 2, z: 2}, end: Coord{y:1, x: 4, z: 2}}, &Brick {start: Coord{y:0, x:4, z:1}, end: Coord {y:3, x:4, z:1}}));
        assert_eq!(false, lies_on_top(&Brick{start: Coord{y: 1, x: 2, z: 2}, end: Coord{y:1, x: 4, z: 2}}, &Brick {start: Coord{y:0, x:5, z:1}, end: Coord {y:3, x:5, z:1}}));
        assert_eq!(false, lies_on_top(&Brick{start: Coord{y: 1, x: 2, z: 2}, end: Coord{y:1, x: 4, z: 2}}, &Brick {start: Coord{y:0, x:1, z:1}, end: Coord {y:3, x:1, z:1}}));
    }

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let (result, _, _) = solve(input, false);
        assert_eq!(result, 5);
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let (_, result, _) = solve(input, true);
        assert_eq!(result, 7);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {
"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
    };
        input
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input22");
        let (part1, part2, _) = solve(input, false);
        assert_eq!(part1, 517);
        assert_eq!(part2, 61276);
    }
}
