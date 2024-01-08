fn main() {
    let input = include_str!("./input24.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct HailStone {
    pub pos: (i128, i128, i128),
    pub vel: (i128, i128, i128),
}

impl HailStone {
    fn new(pos: (i128, i128, i128), vel: (i128, i128, i128)) -> Self {
        Self { pos, vel }
    }

    fn to_vec(&self) -> [i128; 6] {
        [self.pos.0, self.pos.1, self.pos.2, self.vel.0, self.vel.1, self.vel.2]
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut hail_storm: Vec<HailStone> = Vec::new();

    for _ in 0..3 {
        let mut parts = lines.next().unwrap().split("@");
        let pos: Vec<i128> = parts.next().unwrap().split(",").map(|s| s.trim().parse().unwrap()).collect();
        let vel: Vec<i128> = parts.next().unwrap().split(",").map(|s| s.trim().parse().unwrap()).collect();

        hail_storm.push(HailStone::new((pos[0], pos[1], pos[2]), (vel[0], vel[1], vel[2])));
    }

    let [x1, y1, z1, vx1, vy1, vz1] = hail_storm[0].to_vec();
    let [x2, y2, z2, vx2, vy2, vz2] = hail_storm[1].to_vec();
    let [x3, y3, z3, vx3, vy3, vz3] = hail_storm[2].to_vec();

    let mut matrix = [
        [0, vz2 - vz1, vy1 - vy2, 0, z1 - z2, y2 - y1, vy1 * z1 - y1 * vz1 + y2 * vz2 - vy2 * z2],
        [0, vz3 - vz1, vy1 - vy3, 0, z1 - z3, y3 - y1, vy1 * z1 - y1 * vz1 + y3 * vz3 - vy3 * z3],
        [vz1 - vz2, 0, vx2 - vx1, z2 - z1, 0, x1 - x2, vz1 * x1 - z1 * vx1 + z2 * vx2 - vz2 * x2],
        [vz1 - vz3, 0, vx3 - vx1, z3 - z1, 0, x1 - x3, vz1 * x1 - z1 * vx1 + z3 * vx3 - vz3 * x3],
        [vy2 - vy1, vx1 - vx2, 0, y1 - y2, x2 - x1, 0, vx1 * y1 - x1 * vy1 + x2 * vy2 - vx2 * y2],
        [vy3 - vy1, vx1 - vx3, 0, y1 - y3, x3 - x1, 0, vx1 * y1 - x1 * vy1 + x3 * vy3 - vx3 * y3],
    ];

    for pivot in 0..6 {
        for row in &mut matrix[pivot..] {
            if row[pivot] < 0 {
                row.iter_mut().for_each(|n| *n = -*n);
            }
        }

        loop {
            for row in &mut matrix[pivot..] {
                let mut factor: i128 = 0;

                for &next in &row[pivot..] {
                    if next != 0 {
                        if factor == 0 {
                            factor = next.abs();
                        } else {
                            factor = gcd(factor, next.abs());
                        }
                    }
                }

                row[pivot..].iter_mut().for_each(|c| *c /= factor); // normalize rows to not blow
                                                                    // up
            }

            let column = matrix.map(|row| row[pivot]);

            if column[pivot..].iter().filter(|&&c| c > 0).count() == 1 {
                let index: usize = column.iter().rposition(|&c| c > 0).unwrap();
                matrix.swap(pivot, index);
                break;
            }

            let min: i128 = *column[pivot..].iter().filter(|&&c| c > 0).min().unwrap();
            let index: usize = column.iter().rposition(|&c| c == min).unwrap();

            for row in pivot..6 {
                if row != index && column[row] != 0 {
                    let factor: i128 = column[row] / min;

                    for col in pivot..7 {
                        matrix[row][col] -= factor * matrix[index][col];
                    }
                }
            }
        }
    }

    for pivot in (0..6).rev() {
        matrix[pivot][6] /= matrix[pivot][pivot];

        for row in 0..pivot {
            matrix[row][6] -= matrix[pivot][6] * matrix[row][pivot];
        }
    }

    (matrix[0][6] + matrix[1][6] + matrix[2][6]) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3");
        assert_eq!(result, 47);
    }
}
