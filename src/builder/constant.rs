use crate::{Value, TypeFlag};

// constants are stored within their own field in each function
// constants allow for abstracted static collections accessible only at compile time
#[derive(Debug, Clone)]
pub struct Constant {
    collection: bool,
    data: Vec<Value>,
    ident: String,
}

impl Constant {
    pub fn new(data: Vec<Value>, id: &str) -> Self {
        let collection = data.len() > 1;

        // type check the data, ensure each element is the same type
        if collection {
            let head_ty: TypeFlag = data[0].ty;
            for elem in &data {
                if head_ty != elem.ty {
                    panic!("Error: All elements in a collection must be of the same type, expected {:?} instead got {:?} in collection {:?}", head_ty, elem.ty, data.clone())
                }
            }
        }

        Self {
            collection,
            data,
            ident: id.to_string(),
        }
    }

    // collection? 
    pub fn collectp(&self) -> bool {
        self.collection
    }

    pub fn get_ident(&self) -> &str {
        &self.ident
    }

    pub fn get_value(&self) -> Value {
        self.data[0]
    }

    pub fn access_value(&self, idx: usize) -> Value {
        if self.collection {
            match self.data.get(idx) {
                Some(val) => *val,
                None => panic!("Error: Attempt to access past bounds of a constant collection {}", self.ident),
            }
        }else{
            panic!("Error: Attempted to perform indexed access on a constant that is not a collection {}", self.ident);
        }
    }
}