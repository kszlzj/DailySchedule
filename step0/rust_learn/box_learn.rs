// enum List{
//     Cons(i32,List),
//     Nil,
// }错误

enum List{
    Cons(i32,Box<List>),
    Nil
}

fn main(){
    use List::Cons;
    use List::Nil;
    let list = Cons(1,
                    Box::new(Cons(2,
                                    Box::new(Nil))));
                            
}