use std::fmt::{Display, Formatter};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right
}

struct Vector2D {
    x: f32,
    y: f32,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        let frame = Frame {
            width: 60,
            height: 30,
        };
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };
        Game {frame, ball}
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame); //need to borrow?
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut Formatter| {
            write!(fmt, "+");
            for _ in 0..self.frame.width {
                write!(fmt, "-");
            }
            write!(fmt, "+\n")
        };
        top_bottom(&mut fmt);
        for row in 0..self.frame.height {
            write!(fmt, "|");
            // asdf
            write!(fmt, "|\n");
        }
        top_bottom(&mut fmt)
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) { //&mut self? Where did we make the ball variable mutable?
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }
        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }
    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}


fn main() {
    println!("{}", Game::new());
}
