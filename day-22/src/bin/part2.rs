use std::collections::{ HashMap, HashSet, VecDeque };

fn main() {
    let input = include_str!("./input22.txt");
    let output = part2(input);
    dbg!(output);
}

type BrickStructure = HashMap<usize, (Vec<usize>, Vec<usize>)>;

#[derive(Clone, Debug, PartialEq)]
struct Cube {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Cube {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Cube { x, y, z }
    }

    fn lower(&mut self) {
        self.z -= 1
    }

    fn raise(&mut self) {
        self.z += 1
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Brick {
    pub cubes: Vec<Cube>,
    pub id: usize,
}

impl Brick {
    fn new(start: Cube, end: Cube, id: usize) -> Self {
        let mut cubes: Vec<Cube> = Vec::new();
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    cubes.push(Cube::new(x, y, z));
                }
            }
        }
        Brick { cubes, id }
    }

    fn min_z(&self) -> usize {
        let mut min: usize = self.cubes[0].z;
        for cube in &self.cubes {
            if cube.z < min {
                min = cube.z;
            }
        }
        min
    }

    fn max_z(&self) -> usize {
        let mut max: usize = self.cubes[0].z;
        for cube in &self.cubes {
            if cube.z > max {
                max = cube.z;
            }
        }
        max
    }

    fn lower(&mut self) {
        let mut deez: Vec<Cube> = Vec::new();
        for mut cube in self.cubes.clone() {
            cube.lower();
            deez.push(cube);
        }
        self.cubes = deez;
    }

    fn movable(&self, bricks: VecDeque<Brick>) -> bool {
        for brick in bricks {
            for cube in &self.cubes {
                let mut deez: Cube = cube.clone();
                deez.lower();
                if brick.cubes.contains(&deez) {
                    return false;
                }
            }
        }
        true
    }

    fn supporting(&self, bricks: &VecDeque<Brick>) -> Vec<usize> {
        let mut supporting: Vec<usize> = Vec::new();
        for brick in bricks.iter().filter(|x| x.min_z() == self.max_z() + 1) {
            for cube in &self.cubes {
                let mut nuts = cube.clone();
                nuts.raise();
                if brick.cubes.contains(&nuts) {
                    supporting.push(brick.id);
                    break;
                }
            }
        }
        supporting
    }

    fn supported_by(&self, bricks: &VecDeque<Brick>) -> Vec<usize> {
        let mut supported_by: Vec<usize> = Vec::new();
        for brick in bricks.iter().filter(|x| x.max_z() == self.min_z() - 1) {
            for cube in &self.cubes {
                let mut nuts = cube.clone();
                nuts.lower();
                if brick.cubes.contains(&nuts) {
                    supported_by.push(brick.id);
                    break;
                }
            }
        }
        supported_by
    }

}

fn part2(input: &str) -> usize {
    let lines = input.lines();
    let n_bricks: usize = lines.clone().count();
    let mut bricks: VecDeque<Brick> = VecDeque::new();
    let mut secret: usize = 0;

    println!("*********************\nParsing input\n*********************");
    for (id, line) in lines.clone().enumerate() {
        let start: Vec<usize> = line.split('~').next().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
        let start_cube: Cube = Cube::new(start[0], start[1], start[2]);

        let end: Vec<usize> = line.split('~').last().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
        let end_cube: Cube = Cube::new(end[0], end[1], end[2]);

        // Logging progress
        bricks.push_back(Brick::new(start_cube, end_cube, id));
        if (id+1) % 200 == 0 || id == n_bricks - 1 {
            println!("Parsed {}/{} bricks\t({}%)", id+1, n_bricks, ((id+1)*100)/n_bricks);
        }
    }
    bricks.make_contiguous().sort_by(|a, b| a.min_z().cmp(&b.min_z()));
    println!("Done\n");

    println!("*********************\nLetting bricks fall\n*********************");
    let mut fallen_bricks: VecDeque<Brick> = VecDeque::new();

    for i in 1..=bricks.len()  {
        let mut brick = bricks.pop_front().unwrap();
        while brick.min_z() > 1 && brick.movable(fallen_bricks.clone()) {
            brick.lower();
        }
        fallen_bricks.push_back(brick);

        // Logging progress
        if i % 200 == 0 || i == n_bricks {
            println!("Let {}/{} bricks fall\t({}%)", i, n_bricks, (i*100)/n_bricks);
        }
    }
    println!("Done\n");

    println!("*********************\nAnalysing brick structure\n*********************");
    let mut brick_structure: BrickStructure = HashMap::new();
    for (i, brick) in fallen_bricks.clone().into_iter().enumerate() {
        let supporting: Vec<usize> = brick.supporting(&fallen_bricks);
        let supported_by: Vec<usize> = brick.supported_by(&fallen_bricks);

        brick_structure.insert(brick.id, (supported_by, supporting));

        // Logging progress
        if (i+1) % 200 == 0 || i == n_bricks - 1 {
            println!("Analysed {}/{} bricks\t({}%)", i+1, n_bricks, ((i+1)*100)/n_bricks);
        }
    }
    println!("Done\n");

    println!("*********************\nIdentifying loaded bricks\n*********************");
    let mut loaded_bricks: HashSet<usize> = HashSet::new();
    for i in 0..n_bricks {
        loaded_bricks.insert(i);
    }

    for (id, (supported_by, supporting)) in brick_structure.clone() {
        if supported_by.len() >= 2 {
            for deez in supported_by {
                let mut removable: bool = true;
                for nuts in &brick_structure.get(&deez).unwrap().1 {
                    if brick_structure.get(&nuts).unwrap().0.len() == 1 {
                        removable = false;
                        break;
                    }
                }
                if removable {
                    loaded_bricks.remove(&deez);
                }
            }
        }
        if supporting.len() == 0 {
            loaded_bricks.remove(&id);
        }

    }
    println!("Found {} loaded bricks\n", loaded_bricks.len());


    println!("*********************\nDisintegrating loaded bricks\n*********************");
    for loaded in loaded_bricks.clone() {
        let mut fallen: HashSet<usize> = HashSet::new();
        let mut fall_queue: VecDeque<usize> = VecDeque::new();
        fall_queue.push_back(loaded);
        fallen.insert(loaded);
        while !fall_queue.is_empty() {
            let foo: usize = fall_queue.pop_front().unwrap();
            for nuts in &brick_structure.get(&foo).unwrap().1 {
                let mut supported: bool = false;
                for deez in &brick_structure.get(&nuts).unwrap().0 {
                    if !fallen.contains(deez) {
                        supported = true;
                    }
                }
                if !fallen.contains(nuts) && !supported {
                    secret += 1;
                    fall_queue.push_back(*nuts);
                    fallen.insert(*nuts);
                }
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
        let result = part2("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9");
        assert_eq!(result, 7);
    }
}
