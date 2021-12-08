use rltk::{GameState, Rltk, RGB};

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Command {
    fn new(command: &str) -> Command {
        let mut split = command.split(' ');
        let dir = split.next().unwrap();
        let amount: u32 = split.next().unwrap().parse().unwrap();

        match dir {
            "forward" => Command::Forward(amount),
            "up" => Command::Up(amount),
            "down" => Command::Down(amount),
            _ => panic!("Invalid command"),
        }
    }

    fn print(&self) -> String {
        match self {
            Command::Forward(a) => String::from("FORWARD ") + a.to_string().as_str(),
            Command::Down(a) => String::from("DOWN ") + a.to_string().as_str(),
            Command::Up(a) => String::from("UP ") + a.to_string().as_str(),
        }
    }
}

struct State {
    commands: Vec<Command>,
    current: usize,
    pos: u32,
    depth: u32,
    over: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let command = &self.commands[self.current];
        let fg = match command {
            Command::Forward(_) => RGB::named(rltk::GREEN),
            Command::Down(_) => RGB::named(rltk::BLUE),
            Command::Up(_) => RGB::named(rltk::RED),
        };

        ctx.print_color(2, 17, fg, RGB::named(rltk::BLACK), command.print());

        if !self.over {
            match command {
                Command::Forward(amount) => {
                    self.pos = self.pos + amount;
                }
                Command::Up(amount) => {
                    self.depth = self.depth - amount;
                }
                Command::Down(amount) => {
                    self.depth = self.depth + amount;
                }
            }
        }

        ctx.print(30, 10, "Position:");
        ctx.print(30, 12, self.pos.to_string());

        ctx.print(30, 16, "Depth:");
        ctx.print(30, 18, self.depth.to_string());

        ctx.print(30, 22, "Result:");
        ctx.print(30, 24, (self.pos * self.depth).to_string());

        if self.current < self.commands.len() - 1 {
            self.current += 1;
        } else {
            self.over = true;
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let input = include_str!("../input.in");
    let commands: Vec<Command> = input.lines().map(|line| Command::new(line)).collect();

    let ctx = RltkBuilder::simple80x50()
        .with_title("Advent of Code 2021 - Day 2")
        //.with_fps_cap(15.0)
        .build()?;

    let gs = State {
        commands,
        current: 0,
        pos: 0,
        depth: 0,
        over: false,
    };

    rltk::main_loop(ctx, gs)
}
