use std::fmt;
use std::fmt::Display;
use std::io::BufRead;

// todo make direction enum and support stepping in each direction
pub struct Grid<T: Copy> {
    grid: Vec<T>,
    array_width: usize,
    num_rows: usize,
}

impl<T: Copy> Grid<T> {
    pub fn move_pos(&self, pos: usize, coords: (isize, isize)) -> Option<usize> {
        let (x, y) = coords;
        let y_idx = (pos / self.array_width) as isize + y;
        let x_idx = (pos % self.array_width) as isize + x;

        if (x_idx >= 0) && (x_idx < self.array_width as isize) && (y_idx >= 0) && (y_idx < self.num_rows as isize)
        {
            Some(y_idx as usize * self.array_width + x_idx as usize)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn peek(&self, pos: usize) -> T {
        self.grid[pos]
    }
    
    fn parse_data_generic<R: BufRead>(reader: R, process_fun: &dyn Fn(char)->T) -> Grid<T> {
        let mut file_iter = reader.lines().peekable();
        let array_width = file_iter.peek().unwrap().as_ref().unwrap().len();
        let grid = file_iter.map(|l| l.unwrap().chars().map(|c| process_fun(c)).collect::<Vec<_>>()).flatten().collect::<Vec<T>>();
        let num_rows = grid.len() / array_width;
        Grid { grid, array_width, num_rows} }

    pub fn pos_to_coords(&self, pos: usize) -> (usize, usize) {
        ((pos % self.array_width), (pos / self.array_width))
    }

    pub fn coords_to_pos(&self, x: usize, y: usize) -> usize {
        y * self.array_width + x
    }
    
    pub fn get_grid(&self) -> &Vec<T> {
        &self.grid
    }
}

pub trait ParseData<T: Copy> {
    fn parse_data<R: BufRead>(reader: R) -> Grid<T>;
}

impl ParseData<u32> for Grid<u32> {
    fn parse_data<R: BufRead>(reader: R) -> Grid<u32> {
        Grid::parse_data_generic(reader, &|c| c.to_digit(10).unwrap())
    }
}

impl ParseData<char> for Grid<char> {
    fn parse_data<R: BufRead>(reader: R) -> Grid<char> {
        Grid::parse_data_generic(reader, &|c| c)
    }
}

impl<T: Copy + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut row_start = 0;
        while row_start < self.len() {
            write!(f, "{}\n", &self.grid[row_start..row_start + self.array_width].iter().map(|d| d.to_string()).collect::<String>())?;
            row_start += self.array_width;
        }
        write!(f, "\n")
    }
}