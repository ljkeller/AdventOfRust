pub fn visible_one(trees_fp: &str) -> usize {
    let mut visible_trees = 0;

    let mut forest: Vec<Vec<u8>> = Vec::new();
    if let Ok(trees_string) = std::fs::read_to_string(trees_fp) {
        trees_string.lines().for_each(|tree_row| {
            forest.push(
                tree_row
                    .chars()
                    .map(|c| c.to_digit(10).expect("Expect digit") as u8)
                    .collect(),
            )
        });

        for (row_idx, tree_row) in forest.iter().enumerate() {
            for (col_idx, tree) in tree_row.iter().enumerate() {
                visible_trees = if tree_visible(&forest, row_idx, col_idx, &tree) {
                    visible_trees + 1
                } else {
                    visible_trees
                };
            }
        }
    } else {
        println!(
            "Couldnt find file: {} at path {}",
            trees_fp,
            std::env::current_dir().unwrap().display()
        );
    }

    return visible_trees;
}

pub fn visible_two(trees_fp: &str) -> u64 {
    let mut max_view_score = 0;

    let mut forest: Vec<Vec<u8>> = Vec::new();
    if let Ok(trees_string) = std::fs::read_to_string(trees_fp) {
        trees_string.lines().for_each(|tree_row| {
            forest.push(
                tree_row
                    .chars()
                    .map(|c| c.to_digit(10).expect("Expect digit") as u8)
                    .collect(),
            )
        });

        for (row_idx, tree_row) in forest.iter().enumerate() {
            for (col_idx, tree) in tree_row.iter().enumerate() {
                max_view_score =
                    std::cmp::max(max_view_score, tree_score(&forest, row_idx, col_idx, &tree));
            }
        }
    } else {
        println!(
            "Couldnt find file: {} at path {}",
            trees_fp,
            std::env::current_dir().unwrap().display()
        );
    }

    return max_view_score;
}

fn tree_score(
    forest: &Vec<Vec<u8>>,
    target_row: usize,
    target_col: usize,
    target_tree: &u8,
) -> u64 {
    return vis_to_left(forest, target_row, target_col, target_tree)
        * vis_to_right(forest, target_row, target_col, target_tree)
        * vis_to_top(target_row, forest, target_col, target_tree)
        * vis_to_bottom(target_row, forest, target_col, target_tree);
}

// trees on perimeter: visible
// trees not in perimeter: visible only if visible from at least one direction
fn tree_visible(
    forest: &Vec<Vec<u8>>,
    target_row: usize,
    target_col: usize,
    target_tree: &u8,
) -> bool {
    return vis_from_left(forest, target_row, target_col, target_tree)
        || vis_from_right(forest, target_row, target_col, target_tree)
        || vis_from_top(target_row, forest, target_col, target_tree)
        || vis_from_bottom(target_row, forest, target_col, target_tree);
}

fn vis_from_bottom(
    target_row: usize,
    forest: &Vec<Vec<u8>>,
    target_col: usize,
    target_tree: &u8,
) -> bool {
    let mut vis_from_bottom = true;
    for row in target_row + 1..forest.len() {
        let cur_tree = forest[row][target_col];
        if &cur_tree >= target_tree {
            vis_from_bottom = false;
            break;
        }
    }
    vis_from_bottom
}

fn vis_from_top(
    target_row: usize,
    forest: &Vec<Vec<u8>>,
    target_col: usize,
    target_tree: &u8,
) -> bool {
    let mut vis_from_top = true;
    for row in 0..target_row {
        let cur_tree = forest[row][target_col];
        if &cur_tree >= target_tree {
            vis_from_top = false;
            break;
        }
    }
    vis_from_top
}

fn vis_from_right(
    forest: &Vec<Vec<u8>>,
    target_row: usize,
    target_col: usize,
    target_tree: &u8,
) -> bool {
    let mut vis_from_right = true;
    for cur_tree in forest[target_row][target_col + 1..].iter() {
        if cur_tree >= target_tree {
            vis_from_right = false;
            break;
        }
    }
    vis_from_right
}

fn vis_from_left(
    forest: &Vec<Vec<u8>>,
    target_row: usize,
    target_col: usize,
    target_tree: &u8,
) -> bool {
    let mut vis_from_left = true;
    for cur_tree in forest[target_row][..target_col].iter() {
        if cur_tree >= target_tree {
            vis_from_left = false;
            break;
        }
    }
    vis_from_left
}

fn vis_to_bottom(
    target_row: usize,
    forest: &Vec<Vec<u8>>,
    target_col: usize,
    target_tree: &u8,
) -> u64 {
    let mut visible = 0;
    for row in target_row + 1..forest.len() {
        let cur_tree = forest[row][target_col];
        visible += 1;
        if &cur_tree >= target_tree {
            break;
        }
    }
    visible
}

fn vis_to_top(
    target_row: usize,
    forest: &Vec<Vec<u8>>,
    target_col: usize,
    target_tree: &u8,
) -> u64 {
    let mut visible = 0;
    // order matters, as we extend from current tree to top of forest
    for row in (0..target_row).rev() {
        let cur_tree = forest[row][target_col];
        visible += 1;
        if &cur_tree >= target_tree {
            break;
        }
    }
    visible
}

fn vis_to_right(
    forest: &Vec<Vec<u8>>,
    target_row: usize,
    target_col: usize,
    target_tree: &u8,
) -> u64 {
    let mut visible = 0;
    for cur_tree in forest[target_row][target_col + 1..].iter() {
        visible += 1;
        if cur_tree >= target_tree {
            break;
        }
    }
    visible
}

fn vis_to_left(
    forest: &Vec<Vec<u8>>,
    target_row: usize,
    target_col: usize,
    target_tree: &u8,
) -> u64 {
    let mut visible = 0;
    // order matters, as we extend from current tree to left side of forest
    for cur_tree in forest[target_row][..target_col].iter().rev() {
        visible += 1;
        if cur_tree >= target_tree {
            break;
        }
    }
    visible
}
