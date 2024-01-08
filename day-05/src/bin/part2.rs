use std::ops::Range;

fn main() {
    let input = include_str!("./input5.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone, PartialEq)]
struct RangeMap {
    pub dst: usize,
    pub src: usize,
    pub length: usize,
}

impl RangeMap {
    fn new(dst: usize, src: usize, length: usize) -> Self {
        Self { dst, src, length }
    }

    fn from(vec: Vec<usize>) -> Self {
        if vec.len() != 3 {
            panic!("Invalid range map");
        }

        Self::new(vec[0], vec[1], vec[2])
    }

    fn contains_src(&self, src: usize) -> bool {
        self.src <= src && src < self.src + self.length
    }
    fn contains_dst(&self, dst: usize) -> bool {
        self.dst <= dst && dst < self.dst + self.length
    }
    fn map(&self, src: usize) -> usize {
        self.dst + src - self.src
    }
    fn reverse_map(&self, dst: usize) -> usize {
        self.src + dst - self.dst
    }
}

#[derive(Debug, Clone, PartialEq)]
struct IntMap {
    pub ranges: Vec<RangeMap>,
}

impl IntMap {
    fn new(mut ranges: Vec<RangeMap>) -> Self {
        ranges.sort_by_key(|r| r.src);

        Self { ranges }
    }

    fn get(&self, src: usize) -> usize {
        match self.ranges.iter().find(|r| r.contains_src(src)) {
            Some(r) => r.map(src),
            None => src,
        }
    }

    fn ranges_srcs(&self) -> impl Iterator<Item = usize> + '_ {
        std::iter::once(0).chain(self.ranges.iter().map(|r| r.src))
    }

    fn reverse_get(&self, dst: usize) -> usize {
        match self.ranges.iter().find(|r| r.contains_dst(dst)) {
            Some(r) => r.reverse_map(dst),
            None => dst,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MapPipeline {
    pub maps: Vec<IntMap>,
}

impl MapPipeline {
    fn new(maps: Vec<IntMap>) -> Self {
        Self { maps }
    }

    fn levels(&self) -> usize {
        self.maps.len()
    }

    fn get_src(&self, dst: usize, applied_maps_number: usize) -> usize {
        let mut value: usize = dst;
        for m in self.maps.iter().take(applied_maps_number).rev() {
            value = m.reverse_get(value);
        }
        value
    }

    fn get(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |v, m| m.get(v))
    }

    fn ranges_srcs(&self, map_index: usize) -> impl Iterator<Item = usize> + '_ {
        self.maps[map_index].ranges_srcs()
    }

    fn get_from_level(&self, src: usize, already_applied: usize) -> usize {
        self.maps[already_applied..].iter().fold(src, |v, m| m.get(v))
    }
}

fn part2(input: &str) -> usize {

    let mut groups = input.split("\n\n");

    let seed_data: Vec<usize> = groups.next().unwrap().split(":").nth(1).unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    let mut seed_ranges: Vec<Range<usize>> = Vec::new();
    for i in (0..seed_data.len()).step_by(2) {
        seed_ranges.push(seed_data[i]..(seed_data[i]+seed_data[i+1]));
    }

    let mut maps: Vec<IntMap> = Vec::new();
    for group in groups {
        let lines = group.split(":\n").nth(1).unwrap().lines();
        let mut ranges: Vec<RangeMap> = Vec::new();
        for line in lines {
            ranges.push(RangeMap::from(line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect()));
        }
        maps.push(IntMap::new(ranges));
    }
    let pipeline: MapPipeline = MapPipeline::new(maps);

    seed_ranges
        .iter()
        .map(|r| r.start)
        .map(|s| pipeline.get(s))
        .min()
        .unwrap_or_default()
        .min(
            (0..pipeline.levels())
                .map(|level| {
                    pipeline
                        .ranges_srcs(level)
                        .filter(|&start| {
                            let src: usize = pipeline.get_src(start, level);
                            seed_ranges.iter().any(|r| r.contains(&src))
                        })
                        .map(|start| pipeline.get_from_level(start, level))
                        .min()
                        .unwrap_or(usize::MAX)
                })
                .min()
                .unwrap_or(usize::MAX),
        )
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("seeds: 79 14 55 13

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
        assert_eq!(result, 46);
    }
}

