use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use strum::{EnumIter, IntoEnumIterator};

const WORD: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
enum Direction {
    TopLeft,
    TopMiddle,
    TopRight,
    Left,
    Right,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Grid {
    content: Vec<Vec<Letter>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(file_buffer: BufReader<File>) -> Self {
        let width = 0;
        let height = 0;

        let mut grid = Grid {
            content: Vec::new(),
            height,
            width,
        };

        for (x, line) in file_buffer.lines().enumerate() {
            let line = line.expect("Failed to get line");
            grid.content.push(Vec::new());
            for (y, char) in line.chars().enumerate() {
                grid.width = y;
                grid.content[x].push(match char {
                    'X' => Letter::X,
                    'M' => Letter::M,
                    'A' => Letter::A,
                    'S' => Letter::S,
                    _ => panic!("Invalid char in input"),
                });
            }
            grid.height = x;
        }

        return grid;
    }

    fn get(&self, x: usize, y: usize) -> Letter {
        return self.content[x][y];
    }
}

struct Snake {
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,
    allowed_dirs: Vec<Direction>,
    found_chars: usize,
    looking_for: Letter,
}

impl Snake {
    fn new(
        x: usize,
        y: usize,
        x_max: usize,
        y_max: usize,
        directions: Option<Vec<Direction>>,
    ) -> Self {
        let mut snake = Snake {
            x,
            y,
            x_max,
            y_max,
            allowed_dirs: directions.unwrap_or(Direction::iter().collect()),
            found_chars: 1,
            looking_for: WORD[1],
        };
        snake.update_allowed_dirs();
        return snake;
    }

    fn get_new_pos(&self, dir: Direction) -> (usize, usize) {
        match dir {
            Direction::TopLeft => return (self.x - 1, self.y - 1),
            Direction::TopMiddle => return (self.x - 1, self.y),
            Direction::TopRight => return (self.x - 1, self.y + 1),
            Direction::Left => return (self.x, self.y - 1),
            Direction::Right => return (self.x, self.y + 1),
            Direction::BottomLeft => return (self.x + 1, self.y - 1),
            Direction::BottomMiddle => return (self.x + 1, self.y),
            Direction::BottomRight => return (self.x + 1, self.y + 1),
        }
    }

    fn update_allowed_dirs(&mut self) {
        if self.x == 0 {
            self.allowed_dirs = self
                .allowed_dirs
                .iter()
                .cloned()
                .filter(|dir| {
                    return !vec![
                        Direction::TopLeft,
                        Direction::TopMiddle,
                        Direction::TopRight,
                    ]
                    .contains(dir);
                })
                .collect();
        }
        if self.x == self.x_max {
            self.allowed_dirs = self
                .allowed_dirs
                .iter()
                .cloned()
                .filter(|dir| {
                    return !vec![
                        Direction::BottomLeft,
                        Direction::BottomMiddle,
                        Direction::BottomRight,
                    ]
                    .contains(dir);
                })
                .collect();
        }
        if self.y == 0 {
            self.allowed_dirs = self
                .allowed_dirs
                .iter()
                .cloned()
                .filter(|dir| {
                    return !vec![Direction::TopLeft, Direction::Left, Direction::BottomLeft]
                        .contains(dir);
                })
                .collect();
        }
        if self.y == self.y_max {
            self.allowed_dirs = self
                .allowed_dirs
                .iter()
                .cloned()
                .filter(|dir| {
                    return !vec![
                        Direction::TopRight,
                        Direction::Right,
                        Direction::BottomRight,
                    ]
                    .contains(dir);
                })
                .collect();
        }
    }
}

fn main() {
    let input_file = File::open("input.txt").expect("Failed to find input.txt");
    let buf_reader = BufReader::new(input_file);

    let grid = Grid::new(buf_reader);
    let mut snakes: VecDeque<Snake> = VecDeque::new();
    let mut count = 0;

    for x in 0..grid.height {
        for y in 0..grid.width {
            if grid.get(x, y).eq(&Letter::X) {
                let new_snake = Snake::new(x, y, grid.height - 1, grid.width - 1, None);
                snakes.push_back(new_snake);
            }
        }
    }

    while !snakes.is_empty() {
        let snake = snakes.pop_front().unwrap();

        if snake.found_chars == WORD.len() - 1 {
            count += 1;
            continue;
        }

        for dir in snake.allowed_dirs.clone() {
            let (x, y) = snake.get_new_pos(dir);
            if grid.get(x, y) == snake.looking_for {
                let mut new_snake = Snake::new(x, y, snake.x_max, snake.y_max, vec![dir].into());
                new_snake.found_chars += 1;
                new_snake.looking_for = WORD[new_snake.found_chars];
                snakes.push_back(new_snake);
            }
        }
    }
    println!("{}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_grid() {
        let input_file = File::open("test-input.txt").expect("Failed to find input.txt");
        let buf_reader = BufReader::new(input_file);

        let grid = Grid::new(buf_reader);

        assert_eq!(grid.get(0, 0), Letter::S);
        assert_eq!(grid.get(1, 1), Letter::A);
        assert_eq!(grid.get(2, 2), Letter::M);
        assert_eq!(grid.get(0, 2), Letter::X);

        println!(
            "Didn't crash with grid height: {}, and width: {}",
            grid.height, grid.width
        );
        println!("{:?}", grid);
    }
}
