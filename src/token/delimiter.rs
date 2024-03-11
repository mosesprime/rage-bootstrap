macro_rules! define_delimiter {
    ($($name:ident #[$doc:meta])*) => {
        $(
            #[$doc]
            #[derive(Debug)]
            pub struct $name;
        )*

        pub enum DelimiterKind {
            $($name,)*
        }
    };
}

define_delimiter!(
    CurlyBrace /// { ...}
    SquareBracket /// [ ... ]
    Parenthesis /// ( ... )
    AngledBracket /// < ... >
);
