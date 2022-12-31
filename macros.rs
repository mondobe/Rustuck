
macro_rules! TagFrags {
    ($tag:expr, $($frag:expr)*) => {
        Block(vec![
            $(
                If($frag), Add($tag),
            )*
        ])
    };
}

macro_rules! Do {
    ($($instr:expr)*) => {
        Block(vec![
            $(
                $instr,
            )*
        ])
    };
}

macro_rules! routine {
    (:$name:ident= $($instr:expr)*) => {
        {
            let rule_instrs : Vec<Instruction> = vec![
                $(
                    $instr,
                )*
            ];

            Routine {
                name: stringify!($name),
                instrs: rule_instrs
            }
        }
    };
}

macro_rules! lexer {
    ($($rule:expr)*) => {
        Lexer {
            rules: &vec![
                $(
                    $rule,
                )*
            ]
        }
    };
}

macro_rules! parser {
    ($($rule:expr)*) => {
        Parser {
            rules: &vec![
                $(
                    $rule,
                )*
            ]
        }
    };
}

macro_rules! rule {
    ($($matches:expr)+ ; $($tags:expr)+) => {
        Rule {
            matches: vec![
                $(
                    $matches,
                )+
            ],
            tags: vec![
                $(
                    $tags,
                )+
            ],
            repeat: None,
            add_all: false
        }
    };

    ($($matches:expr)+ ;; $($tags:expr)+) => {
        Rule {
            matches: vec![
                $(
                    $matches,
                )+
            ],
            tags: vec![
                $(
                    $tags,
                )+
            ],
            repeat: None,
            add_all: true
        }
    };

    ($($matches:expr)+; $repeat:expr; $($tags:expr)+) => {
        Rule {
            matches: vec![
                $(
                    $matches,
                )+
            ],
            tags: vec![
                $(
                    $tags,
                )+
            ],
            repeat: Some($repeat),
            add_all: false
        }
    };

    ($($matches:expr)+; $repeat:expr;; $($tags:expr)+) => {
        Rule {
            matches: vec![
                $(
                    $matches,
                )+
            ],
            tags: vec![
                $(
                    $tags,
                )+
            ],
            repeat: Some($repeat),
            add_all: true
        }
    };
}
