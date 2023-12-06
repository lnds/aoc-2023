use core::str::Lines;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    end: i64,
}

impl SeedRange {
    fn new(pair: &[i64]) -> Self {
        SeedRange {
            start: pair[0],
            end: pair[0] + pair[1],
        }
    }
    fn find(&self, id: i64) -> bool {
        id >= self.start && id < self.end
    }
}
#[derive(Debug)]
struct Range {
    destination: i64,
    source: i64,
    length: i64,
}

impl Range {
    fn new(line: &str) -> Self {
        let nums = str_to_nums(line);
        Range {
            destination: nums[1],
            source: nums[0],
            length: nums[2],
        }
    }

    fn find(&self, id: i64) -> Option<i64> {
        if id < self.source {
            return Some(id);
        }
        if id < self.source + self.length {
            return Some(self.destination + id - self.source);
        }
        None
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(lines: Vec<&str>) -> Self {
        let mut ranges: Vec<_> = lines.iter().map(|l| Range::new(l)).collect();
        ranges.sort_by_key(|r| r.source);
        Map { ranges }
    }

    fn find(&self, id: i64) -> Option<i64> {
        // assume ranges are sorted
        for range in self.ranges.iter() {
            if let Some(pos) = range.find(id) {
                return Some(pos);
            }
        }
        self.ranges.last().map(|_| id)
    }
}

fn str_to_nums(line: &str) -> Vec<i64> {
    let re = Regex::new(r"(\d+)").unwrap();
    re.find_iter(line)
        .map(|m| m.as_str().parse::<i64>().unwrap_or_default())
        .collect()
}

fn scan_map(name: &str, lines: &mut Lines) -> Map {
    let line = lines.next().unwrap_or_default();
    if !line.starts_with(name) {
        panic!("expecting {}, found: {}", name, line);
    }
    Map::new(lines.take_while(|l| !l.is_empty()).collect())
}

fn part2(input: &str) -> Option<i64> {
    let mut lines = input.lines();
    let line = lines.next().unwrap_or_default();
    let seeds: Vec<i64> = str_to_nums(&line[6..]);
    lines.next();
    let soil_to_seed = scan_map("seed-to-soil", &mut lines);
    let fertilizer_to_soil = scan_map("soil-to-fertilizer", &mut lines);
    let water_to_fertilizer = scan_map("fertilizer-to-water", &mut lines);
    let light_to_water = scan_map("water-to-light", &mut lines);
    let temperature_to_light = scan_map("light-to-temperature", &mut lines);
    let humidity_to_temperature = scan_map("temperature-to-humidity", &mut lines);
    let location_to_humidity = scan_map("humidity-to-location", &mut lines);

    let seed_ranges: Vec<_> = seeds
        .chunks(2)
        .map(SeedRange::new)
        .sorted_by_key(|r| r.start)
        .collect();
    let mut start = i64::MAX;
    for r in location_to_humidity.ranges.iter() {
        if r.destination - r.length < start {
            start = r.destination - r.length;
        }
    }
    for location in start..seed_ranges.last().unwrap().end {
        let humidity = location_to_humidity.find(location)?;
        let temperature = humidity_to_temperature.find(humidity)?;
        let light = temperature_to_light.find(temperature)?;
        let water = light_to_water.find(light)?;
        let fertilizer = water_to_fertilizer.find(water)?;
        let soil = fertilizer_to_soil.find(fertilizer)?;
        let seed = soil_to_seed.find(soil)?;
        for r in seed_ranges.iter() {
            if r.find(seed) {
                return Some(location);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
56 93 4",
        )
        .unwrap_or_default();
        assert_eq!(result, 46);
    }
}
