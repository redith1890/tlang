#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

// TODO: use chars array instead of String to optimaze the Lexer.

use std::collections::HashMap;

struct Lexer
{
    expr: Vec<Token>,
    source: Vec<char>,
    position: usize,
    c: char,
    unique_chars: Vec<char>
}

struct Interpreter
{
    ast: Vec<Token>,
    variables: HashMap<String, Value>,
    op: Vec<usize>,
    position: usize,
    result: Vec<Token>
}

enum Value
{
    STRING(String),
    INT(i32),
    FLOAT(f64)
}

#[derive(Debug, PartialEq, Clone)]
enum Token
{
    ADD,
    SUB,
    MUL,
    DIV,
    FN,
    NEGATIVE,
    OPENPARENTHESIS,
    CLOSEPARENTHESIS,
    LET,
    NONE,
    ERROR,
    NEWLINE,
    STRING(String),
    VARIABLE(String),
    I32(i32),
    OUTOFBOUNDS,
}



// This only works with int
fn is_number(v: &Vec<char>) -> bool
{
    if v[0] == '-'
    {
        return v[1..].iter().all(|&c| c.is_digit(10));
    }
    v.iter().all(|&c| c.is_digit(10))
}

impl Lexer
{

    fn single_char_token(&mut self) -> Option<Token>
    {
        if self.read_char() == false {return Some(Token::NONE)};
        let token: Option<Token>;
        match self.c {
            '+' => token = Some(Token::ADD),
            '-' =>
            {
                if self.read_char() == false {return Some(Token::NONE)};
                // To differentiate between minus and substract
                if self.c == ' '
                {
                    self.position -= 1;
                    token = Some(Token::SUB);
                }
                else
                {
                    token = Some(Token::NEGATIVE);

                }
            }
            '*' => token = Some(Token::MUL),
            '/' => token = Some(Token::DIV),
            ' ' => token = Some(self.next_token()),
            '\0' => token = Some(Token::NONE),
            '\n' => token = Some(Token::NEWLINE),
            '(' => token = Some(Token::OPENPARENTHESIS),
            ')' => token = Some(Token::CLOSEPARENTHESIS),
            _ => token = None
        }
        token
    }

    fn next_token(&mut self) -> Token {

        let mut buffer: Vec<char> = Vec::new();

        // Single char Tokens
        if let Some(token) = self.single_char_token() {
            if token == Token::NEGATIVE
            {
                buffer.push('-');
            }
            else
            {
                return token;
            }
        }

        // Multiple chars tokens
        buffer.push(self.c);

        loop
        {
            if self.read_char() == false {break;}
            let mut uniq = false;

            if self.c == ' '
            {
                break;
            }

            for c in &self.unique_chars
            {
                if self.c == *c
                {
                    self.position -= 1;
                    uniq = true;
                    break;
                }
            }
            if uniq == true {break;}
            buffer.push(self.c);
        }

        if buffer.len() != 0
        {
            let s: String = buffer.iter().collect();
            // Numbers
            if is_number(&buffer)
            {
                return Token::I32(s.parse::<i32>().unwrap());
            }

            // Multi-char reserved words
            match s.as_str()
            {
                "let" => return Token::LET,
                _ => ()
            }

            // Variables
            return Token::VARIABLE(buffer.into_iter().collect());
        }



        Token::NONE
    }

    // TODO: change the bool for a custom token
    fn read_char(&mut self) -> bool
    {
        if self.position >= self.source.len()
        {
            return false;
        }
        self.c = self.source[self.position];
        self.position += 1;


        true
    }

    fn read(&mut self)
    {
        let mut expr: Vec<Token> = Vec::new();
        loop {
            let token = self.next_token();

            match token {
                Token::NONE => {
                    self.expr = expr;
                    break;
                }
                // For now I dont want to save the \n
                Token::NEWLINE => (),
                _ => {
                    expr.push(token);
                }
            }
        }
    }

}



impl Interpreter
{

    fn eval(&mut self)
    {
        while let Some(token) = self.next_token()
        {

            match token
            {
                Token::ADD => self.op.push(self.position),
                Token::SUB => self.op.push(self.position),
                Token::MUL => self.op.push(self.position),
                Token::DIV => self.op.push(self.position),
                Token::I32(_) =>
                {
                    println!("{:?}", self.ast);

                    // println!("op: {:?}, position: {}", self.op, self.position);
                    // println!("{:?}", self.ast[self.position]);
                    if self.can_do_binary_operation()

                    {
                        self.perform_binary_operation();
                        loop
                        {
                            if self.can_do_binary_operation()
                            {
                                println!("{:?}", self.ast);
                                self.perform_binary_operation();
                            }
                            else{break}
                        }
                    }
                },
                Token::OPENPARENTHESIS => {println!("OPENPARENTHESIS\n");},
                Token::CLOSEPARENTHESIS => {println!("\nCLOSEPARENTHESIS");},
                _ => {println!("ERROR");}
            }
            let position = self.position;

            self.position += 1;
        }
        println!("\nResult = {:?}", self.ast);
    }

    fn perform_binary_operation(&mut self) {
        // println!("Doing calculations on position: {}, op: {}", self.position, self.op[self.op.len()-1]);

        let v1 = if let Token::I32(num) = self.ast[self.position - 1] { num } else { unreachable!() };
        let v2 = if let Token::I32(num) = self.ast[self.position] { num } else { unreachable!() };

        if self.ast[self.position - 2] == Token::ADD {
            let result = v1 + v2;

            self.relocate_position_binary_op(result);
        }
        else if self.ast[self.position - 2] == Token::SUB {
            let result = v1 - v2;

            self.relocate_position_binary_op(result);
        }
        else if self.ast[self.position - 2] == Token::MUL {
            let result = v1 * v2;

            self.relocate_position_binary_op(result);
        }
        else if self.ast[self.position - 2] == Token::DIV {
            let result = v1 / v2;

            self.relocate_position_binary_op(result);
        }

    }

    fn relocate_position_binary_op(&mut self, result: i32)
    {
        self.ast.remove(self.position);
        self.ast.remove(self.position - 1);
        self.op.pop();
        self.ast[self.position - 2] = Token::I32(result);
        self.position -= 2;
    }

    fn can_do_binary_operation(&self /* Introduce here a type of binary operation*/ ) -> bool
    {
        let mut is_var2_i32 = false;
        match self.ast[self.position - 1]
        {
            Token::I32(_) => {is_var2_i32 = true;}
            _ => {return false;}
        }

        // Maybe error here !
        if self.position - 2 == self.op[self.op.len()-1] && is_var2_i32
        {
            // println!("El op es: {:?}", self.op);
            return true;
        }

        false
    }

    fn next_token(&mut self) -> Option<&Token>
    {
        if self.position >= self.ast.len() {
            return None;
        }
        Some(&self.ast[self.position])
    }
}





use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

fn main() -> io::Result<()> {
    let start = Instant::now();

    let mut file = File::open("input.txt")?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut lex = Lexer {
        expr: Vec::new(),
        source: code.chars().collect(),
        position: 0,
        c: ' ',
        unique_chars: vec!['(', ')', '+', '-', '*', '/'],
    };

    lex.read();

    let mut interpr = Interpreter
    {
        ast: lex.expr,
        variables: HashMap::new(),
        op: Vec::new(),
        position: 0,
        result: Vec::new()
    };

    interpr.eval();

    let duration = start.elapsed();
    println!("Tiempo de ejecución: {:?}", duration);

    Ok(())
}
