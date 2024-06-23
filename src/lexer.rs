/*struct Position {
    x: u32,
    y: u32,
}
*/

#[derive(Debug, PartialEq)]
enum TokenType {
    Identifier,
    Keyword,
    Literal,
    Operator,
    Delimiter,
    Error,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    data: String,
}

pub struct Tokenizer {
    input: Vec<String>,
    //position: Position,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(input: &String) -> Self {
        Self {
            input: input.split(" ").map(String::from).collect(),
            //position: Position { x: 0, y: 0 },
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) {
        let mut in_comment = false;
        let mut in_string = false;
        let mut string_buffer = String::new();

        for token_str in &self.input {
            if !in_comment && token_str.starts_with("//") {
                in_comment = !in_comment;
                continue;
            }

            let token_str = if in_comment && token_str.contains("\n") {
                in_comment = !in_comment;
                &token_str[token_str.find('\n').unwrap() + 1..].to_string()
            } else {
                token_str
            };

            if in_comment {
                continue;
            }

            if !in_string && token_str.starts_with('"') {
                in_string = true;
                string_buffer.push_str(token_str.trim_start_matches('"'));
                continue;
            }

            if in_string {
                string_buffer.push_str(" ");
                string_buffer.push_str(token_str.trim_end_matches('"'));
                if token_str.ends_with('"') {
                    in_string = false;
                    self.tokens.push(Token {
                        token_type: TokenType::Literal,
                        data: string_buffer.clone(),
                    });
                    string_buffer.clear();
                }
                continue;
            }

            let t_type = if Self::is_keyword(token_str) {
                TokenType::Keyword
            } else if Self::is_literal(token_str) {
                TokenType::Literal
            } else if Self::is_identifier(token_str) {
                TokenType::Identifier
            } else if self.is_operator(token_str) {
                TokenType::Operator
            } else if Self::is_delimiter(token_str) {
                TokenType::Delimiter
            } else {
                TokenType::Error
            };

            self.tokens.push(Token {
                token_type: t_type,
                data: token_str.to_string(),
            });
        }
    }

    //check functions
    fn is_keyword(input: &str) -> bool {
        let keyword_list: [&str; 6] = ["main", "if", "el", "int", "str", "bol"];
        keyword_list.contains(&input)
    }

    fn is_literal(input: &str) -> bool {
        let is_integer = input.chars().all(|c| c.is_ascii_digit() || c == '-');
        let is_bool = input == "true" || input == "false";

        is_integer || is_bool
    }

    fn is_identifier(input: &str) -> bool {
        if input.is_empty() {
            return false;
        }

        let first_char = input.chars().next().unwrap();
        if !first_char.is_ascii_lowercase() {
            return false;
        }

        input
            .chars()
            .skip(1)
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }

    fn is_operator(&self, input: &str) -> bool {
        let operator_list: [&str; 11] = ["+", "-", "*", "/", "%", ":=", "=", "!", "&", "<", ">"];

        if operator_list.contains(&input) {
            return true;
        }

        if input == "|" {
            for token in self.tokens.iter().rev() {
                if token.token_type == TokenType::Delimiter {
                    if token.data == "(" {
                        return true;
                    } else if token.data == ")" {
                        return false;
                    }
                }
            }
        }
        false
    }

    fn is_delimiter(input: &str) -> bool {
        let delimiter_list: [&str; 4] = ["(", ")", ":", "|"];
        delimiter_list.contains(&input)
    }
}

#[cfg(test)]
mod tests {
    use crate::Tokenizer;

    #[test]
    fn test_lexer() {
        let input = "// Hello World\nmain := outln \"Hello World\"".to_string();
        let mut tokenizer = Tokenizer::new(&input);
        tokenizer.tokenize();

        println!("{}", input);
        println!("{:?}", tokenizer.tokens);
        assert_eq!(tokenizer.tokens.len(), 1);
    }
}
