#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// The main change that has to be do it is to use chars array instead of String to optimaze the Lexer.
type Expression = Vec<Token>;

struct Lexer
{
    expr: Vec<Expression>,
    source: Vec<char>,
    position: usize,
    c: char,
    unique_chars: Vec<char>
}

#[derive(Debug, PartialEq)]
enum Token
{
    ADD,
    SUB,
    MUL,
    DIV,
    FN,
    OPENPARENTHESIS,
    CLOSEPARENTHESIS,
    NONE,
    VARIABLE(String),
    U32(u32),
    EXPR(Expression),
    OUTOFBOUNDS,
}

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
            '(' => token = Some(Token::OPENPARENTHESIS),
            ')' => token = Some(Token::CLOSEPARENTHESIS),
            _ => token = None
        }
        token
    }

    fn next_token(&mut self) -> Token {
        if let Some(token) = self.single_char_token() {
            return token;
        }

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
            if is_number(&buffer)
            {
                let s: String = buffer.iter().collect();

                return Token::U32(s.parse::<u32>().unwrap());
            }
            return Token::VARIABLE(buffer.into_iter().collect());
        }


        Token::NONE
    }

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
        let mut current_exp: Option<Expression> = None;

        loop {
            let token = self.next_token();

            match token {
                Token::NONE => break,
                Token::OPENPARENTHESIS => {
                    current_exp = Some(Vec::new());
                },
                Token::CLOSEPARENTHESIS => {
                    if let Some(exp) = current_exp {
                        self.expr.push(exp);
                        current_exp = None;
                    }
                },
                _ => {
                    if let Some(ref mut exp) = current_exp {
                        exp.push(token);
                    }
                }
            }
        }
    }


}


fn main()
{
    let code = "(asdf xdddd + omeg)  ( 31435 + x) ";

    let mut lex = Lexer
    {
        expr: Vec::new(),
        source: code.chars().collect(),
        position: 0,
        c: ' ',
        unique_chars: vec!['+', '(', ')']
    };

    lex.read();

    println!("{:?}", lex.expr);

}
