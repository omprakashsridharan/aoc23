mod readfile;

struct Pair {
    first_index: i32,
    first_digit: i32,
    last_index: i32,
    last_digit: i32,
}

#[derive(Debug)]
struct CalibrationLineItem {
    line: String,
}

impl From<String> for CalibrationLineItem {
    fn from(line: String) -> Self {
        CalibrationLineItem { line }
    }
}

impl CalibrationLineItem {
    fn value(&self) -> Option<i32> {
        let number_indices = self.number_indices();
        let text_indices = self.text_indices();
        if number_indices.is_none() && text_indices.is_none() {
            return None;
        }
        let number_indices = number_indices.unwrap_or_else(|| Pair {
            first_index: i32::MAX,
            first_digit: 0,
            last_index: i32::MAX,
            last_digit: 0,
        });
        let text_indices = text_indices.unwrap_or_else(|| Pair {
            first_index: i32::MAX,
            first_digit: 0,
            last_index: i32::MAX,
            last_digit: 0,
        });
        let first = if number_indices.first_index < text_indices.first_index {
            number_indices.first_digit
        } else {
            text_indices.first_digit
        };
        let second = if number_indices.last_index < text_indices.last_index {
            number_indices.last_digit
        } else {
            text_indices.last_digit
        };
        Some((first * 10) + second)
    }

    fn number_indices(&self) -> Option<Pair> {
        let mut nums: Vec<(i32, i32)> = vec![];
        for (cindex, c) in self.line.char_indices() {
            match c {
                '0' => nums.push((cindex as i32, 0)),
                '1' => nums.push((cindex as i32, 1)),
                '2' => nums.push((cindex as i32, 2)),
                '3' => nums.push((cindex as i32, 3)),
                '4' => nums.push((cindex as i32, 4)),
                '5' => nums.push((cindex as i32, 5)),
                '6' => nums.push((cindex as i32, 6)),
                '7' => nums.push((cindex as i32, 7)),
                '8' => nums.push((cindex as i32, 8)),
                '9' => nums.push((cindex as i32, 9)),
                _ => {}
            }
        }
        if nums.len() == 0 {
            return None;
        }
        Some(Pair {
            first_index: nums.first().unwrap().0,
            first_digit: nums.first().unwrap().1,
            last_index: nums.last().unwrap().0,
            last_digit: nums.last().unwrap().1,
        })
    }

    fn text_indices(&self) -> Option<Pair> {
        None
    }
}

fn main() {
    let input = readfile::readfile_by_lines("input/day1.txt");
    let calibration_line_items: Vec<CalibrationLineItem> = input
        .iter()
        .map(|line| CalibrationLineItem::from(line.to_string()))
        .collect();
    println!(
        "{}",
        calibration_line_items
            .iter()
            .filter(|item| item.value().is_some())
            .map(|item| item.value().unwrap())
            .sum::<i32>()
    );
}
