use std::cmp;

// Define the structure of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor to create a new TreeNode
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

// Function to recursively calculate the maximum depth of a binary tree
fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
    // Example usage: Read a binary tree from the user and calculate the maximum depth
    let root = read_tree();
    let depth = max_depth(&root);
    println!("Maximum depth of the binary tree: {}", depth);
}

// Function to read a binary tree from the user
fn read_tree() -> Option<Box<TreeNode>> {
    println!("Enter the value of the root node (or -1 to represent a null node):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let val: i32 = input.trim().parse().expect("Invalid input");

    if val == -1 {
        return None;
    }

    let mut root = Box::new(TreeNode::new(val));

    println!("Enter the left child of {} (or -1 for null):", val);
    root.left = read_tree();

    println!("Enter the right child of {} (or -1 for null):", val);
    root.right = read_tree();

    Some(root)
}

