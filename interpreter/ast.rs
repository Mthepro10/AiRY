#[derive(Debug, Clone)]
pub enum Value{
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Statement{
    Set{
        name:String
        value:Value
    },
    Read{
        name:String
    },
    Show{
        name:String
    },
}