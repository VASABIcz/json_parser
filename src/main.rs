use std::collections::HashMap;
use std::fmt::Write;

const NUMBER: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    parse(r#"{"web-app": {
  "servlet": [
    {
      "servlet-name": "cofaxCDS",
      "servlet-class": "org.cofax.cds.CDSServlet",
      "init-param": {
        "configGlossary:installationAt": "Philadelphia, PA",
        "configGlossary:adminEmail": "ksm@pobox.com",
        "configGlossary:poweredBy": "Cofax",
        "configGlossary:poweredByIcon": "/images/cofax.gif",
        "configGlossary:staticPath": "/content/static",
        "templateProcessorClass": "org.cofax.WysiwygTemplate",
        "templateLoaderClass": "org.cofax.FilesTemplateLoader",
        "templatePath": "templates",
        "templateOverridePath": "",
        "defaultListTemplate": "listTemplate.htm",
        "defaultFileTemplate": "articleTemplate.htm",
        "useJSP": false,
        "jspListTemplate": "listTemplate.jsp",
        "jspFileTemplate": "articleTemplate.jsp",
        "cachePackageTagsTrack": 200,
        "cachePackageTagsStore": 200,
        "cachePackageTagsRefresh": 60,
        "cacheTemplatesTrack": 100,
        "cacheTemplatesStore": 50,
        "cacheTemplatesRefresh": 15,
        "cachePagesTrack": 200,
        "cachePagesStore": 100,
        "cachePagesRefresh": 10,
        "cachePagesDirtyRead": 10,
        "searchEngineListTemplate": "forSearchEnginesList.htm",
        "searchEngineFileTemplate": "forSearchEngines.htm",
        "searchEngineRobotsDb": "WEB-INF/robots.db",
        "useDataStore": true,
        "dataStoreClass": "org.cofax.SqlDataStore",
        "redirectionClass": "org.cofax.SqlRedirection",
        "dataStoreName": "cofax",
        "dataStoreDriver": "com.microsoft.jdbc.sqlserver.SQLServerDriver",
        "dataStoreUrl": "jdbc:microsoft:sqlserver://LOCALHOST:1433;DatabaseName=goon",
        "dataStoreUser": "sa",
        "dataStorePassword": "dataStoreTestQuery",
        "dataStoreTestQuery": "SET NOCOUNT ON;select test='test';",
        "dataStoreLogFile": "/usr/local/tomcat/logs/datastore.log",
        "dataStoreInitConns": 10,
        "dataStoreMaxConns": 100,
        "dataStoreConnUsageLimit": 100,
        "dataStoreLogLevel": "debug",
        "maxUrlLength": 500}},
    {
      "servlet-name": "cofaxEmail",
      "servlet-class": "org.cofax.cds.EmailServlet",
      "init-param": {
      "mailHost": "mail1",
      "mailHostOverride": "mail2"}},
    {
      "servlet-name": "cofaxAdmin",
      "servlet-class": "org.cofax.cds.AdminServlet"},

    {
      "servlet-name": "fileServlet",
      "servlet-class": "org.cofax.cds.FileServlet"},
    {
      "servlet-name": "cofaxTools",
      "servlet-class": "org.cofax.cms.CofaxToolsServlet",
      "init-param": {
        "templatePath": "toolstemplates/",
        "log": 1,
        "logLocation": "/usr/local/tomcat/logs/CofaxTools.log",
        "logMaxSize": "",
        "dataLog": 1,
        "dataLogLocation": "/usr/local/tomcat/logs/dataLog.log",
        "dataLogMaxSize": "",
        "removePageCache": "/content/admin/remove?cache=pages&id=",
        "removeTemplateCache": "/content/admin/remove?cache=templates&id=",
        "fileTransferFolder": "/usr/local/tomcat/webapps/content/fileTransferFolder",
        "lookInContext": 1,
        "adminGroupID": 4,
        "betaServer": true}}],
  "servlet-mapping": {
    "cofaxCDS": "/",
    "cofaxEmail": "/cofaxutil/aemail/*",
    "cofaxAdmin": "/admin/*",
    "fileServlet": "/static/*",
    "cofaxTools": "/tools/*"},

  "taglib": {
    "taglib-uri": "cofax.tld",
    "taglib-location": "/WEB-INF/tlds/cofax.tld"}}}"#.to_string()).unwrap();
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

#[derive(Debug, Clone)]
enum Parser {
    keyopen,
    keyvalue,
    keydelimiter,

    valuetype,
        stringopen,
        stringvalue,
        keyword,
        number,
    valueEnd,
}

#[derive(Debug, Clone, PartialEq)]
enum Layer {
    Object(String),
    Array(u32),
    Key(String)
}

fn parse(s: String) -> Result<JSON, String> {
    let mut parser = Parser::valuetype;
    let mut indentation = String::new();
    let mut buffer = String::new();
    let mut layers: Vec<Layer> = Vec::new();
    layers.push(Layer::Key(String::new()));

    for (i, c) in s.chars().enumerate() {
        // println!("{:?} {} {}", parser, c, i);
        // println!("{:?}", layers);
        match &parser {
            Parser::keyopen => {
                if c.is_whitespace() { }
                else if c == '"' {
                    parser = Parser::keyvalue
                }
                else {
                    return Err(format!("unexpected token {} {}", c, i))
                }
            },
            Parser::keyvalue => {
                if c == '"' {
                    layers.push(Layer::Key(buffer.clone()));
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
                        print!("{indentation}");
                        println!("{:?}: {}", layers.last().unwrap(), buffer);

                        layers.push(Layer::Object(String::new()));
                        indentation += "  ";
                        Parser::keyopen
                    },
                    '[' => {
                        print!("{indentation}");
                        println!("{:?}: {}", layers.last().unwrap(), buffer);
                        indentation += "  ";

                        layers.push(Layer::Array(0));
                        Parser::valuetype
                    },
                    _ => {
                        if NUMBER.contains(&c) {
                            buffer.write_char(c);
                            Parser::number
                        }
                        else {
                            buffer.write_char(c);
                            Parser::keyword
                        }
                    }
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
                    print!("{indentation}");
                    println!("{:?}: {}", layers.last().unwrap(), buffer);
                    layers.remove(layers.len()-1);
                    buffer.clear();
                }
                else {
                    buffer.write_char(c);
                }
            },
            Parser::valueEnd => {
                if c.is_whitespace() { }
                else if c == ',' {
                    let len = layers.len()-1;
                    parser = match layers.get_mut(len).unwrap() {
                        Layer::Object(_) => {
                            Parser::keyopen
                        }
                        Layer::Array(v) => {
                            *v += 1;
                            Parser::valuetype
                        },
                        _ =>  panic!()
                    };
                }
                else if c == '}' {
                    match layers[layers.len()-2] {
                        Layer::Key(_) => {
                            layers.remove(layers.len()-1);
                            layers.remove(layers.len()-1);
                        }
                        _ => {
                            layers.remove(layers.len()-1);
                        }
                    }
                    if indentation.len() >= 2 {
                        indentation.truncate(indentation.len()-2);
                    }
                }
                else if c == ']' {
                    match layers[layers.len()-2] {
                        Layer::Key(_) => {
                            layers.remove(layers.len()-1);
                            layers.remove(layers.len()-1);
                        }
                        _ => {
                            layers.remove(layers.len()-1);
                        }
                    }
                    if indentation.len() >= 2 {
                        indentation.truncate(indentation.len()-2);
                    }
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
                    print!("{indentation}");
                    println!("{:?}: {}", layers.last().unwrap(), buffer);
                    layers.remove(layers.len()-1);

                    buffer.clear();

                    parser = Parser::valueEnd;
                    if c == ',' {
                        parser = match layers.last().unwrap() {
                            Layer::Object(_) => {
                                Parser::keyopen
                            }
                            Layer::Array(_) => {
                                Parser::valuetype
                            },
                            _ => panic!()
                        }
                    }
                    else if c == '}' {
                        match layers[layers.len()-2] {
                            Layer::Key(_) => {
                                layers.remove(layers.len()-1);
                                layers.remove(layers.len()-1);
                            }
                            _ => {
                                layers.remove(layers.len()-1);
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c == ']' {
                        match layers[layers.len()-2] {
                            Layer::Key(_) => {
                                layers.remove(layers.len()-1);
                                layers.remove(layers.len()-1);
                            }
                            _ => {
                                layers.remove(layers.len()-1);
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c.is_whitespace() {
                        parser = Parser::valueEnd
                    }
                    else {
                        return Err(format!("unexpected token {} {}", c, i))
                    }
                }
            },
            Parser::keyword => {
                if c.is_alphabetic() {
                    buffer.write_char(c);
                }
                else {
                    print!("{indentation}");
                    println!("{:?}: {}", layers.last().unwrap(), buffer);
                    layers.remove(layers.len()-1);
                    buffer.clear();

                    parser = Parser::valueEnd;
                    if c == ',' {
                        parser = match layers.last().unwrap() {
                            Layer::Object(_) => {
                                Parser::keyopen
                            }
                            Layer::Array(_) => {
                                Parser::valuetype
                            },
                            _ => panic!()
                        }
                    }
                    else if c == '}' {
                        match layers[layers.len()-2] {
                            Layer::Key(_) => {
                                layers.remove(layers.len()-1);
                                layers.remove(layers.len()-1);
                            }
                            _ => {
                                layers.remove(layers.len()-1);
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c == ']' {
                        match layers[layers.len()-2] {
                            Layer::Key(_) => {
                                layers.remove(layers.len()-1);
                                layers.remove(layers.len()-1);
                            }
                            _ => {
                                layers.remove(layers.len()-1);
                            }
                        }
                        if indentation.len() >= 2 {
                            indentation.truncate(indentation.len()-2);
                        }
                    }
                    else if c.is_whitespace() {
                        parser = Parser::valueEnd
                    }
                    else {
                        return Err(format!("unexpected token {} {}", c, i))
                    }
                }
            }
            _ => { return Err("sus".to_string()) }
        }
    }
    Ok(JSON::Null)
}