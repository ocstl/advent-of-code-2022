#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    height: usize,
    width: usize,
    grid: Vec<T>,
}

impl<T: Copy> Grid<T> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}
