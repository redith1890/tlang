#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]


// The main change that has to be do it is to use chars array instead of String to optimaze the Lexer.
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
    last_op: usize,
    position: usize,
    result: Vec<Token>
}

// struct Parser<'a>
// {
//     expr: Vec<Token>,
//     position: usize,
//     ast: Vec<ASTNode<'a>>
// }

// enum ASTNode<'a>
// {
//     STRING(String),
//     VARIABLE(String),
//     NUMBER(u32),
//     OP(&'a ASTNode<'a>),
//     ERROR,
//     BINARYOP
//     {
//         operator: Token,
//         operand1: &'a ASTNode<'a>,
//         operand2: &'a ASTNode<'a>,
//     },
//     UNARYOP
//     {
//         operator: Token,
//         operand1: &'a ASTNode<'a>,
//     },
//     ASIGNMENT
//     {
//         identifier: String,
//         // TODO!
//         value: u32,
//     }

// }

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
    OPENPARENTHESIS,
    CLOSEPARENTHESIS,
    LET,
    NONE,
    ERROR,
    NEWLINE,
    STRING(String),
    VARIABLE(String),
    U32(u32),
    OUTOFBOUNDS,
}



// This only works with int
fn is_number(v: &Vec<char>) -> bool
{
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

        // Single char Tokens
        if let Some(token) = self.single_char_token() {
            return token;
        }


        // Multiple chars tokens
        let mut buffer: Vec<char> = Vec::new();
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

                return Token::U32(s.parse::<u32>().unwrap());
            }

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

    // Change the bool for a custom token
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

    fn sum(&mut self) -> Option<u32>
    {
        for exp in &self.expr
        {

        }

        Some(32)
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

// fn extract_u32(token: &Token) -> Option<u32> {
//     if let Token::U32(value) = token {
//         Some(*value)
//     } else {
//         None
//     }
// }
// impl Parser<'_>
// {
//     fn parse(&mut self)
//     {
//         let mut temp: Vec<ASTNode> = Vec::new();

//         loop
//         {
//             if let Some(token) = self.next_token()
//             {
//                 match token
//                 {
//                     Token::OPENPARENTHESIS => self.parse(),
//                     Token::ADD =>
//                     {
//                         let operand1 = self.next_token().unwrap_or(Token::ERROR);
//                         let operand2 = self.next_token().unwrap_or(Token::ERROR);
//                         match (operand1, operand2)
//                         {
//                             (Token::U32(value1), Token::U32(value2)) =>
//                             {
//                                 let ast_temp = ASTNode::BINARYOP
//                                 {
//                                     operator: Token::ADD,
//                                     operand1: &ASTNode::NUMBER(value1),
//                                     operand2: &ASTNode::NUMBER(value2)
//                                 };
//                                 temp.push(ast_temp);
//                             }
//                             _ => ()
//                         }

//                             // let ast_temp = ASTNode
//                             // {
//                             //     operator: Token::ADD,
//                             //     operand1: ASTNode::NUMBER(self.next_token()),
//                             //     operand2: ASTNode::NUMBER(self.next_token())
//                             // }


//                     }
//                     _ => ()
//                 }
//             }
//             else
//             {
//                 break;
//             }
//         }


//     }

//     fn next_token(&mut self) -> Option<Token>
//     {
//         if self.position >= self.expr.len() {
//             return None;
//         }
//         let old_position = self.position;
//         self.position += 1;
//         Some(self.expr[old_position].clone())
//     }

// }



impl Interpreter
{

    fn eval(&mut self)
    {
        println!("{:?}", self.ast);
        while let Some(token) = self.next_token()
        {
            match token
            {
                Token::ADD => self.last_op = self.position - 1,
                Token::U32(_) =>
                {
                    loop
                    {
                        if self.can_do_binary_operation()
                        {
                            self.perform_binary_operation();
                        }
                        else
                        {
                            break;
                        }
                    }

                },
                _ => ()
            }
        }
    }
    fn perform_binary_operation(&mut self) {
        println!(
            "Doing calculations on position: {}, last_op: {}",
            self.position, self.last_op
        );

        let v1 = if let Token::U32(num) = self.ast[self.position] { num } else { unreachable!() };
        let v2 = if let Token::U32(num) = self.ast[self.position - 1] { num } else { unreachable!() };

        if self.ast[self.position - 2] == Token::ADD {
            let result = v1 + v2;

            self.ast.remove(self.position - 1);
            self.ast.remove(self.position - 2);
            self.ast[self.position - 2] = Token::U32(result);
            self.position -= 2;

            println!("{:?}", self.ast);
        }
    }

    fn can_do_binary_operation(&self /* Introduce here a type of binary operation*/ ) -> bool
    {
        let mut is_var2_u32 = false;
        match self.ast[self.position - 1]
        {
            Token::U32(_) => {is_var2_u32 = true;}
            _ => {return false;}
        }

        if self.position - 2 == self.last_op && is_var2_u32
        {
            return true;
        }

        false
    }

    fn next_token(&mut self) -> Option<&Token>
    {
        if self.position >= self.ast.len() {
            return None;
        }
        let old_position = self.position;
        self.position += 1;
        Some(&self.ast[old_position])
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
        unique_chars: vec!['+', '(', ')'],
    };

    lex.read();

    // for i in &lex.expr
    // {
    //     println!("{:?}", i);
    // }

    // let mut parser = Parser
    // {
    //     expr: lex.expr,
    //     position: 0,
    //     ast: Vec::new()
    // };

    // parser.parse();

    let mut interpr = Interpreter
    {
        ast: lex.expr,
        variables: HashMap::new(),
        last_op: 0,
        position: 0,
        result: Vec::new()
    };

    interpr.eval();

    let duration = start.elapsed();
    println!("Tiempo de ejecución: {:?}", duration);

    Ok(())
}
