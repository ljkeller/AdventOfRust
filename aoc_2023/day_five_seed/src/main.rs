use std::{path::Path, str::FromStr};

trait Ints {
    fn ints<F: FromStr>(&self) -> Vec<F>;
}

impl Ints for str {
    fn ints<F: FromStr>(&self) -> Vec<F> {
        let mut ints = Vec::new();
        let toks = self.split_ascii_whitespace();
        toks.for_each(|t| {
            let p = t.parse::<F>();
            match p {
                Ok(p) => ints.push(p),
                _ => {}
            }
        });

        ints
    }
}

struct Interval {
    // destination range start
    drs: usize,
    // source range start
    srs: usize,
    // range len
    rl: usize,
}

// Compressed Mapping Structure (CMS)
struct CMS {
    intervals: Vec<Interval>,
}

impl CMS {
    fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }
    // map source to dest
    // O(len(intervals))
    fn map(&self, source: &usize) -> usize {
        // general case
        let mut dest = *source;

        for interval in self.intervals.iter() {
            if interval.srs <= *source && *source < interval.srs + interval.rl {
                let diff = source - interval.srs;
                dest = interval.drs + diff;
                break;
            }
        }

        dest
    }
}

struct Almanac {
    s2s: CMS,
    s2f: CMS,
    f2w: CMS,
    w2l: CMS,
    l2t: CMS,
    t2h: CMS,
    h2l: CMS,
}

fn intervals(ints: impl Iterator<Item = usize>) -> Vec<Interval> {
    let mut intervals = Vec::<Interval>::new();
    let mut it = ints.into_iter().peekable();
    while it.peek().is_some() {
        intervals.push(Interval {
            drs: it.next().unwrap(),
            srs: it.next().unwrap(),
            rl: it.next().unwrap(),
        });
    }

    intervals
}

impl Almanac {
    fn new() -> Self {
        Self {
            s2s: CMS::new(),
            s2f: CMS::new(),
            f2w: CMS::new(),
            w2l: CMS::new(),
            l2t: CMS::new(),
            t2h: CMS::new(),
            h2l: CMS::new(),
        }
    }

    fn parse_s2s(&mut self, ints: Vec<usize>) {
        self.s2s.intervals = intervals(ints.into_iter());
    }
    fn to_soil(&self, seed: &usize) -> usize {
        self.s2s.map(seed)
    }

    fn parse_s2f(&mut self, ints: Vec<usize>) {
        self.s2f.intervals = intervals(ints.into_iter());
    }
    fn to_fertilizer(&self, soil: &usize) -> usize {
        self.s2f.map(soil)
    }

    fn parse_f2w(&mut self, ints: Vec<usize>) {
        self.f2w.intervals = intervals(ints.into_iter());
    }
    fn to_water(&self, fertilizer: &usize) -> usize {
        self.f2w.map(fertilizer)
    }

    fn parse_w2l(&mut self, ints: Vec<usize>) {
        self.w2l.intervals = intervals(ints.into_iter());
    }
    fn to_light(&self, water: &usize) -> usize {
        self.w2l.map(water)
    }

    fn parse_l2t(&mut self, ints: Vec<usize>) {
        self.l2t.intervals = intervals(ints.into_iter());
    }
    fn to_temperature(&self, light: &usize) -> usize {
        self.l2t.map(light)
    }

    fn parse_t2h(&mut self, ints: Vec<usize>) {
        self.t2h.intervals = intervals(ints.into_iter());
    }
    fn to_humidity(&self, temperature: &usize) -> usize {
        self.t2h.map(temperature)
    }

    fn parse_h2l(&mut self, ints: Vec<usize>) {
        self.h2l.intervals = intervals(ints.into_iter());
    }
    fn to_location(&self, humidity: &usize) -> usize {
        self.h2l.map(humidity)
    }
}

trait Mappable {
    fn to_location(&mut self, global_map: &Almanac);
}

trait MappableInterval {
    fn to_location(&mut self, global_map: &Almanac) -> Vec<usize>;
}

impl Mappable for Vec<usize> {
    fn to_location(&mut self, global_map: &Almanac) {
        self.iter_mut().for_each(|s| {
            let mut tmp = global_map.to_soil(&s);
            tmp = global_map.to_fertilizer(&tmp);
            tmp = global_map.to_water(&tmp);
            tmp = global_map.to_light(&tmp);
            tmp = global_map.to_temperature(&tmp);
            tmp = global_map.to_humidity(&tmp);
            tmp = global_map.to_location(&tmp);
            *s = tmp;
        });
    }
}

impl MappableInterval for Vec<(usize, usize)> {
    fn to_location(&mut self, global_map: &Almanac) -> Vec<usize> {
        let mut optimal_locs: Vec<usize> = Vec::new();
        self.iter_mut().for_each(|s| {
            println!("Parsing new pair! {:?}", s);
            let mut min_dist = usize::MAX;
            for seed_idx in s.0..s.0 + s.1 {
                let mut tmp = global_map.to_soil(&seed_idx);
                tmp = global_map.to_fertilizer(&tmp);
                tmp = global_map.to_water(&tmp);
                tmp = global_map.to_light(&tmp);
                tmp = global_map.to_temperature(&tmp);
                tmp = global_map.to_humidity(&tmp);
                tmp = global_map.to_location(&tmp);
                min_dist = min_dist.min(tmp);
            }
            optimal_locs.push(min_dist);
        });

        optimal_locs
    }
}
fn parse_seed_data(data: &str) -> (Vec<usize>, Almanac) {
    let mut almanac = Almanac::new();

    let mut toks = data.split(":");
    let seeds = toks.nth(1).expect("Seeds should exist").ints();

    let s2s_ints = toks.next().expect("seed to soil").ints::<usize>();
    almanac.parse_s2s(s2s_ints);

    let s2f_ints = toks.next().expect("soil to fert").ints::<usize>();
    almanac.parse_s2f(s2f_ints);

    let f2w_ints = toks.next().expect("fert to water").ints::<usize>();
    almanac.parse_f2w(f2w_ints);

    let w2l_ints = toks.next().expect("water to light").ints::<usize>();
    almanac.parse_w2l(w2l_ints);

    let l2t_ints = toks.next().expect("light to temp").ints::<usize>();
    almanac.parse_l2t(l2t_ints);

    let t2h_ints = toks.next().expect("temp to hum").ints::<usize>();
    almanac.parse_t2h(t2h_ints);

    let h2l_ints = toks.next().expect("hum to loc").ints::<usize>();
    almanac.parse_h2l(h2l_ints);

    (seeds, almanac)
}

fn solve_seed_mapping<P: AsRef<Path>>(seed_path: P) -> (usize, usize) {
    let seeds_data = std::fs::read_to_string(seed_path).expect("Expect seed path");

    let (mut seeds, global_map) = parse_seed_data(&seeds_data);
    let mut seed_pairs = seeds
        .clone()
        .chunks(2)
        .into_iter()
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(usize, usize)>>();
    seeds.to_location(&global_map);
    let optimal_seed_locs = seed_pairs.to_location(&global_map);

    (
        seeds.into_iter().min().expect("Expect min location"),
        optimal_seed_locs
            .into_iter()
            .min()
            .expect("Expect min location"),
    )
}

fn main() {
    println!(
        "(p1, p1) {:?}",
        solve_seed_mapping("aoc_2023/day_five_seed/data/sample1.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(solve_seed_mapping("data/ex1.txt").0, 35);
    }

    // #[test]
    // fn ex2() {
    // }
}
