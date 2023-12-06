use core::str::Lines;
use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
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
            destination: nums[0],
            source: nums[1],
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
    let seed_to_soil = scan_map("seed-to-soil", &mut lines);
    let soil_to_fertilizer = scan_map("soil-to-fertilizer", &mut lines);
    let fertilizer_to_water = scan_map("fertilizer-to-water", &mut lines);
    let water_to_light = scan_map("water-to-light", &mut lines);
    let light_to_temperature = scan_map("light-to-temperature", &mut lines);
    let temperature_to_humidity = scan_map("temperature-to-humidity", &mut lines);
    let humidity_to_location = scan_map("humidity-to-location", &mut lines);
    // println!("seed {:?}, seeds);
    // println!("seed_to_soil {:?}", seed_to_soil);
    // println!("soil_to_fertilizer {:?}", soil_to_fertilizer);
    // println!("fertilizer_to_water {:?}", fertilizer_to_water);
    // println!("water_to_light {:?}", water_to_light);
    // println!("light-to-temperature {:?}", light_to_temperature);
    // println!("temperature_to_humidity {:?}", temperature_to_humidity);
    // println!("humidity_to_location {:?}", humidity_to_location);
    let mut min = i64::MAX;
    for w in seeds.chunks(2) {
        let start = w[0];
        let end = start + w[1];
        println!("START: {}, END: {}", start, end);
        for seed in start..end {
            //    println!("seed: {}", seed);
            let soil = seed_to_soil.find(seed)?;
            // println!("soil: {}", soil);
            let fertilizer = soil_to_fertilizer.find(soil)?;
            // println!("fertilizer: {}", fertilizer);
            let water = fertilizer_to_water.find(fertilizer)?;
            // println!("water: {}", water);
            let light = water_to_light.find(water)?;
            // println!("light: {}", light);
            let temperature = light_to_temperature.find(light)?;
            // println!("temperature: {}", temperature);
            let humidity = temperature_to_humidity.find(temperature)?;
            // println!("humidity: {}", humidity);
            let location = humidity_to_location.find(humidity)?;
            //   println!("location {}", location);
            if location < min {
                min = location;
                println!("min: {}", min);
            }
        }
    }
    Some(min)
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
