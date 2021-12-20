use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::thread::current;

use itertools;

use crate::TreeNode::{Leaf, NonLeaf};

const E1: &str = "";
const E2: &str = "";
const E3: &str = "";
const E4: &str = "";
const INPUT: &str = "";

struct BinaryTree {
    top: TreeNode,
}

impl BinaryTree {}

/// Does not do any reduction yet
fn add(first: TreeNode, second: TreeNode) -> TreeNode {
    NonLeaf(Box::new(first), Box::new(second))
}

impl FromStr for BinaryTree {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let top = TreeNode::from_str(s)?;
        // let depth = calculate_depth(&top);
        Ok(BinaryTree { top })
    }
}

fn calculate_depth(top: &TreeNode) -> u8 {
    match top {
        NonLeaf(l, r) => {
            let left_depth = 1 + calculate_depth(l);
            let right_depth = 1 + calculate_depth(r);
            if left_depth > right_depth { left_depth } else { right_depth }
        }
        Leaf(_) => 0
    }
}


#[derive(Debug, Eq, PartialEq)]
enum TreeNode {
    NonLeaf(Box<TreeNode>, Box<TreeNode>),
    Leaf(u8),
}

impl TreeNode {
    // TODO: This won't work like this...
    //  I need to change it to include a "root" element for every node
    //  I will need that for the explode function, to find the right/left elements
    //  For this, I think I need the Rc<> type
    fn explode(&mut self, current_depth: u8) -> bool {
        if current_depth == 4 {
            match self {
                NonLeaf(l, r) => {
                    match l {
                        NonLeaf(ll, rr) => todo!(),
                        Leaf()
                    }
                }
                Leaf(_) => false
            }
        }

        match self {
            NonLeaf(l, r) => {
                l.explode(current_depth + 1) || r.explode(current_depth + 1)
            }
            Leaf(_) => false
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NonLeaf(a, b) => write!(f, "[{},{}]", a, b)?,
            Leaf(n) => write!(f, "{}", n)?
        }

        Ok(())
    }
}

impl FromStr for TreeNode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.parse::<u8>();
        if result.is_ok() {
            Ok(Leaf(result.unwrap()))
        } else {
            let inner = &s[1..s.len() - 1]; // remove outermost brackets
            if inner.chars().nth(0).unwrap() == '[' {
                let index_closing_bracket = find_index_closing_bracket(inner);
                let left = &inner[0..index_closing_bracket + 1];
                let right = &inner[index_closing_bracket + 2..inner.len()];
                Ok(NonLeaf(Box::new(TreeNode::from_str(left).unwrap()),
                           Box::new(TreeNode::from_str(right).unwrap())))
            } else {
                let (left, right) = inner.split_once(",").unwrap();
                Ok(NonLeaf(Box::new(TreeNode::from_str(left).unwrap()),
                           Box::new(TreeNode::from_str(right).unwrap())))
            }
        }
    }
}

/// Expects s to start with '['
fn find_index_closing_bracket(s: &str) -> usize {
    let mut s_chars = s.chars();
    if s_chars.next().unwrap() != '[' { panic!() }

    let mut count = 1;

    for i in 1..s.len() {
        let nth_char = s_chars.next().unwrap();
        if nth_char == '[' {
            count += 1;
        } else if nth_char == ']' {
            count -= 1;
            if count == 0 { return i; }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tree_node() {
        assert_eq!(Ok(NonLeaf(Box::new(Leaf(1)), Box::new(Leaf(2)))), TreeNode::from_str("[1,2]"));
        println!("{:?}", TreeNode::from_str("[[1,2],3]"));
        assert_eq!("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", TreeNode::from_str("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").unwrap().to_string())
    }

    #[test]
    fn test_depth() {
        let tree = BinaryTree::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        assert_eq!(5, calculate_depth(&tree.top));
    }

    #[test]
    fn add_two() {
        // [1,2] + [[3,4],5]
        let first = TreeNode::from_str("[1,2]").unwrap();
        let second = TreeNode::from_str("[[3,4],5]").unwrap();

        let result = add(first, second);
        println!("{}", result);
    }
}

fn part1() {}


fn part2() {}

fn main() {
    part1();
}