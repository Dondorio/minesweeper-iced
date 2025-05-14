use colored::Colorize;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum GameState {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub enum CellType {
    Safe,
    Bomb,
    // Bool is for uncovering adjacent cells without processing the same cell over and over
    Empty(bool),
}

#[derive(Debug, Clone)]
pub enum CellState {
    Hidden,
    Uncovered,
    Flagged,
}

#[derive(Debug, Clone)]
pub enum CellPressedState {
    Pressed,
    Hovered,
    None,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub state: CellState,
    pub cell_type: CellType,
    pub adjacent_bomb_count: u8,
    pub pressed_state: CellPressedState,
    pub is_exploded: bool,
}

impl Cell {
    fn new(m_type: CellType) -> Cell {
        Cell {
            cell_type: m_type,
            adjacent_bomb_count: 0,
            state: CellState::Hidden,
            pressed_state: CellPressedState::None,
            is_exploded: false,
        }
    }

    pub fn toggle_flagged(&mut self) {
        match self.state {
            CellState::Hidden => self.state = CellState::Flagged,
            CellState::Flagged => self.state = CellState::Hidden,
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub running: bool,
    pub size_x: usize,
    pub size_y: usize,
    pub bomb_count: usize,
    pub cells: Vec<Vec<Cell>>,
    pub state: GameState,
    uncovered_cells: usize,
}

#[allow(dead_code)]
impl Board {
    pub fn new(
        m_size_x: usize,
        m_size_y: usize,
        m_bomb_count: usize,
    ) -> Result<Board, Box<dyn std::error::Error>> {
        if m_size_x <= 0 || m_size_y <= 0 {
            return Err("Board too small".into());
        }

        let c = (0..m_size_x)
            .map(|_| (0..m_size_y).map(|_| Cell::new(CellType::Safe)).collect())
            .collect();

        let mut b = Board {
            cells: c,
            running: true,
            size_x: m_size_x,
            size_y: m_size_y,
            bomb_count: m_bomb_count,
            state: GameState::Playing,
            uncovered_cells: 0,
        };

        match b.place_bombs() {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        b.calculate_adjacent_bombs();

        Ok(b)
    }

    fn place_bombs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.bomb_count as usize > (self.size_x * self.size_y) {
            return Err("Too many bombs".into());
        }

        let mut rng = rand::rng();
        for _ in 0..self.bomb_count {
            let mut x = rng.random_range(0..self.size_x);
            let mut y = rng.random_range(0..self.size_y);
            while let CellType::Bomb = self.cells[x][y].cell_type {
                x = rng.random_range(0..self.size_x);
                y = rng.random_range(0..self.size_y);
            }
            self.cells[x][y].cell_type = CellType::Bomb;
        }
        Ok(())
    }

    fn calculate_adjacent_bombs(&mut self) {
        for i in 0..self.size_x {
            for j in 0..self.size_y {
                let cell_pos = &Vec2 { x: i, y: j };

                let bomb_count = self
                    .get_adjacent_cells(cell_pos)
                    .clone()
                    .iter()
                    .filter(|&i| matches!(self.get_cell(i).cell_type, CellType::Bomb))
                    .count() as u8;

                match bomb_count {
                    0 => {
                        self.get_mut_cell(cell_pos).cell_type = CellType::Empty(false);
                    }
                    _ => self.get_mut_cell(cell_pos).adjacent_bomb_count = bomb_count,
                }
            }
        }
    }

    pub fn get_mut_cell(&mut self, pos: &Vec2) -> &mut Cell {
        &mut self.cells[pos.x][pos.y]
    }

    pub fn get_cell(&self, pos: &Vec2) -> &Cell {
        &self.cells[pos.x][pos.y]
    }

    fn get_adjacent_cells(&mut self, m_pos: &Vec2) -> Vec<Vec2> {
        let min_x = if m_pos.x == 0 { 1 } else { 0 };
        let min_y = if m_pos.y == 0 { 1 } else { 0 };

        let max_x = if m_pos.x + 1 >= self.size_x { 2 } else { 3 };
        let max_y = if m_pos.y + 1 >= self.size_y { 2 } else { 3 };

        (min_x..max_x)
            .into_iter()
            .flat_map(|i| {
                (min_y..max_y).map(move |j| Vec2 {
                    x: (m_pos.x + i - 1),
                    y: (m_pos.y + j - 1),
                })
            })
            .collect::<Vec<_>>()
    }

    pub fn uncover(&mut self, pos: &Vec2) {
        let non_bomb_cells = self.size_x * self.size_y - self.bomb_count as usize;

        if pos.x >= self.size_x || pos.y >= self.size_y {
            return;
        }

        let c = self.get_mut_cell(pos);

        match c.state {
            CellState::Hidden => {
                c.state = CellState::Uncovered;
                match c.cell_type {
                    CellType::Bomb => {
                        c.is_exploded = true;
                        self.stop();
                    }
                    CellType::Empty(false) => {
                        c.cell_type = CellType::Empty(true);
                        self.uncover_adjacent(pos);
                    }
                    _ => {}
                }
                self.uncovered_cells += 1;
            }
            CellState::Uncovered => {
                c.state = CellState::Uncovered;

                let adjacent_bombs = self.get_cell(pos).adjacent_bomb_count;
                let adjacent_flags = self
                    .get_adjacent_cells(pos)
                    .clone()
                    .iter()
                    .filter(|&i| matches!(self.get_cell(i).state, CellState::Flagged))
                    .count() as u8;

                if adjacent_bombs == adjacent_flags {
                    self.uncover_adjacent(pos);
                }
            }
            _ => {}
        }

        if let GameState::Playing = self.state {
            if self.uncovered_cells == non_bomb_cells {
                self.state = GameState::Won;
                self.running = false;
            }
        }
    }

    fn uncover_adjacent(&mut self, pos: &Vec2) {
        for i in self.get_adjacent_cells(pos) {
            if let CellState::Hidden = self.get_cell(&i).state {
                self.uncover(&i);
            }
        }
    }

    pub fn uncover_all(&mut self) {
        for i in 0..self.size_x {
            for j in 0..self.size_y {
                let pos = &Vec2 { x: i, y: j };
                self.get_mut_cell(pos).state = CellState::Uncovered;
            }
        }
    }

    pub fn draw(&self) {
        print!("__|");

        for k in 0..self.size_y {
            print!("{} ", k);
        }

        print! {"\n"};

        for i in 0..self.size_x {
            print!("{} |", i);

            for j in 0..self.size_y {
                let cell = &self.cells[j][i];
                let bomb_count = cell.adjacent_bomb_count.to_string();

                match cell.state {
                    CellState::Hidden => print!("{}", " ".white().bold()),
                    CellState::Flagged => print!("{}", " ".red()),

                    CellState::Uncovered => match cell.cell_type {
                        CellType::Bomb => print!("{}", " ".black()),
                        CellType::Safe => match cell.adjacent_bomb_count {
                            1 => print!("{} ", bomb_count.blue()),
                            2 => print!("{} ", bomb_count.green()),
                            3 => print!("{} ", bomb_count.bright_red().dimmed()),
                            4 => print!("{} ", bomb_count.blue().dimmed()),
                            5 => print!("{} ", bomb_count.red()),
                            6 => print!("{} ", bomb_count.cyan()),
                            7 => print!("{} ", bomb_count.black()),
                            8 => print!("{} ", bomb_count.bright_white()),
                            _ => print!(""),
                        },
                        CellType::Empty(..) => {
                            print!("  ");
                        }
                    },
                }
            }
            print!("\n");
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.state = GameState::Lost;
        self.uncover_all();
    }
}
