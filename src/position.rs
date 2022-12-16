#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub const fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    pub const fn x(self) -> isize {
        self.x
    }

    pub const fn y(self) -> isize {
        self.y
    }

    pub const fn manhattan_distance(self, other: Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Position::new(self.x + rhs.dx, self.y + rhs.dy)
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        self.x += rhs.dx;
        self.y += rhs.dy;
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Direction;

    fn sub(self, rhs: Position) -> Self::Output {
        Direction::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Direction {
    dx: isize,
    dy: isize,
}

impl Direction {
    pub const fn new(dx: isize, dy: isize) -> Self {
        Direction { dx, dy }
    }

    pub const fn signum(self) -> Self {
        Direction::new(self.dx.signum(), self.dy.signum())
    }

    pub fn maximum_norm(self) -> isize {
        self.dx.abs().max(self.dy.abs())
    }
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::new(0, 1)),
            'D' => Ok(Direction::new(0, -1)),
            'L' => Ok(Direction::new(-1, 0)),
            'R' => Ok(Direction::new(1, 0)),
            _ => Err(value),
        }
    }
}
