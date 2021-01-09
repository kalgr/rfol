pub mod data;
pub mod parser;
pub mod tokenizer;

#[test]
fn tokenizer_works() {
    use data::Token::*;
    use tokenizer::Tokenizer;

    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize("(Vx0 (Ex1 (^ (= (a x y) (b x y)) (v (~ (p y)) q))))");
    let gt = vec![
        LParen,
        Forall,
        Symbol("x0".into()),
        LParen,
        Exists,
        Symbol("x1".into()),
        LParen,
        And,
        LParen,
        Equal,
        LParen,
        Symbol("a".into()),
        Symbol("x".into()),
        Symbol("y".into()),
        RParen,
        LParen,
        Symbol("b".into()),
        Symbol("x".into()),
        Symbol("y".into()),
        RParen,
        RParen,
        LParen,
        Or,
        LParen,
        Not,
        LParen,
        Symbol("p".into()),
        Symbol("y".into()),
        RParen,
        RParen,
        Symbol("q".into()),
        RParen,
        RParen,
        RParen,
        RParen,
    ];

    assert_eq!(gt, tokens);
}

#[test]
fn parser_works() {
    use data::Formula;
    use data::Term::*;
    use data::Token::*;
    use parser::Parser;

    let mut parser = Parser::new();
    let tokens = vec![
        LParen,
        Forall,
        Symbol("x0".into()),
        LParen,
        Exists,
        Symbol("x1".into()),
        LParen,
        And,
        LParen,
        Equal,
        LParen,
        Symbol("a".into()),
        Symbol("x".into()),
        Symbol("y".into()),
        RParen,
        LParen,
        Symbol("b".into()),
        Symbol("x".into()),
        Symbol("y".into()),
        RParen,
        RParen,
        LParen,
        Or,
        LParen,
        Not,
        LParen,
        Symbol("p".into()),
        Symbol("y".into()),
        RParen,
        RParen,
        Symbol("q".into()),
        RParen,
        RParen,
        RParen,
        RParen,
    ];
    let gt = Formula::Forall(
        Var("x0".into()),
        Box::new(Formula::Exists(
            Var("x1".into()),
            Box::new(Formula::And(
                Box::new(Formula::Equal(
                    Func("a".into(), vec![Var("x".into()), Var("y".into())]),
                    Func("b".into(), vec![Var("x".into()), Var("y".into())]),
                )),
                Box::new(Formula::Or(
                    Box::new(Formula::Not(Box::new(Formula::Pred(
                        "p".into(),
                        vec![Var("y".into())],
                    )))),
                    Box::new(Formula::Pred("q".into(), vec![])),
                )),
            )),
        )),
    );

    if let Ok(formula) = parser.parse(&tokens) {
        assert_eq!(gt, formula);
    } else {
        panic!("Parse error.");
    }
}

#[test]
fn var_group_works() {
    use data::Formula;
    use data::Term::*;
    use std::collections::HashSet;

    let formula = Formula::Forall(
        Var("x0".into()),
        Box::new(Formula::Exists(
            Var("x1".into()),
            Box::new(Formula::And(
                Box::new(Formula::Equal(
                    Func("a".into(), vec![Var("x".into()), Var("y".into())]),
                    Func("b".into(), vec![Var("x".into()), Var("y".into())]),
                )),
                Box::new(Formula::Or(
                    Box::new(Formula::Not(Box::new(Formula::Pred(
                        "p".into(),
                        vec![Var("y".into())],
                    )))),
                    Box::new(Formula::Pred("q".into(), vec![])),
                )),
            )),
        )),
    );

    let free_vars = formula.get_free_vars();
    let bound_vars = formula.get_bound_vars();

    let mut free_gt = HashSet::new();
    free_gt.insert(Var("x".into()));
    free_gt.insert(Var("y".into()));
    let mut bound_gt = HashSet::new();
    bound_gt.insert(Var("x0".into()));
    bound_gt.insert(Var("x1".into()));

    assert_eq!(free_gt, free_vars);
    assert_eq!(bound_gt, bound_vars);
}

#[test]
fn get_funcs_works() {
    use data::Formula;
    use data::NonLogicalSymbol;
    use data::Term::*;
    use std::collections::HashSet;

    let formula = Formula::Forall(
        Var("x0".into()),
        Box::new(Formula::Exists(
            Var("x1".into()),
            Box::new(Formula::And(
                Box::new(Formula::Equal(
                    Func("a".into(), vec![Var("x".into()), Var("y".into())]),
                    Func("b".into(), vec![Var("x".into()), Var("y".into())]),
                )),
                Box::new(Formula::Or(
                    Box::new(Formula::Not(Box::new(Formula::Pred(
                        "p".into(),
                        vec![Var("y".into())],
                    )))),
                    Box::new(Formula::Pred("q".into(), vec![])),
                )),
            )),
        )),
    );

    let funcs = formula.get_funcs();

    let mut gt = HashSet::new();
    gt.insert(NonLogicalSymbol {
        name: "a".into(),
        arity: 2,
    });
    gt.insert(NonLogicalSymbol {
        name: "b".into(),
        arity: 2,
    });

    assert_eq!(gt, funcs);
}

#[test]
fn get_preds_works() {
    use data::Formula;
    use data::NonLogicalSymbol;
    use data::Term::*;
    use std::collections::HashSet;

    let formula = Formula::Forall(
        Var("x0".into()),
        Box::new(Formula::Exists(
            Var("x1".into()),
            Box::new(Formula::And(
                Box::new(Formula::Equal(
                    Func("a".into(), vec![Var("x".into()), Var("y".into())]),
                    Func("b".into(), vec![Var("x".into()), Var("y".into())]),
                )),
                Box::new(Formula::Or(
                    Box::new(Formula::Not(Box::new(Formula::Pred(
                        "p".into(),
                        vec![Var("y".into())],
                    )))),
                    Box::new(Formula::Pred("q".into(), vec![])),
                )),
            )),
        )),
    );

    let preds = formula.get_preds();

    let mut gt = HashSet::new();
    gt.insert(NonLogicalSymbol {
        name: "p".into(),
        arity: 1,
    });
    gt.insert(NonLogicalSymbol {
        name: "q".into(),
        arity: 0,
    });

    assert_eq!(gt, preds);
}
