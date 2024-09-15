/// A limited emacs reader that understands just enough to be able to turn the
/// sexps from the org-roam database into objects

// use super::testdata::TESTDATA;

use std::{cell::RefCell, rc::Rc};

enum EmacsObject {
    Cons(Cons<EmacsObject, EmacsObject>),
    Atom(Atom),
}

enum Atom {
    Nil(()),
    S(String),
    I(i32),
    F(f32)
}

struct Cons<CAR, CDR> {
    car: Box<CAR>,
    cdr: Box<CDR>
}

enum Sequence {
    List(),
    Array()
}

enum Array {
    String(String),
    Vector,
    CharTable,
    BoolVector
}

pub struct SexpTree {
    pub repr: Option<String>,
    pub parent: Option<Rc<RefCell<SexpTree>>>,
    pub children: Option<Vec<Rc<RefCell<SexpTree>>>>
}

impl SexpTree {
    pub fn parse_from_string_v1(input: String) -> Rc<RefCell<Self>> {
        let mut depth: i8 = 0; //max sexp depth 255
        let mut indents: Vec<usize> = vec![];
        let mut pair: (char, i8);
        let root = Rc::new(
            RefCell::new(
                SexpTree {
                    repr: Some(input.clone()),
                    parent: None,
                    children: None
                }
            )
        );
        let mut depth_overlay = String::new();
        let mut current_node  = Rc::clone(&root);
        for (pos, c) in input.chars().enumerate() {
            pair = (c, depth);

            match pair {
                ('(', _) => {
                    depth += 1;
                    depth_overlay.push_str(depth.clone().to_string().as_str());
                    indents.push(pos);
                    let new_node = Rc::new(
                        RefCell::new(
                            SexpTree {
                                repr: None,
                                parent: Some(Rc::clone(&current_node)),
                                children: None
                            }
                        )
                    );
                    current_node.borrow_mut().add_child(&new_node);
                    current_node = Rc::clone(&new_node);
                },
                (_, 0) => {
                    panic!("S-exp does not begin with an opening parenthesis '('.");
                },
                (')', _) => {
                    depth -= 1;
                    depth_overlay.push_str(depth.clone().to_string().as_str());
                    //*B*eginning *O*f *S*ubstring -> bos
                    let bos = indents.pop();
                    match bos {
                        Some(beg_pos) => {
                            let end_pos = pos + 1;
                            let substr = String::from(&input.clone()[beg_pos..end_pos]);
                            println!("{:?}", substr);
                            current_node.borrow_mut().repr = Some(substr);
                            let parent_ref;
                            match &current_node.borrow().parent {
                                Some(p) => {
                                    parent_ref = Rc::clone(p);
                                },
                                None => {
                                    panic!("Did not find parent node where there was expected to be one.")
                                }
                            }
                            current_node = parent_ref;
                        },
                        None => {panic!("Failed to find the end-position of a substring.")}
                    }
                },
                _ => {
                    if c ==  '"' {
                        depth_overlay.push_str("  ");
                    } else {
                        depth_overlay.push_str(" ");
                    }
                    // println!("hit some edge case");
                }
            }
        };
        // println!("{:?}", input.clone());
        // println!("{:?}", depth_overlay);
        root
    }

    pub fn add_child(&mut self, child: &Rc<RefCell<SexpTree>>) {
        match &mut self.children {
            None => {
                self.children = Some(vec![Rc::clone(child)]);
            },
            Some(v) => {
                v.push(Rc::clone(child));
            }
        }
    }
}

pub fn parse(input: String) // -> EmacsObject
{
    let root: Box<EmacsObject>;
    // let current = &root;
    for c in input.chars() {
        // println!("{}", c);
        match c {
            '(' => {
                println!("paren: {}", c);
            },
            _ => {
                println!("not paren: {}", c);
            }
        }
    }
}
