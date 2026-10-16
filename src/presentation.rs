use crate::puzzle::{Cell, Player, Statement, apply_statement, initial_grid};
use crate::solver::{SearchNode, Turn};
use std::collections::HashMap;

pub fn short_code(stmt: Statement, speaker: Player) -> String {
    let o = match speaker.other() {
        Player::Albert => "A",
        Player::Bernard => "B",
    };
    match stmt {
        Statement::DontKnow => "Don't know".to_string(),
        Statement::Know => "Know".to_string(),
        Statement::KnowOtherDoesntKnow => format!("Know {} doesn't know", o),
        Statement::DontKnowWhetherOtherKnows => format!("? whether {} knows", o),
        Statement::DidntKnowBeforeButKnowNow => "Didn't know -> Know now".to_string(),
        Statement::IfOtherSaidDontKnowIWouldKnow => format!("If {} !know -> Know", o),
    }
}

pub fn description(stmt: Statement, speaker: Player) -> String {
    let other = speaker.other().name();
    match stmt {
        Statement::DontKnow => "I don't know the location.".to_string(),
        Statement::Know => "I know the location.".to_string(),
        Statement::KnowOtherDoesntKnow => {
            format!("I know that {} doesn't know the location.", other)
        }
        Statement::DontKnowWhetherOtherKnows => {
            format!("I don't know whether {} knows the location.", other)
        }
        Statement::DidntKnowBeforeButKnowNow => format!(
            "Before {} said that, I didn't know the location, but I know now.",
            other
        ),
        Statement::IfOtherSaidDontKnowIWouldKnow => format!(
            "If {} said he didn't know, I would know the location.",
            other
        ),
    }
}

/// Generates an exact-width (33 chars) vector of strings representing a single board state.
pub fn format_board_lines(
    title: &str,
    subtitle: &str,
    current: &[Cell],
    previous: Option<&[Cell]>,
    base_all: &[Cell],
) -> Vec<String> {
    let mut row_counts = [0usize; 8];
    let mut col_counts = [0usize; 8];
    for &(r, c) in current {
        row_counts[r] += 1;
        col_counts[c] += 1;
    }

    let mut lines = Vec::with_capacity(14);
    lines.push(format!("{:<33}", title));
    lines.push(format!("{:<33}", subtitle));
    lines.push("    0  1  2  3  4  5  6  7 | R#  ".to_string());
    lines.push("  +------------------------+---  ".to_string());

    for r in 0..8 {
        let mut row_str = format!("r{}|", r);
        for c in 0..8 {
            let cell = (r, c);
            let sym = if current.contains(&cell) {
                " ■ "
            } else if previous.map_or(false, |p| p.contains(&cell)) {
                " × "
            } else if base_all.contains(&cell) {
                " □ "
            } else {
                " · "
            };
            row_str.push_str(sym);
        }
        row_str.push_str(&format!("| {:<2}  ", row_counts[r]));
        lines.push(row_str);
    }

    lines.push("  +------------------------+---  ".to_string());
    let mut col_str = "C#|".to_string();
    for c in 0..8 {
        col_str.push_str(&format!(" {:1} ", col_counts[c]));
    }
    col_str.push_str("|     ");
    lines.push(col_str);

    lines
}

/// Renders a single properly aligned 8x8 ASCII board directly to stdout.
pub fn render_step_visual(current: &[Cell], previous: Option<&[Cell]>, base_all: &[Cell]) {
    for line in format_board_lines("", "", current, previous, base_all)
        .into_iter()
        .skip(2)
    {
        println!("{}", line.trim_end());
    }
    println!();
}

// =========================================================================
// MODE 1A: Vertical Step-by-Step Sequence Visualizer
// =========================================================================
pub fn visualize_sequence(turns: &[(Player, Statement)]) {
    let base = initial_grid();
    let mut history = vec![base.clone()];

    println!("==========================================");
    println!("STEP 0: Initial Grid ({} candidates)", base.len());
    println!("==========================================");
    render_step_visual(&base, None, &base);

    for (i, &(player, stmt)) in turns.iter().enumerate() {
        let current = history.last().unwrap();
        let prev = if history.len() >= 2 {
            Some(history[history.len() - 2].as_slice())
        } else {
            None
        };
        let next = apply_statement(current, prev, player, stmt);
        let eliminated: Vec<Cell> = current
            .iter()
            .copied()
            .filter(|c| !next.contains(c))
            .collect();

        println!("==========================================");
        println!(
            "STEP {}: {} -> \"{}\"",
            i + 1,
            player.name(),
            description(stmt, player)
        );
        println!("  Surviving ({}): {:?}", next.len(), next);
        println!("  Eliminated ({}): {:?}", eliminated.len(), eliminated);
        println!("==========================================");
        render_step_visual(&next, Some(current), &base);

        history.push(next);
    }
}

// =========================================================================
// MODE 1B: Horizontal Side-by-Side Sequence Visualizer
// =========================================================================
pub fn visualize_sequence_horizontal(turns: &[(Player, Statement)], boards_per_row: usize) {
    let base = initial_grid();
    let mut history = vec![base.clone()];
    let mut all_boards = Vec::new();

    all_boards.push(format_board_lines(
        "STEP 0: Initial",
        &format!("({} candidates)", base.len()),
        &base,
        None,
        &base,
    ));

    for (i, &(player, stmt)) in turns.iter().enumerate() {
        let current = history.last().unwrap();
        let prev = if history.len() >= 2 {
            Some(history[history.len() - 2].as_slice())
        } else {
            None
        };
        let next = apply_statement(current, prev, player, stmt);

        let title = format!("STEP {}: {}", i + 1, player.name());
        let subtitle = format!("{} ({} left)", short_code(stmt, player), next.len());

        all_boards.push(format_board_lines(
            &title,
            &subtitle,
            &next,
            Some(current),
            &base,
        ));
        history.push(next);
    }

    for chunk in all_boards.chunks(boards_per_row) {
        let num_lines = chunk[0].len();
        for line_idx in 0..num_lines {
            let combined: Vec<&str> = chunk.iter().map(|b| b[line_idx].as_str()).collect();
            println!("{}", combined.join("   ").trim_end());
        }
        println!();
    }
}

// =========================================================================
// MODE 2: Shortest Isolation Path Per Cell
// =========================================================================
pub fn print_shortest_isolation_per_cell(shortest: &HashMap<Cell, Vec<Turn>>, max_depth: usize) {
    println!("==========================================");
    println!("SHORTEST ISOLATION PATH PER CELL");
    println!("==========================================");
    for cell in initial_grid() {
        match shortest.get(&cell) {
            Some(path) => {
                println!("Cell {:?} ({} steps):", cell, path.len());
                for (i, &(p, s)) in path.iter().enumerate() {
                    println!("  {}. {}: \"{}\"", i + 1, p.name(), description(s, p));
                }
            }
            None => println!(
                "Cell {:?}: Not uniquely isolatable within depth {}",
                cell, max_depth
            ),
        }
    }
    println!();
}

// =========================================================================
// MODE 3: Pruned Hierarchical Search Tree
// =========================================================================
pub fn print_solution_tree(tree: &[SearchNode]) {
    println!("Root ({} cells)", initial_grid().len());
    print_nodes(tree, "");
}

fn print_nodes(nodes: &[SearchNode], prefix: &str) {
    for (idx, node) in nodes.iter().enumerate() {
        let is_last = idx == nodes.len() - 1;
        let (player, stmt) = node.turn;
        let status = if node.cells.len() == 1 {
            format!("🎯 SOLVED: {:?}", node.cells[0])
        } else {
            format!("{} cells: {:?}", node.cells.len(), node.cells)
        };
        println!(
            "{}{}{}: \"{}\" ──> [{}]",
            prefix,
            if is_last { "└── " } else { "├── " },
            player.name(),
            description(stmt, player),
            status
        );
        let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
        print_nodes(&node.children, &child_prefix);
    }
}
