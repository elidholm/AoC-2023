fn main() {
    let input = include_str!("./input24.txt");
    let output = part1(input, 200000000000000, 400000000000000);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct HailStone2D {
    pub pos: (f64, f64),
    pub vel: (f64, f64),
}

impl HailStone2D {
    fn new(pos: (f64, f64), vel: (f64, f64)) -> Self {
        Self { pos, vel }
    }
}

fn collide(s1: HailStone2D, s2: HailStone2D, min: f64, max: f64) -> bool {
    if s2.vel.1*s1.vel.0 - s1.vel.1*s2.vel.0 == 0.0 {
        return false;
    }
    let t: f64 = (s1.vel.1*(s2.pos.0 - s1.pos.0) - s1.vel.0*(s2.pos.1 - s1.pos.1))/(s2.vel.1*s1.vel.0 - s1.vel.1*s2.vel.0);
    let s: f64 = (s2.pos.0 - s1.pos.0 + s2.vel.0*t)/s1.vel.0;

    let collision_x: f64 = s1.pos.0 + s1.vel.0*s;
    let collision_y: f64 = s1.pos.1 + s1.vel.1*s;

    return t >= 0.0 && s >= 0.0 && collision_x >= min && collision_x <= max && collision_y >= min && collision_y <= max;
}

fn part1(input: &str, min: usize, max: usize) -> usize {
    let lines = input.lines();
    let mut secret: usize = 0;

    let mut hail_storm: Vec<HailStone2D> = Vec::new();

    for line in lines {
        let mut parts = line.split("@");
        let pos: Vec<f64> = parts.next().unwrap().split(",").map(|s| s.trim().parse().unwrap()).collect();
        let vel: Vec<f64> = parts.next().unwrap().split(",").map(|s| s.trim().parse().unwrap()).collect();

        hail_storm.push(HailStone2D::new((pos[0], pos[1]), (vel[0], vel[1])));
    }

    for (i, s1) in hail_storm.iter().enumerate() {
        for s2 in hail_storm.iter().skip(i + 1) {
            if collide(s1.clone(), s2.clone(), min as f64, max as f64) {
                secret += 1;
            }
        }
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3", 7, 27);
        assert_eq!(result, 2);
    }
}
