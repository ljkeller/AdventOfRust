use std::io::BufRead;
use slab_tree::{*, iter::PreOrder};

pub fn size_one(terminal_output_fp: &str) -> i64 {
    let mut byte_sum = 0;

    // build root
    let mut tree = TreeBuilder::new().with_root("hello").build();
    tree.get_mut(tree.root_id().unwrap()).unwrap().append("world");

    if let Ok(terminal_out_file) = std::fs::File::open(terminal_output_fp) {
        let terminal_lines = std::io::BufReader::new(terminal_out_file).lines();
        // build tree
        for line in terminal_lines {
            println!("{}", line.unwrap());
        }
    } else {
        println!("Could not find file: {} at path {}", terminal_output_fp, std::env::current_dir().unwrap().display());
    }

    // do a post_order traversal to count datasize, propogating data up
    tree.root().unwrap().traverse_post_order().for_each(|n| {
        println!("{}", n.data());
    });

    return byte_sum;
}

pub fn tmp_two(fp: &str) -> i32 {
    return 0;
}