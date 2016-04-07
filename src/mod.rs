pub mod grammar;
pub mod earley;

#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use grammar;
  use earley;

  #[derive(PartialEq, Eq, Clone, Copy, Hash)]
  enum Nonterminal { S, T }
  #[derive(PartialEq, Eq, Clone, Copy, Hash)]
  enum Terminal { Zero, Plus, LParen, RParen }

  use self::Nonterminal::*;
  use self::Terminal::*;

  fn grammar() -> grammar::T<Nonterminal, Terminal> {
    use grammar::*;
    grammar::T {
      rules:
        vec!(
          Rc::new(Rule::new(S, vec!(nt(S), tm(Plus), nt(T)))),
          Rc::new(Rule::new(S, vec!(nt(T)))),
          Rc::new(Rule::new(T, vec!(tm(Zero)))),
          Rc::new(Rule::new(T, vec!(tm(LParen), nt(S), tm(RParen)))),
        ),
      start: Nonterminal::S,
    }
  }

  #[test]
  fn simpler_test() {
    let grammar = grammar();
    let input = vec!(Zero);

    assert!(earley::run(&grammar, &input));
  }

  #[test]
  fn simple_test() {
    let grammar = grammar();
    let input = vec!(Zero, Plus, LParen, Zero, Plus, Zero, RParen);

    assert!(earley::run(&grammar, &input));
  }

  #[test]
  fn missing_paren() {
    let grammar = grammar();
    let input = vec!(Zero, Plus, LParen, Zero, Plus, Zero);

    assert!(!earley::run(&grammar, &input));
  }

  #[test]
  fn truncated() {
    let grammar = grammar();
    let input = vec!(Zero, Plus);

    assert!(!earley::run(&grammar, &input));
  }

  #[test]
  fn too_long() {
    let grammar = grammar();
    let input = vec!(Zero, Plus, LParen, Zero, Plus, Zero, RParen, Zero);

    assert!(!earley::run(&grammar, &input));
  }
}
