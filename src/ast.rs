#[derive(Debug)]
pub enum Element {
    Typ { name: String, fields: Vec<Field> },
}

#[derive(Debug)]
pub struct Document {
    pub elements: Vec<Box<Element>>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub typ: String,
}
