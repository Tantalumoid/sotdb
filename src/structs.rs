#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    name: String,
    data: Vec<(String, DataType)>,
}
impl Object {
    pub fn new(name: String, data: Vec<(String, DataType)>) -> Object {
        Object { name, data }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_data(&self) -> &Vec<(String, DataType)> {
        &self.data
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}
impl DataType {
    pub fn get_type_anotation(&self) -> &str {
        match self {
            DataType::Str(_) => "<str>",
            DataType::Int(_) => "<int>",
            DataType::Float(_) => "<float>",
            DataType::Bool(_) => "<bool>",
        }
    }
    pub fn get_value(&self) -> String {
        match self {
            DataType::Str(item) => item.to_owned().to_string(),
            DataType::Int(item) => item.to_owned().to_string(),
            DataType::Float(item) => item.to_owned().to_string(),
            DataType::Bool(item) => item.to_owned().to_string(),
        }
    }
}
