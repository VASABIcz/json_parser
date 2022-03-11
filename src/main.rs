use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub enum Parser {
    Keyopen,
    Keyvalue,
    Keydelimiter,

    Valuetype,
    Stringopen,
    Stringvalue,
    Keyword,
    Number,
    ValueEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Layer {
    Object,
    Array(u32),
    Key(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum JSON {
    Object(HashMap<String, JSON>),
    Array(Vec<JSON>),
    Value(String)
}

impl JSON {
    pub fn insert(self: &mut Self, layers: &Vec<Layer>, value: JSON) {
        let mut last: &mut JSON= self;

        for (i, layer) in layers.iter().enumerate() {
            if i != layers.len()-1 {
                match layer {
                    Layer::Key(key) => {
                        match last {
                            JSON::Object(object) => {
                                last = object.get_mut(key).unwrap()
                            },
                            _ => panic!("BRUH")
                        }
                    },
                    Layer:: Array(index) => {
                        match last {
                            JSON::Array(array) => {
                                last = array.get_mut(*index as usize).unwrap().borrow_mut()
                            },
                            _ => panic!("BRUH")
                        }
                    },
                    _ => {}
                }
            }
            else {
                match layer {
                    Layer::Key(key) => {
                        match last {
                            JSON::Object(object) => {
                                object.insert(key.clone(), value.clone());
                            },
                            _ => {
                                println!("{:?}", self);
                                panic!("BRUH")
                            }
                        }
                    },
                    Layer::Array(_) => {
                        match last {
                            JSON::Array(array) => {
                                array.push(value.clone())
                            },
                            _ => panic!("BRUH")
                        }
                    },
                    _ => panic!("BRUH")
                }
            }
        }
    }
    pub fn obj() -> Self {
        Self::Object(HashMap::new())
    }
    pub fn ara() -> Self {
        Self::Array(Vec::new())
    }
}

const NUMBER: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    println!("{:?}", parse(r#"{
    "glossary": {
        "title": "example glossary",
		"GlossDiv": {
            "title": "S",
			"GlossList": {
                "GlossEntry": {
                    "ID": "SGML",
					"SortAs": "SGML",
					"GlossTerm": "Standard Generalized Markup Language",
					"Acronym": "SGML",
					"Abbrev": "ISO 8879:1986",
					"GlossDef": {
                        "para": "A meta-markup language, used to create markup languages such as DocBook.",
						"GlossSeeAlso": ["GML", "XML"]
                    },
					"GlossSee": "markup"
                }
            }
        }
    }
}
"#.to_string()).unwrap());
}

fn parse(s: String) -> Result<JSON, String> {
    let mut parser = Parser::Valuetype;
    let mut indentation = String::new();
    let mut buffer = String::new();
    let mut layers: Vec<Layer> = Vec::new();

    let mut js: Option<JSON> = None;

    for (i, c) in s.chars().enumerate() {
        // println!("{:?} {} {}", parser, c, i);
        // println!("first: {:?}", layers);
        match &parser {
            Parser::Keyopen => {
                if c.is_whitespace() { }
                else if c == '"' {
                    parser = Parser::Keyvalue
                }
                else {
                    return Err(format!("unexpected token {} {} | expected \" Keyopen ", c, i))
                }
            },
            Parser::Keyvalue => {
                if c == '"' {
                    layers.push(Layer::Key(buffer.clone()));
                    buffer.clear();
                    parser = Parser::Keydelimiter
                }
                else {
                    buffer.write_char(c).unwrap();
                }
            },
            Parser::Keydelimiter => {
                if c.is_whitespace() { }
                else if c == ':' {
                    parser = Parser::Valuetype
                }
                else {
                    return Err(format!("unexpected token {} {}| expected \":\" key delimiter", c, i))
                }
            }
            Parser::Valuetype => {
                if c.is_whitespace() { continue }
                parser = match c {
                    '"' => Parser::Stringvalue,
                    '{' => {

                        print!("{indentation}");
                        println!("{:?}: {}", layers.last().unwrap_or(&Layer::Key("root".to_string())), buffer);

                        match &mut js {
                            Some(v) => {
                                v.insert(&layers.clone(), JSON::obj())
                            }
                            None => {
                                js = Some(JSON::obj())
                            }
                        }

                        layers.push(Layer::Object);
                        indentation += "  ";
                        Parser::Keyopen
                    },
                    '[' => {
                        print!("{indentation}");
                        println!("{:?}: {}", layers.last().unwrap_or(&Layer::Key("root".to_string())), buffer);
                        indentation += "  ";

                        match &mut js {
                            Some(v) => {
                                v.insert(&layers.clone(), JSON::ara())
                            }
                            None => {
                                js = Some(JSON::obj())
                            }
                        }

                        layers.push(Layer::Array(0));
                        Parser::Valuetype
                    },
                    _ => {
                        if NUMBER.contains(&c) {
                            buffer.write_char(c).unwrap();
                            Parser::Number
                        }
                        else {
                            buffer.write_char(c).unwrap();
                            Parser::Keyword
                        }
                    }
                }
            },
            Parser::Stringopen => {
                if c.is_whitespace() { }
                else if c == '"' {
                parser = Parser::Stringvalue
                }
                else {
                    return Err(format!("unexpected token {} {}| expected \" string opening", c, i))
                }
            },
            Parser::Stringvalue => {
                if c == '"' {
                    parser = Parser::ValueEnd;
                    print!("{indentation}");
                    println!("{:?}: {}", layers.last().unwrap_or(&Layer::Key("root".to_string())), buffer);

                    match &mut js {
                        Some(v) => {
                            v.insert(&layers.clone(), JSON::Value(buffer.clone()))
                        }
                        None => {
                            js = Some(JSON::Value(buffer.clone()))
                        }
                    }

                    // if layers.len() > 1 {
                    //     layers.remove(layers.len()-1);
                    // }w
                    match layers.last().unwrap() {
                        Layer::Key(_) => {
                            layers.remove(layers.len()-1);
                        }
                        _ => {}
                    }
                    buffer.clear();
                }
                else {
                    buffer.write_char(c).unwrap();
                }
            },
            Parser::ValueEnd => {
                if c.is_whitespace() { }
                else if c == ',' {
                    let len = layers.len()-1;
                    parser = match layers.get_mut(len).unwrap() {
                        Layer::Object => {
                            Parser::Keyopen
                        }
                        Layer::Array(v) => {
                            *v += 1;
                            Parser::Valuetype
                        },
                        _ =>  panic!("{:?}", layers)
                    };
                }
                else if c == '}' {
                    if layers.len() > 2 {
                        match layers[layers.len()-2] {
                            Layer::Key(_) => {
                                layers.remove(layers.len()-1);
                                layers.remove(layers.len()-1);
                            }
                            _ => {
                                layers.remove(layers.len()-1);
                            }
                        }
                    }
                    if indentation.len() >= 2 {
                        indentation.truncate(indentation.len()-2);
                    }
                }
                else if c == ']' {
                    if layers.len() > 2 {
                        match layers[layers.len()-2] {
                            Layer::Key(_) => {
                                layers.remove(layers.len()-1);
                                layers.remove(layers.len()-1);
                            }
                            _ => {
                                layers.remove(layers.len()-1);
                            }
                        }
                    }
                    if indentation.len() >= 2 {
                        indentation.truncate(indentation.len()-2);
                    }
                }
                else {
                    return Err(format!("unexpected token {} {} | expected }} or ] or \",\"", c, i))
                }
            },
            Parser::Number => {
                if NUMBER.contains(&c) {
                    buffer.write_char(c).unwrap();
                }
                else {

                    print!("{indentation}");

                    match &mut js {
                        Some(v) => {
                            v.insert(&layers.clone(), JSON::Value(buffer.clone()))
                        }
                        None => {
                            js = Some(JSON::Value(buffer.clone()))
                        }
                    }

                    match layers.last().unwrap() {
                        Layer::Key(_) => {
                            layers.remove(layers.len()-1);
                        }
                        _ => {}
                    }

                    println!("{:?}: {}", layers.last().unwrap_or(&Layer::Key("root".to_string())), buffer);

                    buffer.clear();

                    parser = Parser::ValueEnd;
                    if c == ',' {
                        match layers.last().unwrap() {
                            Layer::Key(_) => {
                                layers.remove(layers.len()-1);
                            }
                            _ => {}
                        }

                        let len = layers.len()-1;
                        parser = match layers.get_mut(len).unwrap() {
                            Layer::Object => {
                                Parser::Keyopen
                            }
                            Layer::Array(v) => {
                                *v += 1;
                                Parser::Valuetype
                            },
                            _ => panic!("{:?}", layers)
                        }
                    }
                    else if c == '}' {
                        if layers.len() > 2 {
                            match layers[layers.len()-2] {
                                Layer::Key(_) => {
                                    println!("Aids");
                                    layers.remove(layers.len()-1);
                                    layers.remove(layers.len()-1);
                                }
                                _ => {
                                    println!("baids");
                                    layers.remove(layers.len()-1);
                                }
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c == ']' {
                        if layers.len() > 2 {
                            match layers[layers.len()-2] {
                                Layer::Key(_) => {
                                    layers.remove(layers.len()-1);
                                    layers.remove(layers.len()-1);
                                }
                                _ => {
                                    layers.remove(layers.len()-1);
                                }
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c.is_whitespace() {
                        parser = Parser::ValueEnd
                    }
                    else {
                        return Err(format!("unexpected token {} {} | expected }} or ] or \",\" or Number", c, i))
                    }
                }
            },
            Parser::Keyword => {
                if c.is_alphabetic() {
                    buffer.write_char(c).unwrap();
                }
                else {
                    match &mut js {
                        Some(v) => {
                            v.insert(&layers.clone(), JSON::Value(buffer.clone()))
                        }
                        None => {
                            js = Some(JSON::Value(buffer.clone()))
                        }
                    }

                    match layers.last().unwrap() {
                        Layer::Key(_) => {
                            layers.remove(layers.len()-1);
                        }
                        _ => {}
                    }

                    print!("{indentation}");
                    println!("{:?}: {}", layers.last().unwrap_or(&Layer::Key("root".to_string())), buffer);
                    // layers.remove(layers.len()-1);
                    buffer.clear();

                    parser = Parser::ValueEnd;
                    if c == ',' {
                        let len = layers.len()-1;
                        parser = match layers.get_mut(len).unwrap() {
                            Layer::Object => {
                                Parser::Keyopen
                            }
                            Layer::Array(v) => {
                                *v += 1;
                                Parser::Valuetype
                            },
                            _ => panic!()
                        }
                    }
                    else if c == '}' {
                        if layers.len() > 2 {
                            match layers[layers.len()-2] {
                                Layer::Key(_) => {
                                    layers.remove(layers.len()-1);
                                    layers.remove(layers.len()-1);
                                }
                                _ => {
                                    layers.remove(layers.len()-1);
                                }
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c == ']' {
                        if layers.len() > 2 {
                            match layers[layers.len()-2] {
                                Layer::Key(_) => {
                                    layers.remove(layers.len()-1);
                                    layers.remove(layers.len()-1);
                                }
                                _ => {
                                    layers.remove(layers.len()-1);
                                }
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c.is_whitespace() {
                        parser = Parser::ValueEnd
                    }
                    else {
                        return Err(format!("unexpected token {} {} | expected }} or ] or \",\" or chars", c, i))
                    }
                }
            }
            // _ => { return Err("sus".to_string()) }
        }
        // println!("last: {:?}", layers);
    }
    Ok(js.unwrap())
}