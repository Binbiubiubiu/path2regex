macro_rules! lex_token_kind {
    ($($ty:tt $name:tt)+) => {
        #[derive(PartialEq,Eq,Copy,Clone)]
        pub(crate) enum LexTokenKind {
            $($ty,)+
        }

        impl std::fmt::Display for LexTokenKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(LexTokenKind::$ty => $name,)+
                };
                f.write_str(name)
            }
        }

        impl std::fmt::Debug for LexTokenKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self, f)
            }
        }
    };
}

lex_token_kind! {
    Open "OPEN"
    Close "CLOSE"
    Pattern "PATTERN"
    Name "NAME"
    Char "CHAR"
    EscapedChar "ESCAPEDCHAR"
    Modifier "MODIFIER"
    End "END"
}

///
pub(crate) struct LexToken<'a> {
    pub(crate) kind: LexTokenKind,
    pub(crate) index: usize,
    pub(crate) value: &'a str,
}

impl<'a> std::fmt::Display for LexToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("type", &self.kind)
            .field("index", &self.index)
            .field("value", &self.value)
            .finish()
    }
}

impl<'a> std::fmt::Debug for LexToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

///
#[derive(Eq, PartialEq, Clone)]
pub struct Key {
    ///
    pub name: String,
    ///
    pub prefix: String,
    ///
    pub suffix: String,
    ///
    pub pattern: String,
    ///
    pub modifier: String,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("name", &self.name)
            .field("prefix", &self.prefix)
            .field("suffix", &self.suffix)
            .field("pattern", &self.pattern)
            .field("modifier", &self.modifier)
            .finish()
    }
}

/// Token
#[derive(Clone)]
pub enum Token {
    ///
    Static(String),
    ///
    Key(Key),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Static(s) => f.write_str(s),
            Token::Key(Key {
                name,
                prefix,
                suffix,
                pattern,
                modifier,
            }) => f
                .debug_struct("Token")
                .field("name", name)
                .field("prefix", prefix)
                .field("suffix", suffix)
                .field("pattern", pattern)
                .field("modifier", modifier)
                .finish(),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
