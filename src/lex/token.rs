#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Whitespace(char),
    SemiColon,
    Unknown(char),
    Delimiter(char),
    EOF,
    SingleLineComment,
    Module,
    Api,
    DocumentName(&'a str),
    DocumentVersion(&'a str),
    Include,
    IncludeName(&'a str),
    IncludeVersion(&'a str),
    Primitive(&'a str),
    TypeName(&'a str),
    FieldName(&'a str),
    FieldAssignment,
    Rpc,
    BlockStart,
    BlockEnd,
    TypeArgStart,
    TypeArgEnd,
    DocStringStart,
    DocStringText(&'a str),
    DocStringEnd,
}

pub fn is_whitespace(c: char) -> bool {
    return c == ' ' || c == '\t' || c == '\r';
}

pub fn is_primitive<'a>(s: &'a str) -> bool {
    return s == "int" || s == "float" || s == "string" || s == "struct";
}
