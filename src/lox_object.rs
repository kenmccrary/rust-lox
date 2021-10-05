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

impl PartialEq for LoxObject {

    fn eq(&self, other: &Self) -> bool {

        match self {
            LoxObject::Boolean(val) => {
                return *val == other.to_boolean();
            },

            LoxObject::Number(val) => {
                return *val == other.to_number();
            },

            LoxObject::String(val) => {
                return *val == other.to_string();
            },

            LoxObject::Nil => {
                match other {
                    LoxObject::Nil => {
                        return true;

                    }
                    _ => {
                        return false;
                    }
                }
            },
        }
    }
}
