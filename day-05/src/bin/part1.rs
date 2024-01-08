fn main() {
    let input = include_str!("./input5.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct RangeMap {
    pub dst: usize,
    pub src: usize,
    pub length: usize,
}

impl RangeMap {
    fn new(dst: usize, src: usize, length: usize) -> RangeMap {
        RangeMap {
            dst,
            src,
            length,
        }
    }

    fn contains_src(&self, src: usize) -> bool {
        self.src <= src && src < self.src + self.length
    }

    fn map(&self, src: usize) -> usize {
        self.dst + src - self.src
    }
}

#[derive(Debug, Clone)]
struct IntMap {
    pub ranges: Vec<RangeMap>,
}

impl IntMap {
    fn new(mut ranges: Vec<RangeMap>) -> IntMap {
        ranges.sort_by_key(|r| r.src);

        IntMap{ ranges }
    }

    fn get(&self, src: usize) -> usize {
        match self.ranges.iter().find(|r| r.contains_src(src)) {
            Some(r) => r.map(src),
            None => src,
        }
    }
}

#[derive(Debug, Clone)]
struct MapPipeline {
    pub maps: Vec<IntMap>,
}

impl MapPipeline {
    fn new(maps: Vec<IntMap>) -> MapPipeline {
        MapPipeline{ maps }
    }

    fn get(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |v, m| m.get(v))
    }
}

fn part1(input: &str) -> usize {
    let mut groups = input.split("\n\n");

    let seeds: Vec<usize> = groups.next().unwrap().split(":").nth(1).unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    let mut maps: Vec<IntMap> = Vec::new();
    for group in groups {
        let lines = group.split(":\n").nth(1).unwrap().lines();
        let mut ranges: Vec<RangeMap> = Vec::new();
        for line in lines {
            let range: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            ranges.push(RangeMap::new(range[0], range[1], range[2]));
        }
        maps.push(IntMap::new(ranges));
    }
    let pipeline: MapPipeline = MapPipeline::new(maps);

    seeds.into_iter().map(|s| pipeline.get(s)).min().unwrap_or_default()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(result, 35);
    }
}

