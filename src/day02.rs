use crate::Day;

// Register this day in our runner inventory
inventory::submit! {
    Day { number: 2, run }
}

#[derive(Debug, Default)]
struct ColorCount {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl ColorCount {
    fn max_by_element(&self, b: &Self) -> Self {
        Self {
            id: self.id,
            red: self.red.max(b.red),
            green: self.green.max(b.green),
            blue: self.blue.max(b.blue),
        }
    }

    fn is_possible(&self, max: &Self) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

// takes a group like "1 blue, 2 green, 3 red" and returns a ColorCount object
fn parse_group(group: &str) -> ColorCount {
    let mut color_count = ColorCount::default();

    // split group in to its individual colors
    for part in group.split(", ") {
        // split individual color into "number color"
        let mut parts = part.split_whitespace();
        let number = parts.next().unwrap().parse::<usize>().unwrap();
        let color = parts.next().unwrap();

        match color {
            "red" => color_count.red += number,
            "green" => color_count.green += number,
            "blue" => color_count.blue += number,
            _ => unreachable!(), // This should not happen if all colors are known.
        }
    }

    color_count
}

fn parse_game(game_text: &str) -> ColorCount {
    // Split apart the "Game X:" and color groups
    let mut parts = game_text.splitn(2, ": ");

    // Extract the game number from the first part.
    let game_number = parts
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    // Get the color data from the second part.
    let color_data = parts.next().unwrap();

    // Create initial ColorCount object with the game number
    let initial = ColorCount {
        id: game_number,
        ..Default::default()
    };

    // Parse the color groups and collect the maximum values found for each color
    color_data
        .split("; ")
        .map(parse_group)
        .fold(initial, |acc, x| acc.max_by_element(&x))
}

fn parse_games(games_text: &str) -> impl Iterator<Item = ColorCount> + '_ {
    games_text
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_game)
}

fn part1(input: &str) -> usize {
    let max_color_count = ColorCount {
        id: 0,
        red: 12,
        green: 13,
        blue: 14,
    };

    // Find all the games which are <= max_color_count and sum the game ids
    parse_games(input)
        .filter(|x| x.is_possible(&max_color_count))
        .map(|x| x.id)
        .sum()
}

fn part2(input: &str) -> usize {
    // Sum the (red*green*blue) for all games' maximum color counts
    parse_games(input).map(|x| x.power()).sum()
}

const INPUT: &str = include_str!("../data/day02.dat");

fn run() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(part1(INPUT), 2683);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(TEST_INPUT), 2286);
    }
    #[test]
    fn test_part2_actual() {
        assert_eq!(part2(INPUT), 49710);
    }
}
