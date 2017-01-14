#[derive(Debug)]
pub struct Document {
    pub elements: Vec<Element>,
}

#[derive(Debug)]
pub enum Element {
    TypAlias { name: String, typ: Typ },
    Struct { name: String, fields: Vec<Field> },
    Enum { name: String, items: Vec<String> },
}

#[derive(Debug)]
pub enum Typ {
    Ref(String),
    List(Box<Typ>),
    Map(String, Box<Typ>),
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub typ: Typ,
    pub optional: bool,
}
