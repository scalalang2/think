use std::{io, fmt::{Error, self}, marker::PhantomData};

#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};

type BNode<T> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
    value: T,
    left: BNode<T>,
    right: BNode<T>,
}

struct BST<T: Ord> {
    root: BNode<T>,
}

impl<T> BST<T> where
    T: Ord + Copy + fmt::Display,
{
    fn new() -> Self {
        Self {
            root: None,
        }
    }

    fn insert(&mut self, value: T) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            if value < node.value {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }

        // 이게 러스트 방식 맞냐
        *current = Some(Box::new(Node{
            value,
            left: None,
            right: None,
        }));
    }

    fn post_order(node: &BNode<T>) {
        match node {
            Some(n) => {
                Self::post_order(&n.left);
                Self::post_order(&n.right);
                println!("{}", n.value);
            },
            None => return,
        }
    }
}

fn main() {
    let mut bst: BST<i32> = BST::new();
    
    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    // EOF
                    break
                }

                let num:i32 = buf.trim_end().to_string().parse().unwrap();
                bst.insert(num);
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    BST::post_order(&bst.root);
}