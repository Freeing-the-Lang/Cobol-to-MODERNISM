use crate::lexer::Token;
use crate::ast::Node;

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut ast = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(k) if k == "PROGRAM-ID" => {
                if let Some(Token::Identifier(name)) = iter.next() {
                    ast.push(Node::ProgramId(name));
                }
            },
            Token::Keyword(k) if k == "DISPLAY" => {
                if let Some(Token::Identifier(msg)) = iter.next() {
                    ast.push(Node::Display(msg));
                }
            },
            Token::Keyword(k) if k == "STOP" => {
                ast.push(Node::StopRun);
            },
            _ => {}
        }
    }

    ast
}
