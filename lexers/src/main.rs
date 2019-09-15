use lexers::*;

fn main() {
    //ebnf
    let grammar = r#"
    expr   := expr ('+'|'-') term | term ;
    term   := term ('*'|'/') factor | factor ;
    factor := '-' factor | power ;
    power  := ufact '^' factor | ufact ;
    ufact  := ufact '!' | group ;
    group  := num | '(' expr ')' ;
    "#;

    let mut tok = EbnfTokenizer::new(grammar.chars());
    for token in tok {
        println!("{}",token);
    }


    //delimiter
    let source = vec![
        ("this  is a   test ", " ", true),
        ("just,more,tests,hi", ",", true),
        ("another, test, here,going on", " ,", true),
        ("1+2*3/5", "/+*", false),
    ];
    let target = vec![
        vec!["this", "is", "a", "test"],
        vec!["just", "more", "tests", "hi"],
        vec!["another", "test", "here", "going", "on"],
        vec!["1", "+", "2", "*", "3", "/", "5"],
    ];
    for (input, expected) in source.iter().zip(target.iter()) {
        let mut lx = DelimTokenizer::new(input.0.chars(), &input.1, input.2);

        for exp in expected.iter() {
            assert_eq!(*exp, lx.next().unwrap());
        }
        assert_eq!(lx.next(), None);
    }

    //math tokenizer

    let mut mx = MathTokenizer::new("3.4e-2 * sin(x)/(7! % -4) * max(2, x)".chars());
    use lexers::MathToken::*;
    let expect = [
        MathToken::Number(3.4e-2),
        BOp(format!("*")),
        Function(format!("sin"), 0),
        MathToken::OParen,
        Variable(format!("x")),
        MathToken::CParen,
        BOp(format!("/")),
        MathToken::OParen,
        MathToken::Number(7.0),
        UOp(format!("!")),
        BOp(format!("%")),
        UOp(format!("-")),
        MathToken::Number(4.0),
        MathToken::CParen,
        BOp(format!("*")),
        Function(format!("max"), 0),
        MathToken::OParen,
        MathToken::Number(2.0),
        Comma,
        Variable(format!("x")),
        MathToken::CParen,
    ];
    for exp_token in expect.iter() {
        let token = mx.next().unwrap();
        assert_eq!(*exp_token, token);
    }
    assert_eq!(mx.next(), None);

    let mut mx = MathTokenizer::new("x---y".chars());
    let expect = [
        Variable(format!("x")),
        BOp(format!("-")),
        UOp(format!("-")),
        UOp(format!("-")),
        Variable(format!("y")),
    ];
    for exp_token in expect.iter() {
        let token = mx.next().unwrap();
        assert_eq!(*exp_token, token);
    }
    assert_eq!(mx.next(), None);

    //lisp
    use lexers::LispToken::*;
    let inputs = vec!["(+ 3 4 5)", "(max 'a \"hello\")"];
    let expect = vec![
        vec![
            LispToken::OParen,
            Symbol(format!("+")),
            LispToken::Number(3.0),
            LispToken::Number(4.0),
            LispToken::Number(5.0),
            LispToken::CParen,
        ],
        vec![
            LispToken::OParen,
            Symbol(format!("max")),
            Quote,
            Symbol(format!("a")),
            String(format!("\"hello\"")),
            LispToken::CParen,
        ],
    ];
    for (input, expected) in inputs.iter().zip(expect.iter()) {
        let mut lx = LispTokenizer::new(input.chars());
        for exp in expected.iter() {
            assert_eq!(*exp, lx.next().unwrap());
        }
        assert_eq!(lx.next(), None);
    }
}