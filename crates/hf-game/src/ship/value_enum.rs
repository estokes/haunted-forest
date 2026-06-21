//! `str_enum!` generates a fieldless enum plus the string/Value plumbing every
//! published status/mode enum needs: `as_str`/`parse`/`Display`, `From<Enum> for
//! Value` (a netidx String), and `FromValue`. It is exported by path (`use
//! crate::ship::value_enum::str_enum`) rather than `#[macro_use]`, so module
//! declaration order does not matter.

macro_rules! str_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident => $s:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $name {
            $($variant),+
        }

        impl $name {
            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $s,)+
                }
            }

            pub fn parse(s: &str) -> Option<Self> {
                match s {
                    $($s => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::std::convert::From<$name> for ::netidx::publisher::Value {
            fn from(v: $name) -> ::netidx::publisher::Value {
                ::netidx::publisher::Value::String(::arcstr::ArcStr::from(v.as_str()))
            }
        }

        impl ::netidx::publisher::FromValue for $name {
            fn from_value(v: ::netidx::publisher::Value) -> ::anyhow::Result<Self> {
                let s = <::arcstr::ArcStr as ::netidx::publisher::FromValue>::from_value(v)?;
                Self::parse(&s)
                    .ok_or_else(|| ::anyhow::anyhow!("unknown {} value: {}", stringify!($name), s))
            }

            fn get(v: ::netidx::publisher::Value) -> Option<Self> {
                let s = <::arcstr::ArcStr as ::netidx::publisher::FromValue>::get(v)?;
                Self::parse(&s)
            }
        }
    };
}

pub(crate) use str_enum;
