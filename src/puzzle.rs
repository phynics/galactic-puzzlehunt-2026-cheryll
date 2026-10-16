pub type Cell = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Player {
    Albert,  // Knows row
    Bernard, // Knows col
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Albert => Player::Bernard,
            Player::Bernard => Player::Albert,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Player::Albert => "Albert",
            Player::Bernard => "Bernard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Statement {
    DontKnow,
    Know,
    KnowOtherDoesntKnow,
    DontKnowWhetherOtherKnows,
    DidntKnowBeforeButKnowNow,
    IfOtherSaidDontKnowIWouldKnow,
}

impl Statement {
    pub fn all() -> [Statement; 6] {
        [
            Statement::DontKnow,
            Statement::Know,
            Statement::KnowOtherDoesntKnow,
            Statement::DontKnowWhetherOtherKnows,
            Statement::DidntKnowBeforeButKnowNow,
            Statement::IfOtherSaidDontKnowIWouldKnow,
        ]
    }
}

pub fn initial_grid() -> Vec<Cell> {
    vec![
        (0, 2),
        (0, 5),
        (1, 3),
        (1, 6),
        (2, 1),
        (2, 3),
        (3, 2),
        (3, 6),
        (4, 0),
        (4, 5),
        (4, 7),
        (5, 4),
        (5, 7),
        (6, 4),
        (7, 3),
    ]
}

pub fn slice(grid: &[Cell], player: Player, cell: Cell) -> Vec<Cell> {
    grid.iter()
        .copied()
        .filter(|&(r, c)| match player {
            Player::Albert => r == cell.0,
            Player::Bernard => c == cell.1,
        })
        .collect()
}

pub fn knows(grid: &[Cell], player: Player, cell: Cell) -> bool {
    slice(grid, player, cell).len() == 1
}

pub fn apply_statement(
    current: &[Cell],
    previous: Option<&[Cell]>,
    player: Player,
    stmt: Statement,
) -> Vec<Cell> {
    let other = player.other();

    match stmt {
        Statement::DontKnow => current
            .iter()
            .copied()
            .filter(|&cell| !knows(current, player, cell))
            .collect(),

        Statement::Know => current
            .iter()
            .copied()
            .filter(|&cell| knows(current, player, cell))
            .collect(),

        Statement::KnowOtherDoesntKnow => current
            .iter()
            .copied()
            .filter(|&cell| {
                slice(current, player, cell)
                    .into_iter()
                    .all(|c| !knows(current, other, c))
            })
            .collect(),

        Statement::DontKnowWhetherOtherKnows => current
            .iter()
            .copied()
            .filter(|&cell| {
                let s = slice(current, player, cell);
                let some_know = s.iter().any(|&c| knows(current, other, c));
                let some_dont = s.iter().any(|&c| !knows(current, other, c));
                some_know && some_dont
            })
            .collect(),

        Statement::DidntKnowBeforeButKnowNow => {
            let prev = match previous {
                Some(p) => p,
                None => return vec![],
            };
            current
                .iter()
                .copied()
                .filter(|&cell| !knows(prev, player, cell) && knows(current, player, cell))
                .collect()
        }

        Statement::IfOtherSaidDontKnowIWouldKnow => {
            let hypothetical: Vec<Cell> = current
                .iter()
                .copied()
                .filter(|&cell| !knows(current, other, cell))
                .collect();

            // Truth depends only on the speaker's own slice: the speaker may
            // actually sit on a cell the other player would know.
            current
                .iter()
                .copied()
                .filter(|&cell| slice(&hypothetical, player, cell).len() == 1)
                .collect()
        }
    }
}
