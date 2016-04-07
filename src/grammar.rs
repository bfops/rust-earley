use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sigma<Nonterminal, Terminal> {
  Nonterminal(Nonterminal),
  Terminal(Terminal),
}

pub fn nt<Nonterminal, Terminal>(x: Nonterminal) -> Sigma<Nonterminal, Terminal> {
  Sigma::Nonterminal(x)
}

pub fn tm<Nonterminal, Terminal>(x: Terminal) -> Sigma<Nonterminal, Terminal> {
  Sigma::Terminal(x)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule<Nonterminal, Terminal> {
  pub lhs: Nonterminal,
  pub rhs: Vec<Sigma<Nonterminal, Terminal>>,
}

impl<Nonterminal, Terminal> Rule<Nonterminal, Terminal> {
  pub fn new(lhs: Nonterminal, rhs: Vec<Sigma<Nonterminal, Terminal>>) -> Rule<Nonterminal, Terminal> {
    Rule {
      lhs: lhs,
      rhs: rhs,
    }
  }
}

pub struct T<Nonterminal, Terminal> {
  pub rules: Vec<Rc<Rule<Nonterminal, Terminal>>>,
  pub start: Nonterminal,
}
