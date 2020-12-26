#[derive(Clone, Debug, PartialEq)]
enum BNode {
    BTree(i32, Box<BNode>, Box<BNode>),
    Nil,
}

use crate::BNode::{BTree, Nil};

fn insert(btree: &BNode, num: i32) -> BNode{
   match btree {
       Nil => BTree(num, Box::new(Nil), Box::new(Nil)),
       BTree(a, ref b, ref c) => {
           if **b == Nil && **c == Nil {
               if num <= *a {
                   return BTree(*a, Box::new(BTree(num, Box::new(Nil), Box::new(Nil))), (*c).clone());
               } else {
                   return BTree(*a, (*b).clone(), Box::new(BTree(num, Box::new(Nil), Box::new(Nil))));
               }
           }
           if num <= *a {
               BTree(*a, Box::new(insert(&**b, num)), (*c).clone())
           } else {
               BTree(*a, (*b).clone(), Box::new(insert(&**c, num)))
           }
       }
   }
}

fn main(){
    let btree =  BTree(1, 
        Box::new(BTree(-1, Box::new(Nil), Box::new(Nil))), Box::new(BTree(3, 
                                                    Box::new(BTree(2, Box::new(Nil), Box::new(Nil))), Box::new(Nil))));

    println!("original");
    println!("{:#?}", btree);

    let btree = insert(&btree, 2);

    println!("\ninsert 2");
    println!("{:#?}", btree);

    let btree = insert(&btree, -2);

    println!("\ninsert -2");
    println!("{:#?}", btree);

    let btree = insert(&btree, 10);

    println!("\ninsert 10");
    println!("{:#?}", btree);

    let btree = insert(&btree, -10);

    println!("\ninsert -10");
    println!("{:#?}", btree);
}
