#[derive(Debug)]
pub enum Token {
    OpenTag {
        name: String,
        attributes: Vec<(String, String)>,
    },
    CloseTag {
        name: String,
    },
    SelfClosingTag {
        name: String,
        attributes: Vec<(String, String)>,
    },
    Text(String),
}

/**

   1. 문자열 입력
   2.

*/

pub fn tokenize(token: &str) -> Vec<Token> {
    println!("Token: {:?}", token);

    let mut isStart = false;

    let mut tokens: Vec<Token> = Vec::new();

    let mut buffer = String::new();
    let mut in_tag = false;

    for ch in token.chars() {
        if ch == '<' {
            if !buffer.is_empty() {
                tokens.push(Token::Text(buffer.clone()));
                buffer.clear();
            }
            in_tag = true;
        } else if ch == '>' {
            if (buffer.starts_with("/")) {
                tokens.push(Token::CloseTag {
                    name: buffer.trim_start_matches("/").to_string(),
                });
            } else if (in_tag) {
                let (name, attribute) = match buffer.split_once(" ") {
                    Some((name, attribute)) => (name.trim(), attribute.trim()),
                    None => (buffer.trim(), ""),
                };

                let (att_name, att_attribute) = match attribute.split_once("=") {
                    Some((att_name, att_attribute)) => {
                        (att_name.to_string(), att_attribute.trim_matches('"').to_string())
                    }
                    None => (attribute.trim().to_string(), String::new()),
                };

                tokens.push(Token::OpenTag {
                    name: name.to_string(),
                    attributes: vec![(att_name, att_attribute.to_string())],
                });
            }

            in_tag = false;
            buffer.clear();
        } else {
            buffer.push(ch);
        }
    }
    println!("{:?} ", tokens);

    tokens
}
