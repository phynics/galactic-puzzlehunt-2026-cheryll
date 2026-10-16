use crate::puzzle::{Cell, Player, Statement, apply_statement, initial_grid};
use std::collections::{HashMap, HashSet, VecDeque};

pub type Turn = (Player, Statement);

/// One node of the pruned search tree: a turn, the grid it leaves, and follow-ups.
pub struct SearchNode {
    pub turn: Turn,
    pub cells: Vec<Cell>,
    pub children: Vec<SearchNode>,
}

/// All legal, grid-changing, non-empty turns from the end of `history`.
fn successors(history: &[Vec<Cell>], path: &[Turn]) -> Vec<(Turn, Vec<Cell>)> {
    let current = history.last().unwrap();
    let prev = history.len().checked_sub(2).map(|i| history[i].as_slice());
    let mut out = Vec::new();
    for &player in &[Player::Albert, Player::Bernard] {
        for &stmt in &Statement::all() {
            // "Before <other> said that ..." needs the other player to have just spoken.
            if stmt == Statement::DidntKnowBeforeButKnowNow
                && path.last().map_or(true, |&(last, _)| last == player)
            {
                continue;
            }
            let next = apply_statement(current, prev, player, stmt);
            if !next.is_empty() && &next != current {
                out.push(((player, stmt), next));
            }
        }
    }
    out
}

/// BFS: shortest turn sequence isolating each cell, within `max_depth`.
pub fn shortest_isolation_paths(max_depth: usize) -> HashMap<Cell, Vec<Turn>> {
    let base = initial_grid();
    let mut shortest = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((vec![base], Vec::<Turn>::new()));
    // State = (grid, previous grid, last speaker); the latter two matter for
    // DidntKnowBeforeButKnowNow.
    let mut visited: HashSet<(Vec<Cell>, Option<Vec<Cell>>, Option<Player>)> = HashSet::new();

    while let Some((history, path)) = queue.pop_front() {
        let current = history.last().unwrap();
        if current.len() == 1 {
            shortest.entry(current[0]).or_insert_with(|| path.clone());
            continue;
        }
        if path.len() >= max_depth {
            continue;
        }
        for (turn, next) in successors(&history, &path) {
            let key = (next.clone(), Some(current.clone()), Some(turn.0));
            if visited.insert(key) {
                let mut nh = history.clone();
                nh.push(next);
                let mut np = path.clone();
                np.push(turn);
                queue.push_back((nh, np));
            }
        }
    }
    shortest
}

/// Search tree up to `max_depth`, keeping only branches that can still isolate a cell.
pub fn solution_tree(max_depth: usize) -> Vec<SearchNode> {
    expand(&[initial_grid()], &[], max_depth)
}

fn expand(history: &[Vec<Cell>], path: &[Turn], max_depth: usize) -> Vec<SearchNode> {
    if path.len() >= max_depth || history.last().unwrap().len() <= 1 {
        return vec![];
    }
    successors(history, path)
        .into_iter()
        .filter_map(|(turn, cells)| {
            let mut nh = history.to_vec();
            nh.push(cells.clone());
            let mut np = path.to_vec();
            np.push(turn);
            can_reach_unique(&nh, &np, max_depth).then(|| SearchNode {
                turn,
                cells,
                children: expand(&nh, &np, max_depth),
            })
        })
        .collect()
}

fn can_reach_unique(history: &[Vec<Cell>], path: &[Turn], max_depth: usize) -> bool {
    let current = history.last().unwrap();
    if current.len() == 1 {
        return true;
    }
    if path.len() >= max_depth {
        return false;
    }
    successors(history, path).into_iter().any(|(turn, next)| {
        let mut nh = history.to_vec();
        nh.push(next);
        let mut np = path.to_vec();
        np.push(turn);
        can_reach_unique(&nh, &np, max_depth)
    })
}
