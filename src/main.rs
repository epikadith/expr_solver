use std::io;

#[derive(Debug)]
enum Token {
    Num(f64),
    Op(char),
    LeftP,
    RightP
}

fn main() {
    let mut expr = String::new();
    println!("enter the expression");
    io::stdin().read_line(&mut expr).expect("mf");
    let v = tokenize(&expr.trim(), expr.len());
    let size = v.len();
    let v = to_post(v, size);
    if let Some(Token::Num(n)) = solve(v, size) {
        println!("solution to the expression {} is {}", expr.trim(), n);
    }
    else {
        println!("kys");
    }
    
}

fn tokenize(s: &str, size: usize) -> Vec<Token> {
    let mut tks: Vec<Token> = Vec::with_capacity(size);
    let mut flag = false;
    let mut flag2 = false;
    let mut start = 0;
    let mut end = 0;
    let mut mstart = 0;
    let mut mend = 0;

    for (i,c) in s.chars().enumerate() {
        if flag {
            if (tks.is_empty() || matches!(tks.last().unwrap(),Token::LeftP) || matches!(tks.last().unwrap(),Token::Op(_))) && (c.is_digit(10) || c == '.') {
                mend+=1;
                continue;
            }
            else {
                flag = false;
                if mstart==mend {
                    tks.push(Token::Op('-'));
                }
                else {
                    let n: f64 = match (&s[mstart..=mend]).parse() {
                        Ok(num) => num,
                        Err(_) => panic!("enter a valid number bruh"),
                    };
                    tks.push(Token::Num(n));
                }
            }
        }
        if c == ' ' {
            continue;
        }
        if c == '-' {
            flag = true;
            mstart = i;
            mend = i;
        }
        if c.is_digit(10) || c == '.' {
            if !flag2 {
                flag2 = true;
                start = i;
                end = i;
            }
            else {
                end +=1;
            }
            continue;
        }
        if flag2 {
            let n: f64 = match (&s[start..=end]).parse::<f64>() {
                Ok(num) => num,
                Err(_) => panic!("enter a valid number bruh"),
            };
            tks.push(Token::Num(n));
            flag2 = false;
        }
        if c == '-' {
            continue;
        }
        match c {
            '+' => tks.push(Token::Op('+')),
            '*' => tks.push(Token::Op('*')),
            '^' => tks.push(Token::Op('^')),
            '/' => tks.push(Token::Op('/')),
            '%' => tks.push(Token::Op('%')),
            '(' => tks.push(Token::LeftP),
            ')' => tks.push(Token::RightP),
            _ => panic!("bruh invalid")
        }
    }
    if flag {
        let n: f64 = match (&s[mstart..=mend]).parse() {
            Ok(num) => num,
            Err(_) => panic!("enter a valid number bruh"),
        };
        tks.push(Token::Num(n));
    }
    if flag2 {
        let n: f64 = match (&s[start..=end]).parse::<f64>() {
            Ok(num) => num,
            Err(_) => panic!("enter a valid number bruh"),
        };
        tks.push(Token::Num(n));
    }
    return tks;
}

fn solve(v: Vec<Token>, size: usize) -> Option<Token> {
    let mut stack: Vec<Token> = Vec::with_capacity(size);

    for x in v {
        match x {
            Token::Op(o) => {
                let mut a: f64 = 0.0;
                let mut b: f64 = 0.0;
                if let Token::Num(n) = stack.pop().unwrap() {
                    a = n;
                }
                if let Token::Num(n) = stack.pop().unwrap() {
                    b = n;
                }
                if o == '/' && a == 0.0 {
                    panic!("division by zero bruh");
                }
                match o {
                    '+' => stack.push(Token::Num(b+a)),
                    '-' => stack.push(Token::Num(b-a)),
                    '*' => stack.push(Token::Num(b*a)),
                    '/' => stack.push(Token::Num(b/a)),
                    '^' => stack.push(Token::Num(b.powf(a))),
                    _ => panic!("literally impossible to get here"),
                }

            }
            _ => {
                stack.push(x);
            },                
            }
        }
    return stack.pop();
}



fn to_post(v: Vec<Token>, size: usize) -> Vec<Token> {
    let mut e: Vec<Token> = Vec::with_capacity(size);
    let mut stack: Vec<Token> = Vec::new(); 
    for tk in v {
        match tk {
            Token::Num(_) => e.push(tk),
            Token::LeftP => stack.push(tk),
            Token::Op(_) => {
                if stack.is_empty() {
                    stack.push(tk);
                    continue;
                }
                while let Some(tok) = stack.pop() {
                    if prec(&tok) >= prec(&tk) {
                        e.push(tok);
                    }
                    else {
                        stack.push(tok);
                        break;
                    }
                }
                stack.push(tk);

            }
            Token::RightP => {
                while let Some(tok) = stack.pop() {
                    if matches!(tok, Token::LeftP) {
                        break;
                    }
                    e.push(tok);
                }
            }
        }
    }
    while !stack.is_empty() {
        e.push(stack.pop().unwrap());
    }
    return e;
}

fn prec(t: &Token) -> u8 {
    match t {
        Token::LeftP | Token::RightP => 0,
        Token::Num(_) => 1,
        Token::Op(o) => match o {
            '+' | '-' => 2,
            '*' | '/' | '%' => 3,
            '^' => 4,
            _ => panic!("how even"),
        }
    }
}
