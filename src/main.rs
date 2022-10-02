enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
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
        Game {
            frame: Frame {
                width: 60,
                height: 30,
            },
            ball: Ball {
                x: 2,
                y: 4,
                vert_dir: VertDir::Up,
                horiz_dir: HorizDir::Left,
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
