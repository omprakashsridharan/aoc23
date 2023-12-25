mod readfile;

struct Engine {
    parts_sum: i32,
}

impl Engine {
    fn new() -> Self {
        Engine { parts_sum: 0 }
    }

    fn construct_from_matrix(&mut self, matrix: Vec<Vec<char>>) {
        let mut num_str = "".to_string();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j].is_digit(10) {
                    num_str.push(matrix[i][j]);
                } else {
                    if num_str != ""
                        && self.is_valid_part_number(
                            matrix.clone(),
                            i as i32,
                            j as i32,
                            num_str.clone(),
                        )
                    {
                        self.parts_sum += num_str.parse::<i32>().unwrap();
                    }
                    num_str = "".to_string();
                }
            }
        }
    }

    fn is_valid_part_number(
        &mut self,
        matrix: Vec<Vec<char>>,
        i: i32,
        j: i32,
        num_str: String,
    ) -> bool {
        let height = matrix.len() as i32;
        let width = matrix[0].len() as i32;

        let num_length: i32 = num_str.len() as i32;
        let mut adjacent: Vec<String> = vec![];

        let num_start = j - num_length;
        let num_end = j - 1;
        let box_top_left = (i - 1, num_start - 1);
        let box_top_right = (i - 1, num_end + 1);
        let box_bottom_left = (i + 1, num_start - 1);
        let box_bottom_right = (i + 1, num_end + 1);

        for k in box_top_left.1..=box_top_right.1 {
            if k < 0 || k >= width || box_top_left.0 < 0 || box_top_left.0 >= height {
                continue;
            }
            adjacent.push(matrix[box_top_left.0 as usize][k as usize].to_string());
        }
        for k in box_bottom_left.1..=box_bottom_right.1 {
            if k < 0 || k >= width || box_bottom_left.0 < 0 || box_bottom_left.0 >= height {
                continue;
            }
            adjacent.push(matrix[box_bottom_left.0 as usize][k as usize].to_string());
        }
        for k in box_top_left.0..=box_bottom_left.0 {
            if k < 0 || k >= height || box_top_left.1 < 0 || box_top_left.1 >= width {
                continue;
            }
            adjacent.push(matrix[k as usize][box_top_left.1 as usize].to_string());
        }
        for k in box_top_right.0..=box_bottom_right.0 {
            if k < 0 || k >= height || box_top_right.1 < 0 || box_top_right.1 >= width {
                continue;
            }
            adjacent.push(matrix[k as usize][box_top_right.1 as usize].to_string());
        }
        let valid = adjacent.into_iter().filter(|x| *x != ".").count() != 0;

        return valid;
    }
}

fn main() {
    let input = readfile::readfile_by_lines("input/day3.txt");
    let mut input_matrix: Vec<Vec<char>> = vec![];
    for line in input.iter() {
        input_matrix.push(line.chars().collect());
    }
    let mut engine = Engine::new();
    engine.construct_from_matrix(input_matrix);
    println!("{}\n", engine.parts_sum);
}
