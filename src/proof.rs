use crate::language::*;
use std::collections::HashSet;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Sequent {
    pub antecedent: Vec<Formula>,
    pub succedent: Vec<Formula>,
}

impl Sequent {
    pub fn ant_first(&self) -> &Formula {
        &self.antecedent[0]
    }

    pub fn suc_last(&self) -> &Formula {
        &self.succedent.last().unwrap()
    }

    pub fn ant_but_first(&self) -> &[Formula] {
        &self.antecedent[1..]
    }

    pub fn suc_but_last(&self) -> &[Formula] {
        self.succedent.split_last().unwrap().1
    }

    pub fn get_subformulas(&self) -> HashSet<Formula> {
        [self.antecedent.clone(), self.succedent.clone()]
            .concat()
            .iter()
            .flat_map(|f| f.get_subformulas())
            .collect()
    }
}

impl Display for Sequent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} ⇒  {}",
            self.antecedent
                .iter()
                .map(|fml| format!("{}", fml))
                .collect::<Vec<_>>()
                .join(", "),
            self.succedent
                .iter()
                .map(|fml| format!("{}", fml))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Debug for Sequent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} ⇒  {}",
            self.antecedent
                .iter()
                .map(|fml| format!("{}", fml))
                .collect::<Vec<_>>()
                .join(","),
            self.succedent
                .iter()
                .map(|fml| format!("{}", fml))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

macro_rules! sequent{
    ($($ant: expr),* => $($suc: expr),*) => { Sequent{
        antecedent: vec![$($ant),*],
        succedent: vec![$($suc),*]
    }};
}

#[derive(Debug, Clone)]
pub enum LK {
    Axiom(Sequent),
    WeakeningLeft(Box<LK>, Sequent),
    WeakeningRight(Box<LK>, Sequent),
    ContractionLeft(Box<LK>, Sequent),
    ContractionRight(Box<LK>, Sequent),
    ExchangeLeft(Box<LK>, Sequent),
    ExchangeRight(Box<LK>, Sequent),
    AndLeft1(Box<LK>, Sequent),
    AndLeft2(Box<LK>, Sequent),
    AndRight(Box<[LK; 2]>, Sequent),
    OrLeft(Box<[LK; 2]>, Sequent),
    OrRight1(Box<LK>, Sequent),
    OrRight2(Box<LK>, Sequent),
    ImpliesLeft(Box<[LK; 2]>, Sequent),
    ImpliesRight(Box<LK>, Sequent),
    NotLeft(Box<LK>, Sequent),
    NotRight(Box<LK>, Sequent),
    ForallLeft(Box<LK>, Sequent),
    ForallRight(Box<LK>, Sequent),
    ExistsLeft(Box<LK>, Sequent),
    ExistsRight(Box<LK>, Sequent),
    Cut(Box<[LK; 2]>, Sequent),
}

impl LK {
    pub fn last(&self) -> &Sequent {
        use LK::*;
        match self {
            Axiom(s) => s,
            WeakeningLeft(_, s)
            | WeakeningRight(_, s)
            | ContractionLeft(_, s)
            | ContractionRight(_, s)
            | ExchangeLeft(_, s)
            | ExchangeRight(_, s)
            | AndLeft1(_, s)
            | AndLeft2(_, s)
            | AndRight(_, s)
            | OrLeft(_, s)
            | OrRight1(_, s)
            | OrRight2(_, s)
            | ImpliesLeft(_, s)
            | ImpliesRight(_, s)
            | NotLeft(_, s)
            | NotRight(_, s)
            | ForallLeft(_, s)
            | ForallRight(_, s)
            | ExistsLeft(_, s)
            | ExistsRight(_, s)
            | Cut(_, s) => s,
        }
    }

    fn _get_prefix_spaces(s: String) -> u32 {
        let s = s.split('\n').last().unwrap();
        let mut len = 0;
        for c in s.chars() {
            if c == ' ' {
                len += 1;
            } else {
                break;
            }
        }
        len
    }

    fn _get_suffix_spaces(s: String) -> u32 {
        let s = s.split('\n').last().unwrap();
        let mut len = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                len += 1;
            } else {
                break;
            }
        }
        len
    }

    fn _last_line_len(s: String) -> u32 {
        let s = s.split('\n').last().unwrap();
        (s.chars().count() as i32 - LK::_get_prefix_spaces(s.into()) as i32) as u32
    }

    fn _get_label(&self) -> String {
        use LK::*;
        match self {
            Axiom(_) => "(ax)".to_string(),
            WeakeningLeft(_, _) => "(wL)".to_string(),
            WeakeningRight(_, _) => "(wR)".to_string(),
            ContractionLeft(_, _) => "(cL)".to_string(),
            ContractionRight(_, _) => "(cR)".to_string(),
            ExchangeLeft(_, _) => "(xL)".to_string(),
            ExchangeRight(_, _) => "(xR)".to_string(),
            AndLeft1(_, _) => "(∧L1)".to_string(),
            AndLeft2(_, _) => "(∧L2)".to_string(),
            AndRight(_, _) => "(∧R)".to_string(),
            OrLeft(_, _) => "(∨L)".to_string(),
            OrRight1(_, _) => "(∨R1)".to_string(),
            OrRight2(_, _) => "(∨R2)".to_string(),
            ImpliesLeft(_, _) => "(→L)".to_string(),
            ImpliesRight(_, _) => "(→R)".to_string(),
            NotLeft(_, _) => "(¬L)".to_string(),
            NotRight(_, _) => "(¬R)".to_string(),
            ForallLeft(_, _) => "(∀L)".to_string(),
            ForallRight(_, _) => "(∀R)".to_string(),
            ExistsLeft(_, _) => "(∃L)".to_string(),
            ExistsRight(_, _) => "(∃R)".to_string(),
            Cut(_, _) => "(Cut)".to_string(),
        }
    }

    fn _join_sequent_str(
        &self,
        parent_str: String,
        sequent_str: String,
        parent_body_prefix: u32,
        parent_body_len: u32,
    ) -> String {
        let mut parent_str = parent_str;
        let mut sequent_str = sequent_str;
        let sequent_len = sequent_str.chars().count();
        let mut offset =
            (parent_body_len as i32 - sequent_len as i32) / 2 + parent_body_prefix as i32;
        if offset > 0 {
            sequent_str = (0..offset).map(|_| " ").collect::<String>() + &sequent_str;
        } else {
            parent_str = parent_str
                .split("\n")
                .map(|l| (0..-offset).map(|_| " ").collect::<String>() + l)
                .collect::<Vec<_>>()
                .join("\n");
            offset = 0;
        }
        let sep_line = if sequent_len > parent_body_len as usize {
            (0..offset).map(|_| " ").collect::<String>()
                + &(0..sequent_len + 1).map(|_| "-").collect::<String>()
                + &self._get_label()
        } else {
            (0..parent_body_prefix).map(|_| " ").collect::<String>()
                + &(0..parent_body_len + 1).map(|_| "-").collect::<String>()
                + &self._get_label()
        };
        sequent_str = parent_str + "\n" + &sep_line + "\n" + &sequent_str;
        let max_len = sequent_str
            .split("\n")
            .map(|l| l.chars().count())
            .fold(0, |m, v| m.max(v));
        sequent_str = sequent_str
            .split("\n")
            .map(|l| {
                l.to_string()
                    + &(0..(max_len - l.chars().count()))
                        .map(|_| " ")
                        .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        sequent_str
    }

    pub fn to_string(&self) -> String {
        match self {
            LK::Axiom(s) => {
                format!("{}", s)
            }
            LK::WeakeningLeft(parent, sequent)
            | LK::WeakeningRight(parent, sequent)
            | LK::ContractionLeft(parent, sequent)
            | LK::ContractionRight(parent, sequent)
            | LK::ExchangeLeft(parent, sequent)
            | LK::ExchangeRight(parent, sequent)
            | LK::AndLeft1(parent, sequent)
            | LK::AndLeft2(parent, sequent)
            | LK::OrRight1(parent, sequent)
            | LK::OrRight2(parent, sequent)
            | LK::ImpliesRight(parent, sequent)
            | LK::NotLeft(parent, sequent)
            | LK::NotRight(parent, sequent)
            | LK::ForallLeft(parent, sequent)
            | LK::ForallRight(parent, sequent)
            | LK::ExistsLeft(parent, sequent)
            | LK::ExistsRight(parent, sequent) => {
                let parent_str = parent.to_string();
                let parent_len = parent_str.split("\n").last().unwrap().chars().count();
                let prefix_spaces = LK::_get_prefix_spaces(parent_str.clone());
                let suffix_spaces = LK::_get_suffix_spaces(parent_str.clone());
                let parent_body_len = parent_len - prefix_spaces as usize - suffix_spaces as usize;
                let sequent_str = format!("{}", sequent);
                self._join_sequent_str(
                    parent_str,
                    sequent_str,
                    prefix_spaces,
                    parent_body_len as u32,
                )
            }
            LK::AndRight(premises, sequent)
            | LK::OrLeft(premises, sequent)
            | LK::ImpliesLeft(premises, sequent)
            | LK::Cut(premises, sequent) => {
                let [lhs, rhs] = &**premises;
                let mut left_str = lhs.to_string();
                let mut right_str = rhs.to_string();
                let prefix_spaces = LK::_get_prefix_spaces(left_str.clone());
                let suffix_spaces = LK::_get_suffix_spaces(right_str.clone());
                let mut left_lines = left_str.split("\n").collect::<Vec<_>>().len();
                let right_lines = right_str.split("\n").collect::<Vec<_>>().len();
                if left_lines < right_lines {
                    left_str = (0..right_lines - left_lines)
                        .map(|_| "\n")
                        .collect::<String>()
                        + &left_str;
                    left_lines = right_lines;
                } else {
                    right_str = (0..left_lines - right_lines)
                        .map(|_| "\n")
                        .collect::<String>()
                        + &right_str;
                }
                let lefts = left_str.split("\n").collect::<Vec<_>>();
                let rights = right_str.split("\n").collect::<Vec<_>>();
                let parent_str = (0..left_lines)
                    .map(|l| lefts[l].to_string() + "    " + rights[l])
                    .collect::<Vec<_>>()
                    .join("\n");
                let parent_body_len = LK::_last_line_len(parent_str.clone()) as i32
                    - prefix_spaces as i32
                    - suffix_spaces as i32;
                let sequent_str = format!("{}", sequent);
                self._join_sequent_str(
                    parent_str,
                    sequent_str,
                    prefix_spaces,
                    parent_body_len as u32,
                )
            }
        }
    }
}

impl Display for LK {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub trait Proof {
    fn is_valid_inference(&self) -> bool;
}

impl Proof for LK {
    fn is_valid_inference(&self) -> bool {
        match self {
            LK::Axiom(conclusion) => {
                (conclusion.antecedent == conclusion.succedent && conclusion.antecedent.len() > 0)
                    || (conclusion.antecedent.is_empty()
                        && conclusion.succedent.len() == 1
                        && match conclusion.suc_last() {
                            Formula::Equal(s, t) => s == t,
                            _ => false,
                        })
            }
            LK::WeakeningLeft(premise, conclusion) => {
                premise.last().antecedent == conclusion.ant_but_first()
                    && premise.last().succedent == conclusion.succedent
            }
            LK::WeakeningRight(premise, conclusion) => {
                premise.last().antecedent == conclusion.antecedent
                    && premise.last().succedent == conclusion.suc_but_last()
            }
            LK::ContractionLeft(premise, conclusion) => {
                premise.last().ant_first() == &premise.last().antecedent[1]
                    && premise.last().ant_but_first() == &conclusion.antecedent[..]
                    && premise.last().succedent == conclusion.succedent
            }
            LK::ContractionRight(premise, conclusion) => {
                premise.last().antecedent == conclusion.antecedent
                    && premise.last().suc_but_last().last().unwrap() == premise.last().suc_last()
                    && premise.last().suc_but_last() == conclusion.succedent
            }
            LK::ExchangeLeft(premise, conclusion) => {
                if premise.last().succedent == conclusion.succedent {
                    let mut valid = false;
                    for i in 0..premise.last().antecedent.len() - 1 {
                        if premise.last().antecedent[..i] == conclusion.antecedent[..i]
                            && premise.last().antecedent[i + 2..] == conclusion.antecedent[i + 2..]
                            && premise.last().antecedent[i] == conclusion.antecedent[i + 1]
                            && premise.last().antecedent[i + 1] == conclusion.antecedent[i]
                        {
                            valid = true;
                            break;
                        }
                    }
                    valid
                } else {
                    false
                }
            }
            LK::ExchangeRight(premise, conclusion) => {
                if premise.last().antecedent == conclusion.antecedent {
                    let mut valid = false;
                    for i in 0..premise.last().succedent.len() - 1 {
                        if premise.last().succedent[..i] == conclusion.succedent[..i]
                            && premise.last().succedent[i + 2..] == conclusion.succedent[i + 2..]
                            && premise.last().succedent[i] == conclusion.succedent[i + 1]
                            && premise.last().succedent[i + 1] == conclusion.succedent[i]
                        {
                            valid = true;
                            break;
                        }
                    }
                    valid
                } else {
                    false
                }
            }
            LK::AndLeft1(premise, conclusion) => {
                premise.last().ant_but_first() == conclusion.ant_but_first()
                    && premise.last().succedent == conclusion.succedent
                    && if let Formula::And(fml, _) = &conclusion.ant_first() {
                        &**fml == premise.last().ant_first()
                    } else {
                        false
                    }
            }
            LK::AndLeft2(premise, conclusion) => {
                premise.last().ant_but_first() == conclusion.ant_but_first()
                    && premise.last().succedent == conclusion.succedent
                    && if let Formula::And(_, fml) = &conclusion.ant_first() {
                        &**fml == premise.last().ant_first()
                    } else {
                        false
                    }
            }
            LK::AndRight(premises, conclusion) => {
                let [lpremise, rpremise] = &**premises;
                lpremise.last().antecedent == conclusion.antecedent
                    && rpremise.last().antecedent == conclusion.antecedent
                    && lpremise.last().suc_but_last() == conclusion.suc_but_last()
                    && rpremise.last().suc_but_last() == conclusion.suc_but_last()
                    && if let Formula::And(lhs, rhs) = conclusion.suc_last() {
                        lpremise.last().suc_last() == &**lhs && rpremise.last().suc_last() == &**rhs
                    } else {
                        false
                    }
            }
            LK::OrLeft(premises, conclusion) => {
                let [lpremise, rpremise] = &**premises;
                lpremise.last().succedent == conclusion.succedent
                    && rpremise.last().succedent == conclusion.succedent
                    && lpremise.last().ant_but_first() == conclusion.ant_but_first()
                    && rpremise.last().ant_but_first() == conclusion.ant_but_first()
                    && if let Formula::Or(lhs, rhs) = &conclusion.antecedent[0] {
                        lpremise.last().antecedent[0] == **lhs
                            && rpremise.last().antecedent[0] == **rhs
                    } else {
                        false
                    }
            }
            LK::OrRight1(premise, conclusion) => {
                premise.last().antecedent == conclusion.antecedent
                    && premise.last().suc_but_last() == conclusion.suc_but_last()
                    && if let Formula::Or(fml, _) = conclusion.suc_last() {
                        &**fml == premise.last().suc_last()
                    } else {
                        false
                    }
            }
            LK::OrRight2(premise, conclusion) => {
                premise.last().antecedent == conclusion.antecedent
                    && premise.last().suc_but_last() == conclusion.suc_but_last()
                    && if let Formula::Or(_, fml) = conclusion.suc_last() {
                        &**fml == premise.last().suc_last()
                    } else {
                        false
                    }
            }
            LK::ImpliesLeft(premises, conclusion) => {
                let [lpremise, rpremise] = &**premises;
                let gamma = &lpremise.last().antecedent;
                let delta = lpremise.last().suc_but_last();
                let fml1 = &lpremise.last().suc_last();
                let fml2 = &rpremise.last().ant_first();
                let pi = rpremise.last().ant_but_first();
                let sigma = &rpremise.last().succedent;
                conclusion.ant_but_first() == &[gamma, pi].concat()[..]
                    && conclusion.succedent == [delta, sigma].concat()
                    && if let Formula::Implies(lhs, rhs) = &conclusion.ant_first() {
                        &**lhs == *fml1 && **rhs == **fml2
                    } else {
                        false
                    }
            }
            LK::ImpliesRight(premise, conclusion) => {
                premise.last().ant_but_first() == &conclusion.antecedent[..]
                    && premise.last().suc_but_last() == conclusion.suc_but_last()
                    && if let Formula::Implies(lhs, rhs) = conclusion.suc_last() {
                        &**lhs == premise.last().ant_first() && &**rhs == premise.last().suc_last()
                    } else {
                        false
                    }
            }
            LK::NotLeft(premise, conclusion) => {
                &premise.last().antecedent[..] == conclusion.ant_but_first()
                    && premise.last().suc_but_last() == conclusion.succedent
                    && if let Formula::Not(fml) = &conclusion.ant_first() {
                        &**fml == premise.last().suc_last()
                    } else {
                        false
                    }
            }
            LK::NotRight(premise, conclusion) => {
                premise.last().ant_but_first() == &conclusion.antecedent[..]
                    && premise.last().succedent == conclusion.suc_but_last()
                    && if let Formula::Not(fml) = conclusion.suc_last() {
                        &**fml == premise.last().ant_first()
                    } else {
                        false
                    }
            }
            LK::ForallLeft(premise, conclusion) => {
                premise.last().succedent == conclusion.succedent
                    && premise.last().ant_but_first() == conclusion.ant_but_first()
                    && if let Formula::Forall(var, fml) = &conclusion.ant_first() {
                        if !fml.get_bound_vars().contains(var) {
                            let mut valid = false;
                            for term in premise.last().ant_first().get_subterms() {
                                if fml.is_substitutible(var.clone(), term.clone()) {
                                    let tfml = fml.substitute(var.clone(), term);
                                    if &tfml == premise.last().ant_first() {
                                        valid = true;
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            valid
                        } else {
                            false
                        }
                    } else {
                        false
                    }
            }
            LK::ForallRight(premise, conclusion) => {
                premise.last().antecedent == conclusion.antecedent
                    && premise.last().suc_but_last() == conclusion.suc_but_last()
                    && if let Formula::Forall(term, fml) = &conclusion.suc_last() {
                        let mut valid = false;
                        for var in premise.last().suc_last().get_free_vars() {
                            if fml.is_substitutible(term.clone(), var.clone()) {
                                let tfml = fml.substitute(term.clone(), var.clone());
                                if &tfml == premise.last().suc_last() {
                                    if !premise
                                        .last()
                                        .antecedent
                                        .iter()
                                        .flat_map(|f| f.get_free_vars())
                                        .collect::<Vec<Term>>()
                                        .contains(&var.clone())
                                        && !premise
                                            .last()
                                            .suc_but_last()
                                            .iter()
                                            .flat_map(|f| f.get_free_vars())
                                            .collect::<Vec<Term>>()
                                            .contains(&var.clone())
                                    {
                                        valid = true;
                                        break;
                                    }
                                }
                            }
                        }
                        valid
                    } else {
                        false
                    }
            }
            LK::ExistsRight(premise, conclusion) => {
                premise.last().antecedent == conclusion.antecedent
                    && premise.last().suc_but_last() == conclusion.suc_but_last()
                    && if let Formula::Exists(Term::Var(s), fml) = &conclusion.suc_last() {
                        if !fml.get_bound_vars().contains(&var!(s)) {
                            let mut valid = false;
                            for term in premise.last().suc_last().get_subterms() {
                                if fml.is_substitutible(var!(s), term.clone()) {
                                    let tfml = fml.substitute(var!(s), term);
                                    if &tfml == premise.last().suc_last() {
                                        valid = true;
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            valid
                        } else {
                            false
                        }
                    } else {
                        false
                    }
            }
            LK::ExistsLeft(premise, conclusion) => {
                premise.last().succedent == conclusion.succedent
                    && premise.last().ant_but_first() == conclusion.ant_but_first()
                    && if let Formula::Exists(term, fml) = &conclusion.ant_first() {
                        let mut valid = false;
                        for var in premise.last().ant_first().get_free_vars() {
                            if fml.is_substitutible(term.clone(), var.clone()) {
                                let tfml = fml.substitute(term.clone(), var.clone());
                                if &tfml == premise.last().ant_first() {
                                    if !premise
                                        .last()
                                        .succedent
                                        .iter()
                                        .flat_map(|f| f.get_free_vars())
                                        .collect::<Vec<Term>>()
                                        .contains(&var.clone())
                                        && !premise
                                            .last()
                                            .ant_but_first()
                                            .iter()
                                            .flat_map(|f| f.get_free_vars())
                                            .collect::<Vec<Term>>()
                                            .contains(&var.clone())
                                    {
                                        valid = true;
                                        break;
                                    }
                                }
                            }
                        }
                        valid
                    } else {
                        false
                    }
            }
            LK::Cut(premises, conclusion) => {
                let [lpremise, rpremise] = &**premises;
                if lpremise.last().suc_last() == rpremise.last().ant_first() {
                    let gamma = &lpremise.last().antecedent;
                    let delta = lpremise.last().suc_but_last();
                    let pi = &rpremise.last().ant_but_first();
                    let sigma = &rpremise.last().succedent[..];
                    conclusion.antecedent == [gamma, delta].concat()
                        && conclusion.succedent == [pi, sigma].concat()
                } else {
                    false
                }
            }
        }
    }
}
