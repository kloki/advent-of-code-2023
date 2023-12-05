#[derive(Debug)]
pub struct Rule {
    source: usize,
    destination: usize,
    range: usize,
}

impl Rule {
    pub fn apply(&self, input: usize) -> Option<usize> {
        if input >= self.source && input < (self.source + self.range) {
            return Some(self.destination + (input - self.source));
        }

        None
    }
}

#[derive(Debug)]
pub struct Mapping {
    rules: Vec<Rule>,
}

impl Mapping {
    pub fn process(&self, seed: usize) -> usize {
        for r in &self.rules {
            if let Some(r) = r.apply(seed) {
                return r;
            };
        }

        seed
    }
}
#[derive(Debug)]
pub struct Almanac {
    mappings: Vec<Mapping>,
}

impl Almanac {
    pub fn process(&self, seed: usize) -> usize {
        let mut result = seed;
        for m in &self.mappings {
            result = m.process(result);
        }

        result
    }
}
#[derive(Debug)]
pub struct AlmanacParseError;
pub fn parse_input(input: &str) -> Result<(Vec<usize>, Almanac), AlmanacParseError> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (_, seeds) = parts[0].split_once(": ").ok_or(AlmanacParseError)?;
    let seeds = seeds
        .split(" ")
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| AlmanacParseError)?;

    let mut mappings: Vec<Mapping> = Vec::new();
    for mapping in &parts[1..] {
        let (_, rules) = mapping.split_once("map:\n").ok_or(AlmanacParseError)?;
        let rules = rules
            .trim()
            .split("\n")
            .map(|rule| {
                let numbers: Vec<_> = rule
                    .split(" ")
                    .map(|r| r.parse::<usize>().unwrap())
                    .collect();
                Rule {
                    source: numbers[1],
                    destination: numbers[0],
                    range: numbers[2],
                }
            })
            .collect::<Vec<Rule>>();
        mappings.push(Mapping { rules });
    }

    Ok((seeds, Almanac { mappings }))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "seeds: 79 14 55 13

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
56 93 4";
    #[test]
    fn test_parse() {
        let (seeds, _) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(seeds.len(), 4)
    }

    #[test]
    fn test_process_seed() {
        let (seeds, almanac) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(almanac.process(seeds[0]), 82);
        assert_eq!(almanac.process(seeds[1]), 43);
        assert_eq!(almanac.process(seeds[2]), 86);
        assert_eq!(almanac.process(seeds[3]), 35);
    }
}
