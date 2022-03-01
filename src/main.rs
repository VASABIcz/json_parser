use std::io;
use std::collections::HashMap;
use std::fmt::Write;
use crate::JSON::{Null, Number};

const NUMBER: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    parse(r#"{"ships": [{"name": "amogus", "type": "sus"}, {"name": "baka", "type": "UwU"}], "monke": "cringe"}"#.to_string()).unwrap();
}
#[derive(Debug)]
enum JSON {
    Object(HashMap<String, JSON>),
    Boolean(bool),
    Text(String),
    Number(u64),
    Array(Vec<JSON>),
    Null
}

/*
expecting { [
expecting } ]
expecting string
expecting value
 */
#[derive(Debug, Clone)]
enum Parser {
    openObject,

    arrayType,
    arrayEnd,
    arrayString,
    arrayStringOpen,
    arrayStringValue,
    arrayNumber,
    arrayKeyword,

    keyopen,
    keydelimiter,
    keyvalue,
    valuetype,

    stringopen,
    stringvalue,

    keyword,

    valueEnd,

    number
}

#[derive(Debug, Clone)]
enum Layer {
    Object(String),
    Array(usize)
}

// fn parseTokens(s: &str) -> Option<Vec<&str>> {
//     let tokens: Vec<&str> = vec![];
// }

fn add_number() {

}
/*
fn add_string(o: &mut HashMap<String, JSON>, layers: &mut Vec<Layer>, v: String) {
    let mut buf: Option<&mut JSON> = None;

    let mut clone: Vec<Layer> = layers.clone();
    &mut clone.remove(clone.len());

    for layer in layers {
        match layer {
            Layer::Object(v) => {
                match buf {
                    None => {
                        buf =  Some(&mut o.get(v).unwrap());
                    },
                    Some(j) => {
                        match j {
                            JSON::Object(xd) => {
                                buf = Some(&mut xd.get(v).unwrap());
                            },
                            _ => {println!("we are fucked {}", v)}
                        }
                    }
                }
            },
            Layer::Array(v) => {
                match buf {
                    None => {
                        println!("wea are fucked {}", v);
                    },
                    Some(j) => {
                        match j {
                            JSON::Array(xd) => {
                                buf = Some(&mut xd.get(v.clone()).unwrap());
                            },
                            _ => {println!("we are fucked {}", v)}
                        }
                    }
                }
            },
        }
    }
    match buf {
        None => {

        },
        Some(j) => {
            match layers.last().unwrap() {
                Layer::Object(vee) => {
                    match j {
                        JSON::Object(val) => {
                            val.insert(vee.clone(), JSON::Text(v));
                        }
                        _ => {
                            println!("we are fucked")
                        }
                    }
                },
                Layer::Array(va) => {
                    match j {
                        JSON::Array(val) => {
                            val.push(JSON::Text(v))
                        }
                        _ => {
                            println!("we are fucked")
                        }
                    }
                }
            }
        }
    }

}
 */

fn parse(s: String) -> Result<JSON, String> {
    let mut parser = Parser::openObject;

    let mut buffer = String::new();

    let mut object: HashMap<String, JSON> = HashMap::new();
    let mut array: HashMap<String, JSON> = HashMap::new();
    let mut key = String::new();

    let mut layers: Vec<Layer> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        //println!("parsing {parser:?} {c} {i}");
        match &parser {
            Parser::openObject => {
                println!("#OBJECT#");
                if c == '{' {
                    if !key.is_empty() {
                        layers.push(Layer::Object(key.clone()));
                    }

                    parser = Parser::keyopen
                }
                else if c.is_whitespace() { }
                else { return Err(format!("unexpected token {} {}", c, i)) }
            },
            Parser::keyopen => {
                if c.is_whitespace() { }
                else if c == '"' {
                    parser = Parser::keyvalue
                }
            },
            Parser::keyvalue => {
                if c == '"' {
                    println!("parsed key: {buffer}");
                    key = buffer.clone();
                    buffer.clear();
                    parser = Parser::keydelimiter
                }
                else {
                    buffer.write_char(c);
                }
            },
            Parser::keydelimiter => {
                if c.is_whitespace() { }
                else if c == ':' {
                    parser = Parser::valuetype
                }
                else {
                    return Err(format!("unexpected token {} {}", c, i))
                }
            }
            Parser::valuetype => {
                if c.is_whitespace() { continue }
                parser = match c {
                    '"' => Parser::stringvalue,
                    '{' => {
                        println!("#OBJECT#");
                        Parser::keyopen
                    },
                    '[' => {
                        println!("=ARRAY=");
                        Parser::arrayType
                    },
                    _ => {
                        if NUMBER.contains(&c) {
                            buffer.write_char(c);
                            Parser::number
                        }
                        else {
                            println!("unknown char: {} {}", c, i);
                            buffer.write_char(c);
                            Parser::keyword
                        }
                    }
                }
            },
            Parser::arrayType => {
                if c.is_whitespace() { continue }
                parser = match c {
                    '"' => Parser::arrayString,
                    '{' => {
                        println!("#OBJECT#");
                        Parser::keyopen
                    },
                    '[' => {
                        println!("=ARRAY=");
                        Parser::arrayType
                    },
                    _ => {
                        if NUMBER.contains(&c) {
                            buffer.write_char(c);
                            Parser::arrayNumber
                        }
                        else {
                            println!("unknown char: {} {}", c, i);
                            buffer.write_char(c);
                            Parser::arrayKeyword
                        }
                    }
                }
            },
            Parser::arrayStringOpen => {
                if c.is_whitespace() { }
                else if c == '"' {
                    parser = Parser::arrayStringValue
                }
                else {
                    return Err(format!("unexpected token {} {}", c, i))
                }
            },
            Parser::arrayStringValue => {
                if c == '"' {
                    parser = Parser::arrayEnd;
                    println!("{buffer} parsed string");

                    buffer.clear();
                }
                else {
                    buffer.write_char(c);
                }
            },
            Parser::arrayNumber => {
                if NUMBER.contains(&c) {
                    buffer.write_char(c);
                }
                else {
                    parser = Parser::arrayEnd;
                    if c == ',' {
                        parser = Parser::arrayType
                    }
                    println!("parsed number: {buffer}");

                    buffer.clear();
                }
            },
            Parser::arrayEnd => {
                if c.is_whitespace() { }
                else if c == ',' {
                    parser = Parser::arrayType
                }
                else if { c == ']' } {
                    parser = Parser::valueEnd
                }
                else {
                    return Err(format!("unexpected token {} {}", c, i))
                }
            },
            Parser::arrayKeyword => {
                if c.is_alphabetic() {
                    buffer.write_char(c);
                }
                else {
                    parser = Parser::arrayEnd;
                    if c == ',' {
                        parser = Parser::arrayType
                    }
                    println!("parsed keyword: {buffer}");
                    buffer.clear();
                }
            },
            Parser::stringopen => {
                if c.is_whitespace() { }
                else if c == '"' {
                parser = Parser::stringvalue
                }
                else {
                    return Err(format!("unexpected token {} {}", c, i))
                }
            },
            Parser::stringvalue => {
                if c == '"' {
                    parser = Parser::valueEnd;
                    println!("parsed string: {buffer}");

                    object.insert(key.clone(), JSON::Text(buffer.clone()));
                    key.clear();

                    buffer.clear();
                }
                else {
                    buffer.write_char(c);
                }
            },
            Parser::valueEnd => {
                if c.is_whitespace() { }
                else if c == ',' {
                    parser = Parser::keyopen
                }
                else if { c == '}' } {
                    println!("#OBJECT END#");
                }
                else if { c == ']' } {
                    println!("=ARRAY END=")
                }
                else {
                    return Err(format!("unexpected token {} {}", c, i))
                }
            },
            Parser::number => {
                if NUMBER.contains(&c) {
                    buffer.write_char(c);
                }
                else {
                    parser = Parser::valueEnd;
                    if c == ',' {
                        parser = Parser::keyopen
                    }
                    println!("parsed number: {buffer}");

                    object.insert(key.clone(), JSON::Number(buffer.parse::<u64>().unwrap()));
                    key.clear();

                    buffer.clear();
                }
            },
            Parser::keyword => {
                if c.is_alphabetic() {
                    buffer.write_char(c);
                }
                else {
                    parser = Parser::valueEnd;
                    if c == ',' {
                        parser = Parser::keyopen
                    }
                    println!("parsed keyword: {buffer}");
                    buffer.clear();
                }
            }
            _ => { return Err("sus".to_string()) }
        }
    }


    return Ok(JSON::Null)

}