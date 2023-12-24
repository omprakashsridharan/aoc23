mod readfile;

#[derive(Debug, Clone, Copy)]
struct Set {
    blue: i32,
    green: i32,
    red: i32,
}

impl From<String> for Set {
    fn from(value: String) -> Self {
        let set_parts: Vec<&str> = value.split(",").collect();
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for sp in set_parts {
            if sp.contains("blue") {
                let mut b = sp.to_string();
                b = b.replace("blue", "");
                b = b.replace(" ", "");
                blue += b.parse::<i32>().unwrap();
            }
            if sp.contains("red") {
                let mut r = sp.to_string();
                r = r.replace("red", "");
                r = r.replace(" ", "");
                red += r.parse::<i32>().unwrap();
            }
            if sp.contains("green") {
                let mut g = sp.to_string();
                g = g.replace("green", "");
                g = g.replace(" ", "");
                green += g.parse::<i32>().unwrap();
            }
        }
        Set { blue, green, red }
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

impl Game {
    fn is_valid(&self, max_set: Set) -> bool {
        let mut valid = true;
        for set in self.sets.iter() {
            if set.blue > max_set.blue || set.red > max_set.red || set.green > max_set.green {
                valid = false;
            }
        }
        valid
    }
}

impl From<String> for Game {
    fn from(line: String) -> Self {
        let game_parts: Vec<&str> = line.split(":").collect();
        let mut id_string = game_parts[0].to_string();
        id_string = id_string.replace("Game ", "");
        let mut sets: Vec<Set> = vec![];
        for set in game_parts[1].split(";") {
            sets.push(Set::from(set.to_string()));
        }
        Game {
            id: id_string.parse().unwrap(),
            sets,
        }
    }
}

fn main() {
    let input = readfile::readfile_by_lines("input/day2.txt");
    let games: Vec<Game> = input.into_iter().map(|line| Game::from(line)).collect();
    let mut id_sum = 0;
    let max_set = Set {
        blue: 14,
        green: 13,
        red: 12,
    };
    for game in games {
        if game.is_valid(max_set) {
            id_sum += game.id;
        }
    }
    println!("id_sum: {}", id_sum);
}
