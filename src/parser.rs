use crate::ast::*;
use crate::lexer::{Token, TokenType};

pub fn parsing(tokens: &Vec<Token>) -> Prog {
  let mut parser = Parser::new(tokens.to_vec());
  parser.prog();
  if let Some(prog) = parser.prog {
    return prog;
  } else {
    panic!("Error in parsing");
  }
}

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
  prog: Option<Prog>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      pos: 0,
      prog: None,
    }
  }

  pub fn bad_token(&self, msg: &str) -> ! {
    panic!("{}", msg);
  }

  fn expect(&mut self, ty: TokenType) {
    let t = &self.tokens[self.pos];
    if t.ty != ty {
      self.bad_token(&format!("{:?} expected, but got {:?}", ty, t.ty));
    }
    self.pos += 1;
  }

  fn consume(&mut self, ty: TokenType) -> bool {
    let t = &self.tokens[self.pos];
    if t.ty != ty {
      return false;
    }
    self.pos += 1;
    true
  }

  //<factor> ::= "(" <exp> ")" | <unary_op> <factor> | <int> | <id>
  fn factor(&mut self) -> Expr {
    let t = &self.tokens[self.pos];
    self.pos += 1;

    match &t.ty {
      TokenType::Sub => Expr::Unary(UnaryOp::Neg, Box::new(self.factor())),
      TokenType::BNot => Expr::Unary(UnaryOp::BNot, Box::new(self.factor())),
      TokenType::LNot => Expr::Unary(UnaryOp::LNot, Box::new(self.factor())),
      TokenType::Num(val) => Expr::Int(*val),
      TokenType::Ident(name) => Expr::Var(name.clone()),
      TokenType::LeftParen => {
        let e = self.expr();
        self.expect(TokenType::RightParen);
        e
      }
      _ => self.bad_token(&format!("number expected, but got {:?}", t.ty)),
    }
  }

  //<term> ::= <factor> { ("*" | "/") <factor> }
  fn term(&mut self) -> Expr {
    let mut left = self.factor();
    loop {
      if self.consume(TokenType::Mul) {
        left = Expr::Binary(BinaryOp::Mul, Box::new(left), Box::new(self.factor()));
      } else if self.consume(TokenType::Div) {
        left = Expr::Binary(BinaryOp::Div, Box::new(left), Box::new(self.factor()));
      } else {
        return left;
      }
    }
  }

  //<addsub-exp> ::= <term> { ("+" | "-") <term> }
  fn addsub_expr(&mut self) -> Expr {
    let mut left = self.term();
    loop {
      if self.consume(TokenType::Add) {
        left = Expr::Binary(BinaryOp::Add, Box::new(left), Box::new(self.term()));
      } else if self.consume(TokenType::Sub) {
        left = Expr::Binary(BinaryOp::Sub, Box::new(left), Box::new(self.term()));
      } else {
        return left;
      }
    }
  }

  //<relational-exp> ::= <addsub_exp> { ("<" | ">" | "<=" | ">=") <additive-exp> }
  fn relational_expr(&mut self) -> Expr {
    let mut left = self.addsub_expr();
    loop {
      if self.consume(TokenType::Gt) {
        left = Expr::Binary(BinaryOp::Gt, Box::new(left), Box::new(self.addsub_expr()));
      } else if self.consume(TokenType::Ge) {
        left = Expr::Binary(BinaryOp::Ge, Box::new(left), Box::new(self.addsub_expr()));
      } else if self.consume(TokenType::Lt) {
        left = Expr::Binary(BinaryOp::Lt, Box::new(left), Box::new(self.addsub_expr()));
      } else if self.consume(TokenType::Le) {
        left = Expr::Binary(BinaryOp::Le, Box::new(left), Box::new(self.addsub_expr()));
      } else {
        return left;
      }
    }
  }

  //<equality-exp> ::= <relational-exp> { ("!=" | "==") <relational-exp> }
  fn equality_expr(&mut self) -> Expr {
    let mut left = self.relational_expr();
    loop {
      if self.consume(TokenType::Eq) {
        left = Expr::Binary(
          BinaryOp::Eq,
          Box::new(left),
          Box::new(self.relational_expr()),
        );
      } else if self.consume(TokenType::Ne) {
        left = Expr::Binary(
          BinaryOp::Ne,
          Box::new(left),
          Box::new(self.relational_expr()),
        );
      } else {
        return left;
      }
    }
  }

  //<logical-and-exp> ::= <equality-exp> { "&&" <equality-exp> }
  fn logical_and_expr(&mut self) -> Expr {
    let mut left = self.equality_expr();
    loop {
      if self.consume(TokenType::And) {
        left = Expr::Binary(
          BinaryOp::And,
          Box::new(left),
          Box::new(self.equality_expr()),
        );
      } else {
        return left;
      }
    }
  }

  //<logical-or-exp> ::= <logical-and-exp> { "||" <logical-and-exp> }
  fn logical_or_expr(&mut self) -> Expr {
    let mut left = self.logical_and_expr();
    loop {
      if self.consume(TokenType::Or) {
        left = Expr::Binary(
          BinaryOp::Or,
          Box::new(left),
          Box::new(self.logical_and_expr()),
        );
      } else {
        return left;
      }
    }
  }

  //<conditional-exp> ::= <logical-or-exp> [ "?" <exp> ":" <conditional-exp> ]
  fn conditional_expr(&mut self) -> Expr {
    let lor = self.logical_or_expr();
    let t = self.tokens[self.pos].clone();
    if t.ty == TokenType::Question {
      self.pos += 1;
      let e = self.expr();
      self.expect(TokenType::Colon);
      let ne = self.conditional_expr();
      return Expr::Condition(Box::new(lor), Box::new(e), Box::new(ne));
    } else {
      return lor;
    }
  }

  //<exp> ::= <id> "=" <exp> |  <conditional-exp>
  fn expr(&mut self) -> Expr {
    let t = self.tokens[self.pos].clone();
    match &t.ty {
      TokenType::Ident(var) => {
        let n = self.tokens[self.pos + 1].clone();
        if n.ty == TokenType::Assign {
          let nvar = var.clone();
          self.pos += 2;
          return Expr::Assign(nvar, Box::new(self.expr()));
        } else {
          return self.conditional_expr();
        }
      }
      _ => {
        return self.conditional_expr();
      }
    }
  }

  // <statement> ::= "return" <exp> ";"
  // | <exp> ";"
  // | "int" <id> [ = <exp> ] ";"
  // | "if" "(" <exp> ")" <statement> [ "else" <statement> ]
  fn stmt(&mut self) -> Stmt {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Return => {
        self.pos += 1;
        let e = Stmt::Ret(self.expr());
        self.expect(TokenType::Semicolon);
        return e;
      }
      TokenType::Int => {
        self.pos += 1;
        let id = &self.tokens[self.pos];
        if let TokenType::Ident(name) = id.ty.clone() {
          self.pos += 1;
          let n = &self.tokens[self.pos];
          if n.ty == TokenType::Semicolon {
            self.pos += 1;
            return Stmt::Def(name.clone(), None);
          }
          self.expect(TokenType::Assign);
          let e = self.expr();
          self.expect(TokenType::Semicolon);
          return Stmt::Def(name.clone(), Some(e));
        } else {
          self.bad_token("Ident expected");
        }
      }
      TokenType::Ident(_) | TokenType::Num(_) => {
        let e = Stmt::Expr(self.expr());
        self.expect(TokenType::Semicolon);
        return e;
      }
      TokenType::If => {
        self.pos += 1;
        self.expect(TokenType::LeftParen);
        let e = self.expr();
        self.expect(TokenType::RightParen);
        let tst = self.stmt();
        let n = &self.tokens[self.pos];
        if n.ty == TokenType::Else {
          self.pos += 1;
          let fst = self.stmt();
          return Stmt::If(e, Box::new(tst), Some(Box::new(fst)));
        } else {
          return Stmt::If(e, Box::new(tst), None);
        }
      }
      _ => {
        self.bad_token(&format!("stmt() FUN: got {:?} --- ", &t.ty));
      }
    }
  }

  //statements ::= { <statement> }
  fn stmts(&mut self) -> Vec<Stmt> {
    let mut stmts: Vec<Stmt> = vec![];
    loop {
      if self.tokens.len() == self.pos + 1 {
        return stmts;
      } else {
        stmts.push(self.stmt());
      }
    }
  }

  //<function> ::= "int" <id> "(" ")" "{" <statements> "}"
  fn func(&mut self) -> Func {
    self.expect(TokenType::Int);
    self.expect(TokenType::Ident("main".to_string()));
    self.expect(TokenType::LeftParen);
    self.expect(TokenType::RightParen);
    self.expect(TokenType::LeftBrace);
    let body = self.stmts();
    self.expect(TokenType::RightBrace);

    Func {
      name: "main".to_string(),
      stmts: body,
    }
  }

  //fn prog(&mut self) -> Option<Prog> {

  fn prog(&mut self) {
    // Function
    self.prog = Some(Prog { func: self.func() });
    //   self.prog
  }
}
