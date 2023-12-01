use aoc_runner_derive::{aoc, aoc_generator};

fn part1_parse_line(line: &str) -> u32 {
    let first = line.chars().find_map(|x| x.to_digit(10)).unwrap();
    let last = line.chars().rev().find_map(|x| x.to_digit(10)).unwrap();
    first * 10 + last
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input.lines().map(part1_parse_line).sum()
}

#[inline]
fn find_number_string(s: &str, number: &str) -> Option<(usize, usize)> {
    s.find(number).map(|i| (i, i + number.len()))
}

/// Find the start and end indices of a number in a string.
///
/// The string must be a single digit, or one of the following words:
/// one, two, three, four, five, six, seven, eight, nine.
fn find_number(x: &str) -> Option<(usize, usize)> {
    x.find(|x: char| x.is_ascii_digit())
        .map(|i| (i, i + 1))
        .or_else(|| find_number_string(x, "one"))
        .or_else(|| find_number_string(x, "two"))
        .or_else(|| find_number_string(x, "three"))
        .or_else(|| find_number_string(x, "four"))
        .or_else(|| find_number_string(x, "five"))
        .or_else(|| find_number_string(x, "six"))
        .or_else(|| find_number_string(x, "seven"))
        .or_else(|| find_number_string(x, "eight"))
        .or_else(|| find_number_string(x, "nine"))
}

/// Parse a number from a string.
///
/// The string must be a single digit, or one of the following words:
/// one, two, three, four, five, six, seven, eight, nine.
fn parse_number(x: &str) -> u32 {
    if x.len() == 1 {
        x.chars().next().unwrap().to_digit(10).unwrap()
    } else {
        match x {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("Invalid number: {}", x),
        }
    }
}

/// An iterator over all substrings of a string, starting from the beginning
/// and expanding to the end.
struct ExpandingWindow<'a> {
    x: &'a str,
    i: usize,
}

impl<'a> ExpandingWindow<'a> {
    fn new(x: &'a str) -> Self {
        Self { x, i: 0 }
    }
}

impl<'a> Iterator for ExpandingWindow<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.x.len() {
            return None;
        }
        self.i += 1;
        Some(&self.x[..self.i])
    }
}

impl<'a> DoubleEndedIterator for ExpandingWindow<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i >= self.x.len() {
            return None;
        }
        self.i += 1;
        Some(&self.x[self.x.len() - self.i..])
    }
}

fn part2_parse_line(line: &str) -> u32 {
    let first: u32 = ExpandingWindow::new(line)
        .find_map(|x| find_number(x).map(|(i, _)| parse_number(&x[i..])))
        .unwrap();
    let last: u32 = ExpandingWindow::new(line)
        .rev()
        .find_map(|x| find_number(x).map(|(start, end)| parse_number(&x[start..end])))
        .unwrap();
    first * 10 + last
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    input.lines().map(part2_parse_line).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }

    #[test]
    fn test_expanding_window_forward() {
        let x = "abc";
        let got: Vec<_> = ExpandingWindow::new(x).collect();
        assert_eq!(got, vec!["a", "ab", "abc"]);
    }

    #[test]
    fn test_expanding_window_backward() {
        let x = "abc";
        let got: Vec<_> = ExpandingWindow::new(x).rev().collect();
        assert_eq!(got, vec!["c", "bc", "abc"]);
    }
}
