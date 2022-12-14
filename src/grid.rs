#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    height: usize,
    width: usize,
    grid: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(height: usize, width: usize, grid: Vec<T>) -> Option<Self> {
        if height * width == grid.len() {
            Some(Grid {
                height,
                width,
                grid,
            })
        } else {
            None
        }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        if position.x < self.width {
            self.grid.get(position.x + position.y * self.width)
        } else {
            None
        }
    }

    pub fn row(&self, idy: usize) -> impl Iterator<Item = &T> {
        self.grid.iter().skip(idy * self.width).take(self.width)
    }

    pub fn column(&self, idx: usize) -> impl Iterator<Item = &T> {
        self.grid
            .iter()
            .skip(idx)
            .step_by(self.height)
            .take(self.height)
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.height).map(|idy| self.row(idy))
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.width).map(|idx| self.column(idx))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn x(self) -> usize {
        self.x
    }

    pub fn y(self) -> usize {
        self.y
    }

    pub const fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        self.x
            .checked_sub(1)
            .map(|x| Position::new(x, self.y))
            .into_iter()
            .chain(Some(Position::new(self.x + 1, self.y)).into_iter())
            .chain(
                self.y
                    .checked_sub(1)
                    .map(|y| Position::new(self.x, y))
                    .into_iter(),
            )
            .chain(Some(Position::new(self.x, self.y + 1)))
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Option<Self>;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => self.y.checked_sub(1).map(|y| Position::new(self.x, y)),
            Direction::Down => Some(Position::new(self.x, self.y + 1)),
            Direction::Left => self.x.checked_sub(1).map(|x| Position::new(x, self.y)),
            Direction::Right => Some(Position::new(self.x + 1, self.y)),
            Direction::UpLeft => self
                .x
                .checked_sub(1)
                .and_then(|x| self.y.checked_sub(1).map(|y| Position::new(x, y))),
            Direction::DownLeft => self.x.checked_sub(1).map(|x| Position::new(x, self.y + 1)),
            Direction::UpRight => self.y.checked_sub(1).map(|y| Position::new(self.x + 1, y)),
            Direction::DownRight => Some(Position::new(self.x + 1, self.y + 1)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
}

impl Direction {
    pub fn rotate_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::UpLeft => Direction::DownLeft,
            Direction::DownLeft => Direction::DownRight,
            Direction::UpRight => Direction::UpLeft,
            Direction::DownRight => Direction::UpRight,
        }
    }

    pub fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::UpLeft => Direction::UpRight,
            Direction::DownLeft => Direction::UpLeft,
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::UpLeft => Direction::DownRight,
            Direction::DownLeft => Direction::UpRight,
            Direction::UpRight => Direction::DownLeft,
            Direction::DownRight => Direction::UpLeft,
        }
    }
}
