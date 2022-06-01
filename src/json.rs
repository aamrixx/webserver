/* xhci's JSON Parser for the webserver
 * https://github.com/xhci1/webserver */

/* Tokens */
#[derive(Debug, PartialEq)]
pub enum TokKind {
    Identifier,
    LBrace,
    RBrace,
    Colon,
    Comma,
}

#[derive(Debug, PartialEq)]
pub struct Tok {
    pub kind: TokKind,
    pub lit: String,
}

impl Tok {
    pub fn new(k: TokKind, l: &str) -> Self {
        Self {
            kind: k,
            lit: l.to_string(),
        }
    }
}

/* Lexer */
#[derive(Debug)]
pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    next: usize,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Self {
            src: s.chars().collect(),
            pos: 0,
            next: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Tok> {
        let mut tokens = Vec::<Tok>::new();

        while self.src.len()-1 > self.pos {
            let c = self.src[self.pos];

            match c {
                '{' => tokens.push(Tok::new(TokKind::LBrace, "{")),
                '}' => tokens.push(Tok::new(TokKind::RBrace, "}")),
                ':' => tokens.push(Tok::new(TokKind::Colon, ":")),
                ',' => tokens.push(Tok::new(TokKind::Comma, ",")),
                
                '\"' => {
                    let mut buf = String::new();
                    self.advance();

                    while self.src[self.pos] != '\"' {
                        buf.push(self.src[self.pos]);
                        self.advance();
                    }

                    tokens.push(Tok::new(TokKind::Identifier, &buf[..]));
                }

                _ => (),
            }

            self.advance();
        }

        tokens.remove(0);
        return tokens
    }

    pub fn advance(&mut self) {
        self.pos = self.next;
        self.next += 1;
    }
}

/* Parser */
#[derive(Debug)]
pub struct Parser {
    pub iden: String,
    pub data: String,
}

impl Parser {
    pub fn new(i: &str, d: &str) -> Self {
        Self {
            iden: i.to_string(),
            data: d.to_string(),
        }
    }

    pub fn parse(vt: Vec<Tok>) -> Vec<Parser> {
        let mut p = Vec::<Parser>::new();
        let mut line_count: usize = 1;

        if vt[0].kind != TokKind::LBrace {
            println!("Error when parsing config : Missing opening parenthesis : Line {}", line_count);
            std::process::exit(1);
        }

        if vt[vt.len()-1].kind != TokKind::RBrace {
            println!("Error when parsing config : Missing closing parenthesis : At the end");
            std::process::exit(1);
        }

        line_count += 1;
        let mut i: usize = 1;

        while i < vt.len()-1 {
            if vt[i].kind != TokKind::Identifier {
                println!("Error when parsing config : First element is not an identifier : Line {}", line_count);
                std::process::exit(1);
            }

            if vt[i+1].kind != TokKind::Colon {
                println!("Error when parsing config : Missing colon : Line {}", line_count);
                std::process::exit(1);
            }

            if vt[i+2].kind != TokKind::Identifier {
                println!("Error when parsing config : Invlaid data given : Line {}", line_count);
                std::process::exit(1);
            }

            if vt[i+3].kind != TokKind::Comma {
                println!("Error when parsing config : Missing comma : Line {}", line_count);
                std::process::exit(1);
            }

            line_count += 1;
            i += 4;

            p.push(Parser::new(&vt[i-4].lit[..], &vt[i-2].lit[..]));
        }

        return p
    }
}
