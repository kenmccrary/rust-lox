#[derive(Debug)]
pub enum LoxObject {
    Boolean(bool),
    Number(f32),
    String(String),
    Nil,
}

impl LoxObject {

    pub fn to_number(&self) -> f32 {
        match *self {
            LoxObject::Number(val) => return val,
            _ => panic!("invalid cast"),
        }
    }

    pub fn to_boolean(&self) -> bool {
        match *self {
            LoxObject::Boolean(val) => return val,
            _ => panic!("invalid cast"),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            LoxObject::String(val) => return &val,
            _ => panic!("invalid cast"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match *self {
            LoxObject::Nil => return false,
            LoxObject::Boolean(val) => return val,
            _ => return true,
        }
    }
}
