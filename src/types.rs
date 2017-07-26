//mod types;

#[derive(Clone)]
pub enum LispType{
    Integer(i32),
    Float(f32),
    Symbol(String)
}

#[derive(Clone)]
pub enum LispItem{
    List(Vec<LispItem>),
    Atom(LispType)
}