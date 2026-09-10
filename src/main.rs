use crossterm::event::{self, Event, KeyCode};
use std::error::Error;

const CYCLES: usize = 1000;
const START_NUM_RATE: f32 = 0.3;
const DELAY: u64 = 50;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Born,
    Alive,
    Dead,
}

#[derive(Debug)]
struct Cell {
    state: CellState,
    next: CellState,
}

impl Cell {
    fn new(rate: f32) -> Self {
        let st = match rand::random::<f32>() {
            r if r < rate => CellState::Born,
            _ => CellState::Dead,
        };
        Cell {
            state: st,
            next: CellState::Dead,
        }
    }

    fn next_state(&mut self, dead: usize) {
        self.next = match &self.state {
            CellState::Dead if dead == 3 => CellState::Born,
            CellState::Born | CellState::Alive if dead == 2 || dead == 3 => CellState::Alive,
            _ => CellState::Dead,
        };
    }
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    alive_num: usize,
}

impl Board {
    pub fn new() -> Self {
        Board {
            width: 0,
            height: 0,
            cells: vec![],
            alive_num: 0,
        }
    }

    pub fn init( &mut self, width: usize, height: usize, rate: f32) {
        let mut cells: Vec<Cell> = vec![];
        let mut alive_num: usize = 0;

        for _ in 0..width * height {
            let cell = Cell::new(rate);
            if cell.state != CellState::Dead {
                alive_num += 1;
            }
            cells.push(cell);
        }
        self.width = width;
        self.height = height;
        self.cells = cells;
        self.alive_num = alive_num;
    }

    fn count_not_dead_around(&self, i: usize, j: usize) -> usize {
        let i = i as isize;
        let j = j as isize;
        let w = self.width as isize;
        let h = self.height as isize;
        let mut c = 0;

        for ii in (i - 1)..=(i + 1) {
            for jj in (j - 1)..=(j + 1) {
                if ii >= 0
                    && ii < h
                    && jj >= 0
                    && jj < w
                    && !(ii == i && jj == j)
                    && self.cells[(ii * w + jj) as usize].state != CellState::Dead
                {
                    c += 1;
                }
            }
        }
        c
    }

    pub fn update(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let dead = self.count_not_dead_around(i, j);
                let cell = &mut self.cells[i * self.width + j];
                cell.next_state(dead);
            }
        }

        let mut alive_num = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                let cell = &mut self.cells[i * self.width + j];
                cell.state = cell.next;
                if cell.state != CellState::Dead {
                    alive_num += 1;
                }
            }
        }
        self.alive_num = alive_num;
    }

    fn render(&self, frame: &mut ratatui::Frame, gen_num: usize, gen_size: usize) {
        use ratatui::widgets::{Block, Paragraph};

        let mut text = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y * self.width + x];
                let c = match cell.state {
                    CellState::Born => '.',
                    CellState::Alive => '*',
                    CellState::Dead => ' ',
                };
                text.push(c);
            }
            text.push('\n');
        }

        text.push_str(&format!(
            "GENERATION: {}/{} ALIVE: {}",
            gen_num,
            gen_size,
            self.alive_num
        ));

        let paragraph = Paragraph::new(text).block(
            Block::bordered().title(" lifegame "),
        );
        frame.render_widget(paragraph, frame.area());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let n = CYCLES; // TODO getopt
    let r = START_NUM_RATE; // TODO getopt

    let mut board = Board::new();

    ratatui::run(|terminal| {
        let size = terminal.size()?;
        let width = size.width.saturating_sub(2) as usize;
        let height = size.height.saturating_sub(3) as usize;

        board.init(width, height, r);

        let mut i = 1;
        loop {
            terminal.draw(|frame| {
                board.render(frame, i, n);
            })?;

            if i < n {
                board.update();
                i += 1;
            }

            if event::poll(std::time::Duration::from_millis(DELAY))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
        Ok(())
    })
}
