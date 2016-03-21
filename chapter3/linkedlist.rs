#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn insert(self, elem: u32, position: u32) -> List {
        match self {
            List::Cons(head, tail) => {
                if position == 0 {
                    List::Cons(elem, Box::new(List::Cons(head, tail)))
                } else {
                    List::Cons(head, Box::new(tail.insert(elem, position - 1)))
                }
            },
            List::Nil => {
                if position == 0 {
                    List::Cons(elem, Box::new(List::Nil))
                } else {
                    List::Nil
                }
            }
        }
    }

    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => format!("{} {}", head, tail.stringify()),
            List::Nil => format!("Nil")
        }
    }

}


fn main() {
    let list : List = List::new().prepend(1).prepend(2).prepend(3);

    println!("List: {:?}", list);
    println!("Length: {}", list.len());

    println!("List: {}", list.stringify());

    let list2 = list.insert(10, 3);

    println!("List2: {}", list2.stringify());
}
