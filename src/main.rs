mod presentation;
mod puzzle;
mod solver;

fn main() {
    let max_depth = 60;
    let shortest = solver::shortest_isolation_paths(max_depth);
    presentation::print_shortest_isolation_per_cell(&shortest, max_depth);

    // Other views:
    // presentation::visualize_sequence(&sequence);
    // presentation::visualize_sequence_horizontal(&sequence, 4);
    // presentation::print_solution_tree(&solver::solution_tree(3));
}
