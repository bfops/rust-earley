use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use grammar;

mod entry {
  use std::hash::Hash;
  use std::rc::Rc;

  use grammar;

  /// An entry in one of the states of an earley parser.
  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub struct T<Nonterminal, Terminal> {
    pub rule       : Rc<grammar::Rule<Nonterminal, Terminal>>,
    pub cur_pos    : usize,
    pub started_at : usize,
  }

  pub fn new<Nonterminal, Terminal>(rule: Rc<grammar::Rule<Nonterminal, Terminal>>, started_at: usize) -> T<Nonterminal, Terminal> {
    T {
      rule       : rule.clone(),
      cur_pos    : 0,
      started_at : started_at,
    }
  }

  impl<Nonterminal, Terminal> T<Nonterminal, Terminal> where 
    Nonterminal: Eq + Hash + Copy, 
    Terminal: Eq + Hash + Copy,
  {
    pub fn next(&self) -> Option<grammar::Sigma<Nonterminal, Terminal>> {
      if self.cur_pos < self.rule.rhs.len() {
        Some(self.rule.rhs[self.cur_pos].clone())
      } else {
        None
      }
    }

    pub fn step(&self, input: Terminal) -> Option<Self> {
      if self.next() == Some(grammar::Sigma::Terminal(input)) {
        Some(
          T {
            rule       : self.rule.clone(),
            cur_pos    : self.cur_pos + 1,
            started_at : self.started_at,
          }
        )
      } else {
        None
      }
    }

    pub fn recurse(&self, rules: &[Rc<grammar::Rule<Nonterminal, Terminal>>], cur_input_pos: usize) -> Vec<Self> {
      if let Some(grammar::Sigma::Nonterminal(nt)) = self.next() {
        rules.iter()
          .filter(|rule| rule.lhs == nt)
          .map(|rule| {
            T {
              rule: rule.clone(),
              cur_pos: 0,
              started_at: cur_input_pos,
            }
          })
          .collect()
      } else {
        vec!()
      }
    }

    pub fn reduce(&self, k: &[super::T<Nonterminal, Terminal>]) -> Vec<Self> {
      if let None = self.next() {
        k[self.started_at].iter()
          .filter(|entry| entry.next() == Some(grammar::nt(self.rule.lhs)))
          .map(|entry|
            T {
              rule       : entry.rule.clone(),
              cur_pos    : entry.cur_pos + 1,
              started_at : entry.started_at,
            }
          )
          .collect()
      } else {
        vec!()
      }
    }
  }
}

pub type T<Nonterminal, Terminal> = HashSet<entry::T<Nonterminal, Terminal>>;

fn close_entry<Nonterminal, Terminal>(
  rules: &[Rc<grammar::Rule<Nonterminal, Terminal>>], 
  ks: &[T<Nonterminal, Terminal>],
  k: &mut T<Nonterminal, Terminal>,
  entry: entry::T<Nonterminal, Terminal>,
) where
  Nonterminal: Eq + Hash + Copy,
  Terminal: Eq + Hash + Copy,
{
  for entry in entry.recurse(rules, ks.len()) {
    if k.insert(entry.clone()) {
      close_entry(rules, ks, k, entry);
    }
  }
  for entry in entry.reduce(ks) {
    if k.insert(entry.clone()) {
      close_entry(rules, ks, k, entry);
    }
  }
}

fn close<Nonterminal, Terminal>(
  rules: &[Rc<grammar::Rule<Nonterminal, Terminal>>], 
  ks: &[T<Nonterminal, Terminal>], 
  k: &mut T<Nonterminal, Terminal>,
) where
  Nonterminal: Eq + Hash + Copy,
  Terminal: Eq + Hash + Copy,
{
  let entries: Vec<_> = k.iter().cloned().collect();
  for entry in entries {
    close_entry(rules, ks, k, entry);
  }
}

pub fn run<Nonterminal, Terminal>(
  grammar: &grammar::T<Nonterminal, Terminal>, 
  input: &[Terminal],
) -> bool where
  Nonterminal: Eq + Hash + Copy,
  Terminal: Eq + Hash + Copy,
{
  let mut k: Vec<T<_, _>> = vec!();
  {
    let mut k0 = HashSet::new();
    k0.extend(
      grammar.rules.iter()
        .filter(|rule| rule.lhs == grammar.start)
        .map(|rule| entry::new(rule.clone(), 0))
    );
    close(&grammar.rules, &k, &mut k0);
    k.push(k0);
  }

  while k.len() <= input.len() {
    let n = k.len() - 1;
    let input = input[n];
    let mut kn = HashSet::new();
    {
      let kp = &mut k[n];
      kn.extend(kp.iter().filter_map(|p| p.step(input)));
    }
    close(&grammar.rules, &k, &mut kn);

    k.push(kn);
  }

  k[k.len() - 1].iter()
    .any(|entry| {
      entry.next().is_none()          &&
      entry.rule.lhs == grammar.start &&
      entry.started_at == 0
    })
}
