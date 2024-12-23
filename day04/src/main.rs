use std::{fs::File, io::BufRead, io::BufReader};

//0    TopLeft
//1    TopMiddle
//2    TopRight
//3    LeftTop
//4    LeftMiddle
//5    LeftBottom
//6    RightTop
//7    RightMiddle
//8    RightBottom
//9    BottomLeft
//10    BottomMiddle
//11    BottomRight

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

fn main() {
    let input_file = File::open("input.txt").expect("Failed to find input.txt");
    let buf_reader = BufReader::new(input_file);

    let grid = Grid::new(buf_reader);
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
