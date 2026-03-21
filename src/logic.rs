use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
    VictoryRevealed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellContent {
    Empty(u8),
    Mine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
    pub content: CellContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub status: GameStatus,
    pub mine_count: usize,
    pub first_click: bool,
}

impl Grid {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let cells = vec![
            Cell {
                state: CellState::Hidden,
                content: CellContent::Empty(0),
            };
            width * height
        ];

        Grid {
            width,
            height,
            cells,
            status: GameStatus::Playing,
            mine_count,
            first_click: true,
        }
    }

    fn place_mines(&mut self, safe_x: usize, safe_y: usize) {
        let mut rng = rand::rng();
        let mut mines_placed = 0;
        
        while mines_placed < self.mine_count {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);
            
            // CLASSIC RULE: Only the clicked cell is safe
            let is_clicked_cell = x == safe_x && y == safe_y;

            let idx = y * self.width + x;
            if !is_clicked_cell && self.cells[idx].content != CellContent::Mine {
                self.cells[idx].content = CellContent::Mine;
                mines_placed += 1;
            }
        }

        // Calculate proximity numbers
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_cell(x, y).content != CellContent::Mine {
                    let count = self.count_neighbor_mines(x, y);
                    grid_set_cell_content(self, x, y, CellContent::Empty(count));
                }
            }
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y * self.width + x]
    }

    fn count_neighbor_mines(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    if self.get_cell(nx as usize, ny as usize).content == CellContent::Mine {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn count_neighbor_flags(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    if self.get_cell(nx as usize, ny as usize).state == CellState::Flagged {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        if self.status != GameStatus::Playing { return; }

        if self.first_click {
            self.place_mines(x, y);
            self.first_click = false;
        }

        let cell = *self.get_cell(x, y);
        if cell.state == CellState::Revealed {
            if let CellContent::Empty(count) = cell.content {
                if count > 0 && self.count_neighbor_flags(x, y) == count {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                                let neighbor = self.get_cell(nx as usize, ny as usize);
                                if neighbor.state == CellState::Hidden {
                                    self.reveal(nx as usize, ny as usize);
                                }
                            }
                        }
                    }
                }
            }
            return;
        }

        if cell.state != CellState::Hidden { return; }

        match cell.content {
            CellContent::Mine => {
                self.get_cell_mut(x, y).state = CellState::Revealed;
                self.status = GameStatus::Lost;
                for c in &mut self.cells {
                    if c.content == CellContent::Mine {
                        c.state = CellState::Revealed;
                    }
                }
            }
            CellContent::Empty(count) => {
                self.get_cell_mut(x, y).state = CellState::Revealed;
                if count == 0 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                                self.reveal(nx as usize, ny as usize);
                            }
                        }
                    }
                }
            }
        }
        self.check_win();
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.status != GameStatus::Playing { return; }
        let cell = self.get_cell_mut(x, y);
        if cell.state == CellState::Hidden {
            cell.state = CellState::Flagged;
        } else if cell.state == CellState::Flagged {
            cell.state = CellState::Hidden;
        }
    }

    fn check_win(&mut self) {
        if self.status != GameStatus::Playing { return; }
        let all_safe_revealed = self.cells.iter().all(|c| match c.content {
            CellContent::Mine => true,
            CellContent::Empty(_) => c.state == CellState::Revealed,
        });
        if all_safe_revealed {
            self.status = GameStatus::Won;
            for c in &mut self.cells {
                if c.content == CellContent::Mine {
                    c.state = CellState::VictoryRevealed;
                }
            }
        }
    }

    pub fn flagged_count(&self) -> usize {
        self.cells.iter().filter(|c| c.state == CellState::Flagged).count()
    }
}

// Helper to avoid borrow issues during proximity calc
fn grid_set_cell_content(grid: &mut Grid, x: usize, y: usize, content: CellContent) {
    grid.cells[y * grid.width + x].content = content;
}
