use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Display, FromStr, Debug, Clone)]
#[display("x={x}, y={y}")]
struct Coord2D {
    x: isize,
    y: isize,
}

impl Coord2D {
    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

// Parse into ScannerCoord -> (BeaconCoord, DistanceBetweenScannerAndBeacon)
fn parse_input(input: &str) -> HashMap<Coord2D, (Coord2D, usize)> {
    let mut scanner_beacon_map: HashMap<Coord2D, (Coord2D, usize)> = HashMap::new();

    for line in input.lines() {
        let (scanner_line, beacon_line) = line.split_once(':').unwrap();
        let scanner_coord = scanner_line
            .trim()
            .strip_prefix("Sensor at ")
            .unwrap()
            .parse::<Coord2D>()
            .unwrap();
        let beacon_coord = beacon_line
            .trim()
            .strip_prefix("closest beacon is at ")
            .unwrap()
            .parse::<Coord2D>()
            .unwrap();
        let beacon_dist = beacon_coord.distance(&scanner_coord) + 1;
        scanner_beacon_map.insert(scanner_coord, (beacon_coord, beacon_dist));
    }

    scanner_beacon_map
}

// For a row, calculate all x-Coordinates, where no beacon is possible.
//
// Output is a list of ranges, not single x-Coordinates, because that would be
// too much. To calculate those ranges, we find out the WIDTH of the scanner
// overlap into the row and from there calculate the start and end. The width is
// dependent on the distance between the scanner and the row, and the influence
// radius of the scanner.
// Finally we merge overlapping ranges.
fn calculate_ranges_of_row_where_no_beacon_is_possible(
    map: &HashMap<Coord2D, (Coord2D, usize)>,
    row: &isize,
    clamp: Option<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let mut ranges: Vec<(isize, isize)> = vec![];
    for (scanner, (_, radius)) in map {
        let scanner_row_distance = scanner.y.abs_diff(*row) as isize;
        if (*radius as isize) <= scanner_row_distance {
            // row not in scanner influence, skip
            continue;
        }
        let width = 2 * (*radius as isize - scanner_row_distance) - 1;
        let mut start = scanner.x - ((width as isize - 1) / 2);
        let mut end = start + width as isize;

        if let Some((min, max)) = clamp {
            start = start.max(min);
            end = end.min(max);
        }

        if start == end {
            continue;
        }
        ranges.push((start, end));
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    ranges.reverse();

    let mut merged_ranges: Vec<(isize, isize)> = vec![];

    if let Some(mut current_range) = ranges.pop() {
        while let Some(range) = ranges.pop() {
            if current_range.1 < range.0 {
                merged_ranges.push(current_range);
                current_range = range;
                continue;
            }

            current_range.1 = current_range.1.max(range.1);
        }
        merged_ranges.push(current_range);
    }

    merged_ranges
}

pub fn part_one(_input: &str) -> Option<u32> {
    part_one_param(_input, 2000000)
}

pub fn part_one_param(_input: &str, row: isize) -> Option<u32> {
    let sbmap = parse_input(_input);
    let mut tracker = 0;
    let ranges: Vec<(isize, isize)> =
        calculate_ranges_of_row_where_no_beacon_is_possible(&sbmap, &row, None);
    let beacons_in_row = sbmap
        .values()
        .filter(|(b, _)| b.y == row)
        .map(|(b, _)| b)
        .unique();

    for range in ranges {
        let beacons_in_range = beacons_in_row
            .clone()
            .filter(|b| b.x >= range.0 && b.x < range.1)
            .collect_vec();
        tracker += range.1 - range.0 - beacons_in_range.len() as isize;
    }
    Some(tracker as u32)
}

pub fn part_two(_input: &str) -> Option<u64> {
    part_two_param(_input, 4000000)
}

pub fn part_two_param(_input: &str, search_size: isize) -> Option<u64> {
    let sbmap = parse_input(_input);

    let mut tracker: u64 = 0;
    for y in 0..search_size {
        let ranges: Vec<(isize, isize)> =
            calculate_ranges_of_row_where_no_beacon_is_possible(&sbmap, &y, Some((0, search_size)));
        if ranges.len() > 1 {
            tracker = (ranges[0].1 * 4000000) as u64 + y as u64;
            break;
        }
    }
    Some(tracker)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_param(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_param(&input, 20), Some(56000011));
    }
}
